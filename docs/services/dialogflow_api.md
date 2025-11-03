# Dialogflow_api Service



**Resources**: 101

---

## Overview

The dialogflow_api service provides access to 101 resource types:

- [Agent](#agent) [CRUD]
- [Security_setting](#security_setting) [CRUD]
- [Location](#location) [R]
- [Tool](#tool) [CRUD]
- [Changelog](#changelog) [R]
- [Flow](#flow) [CRUD]
- [Conversation](#conversation) [RD]
- [Example](#example) [CRUD]
- [Result](#result) [R]
- [Environment](#environment) [CRUD]
- [Continuous_test_result](#continuous_test_result) [R]
- [Version](#version) [CRUD]
- [Operation](#operation) [CR]
- [Deployment](#deployment) [R]
- [Test_case](#test_case) [CRU]
- [Experiment](#experiment) [CRUD]
- [Session](#session) [C]
- [Entity_type](#entity_type) [CRUD]
- [Webhook](#webhook) [CRUD]
- [Playbook](#playbook) [CRUD]
- [Page](#page) [CRUD]
- [Intent](#intent) [CRUD]
- [Transition_route_group](#transition_route_group) [CRUD]
- [Generator](#generator) [CRUD]
- [Message](#message) [CR]
- [Stateless_suggestion](#stateless_suggestion) [C]
- [Knowledge_base](#knowledge_base) [CRUD]
- [Project](#project) [CRD]
- [Participant](#participant) [CRU]
- [Version](#version) [CRUD]
- [Conversation](#conversation) [CR]
- [Encryption_spec](#encryption_spec) [C]
- [Document](#document) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Session](#session) [CD]
- [Agent](#agent) [CRU]
- [Environment](#environment) [CRUD]
- [Intent](#intent) [CRUD]
- [Suggestion](#suggestion) [CR]
- [Context](#context) [CRUD]
- [Entitie](#entitie) [C]
- [Answer_record](#answer_record) [RU]
- [Operation](#operation) [CR]
- [Phone_number](#phone_number) [CRUD]
- [Sip_trunk](#sip_trunk) [CRUD]
- [Location](#location) [CRD]
- [Tool](#tool) [CRUD]
- [Generator](#generator) [CRUD]
- [Conversation_profile](#conversation_profile) [CRUD]
- [Evaluation](#evaluation) [CRD]
- [Knowledge_base](#knowledge_base) [CRUD]
- [Document](#document) [CRUD]
- [Encryption_spec](#encryption_spec) [C]
- [Answer_record](#answer_record) [RU]
- [Conversation](#conversation) [CR]
- [Operation](#operation) [CR]
- [Location](#location) [CRD]
- [Version](#version) [CRUD]
- [Session](#session) [CD]
- [Evaluation](#evaluation) [CRD]
- [Agent](#agent) [CRU]
- [Intent](#intent) [CRUD]
- [Conversation_dataset](#conversation_dataset) [CRD]
- [Generator](#generator) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Stateless_suggestion](#stateless_suggestion) [C]
- [Conversation_profile](#conversation_profile) [CRUD]
- [Message](#message) [R]
- [Tool](#tool) [CRUD]
- [Project](#project) [CRD]
- [Entitie](#entitie) [C]
- [Suggestion](#suggestion) [C]
- [Environment](#environment) [CRUD]
- [Context](#context) [CRUD]
- [Participant](#participant) [CRU]
- [Conversation_model](#conversation_model) [CRD]
- [Sip_trunk](#sip_trunk) [CRUD]
- [Version](#version) [CRUD]
- [Flow](#flow) [CRUD]
- [Playbook](#playbook) [CRUD]
- [Example](#example) [CRUD]
- [Continuous_test_result](#continuous_test_result) [R]
- [Intent](#intent) [CRUD]
- [Session](#session) [C]
- [Result](#result) [R]
- [Environment](#environment) [CRUD]
- [Deployment](#deployment) [R]
- [Tool](#tool) [CRUD]
- [Experiment](#experiment) [CRUD]
- [Location](#location) [R]
- [Changelog](#changelog) [R]
- [Transition_route_group](#transition_route_group) [CRUD]
- [Generator](#generator) [CRUD]
- [Security_setting](#security_setting) [CRUD]
- [Agent](#agent) [CRUD]
- [Webhook](#webhook) [CRUD]
- [Entity_type](#entity_type) [CRUD]
- [Page](#page) [CRUD]
- [Test_case](#test_case) [CRU]
- [Operation](#operation) [CR]
- [Operation](#operation) [CR]

---

## Resources


### Agent

Creates an agent in the specified location. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text_to_speech_settings` | String |  | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `avatar_uri` | String |  | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `display_name` | String |  | Required. The human-readable name of the agent, unique within the location. |
| `client_certificate_settings` | String |  | Optional. Settings for custom client certificates. |
| `enable_stackdriver_logging` | bool |  | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `locked` | bool |  | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `personalization_settings` | String |  | Optional. Settings for end user personalization. |
| `satisfies_pzi` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `speech_to_text_settings` | String |  | Speech recognition related settings. |
| `time_zone` | String |  | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `answer_feedback_settings` | String |  | Optional. Answer feedback collection settings. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `git_integration_settings` | String |  | Git integration settings for this agent. |
| `start_playbook` | String |  | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `security_settings` | String |  | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `name` | String |  | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `description` | String |  | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `enable_multi_language_training` | bool |  | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `default_language_code` | String |  | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `gen_app_builder_settings` | String |  | Gen App Builder-related agent-level settings. |
| `satisfies_pzs` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `enable_spell_correction` | bool |  | Indicates if automatic spell correction is enabled in detect intent requests. |
| `start_flow` | String |  | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `bigquery_export_settings` | String |  | Optional. The BigQuery export settings for this agent. The conversation data will be exported to BigQuery tables if it is enabled. By default, BigQuery export settings will not be exported with agent. You need to set include_bigquery_export_settings to include it in the exported agent. |
| `supported_language_codes` | Vec<String> |  | The list of all languages supported by the agent (except for the `default_language_code`). |
| `parent` | String | ✅ | Required. The location to create a agent for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `text_to_speech_settings` | String | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `avatar_uri` | String | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `display_name` | String | Required. The human-readable name of the agent, unique within the location. |
| `client_certificate_settings` | String | Optional. Settings for custom client certificates. |
| `enable_stackdriver_logging` | bool | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `locked` | bool | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `personalization_settings` | String | Optional. Settings for end user personalization. |
| `satisfies_pzi` | bool | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `speech_to_text_settings` | String | Speech recognition related settings. |
| `time_zone` | String | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `answer_feedback_settings` | String | Optional. Answer feedback collection settings. |
| `advanced_settings` | String | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `git_integration_settings` | String | Git integration settings for this agent. |
| `start_playbook` | String | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `security_settings` | String | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `name` | String | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `description` | String | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `enable_multi_language_training` | bool | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `default_language_code` | String | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `gen_app_builder_settings` | String | Gen App Builder-related agent-level settings. |
| `satisfies_pzs` | bool | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `enable_spell_correction` | bool | Indicates if automatic spell correction is enabled in detect intent requests. |
| `start_flow` | String | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `bigquery_export_settings` | String | Optional. The BigQuery export settings for this agent. The conversation data will be exported to BigQuery tables if it is enabled. By default, BigQuery export settings will not be exported with agent. You need to set include_bigquery_export_settings to include it in the exported agent. |
| `supported_language_codes` | Vec<String> | The list of all languages supported by the agent (except for the `default_language_code`). |


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
agent_text_to_speech_settings = agent.text_to_speech_settings
agent_avatar_uri = agent.avatar_uri
agent_display_name = agent.display_name
agent_client_certificate_settings = agent.client_certificate_settings
agent_enable_stackdriver_logging = agent.enable_stackdriver_logging
agent_locked = agent.locked
agent_personalization_settings = agent.personalization_settings
agent_satisfies_pzi = agent.satisfies_pzi
agent_speech_to_text_settings = agent.speech_to_text_settings
agent_time_zone = agent.time_zone
agent_answer_feedback_settings = agent.answer_feedback_settings
agent_advanced_settings = agent.advanced_settings
agent_git_integration_settings = agent.git_integration_settings
agent_start_playbook = agent.start_playbook
agent_security_settings = agent.security_settings
agent_name = agent.name
agent_description = agent.description
agent_enable_multi_language_training = agent.enable_multi_language_training
agent_default_language_code = agent.default_language_code
agent_gen_app_builder_settings = agent.gen_app_builder_settings
agent_satisfies_pzs = agent.satisfies_pzs
agent_enable_spell_correction = agent.enable_spell_correction
agent_start_flow = agent.start_flow
agent_bigquery_export_settings = agent.bigquery_export_settings
agent_supported_language_codes = agent.supported_language_codes
```

---


### Security_setting

Create security settings in the specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redaction_scope` | String |  | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_strategy` | String |  | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `display_name` | String |  | Required. The human-readable name of the security settings, unique within the location. |
| `retention_window_days` | i64 |  | Retains data in interaction logging for the specified number of days. This does not apply to Cloud logging, which is owned by the user - not Dialogflow. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `name` | String |  | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `inspect_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `purge_data_types` | Vec<String> |  | List of types of data to remove when retention settings triggers purge. |
| `insights_export_settings` | String |  | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `audio_export_settings` | String |  | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `deidentify_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `redaction_strategy` | String |  | Strategy that defines how we do redaction. |
| `parent` | String | ✅ | Required. The location to create an SecuritySettings for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `redaction_scope` | String | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_strategy` | String | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `display_name` | String | Required. The human-readable name of the security settings, unique within the location. |
| `retention_window_days` | i64 | Retains data in interaction logging for the specified number of days. This does not apply to Cloud logging, which is owned by the user - not Dialogflow. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `name` | String | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `inspect_template` | String | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `purge_data_types` | Vec<String> | List of types of data to remove when retention settings triggers purge. |
| `insights_export_settings` | String | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `audio_export_settings` | String | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `deidentify_template` | String | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
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
security_setting_redaction_scope = security_setting.redaction_scope
security_setting_retention_strategy = security_setting.retention_strategy
security_setting_display_name = security_setting.display_name
security_setting_retention_window_days = security_setting.retention_window_days
security_setting_name = security_setting.name
security_setting_inspect_template = security_setting.inspect_template
security_setting_purge_data_types = security_setting.purge_data_types
security_setting_insights_export_settings = security_setting.insights_export_settings
security_setting_audio_export_settings = security_setting.audio_export_settings
security_setting_deidentify_template = security_setting.deidentify_template
security_setting_redaction_strategy = security_setting.redaction_strategy
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---


### Tool

Creates a Tool in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extension_spec` | String |  | Vertex extension tool specification. |
| `connector_spec` | String |  | Integration connectors tool specification. |
| `name` | String |  | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `display_name` | String |  | Required. The human-readable name of the Tool, unique within an agent. |
| `function_spec` | String |  | Client side executed function specification. |
| `tool_type` | String |  | Output only. The tool type. |
| `description` | String |  | Required. High level description of the Tool and its usage. |
| `open_api_spec` | String |  | OpenAPI specification of the Tool. |
| `data_store_spec` | String |  | Data store search tool specification. |
| `parent` | String | ✅ | Required. The agent to create a Tool for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extension_spec` | String | Vertex extension tool specification. |
| `connector_spec` | String | Integration connectors tool specification. |
| `name` | String | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `display_name` | String | Required. The human-readable name of the Tool, unique within an agent. |
| `function_spec` | String | Client side executed function specification. |
| `tool_type` | String | Output only. The tool type. |
| `description` | String | Required. High level description of the Tool and its usage. |
| `open_api_spec` | String | OpenAPI specification of the Tool. |
| `data_store_spec` | String | Data store search tool specification. |


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
tool_extension_spec = tool.extension_spec
tool_connector_spec = tool.connector_spec
tool_name = tool.name
tool_display_name = tool.display_name
tool_function_spec = tool.function_spec
tool_tool_type = tool.tool_type
tool_description = tool.description
tool_open_api_spec = tool.open_api_spec
tool_data_store_spec = tool.data_store_spec
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
| `action` | String | The action of the change. |
| `resource` | String | The affected resource name of the change. |
| `name` | String | The unique identifier of the changelog. Format: `projects//locations//agents//changelogs/`. |
| `type` | String | The affected resource type. |
| `display_name` | String | The affected resource display name of the change. |
| `create_time` | String | The timestamp of the change. |
| `language_code` | String | The affected language code of the change. |
| `user_email` | String | Email address of the authenticated user. |


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
changelog_action = changelog.action
changelog_resource = changelog.resource
changelog_name = changelog.name
changelog_type = changelog.type
changelog_display_name = changelog.display_name
changelog_create_time = changelog.create_time
changelog_language_code = changelog.language_code
changelog_user_email = changelog.user_email
```

---


### Flow

Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nlu_settings` | String |  | NLU related settings of the flow. |
| `transition_route_groups` | Vec<String> |  | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `transition_routes` | Vec<String> |  | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `event_handlers` | Vec<String> |  | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this flow. |
| `display_name` | String |  | Required. The human-readable name of the flow. |
| `description` | String |  | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `locked` | bool |  | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this flow. |
| `multi_language_settings` | String |  | Optional. Multi-lingual agent settings for this flow. |
| `name` | String |  | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `parent` | String | ✅ | Required. The agent to create a flow for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nlu_settings` | String | NLU related settings of the flow. |
| `transition_route_groups` | Vec<String> | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `transition_routes` | Vec<String> | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `event_handlers` | Vec<String> | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this flow. |
| `display_name` | String | Required. The human-readable name of the flow. |
| `description` | String | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `locked` | bool | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `advanced_settings` | String | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this flow. |
| `multi_language_settings` | String | Optional. Multi-lingual agent settings for this flow. |
| `name` | String | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |


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
flow_transition_routes = flow.transition_routes
flow_event_handlers = flow.event_handlers
flow_output_parameter_definitions = flow.output_parameter_definitions
flow_display_name = flow.display_name
flow_description = flow.description
flow_locked = flow.locked
flow_advanced_settings = flow.advanced_settings
flow_knowledge_connector_settings = flow.knowledge_connector_settings
flow_input_parameter_definitions = flow.input_parameter_definitions
flow_multi_language_settings = flow.multi_language_settings
flow_name = flow.name
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
| `duration` | String | Duration of the conversation. |
| `language_code` | String | The language of the conversation, which is the language of the first request in the conversation. |
| `flows` | Vec<String> | All the Flow the conversation has went through. Only `name` and `display_name` are filled in this message. |
| `metrics` | String | Conversation metrics. |
| `interactions` | Vec<String> | Interactions of the conversation. Only populated for `GetConversation` and empty for `ListConversations`. |
| `pages` | Vec<String> | All the Page the conversation has went through. Only `name` and `display_name` are filled in this message. |
| `flow_versions` | HashMap<String, String> | Flow versions used in the conversation. |
| `environment` | String | Environment of the conversation. Only `name` and `display_name` are filled in this message. |
| `name` | String | Identifier. The identifier of the conversation. If conversation ID is reused, interactions happened later than 48 hours of the conversation's create time will be ignored. Format: `projects//locations//agents//conversations/` |
| `start_time` | String | Start time of the conversation, which is the time of the first request of the conversation. |
| `intents` | Vec<String> | All the matched Intent in the conversation. Only `name` and `display_name` are filled in this message. |


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
conversation_duration = conversation.duration
conversation_language_code = conversation.language_code
conversation_flows = conversation.flows
conversation_metrics = conversation.metrics
conversation_interactions = conversation.interactions
conversation_pages = conversation.pages
conversation_flow_versions = conversation.flow_versions
conversation_environment = conversation.environment
conversation_name = conversation.name
conversation_start_time = conversation.start_time
conversation_intents = conversation.intents
```

---


### Example

Creates an example in the specified playbook.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `playbook_input` | String |  | Optional. The input to the playbook in the example. |
| `language_code` | String |  | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `name` | String |  | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `actions` | Vec<String> |  | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `create_time` | String |  | Output only. The timestamp of initial example creation. |
| `token_count` | String |  | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `description` | String |  | Optional. The high level concise description of the example. The max number of characters is 200. |
| `playbook_output` | String |  | Optional. The output of the playbook in the example. |
| `update_time` | String |  | Output only. Last time the example was updated. |
| `conversation_state` | String |  | Required. Example's output state. |
| `display_name` | String |  | Required. The display name of the example. |
| `parent` | String | ✅ | Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `playbook_input` | String | Optional. The input to the playbook in the example. |
| `language_code` | String | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `name` | String | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `actions` | Vec<String> | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `create_time` | String | Output only. The timestamp of initial example creation. |
| `token_count` | String | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `description` | String | Optional. The high level concise description of the example. The max number of characters is 200. |
| `playbook_output` | String | Optional. The output of the playbook in the example. |
| `update_time` | String | Output only. Last time the example was updated. |
| `conversation_state` | String | Required. Example's output state. |
| `display_name` | String | Required. The display name of the example. |


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
example_playbook_input = example.playbook_input
example_language_code = example.language_code
example_name = example.name
example_actions = example.actions
example_create_time = example.create_time
example_token_count = example.token_count
example_description = example.description
example_playbook_output = example.playbook_output
example_update_time = example.update_time
example_conversation_state = example.conversation_state
example_display_name = example.display_name
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
| `test_result` | String | Whether the test case passed in the agent environment. |
| `environment` | String | Environment where the test was run. If not set, it indicates the draft environment. |
| `conversation_turns` | Vec<String> | The conversation turns uttered during the test case replay in chronological order. |
| `name` | String | The resource name for the test case result. Format: `projects//locations//agents//testCases//results/`. |
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
result_test_result = result.test_result
result_environment = result.environment
result_conversation_turns = result.conversation_turns
result_name = result.name
result_test_time = result.test_time
```

---


### Environment

Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `description` | String |  | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `update_time` | String |  | Output only. Update time of this environment. |
| `version_configs` | Vec<String> |  | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `test_cases_config` | String |  | The test cases config for continuous tests of this environment. |
| `webhook_config` | String |  | The webhook configuration for this environment. |
| `display_name` | String |  | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `description` | String | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `update_time` | String | Output only. Update time of this environment. |
| `version_configs` | Vec<String> | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `test_cases_config` | String | The test cases config for continuous tests of this environment. |
| `webhook_config` | String | The webhook configuration for this environment. |
| `display_name` | String | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |


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
environment_name = environment.name
environment_description = environment.description
environment_update_time = environment.update_time
environment_version_configs = environment.version_configs
environment_test_cases_config = environment.test_cases_config
environment_webhook_config = environment.webhook_config
environment_display_name = environment.display_name
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


### Version

Creates a version for the specified Playbook.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Last time the playbook version was created or modified. |
| `description` | String |  | Optional. The description of the playbook version. |
| `examples` | Vec<String> |  | Output only. Snapshot of the examples belonging to the playbook when the playbook version is created. |
| `name` | String |  | The unique identifier of the playbook version. Format: `projects//locations//agents//playbooks//versions/`. |
| `playbook` | String |  | Output only. Snapshot of the playbook when the playbook version is created. |
| `parent` | String | ✅ | Required. The playbook to create a version for. Format: `projects//locations//agents//playbooks/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Last time the playbook version was created or modified. |
| `description` | String | Optional. The description of the playbook version. |
| `examples` | Vec<String> | Output only. Snapshot of the examples belonging to the playbook when the playbook version is created. |
| `name` | String | The unique identifier of the playbook version. Format: `projects//locations//agents//playbooks//versions/`. |
| `playbook` | String | Output only. Snapshot of the playbook when the playbook version is created. |


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
    parent = "value"  # Required. The playbook to create a version for. Format: `projects//locations//agents//playbooks/`.
}

# Access version outputs
version_id = version.id
version_update_time = version.update_time
version_description = version.description
version_examples = version.examples
version_name = version.name
version_playbook = version.playbook
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
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
| `flow_version` | String | The name of the flow version for this deployment. Format: projects//locations//agents//flows//versions/. |
| `result` | String | Result of the deployment. |
| `state` | String | The current state of the deployment. |


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
deployment_flow_version = deployment.flow_version
deployment_result = deployment.result
deployment_state = deployment.state
```

---


### Test_case

Creates a test case for the given agent.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_case_conversation_turns` | Vec<String> |  | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `display_name` | String |  | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_config` | String |  | Config for the test case. |
| `name` | String |  | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `last_test_result` | String |  | The latest test result. |
| `notes` | String |  | Additional freeform notes about the test case. Limit of 400 characters. |
| `creation_time` | String |  | Output only. When the test was created. |
| `tags` | Vec<String> |  | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |
| `parent` | String | ✅ | Required. The agent to create the test case for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_case_conversation_turns` | Vec<String> | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `display_name` | String | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_config` | String | Config for the test case. |
| `name` | String | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `last_test_result` | String | The latest test result. |
| `notes` | String | Additional freeform notes about the test case. Limit of 400 characters. |
| `creation_time` | String | Output only. When the test was created. |
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
test_case_test_case_conversation_turns = test_case.test_case_conversation_turns
test_case_display_name = test_case.display_name
test_case_test_config = test_case.test_config
test_case_name = test_case.name
test_case_last_test_result = test_case.last_test_result
test_case_notes = test_case.notes
test_case_creation_time = test_case.creation_time
test_case_tags = test_case.tags
```

---


### Experiment

Creates an Experiment in the specified Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `create_time` | String |  | Creation time of this experiment. |
| `result` | String |  | Inference result of the experiment. |
| `rollout_state` | String |  | State of the auto rollout process. |
| `start_time` | String |  | Start time of this experiment. |
| `definition` | String |  | The definition of the experiment. |
| `name` | String |  | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `rollout_failure_reason` | String |  | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `description` | String |  | The human-readable description of the experiment. |
| `last_update_time` | String |  | Last update time of this experiment. |
| `experiment_length` | String |  | Maximum number of days to run the experiment. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `rollout_config` | String |  | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `variants_history` | Vec<String> |  | The history of updates to the experiment variants. |
| `state` | String |  | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `end_time` | String |  | End time of this experiment. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `create_time` | String | Creation time of this experiment. |
| `result` | String | Inference result of the experiment. |
| `rollout_state` | String | State of the auto rollout process. |
| `start_time` | String | Start time of this experiment. |
| `definition` | String | The definition of the experiment. |
| `name` | String | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `rollout_failure_reason` | String | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `description` | String | The human-readable description of the experiment. |
| `last_update_time` | String | Last update time of this experiment. |
| `experiment_length` | String | Maximum number of days to run the experiment. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `rollout_config` | String | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `variants_history` | Vec<String> | The history of updates to the experiment variants. |
| `state` | String | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `end_time` | String | End time of this experiment. |


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
experiment_display_name = experiment.display_name
experiment_create_time = experiment.create_time
experiment_result = experiment.result
experiment_rollout_state = experiment.rollout_state
experiment_start_time = experiment.start_time
experiment_definition = experiment.definition
experiment_name = experiment.name
experiment_rollout_failure_reason = experiment.rollout_failure_reason
experiment_description = experiment.description
experiment_last_update_time = experiment.last_update_time
experiment_experiment_length = experiment.experiment_length
experiment_rollout_config = experiment.rollout_config
experiment_variants_history = experiment.variants_history
experiment_state = experiment.state
experiment_end_time = experiment.end_time
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


### Entity_type

Creates a session entity type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entities` | Vec<String> |  | Required. The collection of entities to override or supplement the custom entity type. |
| `name` | String |  | Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. |
| `entity_override_mode` | String |  | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `parent` | String | ✅ | Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entities` | Vec<String> | Required. The collection of entities to override or supplement the custom entity type. |
| `name` | String | Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. |
| `entity_override_mode` | String | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |


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
    parent = "value"  # Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_entities = entity_type.entities
entity_type_name = entity_type.name
entity_type_entity_override_mode = entity_type.entity_override_mode
```

---


### Webhook

Creates a webhook in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `disabled` | bool |  | Indicates whether the webhook is disabled. |
| `display_name` | String |  | Required. The human-readable name of the webhook, unique within the agent. |
| `service_directory` | String |  | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `timeout` | String |  | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `generic_web_service` | String |  | Configuration for a generic web service. |
| `parent` | String | ✅ | Required. The agent to create a webhook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `disabled` | bool | Indicates whether the webhook is disabled. |
| `display_name` | String | Required. The human-readable name of the webhook, unique within the agent. |
| `service_directory` | String | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `timeout` | String | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `generic_web_service` | String | Configuration for a generic web service. |


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
webhook_name = webhook.name
webhook_disabled = webhook.disabled
webhook_display_name = webhook.display_name
webhook_service_directory = webhook.service_directory
webhook_timeout = webhook.timeout
webhook_generic_web_service = webhook.generic_web_service
```

---


### Playbook

Creates a playbook in a specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_actions` | Vec<String> |  | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `handlers` | Vec<String> |  | Optional. A list of registered handlers to execute based on the specified triggers. |
| `playbook_type` | String |  | Optional. Type of the playbook. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this playbook. |
| `referenced_playbooks` | Vec<String> |  | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `name` | String |  | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `speech_settings` | String |  | Optional. Playbook level Settings for speech to text detection. |
| `instruction` | String |  | Instruction to accomplish target goal. |
| `code_block` | String |  | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `referenced_tools` | Vec<String> |  | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `create_time` | String |  | Output only. The timestamp of initial playbook creation. |
| `referenced_flows` | Vec<String> |  | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `llm_model_settings` | String |  | Optional. Llm model settings for the playbook. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this playbook. |
| `display_name` | String |  | Required. The human-readable name of the playbook, unique within an agent. |
| `token_count` | String |  | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `goal` | String |  | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `update_time` | String |  | Output only. Last time the playbook version was updated. |
| `parent` | String | ✅ | Required. The agent to create a playbook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inline_actions` | Vec<String> | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `handlers` | Vec<String> | Optional. A list of registered handlers to execute based on the specified triggers. |
| `playbook_type` | String | Optional. Type of the playbook. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this playbook. |
| `referenced_playbooks` | Vec<String> | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `name` | String | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `speech_settings` | String | Optional. Playbook level Settings for speech to text detection. |
| `instruction` | String | Instruction to accomplish target goal. |
| `code_block` | String | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `referenced_tools` | Vec<String> | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `create_time` | String | Output only. The timestamp of initial playbook creation. |
| `referenced_flows` | Vec<String> | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `llm_model_settings` | String | Optional. Llm model settings for the playbook. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this playbook. |
| `display_name` | String | Required. The human-readable name of the playbook, unique within an agent. |
| `token_count` | String | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `goal` | String | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `update_time` | String | Output only. Last time the playbook version was updated. |


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
playbook_inline_actions = playbook.inline_actions
playbook_handlers = playbook.handlers
playbook_playbook_type = playbook.playbook_type
playbook_input_parameter_definitions = playbook.input_parameter_definitions
playbook_referenced_playbooks = playbook.referenced_playbooks
playbook_name = playbook.name
playbook_speech_settings = playbook.speech_settings
playbook_instruction = playbook.instruction
playbook_code_block = playbook.code_block
playbook_referenced_tools = playbook.referenced_tools
playbook_create_time = playbook.create_time
playbook_referenced_flows = playbook.referenced_flows
playbook_llm_model_settings = playbook.llm_model_settings
playbook_output_parameter_definitions = playbook.output_parameter_definitions
playbook_display_name = playbook.display_name
playbook_token_count = playbook.token_count
playbook_goal = playbook.goal
playbook_update_time = playbook.update_time
```

---


### Page

Creates a page in the specified flow.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `form` | String |  | The form associated with the page, used for collecting parameters relevant to the page. |
| `name` | String |  | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `description` | String |  | The description of the page. The maximum length is 500 characters. |
| `transition_route_groups` | Vec<String> |  | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String |  | Required. The human-readable name of the page, unique within the flow. |
| `transition_routes` | Vec<String> |  | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `event_handlers` | Vec<String> |  | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `entry_fulfillment` | String |  | The fulfillment to call when the session is entering the page. |
| `parent` | String | ✅ | Required. The flow to create a page for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `form` | String | The form associated with the page, used for collecting parameters relevant to the page. |
| `name` | String | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `advanced_settings` | String | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `description` | String | The description of the page. The maximum length is 500 characters. |
| `transition_route_groups` | Vec<String> | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String | Required. The human-readable name of the page, unique within the flow. |
| `transition_routes` | Vec<String> | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `event_handlers` | Vec<String> | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
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
page_form = page.form
page_name = page.name
page_advanced_settings = page.advanced_settings
page_description = page.description
page_transition_route_groups = page.transition_route_groups
page_display_name = page.display_name
page_transition_routes = page.transition_routes
page_event_handlers = page.event_handlers
page_knowledge_connector_settings = page.knowledge_connector_settings
page_entry_fulfillment = page.entry_fulfillment
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | Vec<String> |  | The collection of parameters associated with the intent. |
| `labels` | HashMap<String, String> |  | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys-contextual" means the intent is a contextual intent. |
| `is_fallback` | bool |  | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `training_phrases` | Vec<String> |  | The collection of training phrases the agent is trained on to identify the intent. |
| `name` | String |  | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `priority` | i64 |  | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `description` | String |  | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `display_name` | String |  | Required. The human-readable name of the intent, unique within the agent. |
| `parent` | String | ✅ | Required. The agent to create an intent for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | The collection of parameters associated with the intent. |
| `labels` | HashMap<String, String> | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys-contextual" means the intent is a contextual intent. |
| `is_fallback` | bool | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `training_phrases` | Vec<String> | The collection of training phrases the agent is trained on to identify the intent. |
| `name` | String | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `priority` | i64 | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `description` | String | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
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
intent_parameters = intent.parameters
intent_labels = intent.labels
intent_is_fallback = intent.is_fallback
intent_training_phrases = intent.training_phrases
intent_name = intent.name
intent_priority = intent.priority
intent_description = intent.description
intent_display_name = intent.display_name
```

---


### Transition_route_group

Creates an TransitionRouteGroup in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `display_name` | String |  | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> |  | Transition routes associated with the TransitionRouteGroup. |
| `parent` | String | ✅ | Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
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


### Generator

Creates a generator in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prompt_text` | String |  | Required. Prompt for the LLM model. |
| `display_name` | String |  | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String |  | The LLM model settings. |
| `model_parameter` | String |  | Parameters passed to the LLM to configure its behavior. |
| `name` | String |  | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `placeholders` | Vec<String> |  | Optional. List of custom placeholders in the prompt text. |
| `parent` | String | ✅ | Required. The agent to create a generator for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prompt_text` | String | Required. Prompt for the LLM model. |
| `display_name` | String | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String | The LLM model settings. |
| `model_parameter` | String | Parameters passed to the LLM to configure its behavior. |
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
generator_display_name = generator.display_name
generator_llm_model_settings = generator.llm_model_settings
generator_model_parameter = generator.model_parameter
generator_name = generator.name
generator_placeholders = generator.placeholders
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
| `generator` | String |  | Uncreated generator. It should be a complete generator that includes all information about the generator. |
| `generator_name` | String |  | The resource name of the existing created generator. Format: `projects//locations//generators/` |
| `context_references` | HashMap<String, String> |  | Optional. A section of ingested context information. The key is the name of the context reference and the value contains the contents of the context reference. The key is used to incorporate ingested context references to enhance the generator. |
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


### Project

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `display_name` | String |  | Required. The name of this agent. |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `classification_threshold` | f64 | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `match_mode` | String | Optional. Determines how intents are detected from user queries. |
| `parent` | String | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `default_language_code` | String | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `api_version` | String | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `supported_language_codes` | Vec<String> | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `display_name` | String | Required. The name of this agent. |
| `avatar_uri` | String | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `enable_logging` | bool | Optional. Determines whether this agent should log conversation queries. |
| `description` | String | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |


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
project_classification_threshold = project.classification_threshold
project_match_mode = project.match_mode
project_parent = project.parent
project_tier = project.tier
project_time_zone = project.time_zone
project_default_language_code = project.default_language_code
project_api_version = project.api_version
project_supported_language_codes = project.supported_language_codes
project_display_name = project.display_name
project_avatar_uri = project.avatar_uri
project_enable_logging = project.enable_logging
project_description = project.description
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
| `obfuscated_external_user_id` | String |  | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow uses this user id for billing and measurement. If a user with the same obfuscated_external_user_id is created in a later conversation, Dialogflow will know it's the same user. Dialogflow also uses this user id for Agent Assist suggestion personalization. For example, Dialogflow can use it to provide personalized smart reply suggestions for this user. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `role` | String |  | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `parent` | String | ✅ | Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents_metadata_filters` | HashMap<String, String> | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `name` | String | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `obfuscated_external_user_id` | String | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow uses this user id for billing and measurement. If a user with the same obfuscated_external_user_id is created in a later conversation, Dialogflow will know it's the same user. Dialogflow also uses this user id for Agent Assist suggestion personalization. For example, Dialogflow can use it to provide personalized smart reply suggestions for this user. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `role` | String | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |


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
participant_obfuscated_external_user_id = participant.obfuscated_external_user_id
participant_role = participant.role
```

---


### Version

Creates an agent version. The new version points to the agent instance in the "default" environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_number` | i64 |  | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `description` | String |  | Optional. The developer-provided description of this version. |
| `create_time` | String |  | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `status` | String |  | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `name` | String |  | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `parent` | String | ✅ | Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_number` | i64 | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `description` | String | Optional. The developer-provided description of this version. |
| `create_time` | String | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `status` | String | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `name` | String | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |


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
version_version_number = version.version_number
version_description = version.description
version_create_time = version.create_time
version_status = version.status
version_name = version.name
```

---


### Conversation

Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time the conversation was finished. |
| `conversation_profile` | String |  | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `lifecycle_state` | String |  | Output only. The current state of the Conversation. |
| `phone_number` | String |  | Output only. Required if the conversation is to be connected over telephony. |
| `ingested_context_references` | HashMap<String, String> |  | Output only. The context reference updates provided by external systems. |
| `start_time` | String |  | Output only. The time the conversation was started. |
| `conversation_stage` | String |  | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `telephony_connection_info` | String |  | Output only. The telephony connection information. |
| `name` | String |  | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `parent` | String | ✅ | Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time the conversation was finished. |
| `conversation_profile` | String | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `lifecycle_state` | String | Output only. The current state of the Conversation. |
| `phone_number` | String | Output only. Required if the conversation is to be connected over telephony. |
| `ingested_context_references` | HashMap<String, String> | Output only. The context reference updates provided by external systems. |
| `start_time` | String | Output only. The time the conversation was started. |
| `conversation_stage` | String | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `telephony_connection_info` | String | Output only. The telephony connection information. |
| `name` | String | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |


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
conversation_end_time = conversation.end_time
conversation_conversation_profile = conversation.conversation_profile
conversation_lifecycle_state = conversation.lifecycle_state
conversation_phone_number = conversation.phone_number
conversation_ingested_context_references = conversation.ingested_context_references
conversation_start_time = conversation.start_time
conversation_conversation_stage = conversation.conversation_stage
conversation_telephony_connection_info = conversation.telephony_connection_info
conversation_name = conversation.name
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


### Document

Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead. |
| `knowledge_types` | Vec<String> |  | Required. The knowledge type of document content. |
| `latest_reload_status` | String |  | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `mime_type` | String |  | Required. The MIME type of this document. |
| `name` | String |  | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `raw_content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `enable_auto_reload` | bool |  | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `state` | String |  | Output only. The current state of the document. |
| `content_uri` | String |  | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `parent` | String | ✅ | Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. Note: This field is in the process of being deprecated, please use raw_content instead. |
| `knowledge_types` | Vec<String> | Required. The knowledge type of document content. |
| `latest_reload_status` | String | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `mime_type` | String | Required. The MIME type of this document. |
| `name` | String | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `raw_content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `enable_auto_reload` | bool | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `state` | String | Output only. The current state of the document. |
| `content_uri` | String | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |


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
document_content = document.content
document_knowledge_types = document.knowledge_types
document_latest_reload_status = document.latest_reload_status
document_metadata = document.metadata
document_mime_type = document.mime_type
document_name = document.name
document_raw_content = document.raw_content
document_enable_auto_reload = document.enable_auto_reload
document_state = document.state
document_content_uri = document.content_uri
```

---


### Entity_type

Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented. |
| `entities` | Vec<String> |  | Required. The collection of entities associated with this session entity type. |
| `entity_override_mode` | String |  | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `parent` | String | ✅ | Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented. |
| `entities` | Vec<String> | Required. The collection of entities associated with this session entity type. |
| `entity_override_mode` | String | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |


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
    parent = "value"  # Required. The session to create a session entity type for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_name = entity_type.name
entity_type_entities = entity_type.entities
entity_type_entity_override_mode = entity_type.entity_override_mode
```

---


### Session

Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_audio_config_mask` | String |  | Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety. |
| `query_input` | String |  | Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated. |
| `input_audio` | String |  | The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data. |
| `query_params` | String |  | The parameters of this query. |
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
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `agents` | Vec<String> | The list of agents. There will be a maximum number of items returned based on the page_size field in the request. |


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
agent_next_page_token = agent.next_page_token
agent_agents = agent.agents
```

---


### Environment

Creates an agent environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` |
| `state` | String |  | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String |  | Optional. Text to speech settings for this environment. |
| `update_time` | String |  | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `fulfillment` | String |  | Optional. The fulfillment settings to use for this environment. |
| `agent_version` | String |  | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `description` | String |  | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `parent` | String | ✅ | Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` |
| `state` | String | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String | Optional. Text to speech settings for this environment. |
| `update_time` | String | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `fulfillment` | String | Optional. The fulfillment settings to use for this environment. |
| `agent_version` | String | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `description` | String | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |


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
environment_name = environment.name
environment_state = environment.state
environment_text_to_speech_settings = environment.text_to_speech_settings
environment_update_time = environment.update_time
environment_fulfillment = environment.fulfillment
environment_agent_version = environment.agent_version
environment_description = environment.description
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/` |
| `events` | Vec<String> |  | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `output_contexts` | Vec<String> |  | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `training_phrases` | Vec<String> |  | Optional. The collection of examples that the agent is trained on. |
| `parameters` | Vec<String> |  | Optional. The collection of parameters associated with the intent. |
| `display_name` | String |  | Required. The name of this intent. |
| `root_followup_intent_name` | String |  | Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`. |
| `end_interaction` | bool |  | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `input_context_names` | Vec<String> |  | Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/` |
| `ml_disabled` | bool |  | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `ml_enabled` | bool |  | Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false. |
| `messages` | Vec<String> |  | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `priority` | i64 |  | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `reset_contexts` | bool |  | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `action` | String |  | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `webhook_state` | String |  | Optional. Indicates whether webhooks are enabled for the intent. |
| `live_agent_handoff` | bool |  | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `followup_intent_info` | Vec<String> |  | Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `parent_followup_intent_name` | String |  | Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `default_response_platforms` | Vec<String> |  | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `is_fallback` | bool |  | Optional. Indicates whether this is a fallback intent. |
| `parent` | String | ✅ | Required. The agent to create a intent for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/` |
| `events` | Vec<String> | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `output_contexts` | Vec<String> | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `training_phrases` | Vec<String> | Optional. The collection of examples that the agent is trained on. |
| `parameters` | Vec<String> | Optional. The collection of parameters associated with the intent. |
| `display_name` | String | Required. The name of this intent. |
| `root_followup_intent_name` | String | Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`. |
| `end_interaction` | bool | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `input_context_names` | Vec<String> | Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/` |
| `ml_disabled` | bool | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `ml_enabled` | bool | Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false. |
| `messages` | Vec<String> | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `priority` | i64 | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `reset_contexts` | bool | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `action` | String | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `webhook_state` | String | Optional. Indicates whether webhooks are enabled for the intent. |
| `live_agent_handoff` | bool | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `followup_intent_info` | Vec<String> | Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `parent_followup_intent_name` | String | Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `default_response_platforms` | Vec<String> | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `is_fallback` | bool | Optional. Indicates whether this is a fallback intent. |


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
intent_name = intent.name
intent_events = intent.events
intent_output_contexts = intent.output_contexts
intent_training_phrases = intent.training_phrases
intent_parameters = intent.parameters
intent_display_name = intent.display_name
intent_root_followup_intent_name = intent.root_followup_intent_name
intent_end_interaction = intent.end_interaction
intent_input_context_names = intent.input_context_names
intent_ml_disabled = intent.ml_disabled
intent_ml_enabled = intent.ml_enabled
intent_messages = intent.messages
intent_priority = intent.priority
intent_reset_contexts = intent.reset_contexts
intent_action = intent.action
intent_webhook_state = intent.webhook_state
intent_live_agent_handoff = intent.live_agent_handoff
intent_followup_intent_info = intent.followup_intent_info
intent_parent_followup_intent_name = intent.parent_followup_intent_name
intent_default_response_platforms = intent.default_response_platforms
intent_is_fallback = intent.is_fallback
```

---


### Suggestion

Get answers for the given query based on knowledge documents.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exact_search` | bool |  | Optional. Whether to search the query exactly without query rewrite. |
| `latest_message` | String |  | Optional. The name of the latest conversation message when the request is triggered. Format: `projects//locations//conversations//messages/`. |
| `conversation` | String |  | Optional. The conversation (between human agent and end user) where the search request is triggered. Format: `projects//locations//conversations/`. |
| `query` | String |  | Required. The natural language text query for knowledge search. |
| `end_user_metadata` | HashMap<String, String> |  | Optional. Information about the end-user to improve the relevance and accuracy of generative answers. This will be interpreted and used by a language model, so, for good results, the data should be self-descriptive, and in a simple structure. Example: ```json { "subscription plan": "Business Premium Plus", "devices owned": [ {"model": "Google Pixel 7"}, {"model": "Google Pixel Tablet"} ] } ``` |
| `query_source` | String |  | Optional. The source of the query in the request. |
| `session_id` | String |  | Required. The ID of the search session. The session_id can be combined with Dialogflow V3 Agent ID retrieved from conversation profile or on its own to identify a search session. The search history of the same session will impact the search result. It's up to the API caller to choose an appropriate `Session ID`. It can be a random number or some type of session identifiers (preferably hashed). The length must not exceed 36 characters. |
| `parent` | String |  | Required. The parent resource contains the conversation profile Format: 'projects/' or `projects//locations/`. |
| `search_config` | String |  | Optional. Configuration specific to search queries with data stores. |
| `conversation_profile` | String |  | Required. The conversation profile used to configure the search. Format: `projects//locations//conversationProfiles/`. |
| `parent` | String | ✅ | Required. The parent resource contains the conversation profile Format: 'projects/' or `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Optional. Token to retrieve the next page of results or empty if there are no more results in the list. |
| `suggestions` | Vec<String> | Required. The list of suggestions. There will be a maximum number of items returned based on the page_size field in the request. `suggestions` is sorted by `create_time` in descending order. |


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
    parent = "value"  # Required. The parent resource contains the conversation profile Format: 'projects/' or `projects//locations/`.
}

# Access suggestion outputs
suggestion_id = suggestion.id
suggestion_next_page_token = suggestion.next_page_token
suggestion_suggestions = suggestion.suggestions
```

---


### Context

Creates a context. If the specified context already exists, overrides the context.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lifespan_count` | i64 |  | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `name` | String |  | Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `parameters` | HashMap<String, String> |  | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `parent` | String | ✅ | Required. The session to create a context for. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`, If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifespan_count` | i64 | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `name` | String | Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `parameters` | HashMap<String, String> | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |


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
context_lifespan_count = context.lifespan_count
context_name = context.name
context_parameters = context.parameters
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


### Answer_record

Deprecated. Retrieves a specific answer record.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer_feedback` | String |  | Optional. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer. |
| `agent_assistant_record` | String |  | Output only. The record for human agent assistant. |
| `name` | String |  | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |
| `name` | String | ✅ | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `answer_feedback` | String | Optional. The AnswerFeedback for this record. You can set this with AnswerRecords.UpdateAnswerRecord in order to give us feedback about this answer. |
| `agent_assistant_record` | String | Output only. The record for human agent assistant. |
| `name` | String | The unique identifier of this answer record. Required for AnswerRecords.UpdateAnswerRecord method. Format: `projects//locations//answerRecords/`. |


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
answer_record_answer_feedback = answer_record.answer_feedback
answer_record_agent_assistant_record = answer_record.agent_assistant_record
answer_record_name = answer_record.name
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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


### Sip_trunk

Creates a SipTrunk for a specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connections` | Vec<String> |  | Output only. Connections of the SIP trunk. |
| `expected_hostname` | Vec<String> |  | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `display_name` | String |  | Optional. Human readable alias for this trunk. |
| `name` | String |  | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `parent` | String | ✅ | Required. The location to create a SIP trunk for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | Output only. Connections of the SIP trunk. |
| `expected_hostname` | Vec<String> | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `display_name` | String | Optional. Human readable alias for this trunk. |
| `name` | String | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |


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
sip_trunk_expected_hostname = sip_trunk.expected_hostname
sip_trunk_display_name = sip_trunk.display_name
sip_trunk_name = sip_trunk.name
```

---


### Location

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/` or `projects//locations/` |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `display_name` | String |  | Required. The name of this agent. |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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

# Create location
location = provider.dialogflow_api.Location {
    parent = "value"  # Required. The project of this agent. Format: `projects/` or `projects//locations/`
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Tool

Creates a tool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_spec` | String |  | Integration connectors tool specification. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `extension_spec` | String |  | Vertex extension tool specification. |
| `tool_key` | String |  | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `display_name` | String |  | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String |  | Client side executed function specification. |
| `name` | String |  | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `open_api_spec` | String |  | OpenAPI tool. |
| `action_confirmation_requirement` | HashMap<String, String> |  | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `create_time` | String |  | Output only. Creation time of this tool. |
| `description` | String |  | Optional. A human readable description of the tool. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String |  | Output only. Update time of this tool. |
| `parent` | String | ✅ | Required. The project/location to create tool for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_spec` | String | Integration connectors tool specification. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `extension_spec` | String | Vertex extension tool specification. |
| `tool_key` | String | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `display_name` | String | Optional. A human readable short name of the tool, to be shown on the UI. |
| `function_spec` | String | Client side executed function specification. |
| `name` | String | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `open_api_spec` | String | OpenAPI tool. |
| `action_confirmation_requirement` | HashMap<String, String> | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `create_time` | String | Output only. Creation time of this tool. |
| `description` | String | Optional. A human readable description of the tool. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String | Output only. Update time of this tool. |


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
tool_satisfies_pzi = tool.satisfies_pzi
tool_extension_spec = tool.extension_spec
tool_tool_key = tool.tool_key
tool_display_name = tool.display_name
tool_function_spec = tool.function_spec
tool_name = tool.name
tool_open_api_spec = tool.open_api_spec
tool_action_confirmation_requirement = tool.action_confirmation_requirement
tool_create_time = tool.create_time
tool_description = tool.description
tool_satisfies_pzs = tool.satisfies_pzs
tool_update_time = tool.update_time
```

---


### Generator

Creates a generator.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `free_form_context` | String |  | Input of free from generator to LLM. |
| `suggestion_deduping_config` | String |  | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `published_model` | String |  | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `create_time` | String |  | Output only. Creation time of this generator. |
| `agent_coaching_context` | String |  | Input of Agent Coaching feature. |
| `summarization_context` | String |  | Input of Summarization feature. |
| `name` | String |  | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `tools` | Vec<String> |  | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `trigger_event` | String |  | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String |  | Output only. Update time of this generator. |
| `inference_parameter` | String |  | Optional. Inference parameters for this generator. |
| `description` | String |  | Optional. Human readable description of the generator. |
| `parent` | String | ✅ | Required. The project/location to create generator for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `free_form_context` | String | Input of free from generator to LLM. |
| `suggestion_deduping_config` | String | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `published_model` | String | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `create_time` | String | Output only. Creation time of this generator. |
| `agent_coaching_context` | String | Input of Agent Coaching feature. |
| `summarization_context` | String | Input of Summarization feature. |
| `name` | String | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `tools` | Vec<String> | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `trigger_event` | String | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String | Output only. Update time of this generator. |
| `inference_parameter` | String | Optional. Inference parameters for this generator. |
| `description` | String | Optional. Human readable description of the generator. |


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
generator_free_form_context = generator.free_form_context
generator_suggestion_deduping_config = generator.suggestion_deduping_config
generator_published_model = generator.published_model
generator_create_time = generator.create_time
generator_agent_coaching_context = generator.agent_coaching_context
generator_summarization_context = generator.summarization_context
generator_name = generator.name
generator_tools = generator.tools
generator_trigger_event = generator.trigger_event
generator_update_time = generator.update_time
generator_inference_parameter = generator.inference_parameter
generator_description = generator.description
```

---


### Conversation_profile

Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the conversation profile. |
| `name` | String |  | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `display_name` | String |  | Required. Human readable name for this profile. Max length 1024 bytes. |
| `tts_config` | String |  | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `new_recognition_result_notification_config` | String |  | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `human_agent_handoff_config` | String |  | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `update_time` | String |  | Output only. Update time of the conversation profile. |
| `logging_config` | String |  | Configuration for logging conversation lifecycle events. |
| `language_code` | String |  | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `automated_agent_config` | String |  | Configuration for an automated agent to use with this profile. |
| `new_message_event_notification_config` | String |  | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `human_agent_assistant_config` | String |  | Configuration for agent assistance to use with this profile. |
| `security_settings` | String |  | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `stt_config` | String |  | Settings for speech transcription. |
| `notification_config` | String |  | Configuration for publishing conversation lifecycle events. |
| `time_zone` | String |  | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `parent` | String | ✅ | Required. The project to create a conversation profile for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the conversation profile. |
| `name` | String | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `display_name` | String | Required. Human readable name for this profile. Max length 1024 bytes. |
| `tts_config` | String | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `new_recognition_result_notification_config` | String | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `human_agent_handoff_config` | String | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `update_time` | String | Output only. Update time of the conversation profile. |
| `logging_config` | String | Configuration for logging conversation lifecycle events. |
| `language_code` | String | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `automated_agent_config` | String | Configuration for an automated agent to use with this profile. |
| `new_message_event_notification_config` | String | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `human_agent_assistant_config` | String | Configuration for agent assistance to use with this profile. |
| `security_settings` | String | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `stt_config` | String | Settings for speech transcription. |
| `notification_config` | String | Configuration for publishing conversation lifecycle events. |
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
conversation_profile_name = conversation_profile.name
conversation_profile_display_name = conversation_profile.display_name
conversation_profile_tts_config = conversation_profile.tts_config
conversation_profile_new_recognition_result_notification_config = conversation_profile.new_recognition_result_notification_config
conversation_profile_human_agent_handoff_config = conversation_profile.human_agent_handoff_config
conversation_profile_update_time = conversation_profile.update_time
conversation_profile_logging_config = conversation_profile.logging_config
conversation_profile_language_code = conversation_profile.language_code
conversation_profile_automated_agent_config = conversation_profile.automated_agent_config
conversation_profile_new_message_event_notification_config = conversation_profile.new_message_event_notification_config
conversation_profile_human_agent_assistant_config = conversation_profile.human_agent_assistant_config
conversation_profile_security_settings = conversation_profile.security_settings
conversation_profile_stt_config = conversation_profile.stt_config
conversation_profile_notification_config = conversation_profile.notification_config
conversation_profile_time_zone = conversation_profile.time_zone
```

---


### Evaluation

Creates evaluation of a generator.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `complete_time` | String |  | Output only. Completion time of this generator evaluation. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the model. The field is an aggregated value of ZI status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `initial_generator` | String |  | Required. The initial generator that was used when creating this evaluation. This is a copy of the generator read from storage when creating the evaluation. |
| `display_name` | String |  | Optional. The display name of the generator evaluation. At most 64 bytes long. |
| `name` | String |  | Output only. Identifier. The resource name of the evaluation. Format: `projects//locations//generators// evaluations/` |
| `evaluation_status` | String |  | Output only. The result status of the evaluation pipeline. Provides the status information including if the evaluation is still in progress, completed or failed with certain error and user actionable message. |
| `create_time` | String |  | Output only. Creation time of this generator evaluation. |
| `generator_evaluation_config` | String |  | Required. The configuration of the evaluation task. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the model. The field is an aggregated value of ZS status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `summarization_metrics` | String |  | Output only. Only available when the summarization generator is provided. |
| `parent` | String | ✅ | Required. The generator resource name. Format: `projects//locations//generators/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `complete_time` | String | Output only. Completion time of this generator evaluation. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the model. The field is an aggregated value of ZI status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `initial_generator` | String | Required. The initial generator that was used when creating this evaluation. This is a copy of the generator read from storage when creating the evaluation. |
| `display_name` | String | Optional. The display name of the generator evaluation. At most 64 bytes long. |
| `name` | String | Output only. Identifier. The resource name of the evaluation. Format: `projects//locations//generators// evaluations/` |
| `evaluation_status` | String | Output only. The result status of the evaluation pipeline. Provides the status information including if the evaluation is still in progress, completed or failed with certain error and user actionable message. |
| `create_time` | String | Output only. Creation time of this generator evaluation. |
| `generator_evaluation_config` | String | Required. The configuration of the evaluation task. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the model. The field is an aggregated value of ZS status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `summarization_metrics` | String | Output only. Only available when the summarization generator is provided. |


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
evaluation_complete_time = evaluation.complete_time
evaluation_satisfies_pzi = evaluation.satisfies_pzi
evaluation_initial_generator = evaluation.initial_generator
evaluation_display_name = evaluation.display_name
evaluation_name = evaluation.name
evaluation_evaluation_status = evaluation.evaluation_status
evaluation_create_time = evaluation.create_time
evaluation_generator_evaluation_config = evaluation.generator_evaluation_config
evaluation_satisfies_pzs = evaluation.satisfies_pzs
evaluation_summarization_metrics = evaluation.summarization_metrics
```

---


### Knowledge_base

Creates a knowledge base.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String |  | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, expect this to be present for non en-us languages. When unspecified, the default language code en-us applies. |
| `display_name` | String |  | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `name` | String |  | The knowledge base resource name. The name must be empty when creating a knowledge base. Format: `projects//locations//knowledgeBases/`. |
| `parent` | String | ✅ | Required. The project to create a knowledge base for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `language_code` | String | Language which represents the KnowledgeBase. When the KnowledgeBase is created/updated, expect this to be present for non en-us languages. When unspecified, the default language code en-us applies. |
| `display_name` | String | Required. The display name of the knowledge base. The name must be 1024 bytes or less; otherwise, the creation request fails. |
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
knowledge_base_language_code = knowledge_base.language_code
knowledge_base_display_name = knowledge_base.display_name
knowledge_base_name = knowledge_base.name
```

---


### Document

Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_auto_reload` | bool |  | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `name` | String |  | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `latest_reload_status` | String |  | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `state` | String |  | Output only. The current state of the document. |
| `knowledge_types` | Vec<String> |  | Required. The knowledge type of document content. |
| `content_uri` | String |  | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `raw_content` | String |  | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `display_name` | String |  | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
| `mime_type` | String |  | Required. The MIME type of this document. |
| `parent` | String | ✅ | Required. The knowledge base to create a document for. Format: `projects//locations//knowledgeBases/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_auto_reload` | bool | Optional. If true, we try to automatically reload the document every day (at a time picked by the system). If false or unspecified, we don't try to automatically reload the document. Currently you can only enable automatic reload for documents sourced from a public url, see `source` field for the source types. Reload status can be tracked in `latest_reload_status`. If a reload fails, we will keep the document unchanged. If a reload fails with internal errors, the system will try to reload the document on the next day. If a reload fails with non-retriable errors (e.g. PERMISSION_DENIED), the system will not try to reload the document anymore. You need to manually reload the document successfully by calling `ReloadDocument` and clear the errors. |
| `name` | String | Optional. The document resource name. The name must be empty when creating a document. Format: `projects//locations//knowledgeBases//documents/`. |
| `latest_reload_status` | String | Output only. The time and status of the latest reload. This reload may have been triggered automatically or manually and may not have succeeded. |
| `state` | String | Output only. The current state of the document. |
| `knowledge_types` | Vec<String> | Required. The knowledge type of document content. |
| `content_uri` | String | The URI where the file content is located. For documents stored in Google Cloud Storage, these URIs must have the form `gs:///`. NOTE: External URLs must correspond to public webpages, i.e., they must be indexed by Google Search. In particular, URLs for showing documents in Google Cloud Storage (i.e. the URL in your browser) are not supported. Instead use the `gs://` format URI described above. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the document. The metadata supports arbitrary key-value pairs. Suggested use cases include storing a document's title, an external URL distinct from the document's content_uri, etc. The max size of a `key` or a `value` of the metadata is 1024 bytes. |
| `raw_content` | String | The raw content of the document. This field is only permitted for EXTRACTIVE_QA and FAQ knowledge types. |
| `display_name` | String | Required. The display name of the document. The name must be 1024 bytes or less; otherwise, the creation request fails. |
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
document_name = document.name
document_latest_reload_status = document.latest_reload_status
document_state = document.state
document_knowledge_types = document.knowledge_types
document_content_uri = document.content_uri
document_metadata = document.metadata
document_raw_content = document.raw_content
document_display_name = document.display_name
document_mime_type = document.mime_type
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
| `next_page_token` | String | A token to retrieve next page of results. Or empty if there are no more results. Pass this value in the ListAnswerRecordsRequest.page_token field in the subsequent call to `ListAnswerRecords` method to retrieve the next page of results. |
| `answer_records` | Vec<String> | The list of answer records. |


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
answer_record_next_page_token = answer_record.next_page_token
answer_record_answer_records = answer_record.answer_records
```

---


### Conversation

Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ingested_context_references` | HashMap<String, String> |  | Output only. The context reference updates provided by external systems. |
| `start_time` | String |  | Output only. The time the conversation was started. |
| `conversation_profile` | String |  | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `phone_number` | String |  | Output only. It will not be empty if the conversation is to be connected over telephony. |
| `end_time` | String |  | Output only. The time the conversation was finished. |
| `name` | String |  | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `telephony_connection_info` | String |  | Output only. The telephony connection information. |
| `conversation_stage` | String |  | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `lifecycle_state` | String |  | Output only. The current state of the Conversation. |
| `parent` | String | ✅ | Required. Resource identifier of the project creating the conversation. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingested_context_references` | HashMap<String, String> | Output only. The context reference updates provided by external systems. |
| `start_time` | String | Output only. The time the conversation was started. |
| `conversation_profile` | String | Required. The Conversation Profile to be used to configure this Conversation. This field cannot be updated. Format: `projects//locations//conversationProfiles/`. |
| `phone_number` | String | Output only. It will not be empty if the conversation is to be connected over telephony. |
| `end_time` | String | Output only. The time the conversation was finished. |
| `name` | String | Output only. Identifier. The unique identifier of this conversation. Format: `projects//locations//conversations/`. |
| `telephony_connection_info` | String | Output only. The telephony connection information. |
| `conversation_stage` | String | Optional. The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE. |
| `lifecycle_state` | String | Output only. The current state of the Conversation. |


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
conversation_ingested_context_references = conversation.ingested_context_references
conversation_start_time = conversation.start_time
conversation_conversation_profile = conversation.conversation_profile
conversation_phone_number = conversation.phone_number
conversation_end_time = conversation.end_time
conversation_name = conversation.name
conversation_telephony_connection_info = conversation.telephony_connection_info
conversation_conversation_stage = conversation.conversation_stage
conversation_lifecycle_state = conversation.lifecycle_state
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Location

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/`. |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `display_name` | String |  | Required. The name of this agent. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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

# Create location
location = provider.dialogflow_api.Location {
    parent = "value"  # Required. The project of this agent. Format: `projects/`.
}

# Access location outputs
location_id = location.id
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Version

Creates an agent version. The new version points to the agent instance in the "default" environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The developer-provided description of this version. |
| `name` | String |  | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `version_number` | i64 |  | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `status` | String |  | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `create_time` | String |  | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |
| `parent` | String | ✅ | Required. The agent to create a version for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The developer-provided description of this version. |
| `name` | String | Output only. The unique identifier of this agent version. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `version_number` | i64 | Output only. The sequential number of this version. This field is read-only which means it cannot be set by create and update methods. |
| `status` | String | Output only. The status of this version. This field is read-only and cannot be set by create and update methods. |
| `create_time` | String | Output only. The creation time of this version. This field is read-only, i.e., it cannot be set by create and update methods. |


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
version_description = version.description
version_name = version.name
version_version_number = version.version_number
version_status = version.status
version_create_time = version.create_time
```

---


### Session

Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_params` | String |  | The parameters of this query. |
| `output_audio_config` | String |  | Instructs the speech synthesizer how to generate the output audio. If this field is not set and agent-level speech synthesizer is not configured, no output audio is generated. |
| `output_audio_config_mask` | String |  | Mask for output_audio_config indicating which settings in this request-level config should override speech synthesizer settings defined at agent-level. If unspecified or empty, output_audio_config replaces the agent-level config in its entirety. |
| `query_input` | String |  | Required. The input specification. It can be set to: 1. an audio config which instructs the speech recognizer how to process the speech audio, 2. a conversational query in the form of text, or 3. an event that specifies which intent to trigger. |
| `input_audio` | String |  | The natural language speech audio to be processed. This field should be populated iff `query_input` is set to an input audio config. A single request can contain up to 1 minute of speech audio data. |
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


### Evaluation

Creates evaluation of a generator.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Creation time of this generator evaluation. |
| `initial_generator` | String |  | Required. The initial generator that was used when creating this evaluation. This is a copy of the generator read from storage when creating the evaluation. |
| `name` | String |  | Output only. Identifier. The resource name of the evaluation. Format: `projects//locations//generators// evaluations/` |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the model. The field is an aggregated value of ZS status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `generator_evaluation_config` | String |  | Required. The configuration of the evaluation task. |
| `evaluation_status` | String |  | Output only. The result status of the evaluation pipeline. Provides the status information including if the evaluation is still in progress, completed or failed with certain error and user actionable message. |
| `display_name` | String |  | Optional. The display name of the generator evaluation. At most 64 bytes long. |
| `complete_time` | String |  | Output only. Completion time of this generator evaluation. |
| `summarization_metrics` | String |  | Output only. Only available when the summarization generator is provided. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the model. The field is an aggregated value of ZI status of its underlying dependencies. See more details in go/zicy-resource-placement#resource-status |
| `parent` | String | ✅ | Required. The generator resource name. Format: `projects//locations//generators/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time of this model. |
| `name` | String | The resource name of the evaluation. Format: `projects//conversationModels//evaluations/` |
| `evaluation_config` | String | Optional. The configuration of the evaluation task. |
| `raw_human_eval_template_csv` | String | Output only. Human eval template in csv format. It takes real-world conversations provided through input dataset, generates example suggestions for customer to verify quality of the model. For Smart Reply, the generated csv file contains columns of Context, (Suggestions,Q1,Q2)*3, Actual reply. Context contains at most 10 latest messages in the conversation prior to the current suggestion. Q1: "Would you send it as the next message of agent?" Evaluated based on whether the suggest is appropriate to be sent by agent in current context. Q2: "Does the suggestion move the conversation closer to resolution?" Evaluated based on whether the suggestion provide solutions, or answers customer's question or collect information from customer to resolve the customer's issue. Actual reply column contains the actual agent reply sent in the context. |
| `smart_reply_metrics` | String | Output only. Only available when model is for smart reply. |
| `display_name` | String | Optional. The display name of the model evaluation. At most 64 bytes long. |


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
evaluation_create_time = evaluation.create_time
evaluation_name = evaluation.name
evaluation_evaluation_config = evaluation.evaluation_config
evaluation_raw_human_eval_template_csv = evaluation.raw_human_eval_template_csv
evaluation_smart_reply_metrics = evaluation.smart_reply_metrics
evaluation_display_name = evaluation.display_name
```

---


### Agent

Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_uri` | String |  | Required. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline. Dialogflow performs a write operation for the Cloud Storage object on the caller's behalf, so your request authentication must have write permissions for the object. For more information, see [Dialogflow access control](https://cloud.google.com/dialogflow/cx/docs/concept/access-control#storage). |
| `parent` | String | ✅ | Required. The project that the agent to export is associated with. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validation_errors` | Vec<String> | Contains all validation errors. |


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
    parent = "value"  # Required. The project that the agent to export is associated with. Format: `projects/`.
}

# Access agent outputs
agent_id = agent.id
agent_validation_errors = agent.validation_errors
```

---


### Intent

Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_contexts` | Vec<String> |  | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `followup_intent_info` | Vec<String> |  | Output only. Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `ml_disabled` | bool |  | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `priority` | i64 |  | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `live_agent_handoff` | bool |  | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `parameters` | Vec<String> |  | Optional. The collection of parameters associated with the intent. |
| `action` | String |  | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `default_response_platforms` | Vec<String> |  | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `is_fallback` | bool |  | Optional. Indicates whether this is a fallback intent. |
| `parent_followup_intent_name` | String |  | Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `end_interaction` | bool |  | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `reset_contexts` | bool |  | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `messages` | Vec<String> |  | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `display_name` | String |  | Required. The name of this intent. |
| `training_phrases` | Vec<String> |  | Optional. The collection of examples that the agent is trained on. |
| `name` | String |  | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`. |
| `events` | Vec<String> |  | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `webhook_state` | String |  | Optional. Indicates whether webhooks are enabled for the intent. |
| `input_context_names` | Vec<String> |  | Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`. |
| `root_followup_intent_name` | String |  | Output only. Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`. |
| `parent` | String | ✅ | Required. The agent to create a intent for. Format: `projects//agent`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `output_contexts` | Vec<String> | Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`. |
| `followup_intent_info` | Vec<String> | Output only. Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output. |
| `ml_disabled` | bool | Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. |
| `priority` | i64 | Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `live_agent_handoff` | bool | Optional. Indicates that a live agent should be brought in to handle the interaction with the user. In most cases, when you set this flag to true, you would also want to set end_interaction to true as well. Default is false. |
| `parameters` | Vec<String> | Optional. The collection of parameters associated with the intent. |
| `action` | String | Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces. |
| `default_response_platforms` | Vec<String> | Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform). |
| `is_fallback` | bool | Optional. Indicates whether this is a fallback intent. |
| `parent_followup_intent_name` | String | Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`. |
| `end_interaction` | bool | Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false. |
| `reset_contexts` | bool | Optional. Indicates whether to delete all contexts in the current session when this intent is matched. |
| `messages` | Vec<String> | Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console. |
| `display_name` | String | Required. The name of this intent. |
| `training_phrases` | Vec<String> | Optional. The collection of examples that the agent is trained on. |
| `name` | String | Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`. |
| `events` | Vec<String> | Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters. |
| `webhook_state` | String | Optional. Indicates whether webhooks are enabled for the intent. |
| `input_context_names` | Vec<String> | Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`. |
| `root_followup_intent_name` | String | Output only. Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`. |


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
intent_output_contexts = intent.output_contexts
intent_followup_intent_info = intent.followup_intent_info
intent_ml_disabled = intent.ml_disabled
intent_priority = intent.priority
intent_live_agent_handoff = intent.live_agent_handoff
intent_parameters = intent.parameters
intent_action = intent.action
intent_default_response_platforms = intent.default_response_platforms
intent_is_fallback = intent.is_fallback
intent_parent_followup_intent_name = intent.parent_followup_intent_name
intent_end_interaction = intent.end_interaction
intent_reset_contexts = intent.reset_contexts
intent_messages = intent.messages
intent_display_name = intent.display_name
intent_training_phrases = intent.training_phrases
intent_name = intent.name
intent_events = intent.events
intent_webhook_state = intent.webhook_state
intent_input_context_names = intent.input_context_names
intent_root_followup_intent_name = intent.root_followup_intent_name
```

---


### Conversation_dataset

Creates a new conversation dataset. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationDatasetOperationMetadata - `response`: ConversationDataset

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conversation_info` | String |  | Output only. Metadata set during conversation data import. |
| `name` | String |  | Output only. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/` |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the dataset. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the dataset. |
| `display_name` | String |  | Required. The display name of the dataset. Maximum of 64 bytes. |
| `conversation_count` | String |  | Output only. The number of conversations this conversation dataset contains. |
| `description` | String |  | Optional. The description of the dataset. Maximum of 10000 bytes. |
| `input_config` | String |  | Output only. Input configurations set during conversation data import. |
| `create_time` | String |  | Output only. Creation time of this dataset. |
| `parent` | String | ✅ | Required. The project to create conversation dataset for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conversation_info` | String | Output only. Metadata set during conversation data import. |
| `name` | String | Output only. ConversationDataset resource name. Format: `projects//locations//conversationDatasets/` |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the dataset. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the dataset. |
| `display_name` | String | Required. The display name of the dataset. Maximum of 64 bytes. |
| `conversation_count` | String | Output only. The number of conversations this conversation dataset contains. |
| `description` | String | Optional. The description of the dataset. Maximum of 10000 bytes. |
| `input_config` | String | Output only. Input configurations set during conversation data import. |
| `create_time` | String | Output only. Creation time of this dataset. |


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
conversation_dataset_conversation_info = conversation_dataset.conversation_info
conversation_dataset_name = conversation_dataset.name
conversation_dataset_satisfies_pzs = conversation_dataset.satisfies_pzs
conversation_dataset_satisfies_pzi = conversation_dataset.satisfies_pzi
conversation_dataset_display_name = conversation_dataset.display_name
conversation_dataset_conversation_count = conversation_dataset.conversation_count
conversation_dataset_description = conversation_dataset.description
conversation_dataset_input_config = conversation_dataset.input_config
conversation_dataset_create_time = conversation_dataset.create_time
```

---


### Generator

Creates a generator.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suggestion_deduping_config` | String |  | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `free_form_context` | String |  | Input of free from generator to LLM. |
| `published_model` | String |  | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `name` | String |  | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `create_time` | String |  | Output only. Creation time of this generator. |
| `description` | String |  | Optional. Human readable description of the generator. |
| `tools` | Vec<String> |  | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `summarization_context` | String |  | Input of prebuilt Summarization feature. |
| `inference_parameter` | String |  | Optional. Inference parameters for this generator. |
| `trigger_event` | String |  | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String |  | Output only. Update time of this generator. |
| `agent_coaching_context` | String |  | Input of prebuilt Agent Coaching feature. |
| `parent` | String | ✅ | Required. The project/location to create generator for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suggestion_deduping_config` | String | Optional. Configuration for suggestion deduping. This is only applicable to AI Coach feature. |
| `free_form_context` | String | Input of free from generator to LLM. |
| `published_model` | String | Optional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: `text-bison` * To use a stable model version, specify the version number as well. Example: `text-bison@002`. |
| `name` | String | Output only. Identifier. The resource name of the generator. Format: `projects//locations//generators/` |
| `create_time` | String | Output only. Creation time of this generator. |
| `description` | String | Optional. Human readable description of the generator. |
| `tools` | Vec<String> | Optional. Resource names of the tools that the generator can choose from. Format: `projects//locations//tools/`. |
| `summarization_context` | String | Input of prebuilt Summarization feature. |
| `inference_parameter` | String | Optional. Inference parameters for this generator. |
| `trigger_event` | String | Optional. The trigger event of the generator. It defines when the generator is triggered in a conversation. |
| `update_time` | String | Output only. Update time of this generator. |
| `agent_coaching_context` | String | Input of prebuilt Agent Coaching feature. |


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
generator_suggestion_deduping_config = generator.suggestion_deduping_config
generator_free_form_context = generator.free_form_context
generator_published_model = generator.published_model
generator_name = generator.name
generator_create_time = generator.create_time
generator_description = generator.description
generator_tools = generator.tools
generator_summarization_context = generator.summarization_context
generator_inference_parameter = generator.inference_parameter
generator_trigger_event = generator.trigger_event
generator_update_time = generator.update_time
generator_agent_coaching_context = generator.agent_coaching_context
```

---


### Entity_type

Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_fuzzy_extraction` | bool |  | Optional. Enables fuzzy entity extraction during classification. |
| `display_name` | String |  | Required. The name of the entity type. |
| `auto_expansion_mode` | String |  | Optional. Indicates whether the entity type can be automatically expanded. |
| `entities` | Vec<String> |  | Optional. The collection of entity entries associated with the entity type. |
| `kind` | String |  | Required. Indicates the kind of entity type. |
| `name` | String |  | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Format: `projects//agent/entityTypes/`. |
| `parent` | String | ✅ | Required. The agent to create a entity type for. Format: `projects//agent`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_fuzzy_extraction` | bool | Optional. Enables fuzzy entity extraction during classification. |
| `display_name` | String | Required. The name of the entity type. |
| `auto_expansion_mode` | String | Optional. Indicates whether the entity type can be automatically expanded. |
| `entities` | Vec<String> | Optional. The collection of entity entries associated with the entity type. |
| `kind` | String | Required. Indicates the kind of entity type. |
| `name` | String | The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Format: `projects//agent/entityTypes/`. |


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
    parent = "value"  # Required. The agent to create a entity type for. Format: `projects//agent`.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_enable_fuzzy_extraction = entity_type.enable_fuzzy_extraction
entity_type_display_name = entity_type.display_name
entity_type_auto_expansion_mode = entity_type.auto_expansion_mode
entity_type_entities = entity_type.entities
entity_type_kind = entity_type.kind
entity_type_name = entity_type.name
```

---


### Stateless_suggestion

Generates and returns a suggestion for a conversation that does not have a resource created for it.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_settings` | String |  | Optional. Name of the CX SecuritySettings which is used to redact generated response. If this field is empty, try to fetch v2 security_settings, which is a project level setting. If this field is empty and no v2 security_settings set up in this project, no redaction will be done. Format: `projects//locations//securitySettings/`. |
| `context_references` | HashMap<String, String> |  | Optional. A section of ingested context information. The key is the name of the context reference and the value contains the contents of the context reference. The key is used to incorporate ingested context references to enhance the generator. |
| `generator` | String |  | Uncreated generator. It should be a complete generator that includes all information about the generator. |
| `generator_name` | String |  | The resource name of the existing created generator. Format: `projects//locations//generators/` |
| `conversation_context` | String |  | Optional. Context of the conversation, including transcripts. |
| `trigger_events` | Vec<String> |  | Optional. A list of trigger events. Generator will be triggered only if it's trigger event is included here. |
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


### Conversation_profile

Creates a conversation profile in the specified project. ConversationProfile.create_time and ConversationProfile.update_time aren't populated in the response. You can retrieve them via GetConversationProfile API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_recognition_result_notification_config` | String |  | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `display_name` | String |  | Required. Human readable name for this profile. Max length 1024 bytes. |
| `time_zone` | String |  | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `logging_config` | String |  | Configuration for logging conversation lifecycle events. |
| `language_code` | String |  | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-US languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `human_agent_handoff_config` | String |  | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `create_time` | String |  | Output only. Create time of the conversation profile. |
| `stt_config` | String |  | Settings for speech transcription. |
| `update_time` | String |  | Output only. Update time of the conversation profile. |
| `automated_agent_config` | String |  | Configuration for an automated agent to use with this profile. |
| `human_agent_assistant_config` | String |  | Configuration for agent assistance to use with this profile. |
| `notification_config` | String |  | Configuration for publishing conversation lifecycle events. |
| `name` | String |  | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `security_settings` | String |  | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `new_message_event_notification_config` | String |  | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `tts_config` | String |  | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |
| `parent` | String | ✅ | Required. The project to create a conversation profile for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `new_recognition_result_notification_config` | String | Optional. Configuration for publishing transcription intermediate results. Event will be sent in format of ConversationEvent. If configured, the following information will be populated as ConversationEvent Pub/Sub message attributes: - "participant_id" - "participant_role" - "message_id" |
| `display_name` | String | Required. Human readable name for this profile. Max length 1024 bytes. |
| `time_zone` | String | The time zone of this conversational profile from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. Defaults to America/New_York. |
| `logging_config` | String | Configuration for logging conversation lifecycle events. |
| `language_code` | String | Language code for the conversation profile. If not specified, the language is en-US. Language at ConversationProfile should be set for all non en-US languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `human_agent_handoff_config` | String | Configuration for connecting to a live agent. Currently, this feature is not general available, please contact Google to get access. |
| `create_time` | String | Output only. Create time of the conversation profile. |
| `stt_config` | String | Settings for speech transcription. |
| `update_time` | String | Output only. Update time of the conversation profile. |
| `automated_agent_config` | String | Configuration for an automated agent to use with this profile. |
| `human_agent_assistant_config` | String | Configuration for agent assistance to use with this profile. |
| `notification_config` | String | Configuration for publishing conversation lifecycle events. |
| `name` | String | The unique identifier of this conversation profile. Format: `projects//locations//conversationProfiles/`. |
| `security_settings` | String | Name of the CX SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `new_message_event_notification_config` | String | Configuration for publishing new message events. Event will be sent in format of ConversationEvent |
| `tts_config` | String | Configuration for Text-to-Speech synthesization. Used by Phone Gateway to specify synthesization options. If agent defines synthesization options as well, agent settings overrides the option here. |


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
conversation_profile_new_recognition_result_notification_config = conversation_profile.new_recognition_result_notification_config
conversation_profile_display_name = conversation_profile.display_name
conversation_profile_time_zone = conversation_profile.time_zone
conversation_profile_logging_config = conversation_profile.logging_config
conversation_profile_language_code = conversation_profile.language_code
conversation_profile_human_agent_handoff_config = conversation_profile.human_agent_handoff_config
conversation_profile_create_time = conversation_profile.create_time
conversation_profile_stt_config = conversation_profile.stt_config
conversation_profile_update_time = conversation_profile.update_time
conversation_profile_automated_agent_config = conversation_profile.automated_agent_config
conversation_profile_human_agent_assistant_config = conversation_profile.human_agent_assistant_config
conversation_profile_notification_config = conversation_profile.notification_config
conversation_profile_name = conversation_profile.name
conversation_profile_security_settings = conversation_profile.security_settings
conversation_profile_new_message_event_notification_config = conversation_profile.new_message_event_notification_config
conversation_profile_tts_config = conversation_profile.tts_config
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


### Tool

Creates a tool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extension_spec` | String |  | Vertex extension tool specification. |
| `open_api_spec` | String |  | OpenAPI tool. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String |  | Output only. Update time of this tool. |
| `connector_spec` | String |  | Integration connectors tool specification. |
| `action_confirmation_requirement` | HashMap<String, String> |  | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `display_name` | String |  | Optional. A human readable short name of the tool, to be shown on the UI. |
| `create_time` | String |  | Output only. Creation time of this tool. |
| `name` | String |  | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `tool_key` | String |  | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `description` | String |  | Optional. A human readable description of the tool. |
| `function_spec` | String |  | Client side executed function specification. |
| `parent` | String | ✅ | Required. The project/location to create tool for. Format: `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extension_spec` | String | Vertex extension tool specification. |
| `open_api_spec` | String | OpenAPI tool. |
| `satisfies_pzs` | bool | Output only. A read only boolean field reflecting Zone Separation status of the tool. If the field is absent, it means the status is unknown. |
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the tool. If the field is absent, it means the status is unknown. |
| `update_time` | String | Output only. Update time of this tool. |
| `connector_spec` | String | Integration connectors tool specification. |
| `action_confirmation_requirement` | HashMap<String, String> | Optional. Confirmation requirement for the actions. Each key is an action name in the action_schemas. If an action's confirmation requirement is unspecified (either the key is not present, or its value is CONFIRMATION_REQUIREMENT_UNSPECIFIED), the requirement is inferred from the action's method_type - confirmation is not required if and only if method_type is GET. |
| `display_name` | String | Optional. A human readable short name of the tool, to be shown on the UI. |
| `create_time` | String | Output only. Creation time of this tool. |
| `name` | String | Output only. Identifier. The resource name of the tool. Format: `projects//locations//tools/`. |
| `tool_key` | String | Required. A human readable short name of the tool, which should be unique within the project. It should only contain letters, numbers, and underscores, and it will be used by LLM to identify the tool. |
| `description` | String | Optional. A human readable description of the tool. |
| `function_spec` | String | Client side executed function specification. |


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
tool_extension_spec = tool.extension_spec
tool_open_api_spec = tool.open_api_spec
tool_satisfies_pzs = tool.satisfies_pzs
tool_satisfies_pzi = tool.satisfies_pzi
tool_update_time = tool.update_time
tool_connector_spec = tool.connector_spec
tool_action_confirmation_requirement = tool.action_confirmation_requirement
tool_display_name = tool.display_name
tool_create_time = tool.create_time
tool_name = tool.name
tool_tool_key = tool.tool_key
tool_description = tool.description
tool_function_spec = tool.function_spec
```

---


### Project

Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `supported_language_codes` | Vec<String> |  | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `avatar_uri` | String |  | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `description` | String |  | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `api_version` | String |  | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `default_language_code` | String |  | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `enable_logging` | bool |  | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String |  | Optional. Determines how intents are detected from user queries. |
| `parent` | String |  | Required. The project of this agent. Format: `projects/`. |
| `tier` | String |  | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String |  | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `display_name` | String |  | Required. The name of this agent. |
| `classification_threshold` | f64 |  | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |
| `parent` | String | ✅ | Required. The project of this agent. Format: `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_language_codes` | Vec<String> | Optional. The list of all languages supported by this agent (except for the `default_language_code`). |
| `avatar_uri` | String | Optional. The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `description` | String | Optional. The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `api_version` | String | Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version. |
| `default_language_code` | String | Required. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the `Update` method. |
| `enable_logging` | bool | Optional. Determines whether this agent should log conversation queries. |
| `match_mode` | String | Optional. Determines how intents are detected from user queries. |
| `parent` | String | Required. The project of this agent. Format: `projects/`. |
| `tier` | String | Optional. The agent tier. If not specified, TIER_STANDARD is assumed. |
| `time_zone` | String | Required. The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `display_name` | String | Required. The name of this agent. |
| `classification_threshold` | f64 | Optional. To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a fallback intent will be triggered or, if there are no fallback intents defined, no intent will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used. |


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
project_supported_language_codes = project.supported_language_codes
project_avatar_uri = project.avatar_uri
project_description = project.description
project_api_version = project.api_version
project_default_language_code = project.default_language_code
project_enable_logging = project.enable_logging
project_match_mode = project.match_mode
project_parent = project.parent
project_tier = project.tier
project_time_zone = project.time_zone
project_display_name = project.display_name
project_classification_threshold = project.classification_threshold
```

---


### Entitie

Updates or creates multiple entities in the specified entity type. This method does not affect entities in the entity type that aren't explicitly specified in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training). 

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | Optional. The mask to control which fields get updated. |
| `language_code` | String |  | Optional. The language used to access language-specific data. If not specified, the agent's default language is used. For more information, see [Multilingual intent and entity data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity). |
| `entities` | Vec<String> |  | Required. The entities to update or create. |
| `parent` | String | ✅ | Required. The name of the entity type to update or create entities in. Format: `projects//agent/entityTypes/`. |



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
    parent = "value"  # Required. The name of the entity type to update or create entities in. Format: `projects//agent/entityTypes/`.
}

```

---


### Suggestion

Suggests summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context_size` | i64 |  | Optional. Max number of messages prior to and including [latest_message] to use as context when compiling the suggestion. By default 500 and at most 1000. |
| `latest_message` | String |  | Optional. The name of the latest conversation message used as context for compiling suggestion. If empty, the latest message of the conversation will be used. Format: `projects//locations//conversations//messages/`. |
| `assist_query_params` | String |  | Optional. Parameters for a human assist query. Only used for POC/demo purpose. |
| `conversation` | String | ✅ | Required. The conversation to fetch suggestion for. Format: `projects//locations//conversations/`. |



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
    conversation = "value"  # Required. The conversation to fetch suggestion for. Format: `projects//locations//conversations/`.
}

```

---


### Environment

Creates an agent environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `state` | String |  | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `fulfillment` | String |  | Optional. The fulfillment settings to use for this environment. |
| `update_time` | String |  | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String |  | Optional. Text to speech settings for this environment. |
| `agent_version` | String |  | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `name` | String |  | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`. |
| `parent` | String | ✅ | Required. The agent to create an environment for. Supported formats: - `projects//agent` - `projects//locations//agent` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The developer-provided description for this environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `state` | String | Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `fulfillment` | String | Optional. The fulfillment settings to use for this environment. |
| `update_time` | String | Output only. The last update time of this environment. This field is read-only, i.e., it cannot be set by create and update methods. |
| `text_to_speech_settings` | String | Optional. Text to speech settings for this environment. |
| `agent_version` | String | Optional. The agent version loaded into this environment. Supported formats: - `projects//agent/versions/` - `projects//locations//agent/versions/` |
| `name` | String | Output only. The unique identifier of this agent environment. Supported formats: - `projects//agent/environments/` - `projects//locations//agent/environments/` The environment ID for the default environment is `-`. |


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
environment_state = environment.state
environment_fulfillment = environment.fulfillment
environment_update_time = environment.update_time
environment_text_to_speech_settings = environment.text_to_speech_settings
environment_agent_version = environment.agent_version
environment_name = environment.name
```

---


### Context

Creates a context. If the specified context already exists, overrides the context.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `parameters` | HashMap<String, String> |  | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
| `lifespan_count` | i64 |  | Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries. |
| `parent` | String | ✅ | Required. The session to create a context for. Format: `projects//agent/sessions/` or `projects//agent/environments//users//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in `a-zA-Z0-9_-%` and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size` |
| `parameters` | HashMap<String, String> | Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: * MapKey type: string * MapKey value: parameter name * MapValue type: If parameter's entity type is a composite entity then use map, otherwise, depending on the parameter value type, it could be one of string, number, boolean, null, list or map. * MapValue value: If parameter's entity type is a composite entity then use map from composite entity property names to property values, otherwise, use parameter value. |
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
context_name = context.name
context_parameters = context.parameters
context_lifespan_count = context.lifespan_count
```

---


### Participant

Creates a new participant in a conversation.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `documents_metadata_filters` | HashMap<String, String> |  | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `role` | String |  | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String |  | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow returns an error if you try to add a user id for a non-END_USER participant. Dialogflow uses this user id for billing and measurement purposes. For example, Dialogflow determines whether a user in one conversation returned in a later conversation. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `sip_recording_media_label` | String |  | Optional. Label applied to streams representing this participant in SIPREC XML metadata and SDP. This is used to assign transcriptions from that media stream to this participant. This field can be updated. |
| `name` | String |  | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |
| `parent` | String | ✅ | Required. Resource identifier of the conversation adding the participant. Format: `projects//locations//conversations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents_metadata_filters` | HashMap<String, String> | Optional. Key-value filters on the metadata of documents returned by article suggestion. If specified, article suggestion only returns suggested documents that match all filters in their Document.metadata. Multiple values for a metadata key should be concatenated by comma. For example, filters to match all documents that have 'US' or 'CA' in their market metadata values and 'agent' in their user metadata values will be ``` documents_metadata_filters { key: "market" value: "US,CA" } documents_metadata_filters { key: "user" value: "agent" } ``` |
| `role` | String | Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable. |
| `obfuscated_external_user_id` | String | Optional. Obfuscated user id that should be associated with the created participant. You can specify a user id as follows: 1. If you set this field in CreateParticipantRequest or UpdateParticipantRequest, Dialogflow adds the obfuscated user id with the participant. 2. If you set this field in AnalyzeContent or StreamingAnalyzeContent, Dialogflow will update Participant.obfuscated_external_user_id. Dialogflow returns an error if you try to add a user id for a non-END_USER participant. Dialogflow uses this user id for billing and measurement purposes. For example, Dialogflow determines whether a user in one conversation returned in a later conversation. Note: * Please never pass raw user ids to Dialogflow. Always obfuscate your user id first. * Dialogflow only accepts a UTF-8 encoded string, e.g., a hex digest of a hash function like SHA-512. * The length of the user id must be <= 256 characters. |
| `sip_recording_media_label` | String | Optional. Label applied to streams representing this participant in SIPREC XML metadata and SDP. This is used to assign transcriptions from that media stream to this participant. This field can be updated. |
| `name` | String | Optional. The unique identifier of this participant. Format: `projects//locations//conversations//participants/`. |


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
participant_role = participant.role
participant_obfuscated_external_user_id = participant.obfuscated_external_user_id
participant_sip_recording_media_label = participant.sip_recording_media_label
participant_name = participant.name
```

---


### Conversation_model

Creates a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationModelOperationMetadata - `response`: ConversationModel

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. A read only boolean field reflecting Zone Isolation status of the model. |
| `display_name` | String |  | Required. The display name of the model. At most 64 bytes long. |
| `datasets` | Vec<String> |  | Required. Datasets used to create model. |
| `name` | String |  | ConversationModel resource name. Format: `projects//conversationModels/` |
| `smart_reply_model_metadata` | String |  | Metadata for smart reply models. |
| `state` | String |  | Output only. State of the model. A model can only serve prediction requests after it gets deployed. |
| `article_suggestion_model_metadata` | String |  | Metadata for article suggestion models. |
| `create_time` | String |  | Output only. Creation time of this model. |
| `satisfies_pzs` | bool |  | Output only. A read only boolean field reflecting Zone Separation status of the model. |
| `language_code` | String |  | Language code for the conversation model. If not specified, the language is en-US. Language at ConversationModel should be set for all non en-us languages. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: "en-US". |
| `parent` | String | ✅ | The project to create conversation model for. Format: `projects/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. A read only boolean field reflecting Zone Isolation status of the model. |
| `display_name` | String | Required. The display name of the model. At most 64 bytes long. |
| `datasets` | Vec<String> | Required. Datasets used to create model. |
| `name` | String | ConversationModel resource name. Format: `projects//conversationModels/` |
| `smart_reply_model_metadata` | String | Metadata for smart reply models. |
| `state` | String | Output only. State of the model. A model can only serve prediction requests after it gets deployed. |
| `article_suggestion_model_metadata` | String | Metadata for article suggestion models. |
| `create_time` | String | Output only. Creation time of this model. |
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
conversation_model_satisfies_pzi = conversation_model.satisfies_pzi
conversation_model_display_name = conversation_model.display_name
conversation_model_datasets = conversation_model.datasets
conversation_model_name = conversation_model.name
conversation_model_smart_reply_model_metadata = conversation_model.smart_reply_model_metadata
conversation_model_state = conversation_model.state
conversation_model_article_suggestion_model_metadata = conversation_model.article_suggestion_model_metadata
conversation_model_create_time = conversation_model.create_time
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
| `display_name` | String |  | Optional. Human readable alias for this trunk. |
| `connections` | Vec<String> |  | Output only. Connections of the SIP trunk. |
| `name` | String |  | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `expected_hostname` | Vec<String> |  | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |
| `parent` | String | ✅ | Required. The location to create a SIP trunk for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. Human readable alias for this trunk. |
| `connections` | Vec<String> | Output only. Connections of the SIP trunk. |
| `name` | String | Identifier. The unique identifier of the SIP trunk. Format: `projects//locations//sipTrunks/`. |
| `expected_hostname` | Vec<String> | Required. The expected hostnames in the peer certificate from partner that is used for TLS authentication. |


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
sip_trunk_display_name = sip_trunk.display_name
sip_trunk_connections = sip_trunk.connections
sip_trunk_name = sip_trunk.name
sip_trunk_expected_hostname = sip_trunk.expected_hostname
```

---


### Version

Creates a Version in the specified Flow. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateVersionOperationMetadata - `response`: Version

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation. |
| `state` | String |  | Output only. The state of this version. This field is read-only and cannot be set by create and update methods. |
| `nlu_settings` | String |  | Output only. The NLU settings of the flow at version creation. |
| `description` | String |  | The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `create_time` | String |  | Output only. Create time of the version. |
| `display_name` | String |  | Required. The human-readable name of the version. Limit of 64 characters. |
| `parent` | String | ✅ | Required. The Flow to create an Version for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation. |
| `state` | String | Output only. The state of this version. This field is read-only and cannot be set by create and update methods. |
| `nlu_settings` | String | Output only. The NLU settings of the flow at version creation. |
| `description` | String | The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `create_time` | String | Output only. Create time of the version. |
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
version_name = version.name
version_state = version.state
version_nlu_settings = version.nlu_settings
version_description = version.description
version_create_time = version.create_time
version_display_name = version.display_name
```

---


### Flow

Creates a flow in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `locked` | bool |  | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `nlu_settings` | String |  | NLU related settings of the flow. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `description` | String |  | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `transition_routes` | Vec<String> |  | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this flow. |
| `multi_language_settings` | String |  | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this flow. |
| `transition_route_groups` | Vec<String> |  | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> |  | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `display_name` | String |  | Required. The human-readable name of the flow. |
| `name` | String |  | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |
| `parent` | String | ✅ | Required. The agent to create a flow for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locked` | bool | Indicates whether the flow is locked for changes. If the flow is locked, modifications to the flow will be rejected. |
| `nlu_settings` | String | NLU related settings of the flow. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `advanced_settings` | String | Hierarchical advanced settings for this flow. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `description` | String | The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `transition_routes` | Vec<String> | A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying "help" or "can I talk to a human?", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evaluated in the following order: * TransitionRoutes with intent specified. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this flow. |
| `multi_language_settings` | String | Optional. Multi-lingual agent settings for this flow. |
| `output_parameter_definitions` | Vec<String> | Optional. Defined structured output parameters for this flow. |
| `transition_route_groups` | Vec<String> | A flow's transition route group serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition route groups. Transition route groups defined in the page have higher priority than those defined in the flow. Format: `projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> | A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored. |
| `display_name` | String | Required. The human-readable name of the flow. |
| `name` | String | The unique identifier of the flow. Format: `projects//locations//agents//flows/`. |


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
flow_locked = flow.locked
flow_nlu_settings = flow.nlu_settings
flow_knowledge_connector_settings = flow.knowledge_connector_settings
flow_advanced_settings = flow.advanced_settings
flow_description = flow.description
flow_transition_routes = flow.transition_routes
flow_input_parameter_definitions = flow.input_parameter_definitions
flow_multi_language_settings = flow.multi_language_settings
flow_output_parameter_definitions = flow.output_parameter_definitions
flow_transition_route_groups = flow.transition_route_groups
flow_event_handlers = flow.event_handlers
flow_display_name = flow.display_name
flow_name = flow.name
```

---


### Playbook

Creates a playbook in a specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instruction` | String |  | Instruction to accomplish target goal. |
| `referenced_tools` | Vec<String> |  | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `inline_actions` | Vec<String> |  | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `playbook_type` | String |  | Optional. Type of the playbook. |
| `llm_model_settings` | String |  | Optional. Llm model settings for the playbook. |
| `referenced_flows` | Vec<String> |  | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `display_name` | String |  | Required. The human-readable name of the playbook, unique within an agent. |
| `code_block` | String |  | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `name` | String |  | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `handlers` | Vec<String> |  | Optional. A list of registered handlers to execuate based on the specified triggers. |
| `update_time` | String |  | Output only. Last time the playbook version was updated. |
| `create_time` | String |  | Output only. The timestamp of initial playbook creation. |
| `referenced_playbooks` | Vec<String> |  | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `input_parameter_definitions` | Vec<String> |  | Optional. Defined structured input parameters for this playbook. |
| `goal` | String |  | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
| `token_count` | String |  | Output only. Estimated number of tokes current playbook takes when sent to the LLM. |
| `output_parameter_definitions` | Vec<String> |  | Optional. Defined structured output parameters for this playbook. |
| `parent` | String | ✅ | Required. The agent to create a playbook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instruction` | String | Instruction to accomplish target goal. |
| `referenced_tools` | Vec<String> | Optional. The resource name of tools referenced by the current playbook in the instructions. If not provided explicitly, they are will be implied using the tool being referenced in goal and steps. |
| `inline_actions` | Vec<String> | Optional. Output only. Names of inline actions scoped to this playbook. These actions are in addition to those belonging to referenced tools, child playbooks, and flows, e.g. actions that are defined in the playbook's code block. |
| `playbook_type` | String | Optional. Type of the playbook. |
| `llm_model_settings` | String | Optional. Llm model settings for the playbook. |
| `referenced_flows` | Vec<String> | Output only. The resource name of flows referenced by the current playbook in the instructions. |
| `display_name` | String | Required. The human-readable name of the playbook, unique within an agent. |
| `code_block` | String | Optional. The playbook's scoped code block, which may implement handlers and actions. |
| `name` | String | The unique identifier of the playbook. Format: `projects//locations//agents//playbooks/`. |
| `handlers` | Vec<String> | Optional. A list of registered handlers to execuate based on the specified triggers. |
| `update_time` | String | Output only. Last time the playbook version was updated. |
| `create_time` | String | Output only. The timestamp of initial playbook creation. |
| `referenced_playbooks` | Vec<String> | Output only. The resource name of other playbooks referenced by the current playbook in the instructions. |
| `input_parameter_definitions` | Vec<String> | Optional. Defined structured input parameters for this playbook. |
| `goal` | String | Required. High level description of the goal the playbook intend to accomplish. A goal should be concise since it's visible to other playbooks that may reference this playbook. |
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
playbook_instruction = playbook.instruction
playbook_referenced_tools = playbook.referenced_tools
playbook_inline_actions = playbook.inline_actions
playbook_playbook_type = playbook.playbook_type
playbook_llm_model_settings = playbook.llm_model_settings
playbook_referenced_flows = playbook.referenced_flows
playbook_display_name = playbook.display_name
playbook_code_block = playbook.code_block
playbook_name = playbook.name
playbook_handlers = playbook.handlers
playbook_update_time = playbook.update_time
playbook_create_time = playbook.create_time
playbook_referenced_playbooks = playbook.referenced_playbooks
playbook_input_parameter_definitions = playbook.input_parameter_definitions
playbook_goal = playbook.goal
playbook_token_count = playbook.token_count
playbook_output_parameter_definitions = playbook.output_parameter_definitions
```

---


### Example

Creates an example in the specified playbook.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. The high level concise description of the example. The max number of characters is 200. |
| `conversation_state` | String |  | Required. Example's output state. |
| `create_time` | String |  | Output only. The timestamp of initial example creation. |
| `playbook_output` | String |  | Optional. The output of the playbook in the example. |
| `playbook_input` | String |  | Optional. The input to the playbook in the example. |
| `display_name` | String |  | Required. The display name of the example. |
| `name` | String |  | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `actions` | Vec<String> |  | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `update_time` | String |  | Output only. Last time the example was updated. |
| `token_count` | String |  | Output only. Estimated number of tokes current example takes when sent to the LLM. |
| `language_code` | String |  | Optional. The language code of the example. If not specified, the agent's default language is used. Note: languages must be enabled in the agent before they can be used. Note: example's language code is not currently used in dialogflow agents. |
| `parent` | String | ✅ | Required. The playbook to create an example for. Format: `projects//locations//agents//playbooks/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. The high level concise description of the example. The max number of characters is 200. |
| `conversation_state` | String | Required. Example's output state. |
| `create_time` | String | Output only. The timestamp of initial example creation. |
| `playbook_output` | String | Optional. The output of the playbook in the example. |
| `playbook_input` | String | Optional. The input to the playbook in the example. |
| `display_name` | String | Required. The display name of the example. |
| `name` | String | The unique identifier of the playbook example. Format: `projects//locations//agents//playbooks//examples/`. |
| `actions` | Vec<String> | Required. The ordered list of actions performed by the end user and the Dialogflow agent. |
| `update_time` | String | Output only. Last time the example was updated. |
| `token_count` | String | Output only. Estimated number of tokes current example takes when sent to the LLM. |
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
example_description = example.description
example_conversation_state = example.conversation_state
example_create_time = example.create_time
example_playbook_output = example.playbook_output
example_playbook_input = example.playbook_input
example_display_name = example.display_name
example_name = example.name
example_actions = example.actions
example_update_time = example.update_time
example_token_count = example.token_count
example_language_code = example.language_code
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


### Intent

Creates an intent in the specified agent. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | Vec<String> |  | The collection of parameters associated with the intent. |
| `name` | String |  | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `is_fallback` | bool |  | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `labels` | HashMap<String, String> |  | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent. |
| `priority` | i64 |  | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `display_name` | String |  | Required. The human-readable name of the intent, unique within the agent. |
| `description` | String |  | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `training_phrases` | Vec<String> |  | The collection of training phrases the agent is trained on to identify the intent. |
| `parent` | String | ✅ | Required. The agent to create an intent for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | The collection of parameters associated with the intent. |
| `name` | String | The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`. |
| `is_fallback` | bool | Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event. |
| `labels` | HashMap<String, String> | The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent. |
| `priority` | i64 | The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests. |
| `display_name` | String | Required. The human-readable name of the intent, unique within the agent. |
| `description` | String | Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters. |
| `training_phrases` | Vec<String> | The collection of training phrases the agent is trained on to identify the intent. |


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
intent_parameters = intent.parameters
intent_name = intent.name
intent_is_fallback = intent.is_fallback
intent_labels = intent.labels
intent_priority = intent.priority
intent_display_name = intent.display_name
intent_description = intent.description
intent_training_phrases = intent.training_phrases
```

---


### Session

Returns preliminary intent match results, doesn't change the session status.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_input` | String |  | Required. The input specification. |
| `query_params` | String |  | The parameters of this query. |
| `persist_parameter_changes` | bool |  | Persist session parameter changes from `query_params`. |
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


### Result

Gets a test case result.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conversation_turns` | Vec<String> | The conversation turns uttered during the test case replay in chronological order. |
| `environment` | String | Environment where the test was run. If not set, it indicates the draft environment. |
| `test_time` | String | The time that the test was run. |
| `test_result` | String | Whether the test case passed in the agent environment. |
| `name` | String | The resource name for the test case result. Format: `projects//locations//agents//testCases//results/`. |


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
result_conversation_turns = result.conversation_turns
result_environment = result.environment
result_test_time = result.test_time
result_test_result = result.test_result
result_name = result.name
```

---


### Environment

Creates an Environment in the specified Agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: Environment

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Update time of this environment. |
| `description` | String |  | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `test_cases_config` | String |  | The test cases config for continuous tests of this environment. |
| `display_name` | String |  | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `version_configs` | Vec<String> |  | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String |  | The webhook configuration for this environment. |
| `name` | String |  | The name of the environment. Format: `projects//locations//agents//environments/`. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Update time of this environment. |
| `description` | String | The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `test_cases_config` | String | The test cases config for continuous tests of this environment. |
| `display_name` | String | Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters. |
| `version_configs` | Vec<String> | A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned. |
| `webhook_config` | String | The webhook configuration for this environment. |
| `name` | String | The name of the environment. Format: `projects//locations//agents//environments/`. |


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
environment_description = environment.description
environment_test_cases_config = environment.test_cases_config
environment_display_name = environment.display_name
environment_version_configs = environment.version_configs
environment_webhook_config = environment.webhook_config
environment_name = environment.name
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
| `end_time` | String | End time of this deployment. |
| `result` | String | Result of the deployment. |
| `start_time` | String | Start time of this deployment. |
| `name` | String | The name of the deployment. Format: projects//locations//agents//environments//deployments/. |
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
deployment_end_time = deployment.end_time
deployment_result = deployment.result
deployment_start_time = deployment.start_time
deployment_name = deployment.name
deployment_state = deployment.state
deployment_flow_version = deployment.flow_version
```

---


### Tool

Creates a Tool in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Required. High level description of the Tool and its usage. |
| `function_spec` | String |  | Client side executed function specification. |
| `tool_type` | String |  | Output only. The tool type. |
| `display_name` | String |  | Required. The human-readable name of the Tool, unique within an agent. |
| `open_api_spec` | String |  | OpenAPI specification of the Tool. |
| `name` | String |  | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `data_store_spec` | String |  | Data store search tool specification. |
| `parent` | String | ✅ | Required. The agent to create a Tool for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Required. High level description of the Tool and its usage. |
| `function_spec` | String | Client side executed function specification. |
| `tool_type` | String | Output only. The tool type. |
| `display_name` | String | Required. The human-readable name of the Tool, unique within an agent. |
| `open_api_spec` | String | OpenAPI specification of the Tool. |
| `name` | String | The unique identifier of the Tool. Format: `projects//locations//agents//tools/`. |
| `data_store_spec` | String | Data store search tool specification. |


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
tool_description = tool.description
tool_function_spec = tool.function_spec
tool_tool_type = tool.tool_type
tool_display_name = tool.display_name
tool_open_api_spec = tool.open_api_spec
tool_name = tool.name
tool_data_store_spec = tool.data_store_spec
```

---


### Experiment

Creates an Experiment in the specified Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `definition` | String |  | The definition of the experiment. |
| `variants_history` | Vec<String> |  | The history of updates to the experiment variants. |
| `experiment_length` | String |  | Maximum number of days to run the experiment/rollout. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `start_time` | String |  | Start time of this experiment. |
| `rollout_state` | String |  | State of the auto rollout process. |
| `name` | String |  | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `create_time` | String |  | Creation time of this experiment. |
| `rollout_failure_reason` | String |  | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `rollout_config` | String |  | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `last_update_time` | String |  | Last update time of this experiment. |
| `end_time` | String |  | End time of this experiment. |
| `state` | String |  | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `display_name` | String |  | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `result` | String |  | Inference result of the experiment. |
| `description` | String |  | The human-readable description of the experiment. |
| `parent` | String | ✅ | Required. The Agent to create an Environment for. Format: `projects//locations//agents//environments/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `definition` | String | The definition of the experiment. |
| `variants_history` | Vec<String> | The history of updates to the experiment variants. |
| `experiment_length` | String | Maximum number of days to run the experiment/rollout. If auto-rollout is not enabled, default value and maximum will be 30 days. If auto-rollout is enabled, default value and maximum will be 6 days. |
| `start_time` | String | Start time of this experiment. |
| `rollout_state` | String | State of the auto rollout process. |
| `name` | String | The name of the experiment. Format: projects//locations//agents//environments//experiments/. |
| `create_time` | String | Creation time of this experiment. |
| `rollout_failure_reason` | String | The reason why rollout has failed. Should only be set when state is ROLLOUT_FAILED. |
| `rollout_config` | String | The configuration for auto rollout. If set, there should be exactly two variants in the experiment (control variant being the default version of the flow), the traffic allocation for the non-control variant will gradually increase to 100% when conditions are met, and eventually replace the control variant to become the default version of the flow. |
| `last_update_time` | String | Last update time of this experiment. |
| `end_time` | String | End time of this experiment. |
| `state` | String | The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE. |
| `display_name` | String | Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters. |
| `result` | String | Inference result of the experiment. |
| `description` | String | The human-readable description of the experiment. |


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
experiment_definition = experiment.definition
experiment_variants_history = experiment.variants_history
experiment_experiment_length = experiment.experiment_length
experiment_start_time = experiment.start_time
experiment_rollout_state = experiment.rollout_state
experiment_name = experiment.name
experiment_create_time = experiment.create_time
experiment_rollout_failure_reason = experiment.rollout_failure_reason
experiment_rollout_config = experiment.rollout_config
experiment_last_update_time = experiment.last_update_time
experiment_end_time = experiment.end_time
experiment_state = experiment.state
experiment_display_name = experiment.display_name
experiment_result = experiment.result
experiment_description = experiment.description
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
location_labels = location.labels
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
| `language_code` | String | The affected language code of the change. |
| `user_email` | String | Email address of the authenticated user. |
| `name` | String | The unique identifier of the changelog. Format: `projects//locations//agents//changelogs/`. |
| `resource` | String | The affected resource name of the change. |
| `action` | String | The action of the change. |
| `type` | String | The affected resource type. |
| `create_time` | String | The timestamp of the change. |


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
changelog_language_code = changelog.language_code
changelog_user_email = changelog.user_email
changelog_name = changelog.name
changelog_resource = changelog.resource
changelog_action = changelog.action
changelog_type = changelog.type
changelog_create_time = changelog.create_time
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
| `name` | String |  | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` . |
| `parent` | String | ✅ | Required. The flow to create an TransitionRouteGroup for. Format: `projects//locations//agents//flows/` or `projects//locations//agents/` for agent-level groups. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The human-readable name of the transition route group, unique within the flow. The display name can be no longer than 30 characters. |
| `transition_routes` | Vec<String> | Transition routes associated with the TransitionRouteGroup. |
| `name` | String | The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/` . |


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


### Generator

Creates a generator in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `placeholders` | Vec<String> |  | Optional. List of custom placeholders in the prompt text. |
| `prompt_text` | String |  | Required. Prompt for the LLM model. |
| `display_name` | String |  | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String |  | The LLM model settings. |
| `model_parameter` | String |  | Parameters passed to the LLM to configure its behavior. |
| `name` | String |  | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |
| `parent` | String | ✅ | Required. The agent to create a generator for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `placeholders` | Vec<String> | Optional. List of custom placeholders in the prompt text. |
| `prompt_text` | String | Required. Prompt for the LLM model. |
| `display_name` | String | Required. The human-readable name of the generator, unique within the agent. The prompt contains pre-defined parameters such as $conversation, $last-user-utterance, etc. populated by Dialogflow. It can also contain custom placeholders which will be resolved during fulfillment. |
| `llm_model_settings` | String | The LLM model settings. |
| `model_parameter` | String | Parameters passed to the LLM to configure its behavior. |
| `name` | String | The unique identifier of the generator. Must be set for the Generators.UpdateGenerator method. Generators.CreateGenerate populates the name automatically. Format: `projects//locations//agents//generators/`. |


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
generator_placeholders = generator.placeholders
generator_prompt_text = generator.prompt_text
generator_display_name = generator.display_name
generator_llm_model_settings = generator.llm_model_settings
generator_model_parameter = generator.model_parameter
generator_name = generator.name
```

---


### Security_setting

Create security settings in the specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deidentify_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `inspect_template` | String |  | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `insights_export_settings` | String |  | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `name` | String |  | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `display_name` | String |  | Required. The human-readable name of the security settings, unique within the location. |
| `redaction_strategy` | String |  | Strategy that defines how we do redaction. |
| `audio_export_settings` | String |  | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `redaction_scope` | String |  | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_strategy` | String |  | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `purge_data_types` | Vec<String> |  | List of types of data to remove when retention settings triggers purge. |
| `retention_window_days` | i64 |  | Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |
| `parent` | String | ✅ | Required. The location to create an SecuritySettings for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deidentify_template` | String | [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. The `DLP De-identify Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, Dialogflow replaces sensitive info with `[redacted]` text. The template name will have one of the following formats: `projects//locations//deidentifyTemplates/` OR `organizations//locations//deidentifyTemplates/` Note: `deidentify_template` must be located in the same region as the `SecuritySettings`. |
| `inspect_template` | String | [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. The `DLP Inspect Templates Reader` role is needed on the Dialogflow service identity service account (has the form `service-PROJECT_NUMBER@gcp-sa-dialogflow.iam.gserviceaccount.com`) for your agent's project. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects//locations//inspectTemplates/` OR `organizations//locations//inspectTemplates/` Note: `inspect_template` must be located in the same region as the `SecuritySettings`. |
| `insights_export_settings` | String | Controls conversation exporting settings to Insights after conversation is completed. If retention_strategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here. |
| `name` | String | Resource name of the settings. Required for the SecuritySettingsService.UpdateSecuritySettings method. SecuritySettingsService.CreateSecuritySettings populates the name automatically. Format: `projects//locations//securitySettings/`. |
| `display_name` | String | Required. The human-readable name of the security settings, unique within the location. |
| `redaction_strategy` | String | Strategy that defines how we do redaction. |
| `audio_export_settings` | String | Controls audio export settings for post-conversation analytics when ingesting audio to conversations via Participants.AnalyzeContent or Participants.StreamingAnalyzeContent. If retention_strategy is set to REMOVE_AFTER_CONVERSATION or audio_export_settings.gcs_bucket is empty, audio export is disabled. If audio export is enabled, audio is recorded and saved to audio_export_settings.gcs_bucket, subject to retention policy of audio_export_settings.gcs_bucket. This setting won't effect audio input for implicit sessions via Sessions.DetectIntent or Sessions.StreamingDetectIntent. |
| `redaction_scope` | String | Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging. |
| `retention_strategy` | String | Specifies the retention behavior defined by SecuritySettings.RetentionStrategy. |
| `purge_data_types` | Vec<String> | List of types of data to remove when retention settings triggers purge. |
| `retention_window_days` | i64 | Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL. When data retention configuration is changed, it only applies to the data created after the change; the TTL of existing data created before the change stays intact. |


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
security_setting_deidentify_template = security_setting.deidentify_template
security_setting_inspect_template = security_setting.inspect_template
security_setting_insights_export_settings = security_setting.insights_export_settings
security_setting_name = security_setting.name
security_setting_display_name = security_setting.display_name
security_setting_redaction_strategy = security_setting.redaction_strategy
security_setting_audio_export_settings = security_setting.audio_export_settings
security_setting_redaction_scope = security_setting.redaction_scope
security_setting_retention_strategy = security_setting.retention_strategy
security_setting_purge_data_types = security_setting.purge_data_types
security_setting_retention_window_days = security_setting.retention_window_days
```

---


### Agent

Creates an agent in the specified location. Note: You should always train flows prior to sending them queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_stackdriver_logging` | bool |  | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `enable_spell_correction` | bool |  | Indicates if automatic spell correction is enabled in detect intent requests. |
| `locked` | bool |  | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `default_language_code` | String |  | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `security_settings` | String |  | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `client_certificate_settings` | String |  | Optional. Settings for custom client certificates. |
| `start_playbook` | String |  | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `display_name` | String |  | Required. The human-readable name of the agent, unique within the location. |
| `avatar_uri` | String |  | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `personalization_settings` | String |  | Optional. Settings for end user personalization. |
| `speech_to_text_settings` | String |  | Speech recognition related settings. |
| `gen_app_builder_settings` | String |  | Gen App Builder-related agent-level settings. |
| `supported_language_codes` | Vec<String> |  | The list of all languages supported by the agent (except for the `default_language_code`). |
| `name` | String |  | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `enable_multi_language_training` | bool |  | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `description` | String |  | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `satisfies_pzi` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `text_to_speech_settings` | String |  | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `satisfies_pzs` | bool |  | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `answer_feedback_settings` | String |  | Optional. Answer feedback collection settings. |
| `start_flow` | String |  | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `git_integration_settings` | String |  | Git integration settings for this agent. |
| `time_zone` | String |  | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |
| `parent` | String | ✅ | Required. The location to create a agent for. Format: `projects//locations/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_stackdriver_logging` | bool | Indicates if stackdriver logging is enabled for the agent. Please use agent.advanced_settings instead. |
| `enable_spell_correction` | bool | Indicates if automatic spell correction is enabled in detect intent requests. |
| `locked` | bool | Indicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for RestoreAgent. |
| `default_language_code` | String | Required. Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method. |
| `security_settings` | String | Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`. |
| `client_certificate_settings` | String | Optional. Settings for custom client certificates. |
| `start_playbook` | String | Name of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//playbooks/`. Currently only the default playbook with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `advanced_settings` | String | Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `display_name` | String | Required. The human-readable name of the agent, unique within the location. |
| `avatar_uri` | String | The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration. |
| `personalization_settings` | String | Optional. Settings for end user personalization. |
| `speech_to_text_settings` | String | Speech recognition related settings. |
| `gen_app_builder_settings` | String | Gen App Builder-related agent-level settings. |
| `supported_language_codes` | Vec<String> | The list of all languages supported by the agent (except for the `default_language_code`). |
| `name` | String | The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`. |
| `enable_multi_language_training` | bool | Optional. Enable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent. |
| `description` | String | The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected. |
| `satisfies_pzi` | bool | Optional. Output only. A read only boolean field reflecting Zone Isolation status of the agent. |
| `text_to_speech_settings` | String | Settings on instructing the speech synthesizer on how to generate the output audio content. |
| `satisfies_pzs` | bool | Optional. Output only. A read only boolean field reflecting Zone Separation status of the agent. |
| `answer_feedback_settings` | String | Optional. Answer feedback collection settings. |
| `start_flow` | String | Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`. Currently only the default start flow with id "00000000-0000-0000-0000-000000000000" is allowed. |
| `git_integration_settings` | String | Git integration settings for this agent. |
| `time_zone` | String | Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. |


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
agent_enable_stackdriver_logging = agent.enable_stackdriver_logging
agent_enable_spell_correction = agent.enable_spell_correction
agent_locked = agent.locked
agent_default_language_code = agent.default_language_code
agent_security_settings = agent.security_settings
agent_client_certificate_settings = agent.client_certificate_settings
agent_start_playbook = agent.start_playbook
agent_advanced_settings = agent.advanced_settings
agent_display_name = agent.display_name
agent_avatar_uri = agent.avatar_uri
agent_personalization_settings = agent.personalization_settings
agent_speech_to_text_settings = agent.speech_to_text_settings
agent_gen_app_builder_settings = agent.gen_app_builder_settings
agent_supported_language_codes = agent.supported_language_codes
agent_name = agent.name
agent_enable_multi_language_training = agent.enable_multi_language_training
agent_description = agent.description
agent_satisfies_pzi = agent.satisfies_pzi
agent_text_to_speech_settings = agent.text_to_speech_settings
agent_satisfies_pzs = agent.satisfies_pzs
agent_answer_feedback_settings = agent.answer_feedback_settings
agent_start_flow = agent.start_flow
agent_git_integration_settings = agent.git_integration_settings
agent_time_zone = agent.time_zone
```

---


### Webhook

Creates a webhook in the specified agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disabled` | bool |  | Indicates whether the webhook is disabled. |
| `generic_web_service` | String |  | Configuration for a generic web service. |
| `timeout` | String |  | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `display_name` | String |  | Required. The human-readable name of the webhook, unique within the agent. |
| `service_directory` | String |  | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `name` | String |  | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |
| `parent` | String | ✅ | Required. The agent to create a webhook for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disabled` | bool | Indicates whether the webhook is disabled. |
| `generic_web_service` | String | Configuration for a generic web service. |
| `timeout` | String | Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds. |
| `display_name` | String | Required. The human-readable name of the webhook, unique within the agent. |
| `service_directory` | String | Configuration for a [Service Directory](https://cloud.google.com/service-directory) service. |
| `name` | String | The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`. |


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
webhook_disabled = webhook.disabled
webhook_generic_web_service = webhook.generic_web_service
webhook_timeout = webhook.timeout
webhook_display_name = webhook.display_name
webhook_service_directory = webhook.service_directory
webhook_name = webhook.name
```

---


### Entity_type

Creates a session entity type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_override_mode` | String |  | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `entities` | Vec<String> |  | Required. The collection of entities to override or supplement the custom entity type. |
| `name` | String |  | Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. |
| `parent` | String | ✅ | Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_override_mode` | String | Required. Indicates whether the additional data should override or supplement the custom entity type definition. |
| `entities` | Vec<String> | Required. The collection of entities to override or supplement the custom entity type. |
| `name` | String | Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. |


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
    parent = "value"  # Required. The session to create a session entity type for. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/`. If `Environment ID` is not specified, we assume default 'draft' environment.
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_entity_override_mode = entity_type.entity_override_mode
entity_type_entities = entity_type.entities
entity_type_name = entity_type.name
```

---


### Page

Creates a page in the specified flow. Note: You should always train a flow prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entry_fulfillment` | String |  | The fulfillment to call when the session is entering the page. |
| `form` | String |  | The form associated with the page, used for collecting parameters relevant to the page. |
| `description` | String |  | The description of the page. The maximum length is 500 characters. |
| `transition_route_groups` | Vec<String> |  | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> |  | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `transition_routes` | Vec<String> |  | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `knowledge_connector_settings` | String |  | Optional. Knowledge connector configuration. |
| `name` | String |  | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `advanced_settings` | String |  | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `display_name` | String |  | Required. The human-readable name of the page, unique within the flow. |
| `parent` | String | ✅ | Required. The flow to create a page for. Format: `projects//locations//agents//flows/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entry_fulfillment` | String | The fulfillment to call when the session is entering the page. |
| `form` | String | The form associated with the page, used for collecting parameters relevant to the page. |
| `description` | String | The description of the page. The maximum length is 500 characters. |
| `transition_route_groups` | Vec<String> | Ordered list of `TransitionRouteGroups` added to the page. Transition route groups must be unique within a page. If the page links both flow-level transition route groups and agent-level transition route groups, the flow-level ones will have higher priority and will be put before the agent-level ones. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/` or `projects//locations//agents//transitionRouteGroups/` for agent-level groups. |
| `event_handlers` | Vec<String> | Handlers associated with the page to handle events such as webhook errors, no match or no input. |
| `transition_routes` | Vec<String> | A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evaluated in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified. |
| `knowledge_connector_settings` | String | Optional. Knowledge connector configuration. |
| `name` | String | The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`. |
| `advanced_settings` | String | Hierarchical advanced settings for this page. The settings exposed at the lower level overrides the settings exposed at the higher level. |
| `display_name` | String | Required. The human-readable name of the page, unique within the flow. |


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
page_entry_fulfillment = page.entry_fulfillment
page_form = page.form
page_description = page.description
page_transition_route_groups = page.transition_route_groups
page_event_handlers = page.event_handlers
page_transition_routes = page.transition_routes
page_knowledge_connector_settings = page.knowledge_connector_settings
page_name = page.name
page_advanced_settings = page.advanced_settings
page_display_name = page.display_name
```

---


### Test_case

Creates a test case for the given agent.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_time` | String |  | Output only. When the test was created. |
| `display_name` | String |  | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_case_conversation_turns` | Vec<String> |  | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `last_test_result` | String |  | The latest test result. |
| `test_config` | String |  | Config for the test case. |
| `name` | String |  | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `notes` | String |  | Additional freeform notes about the test case. Limit of 400 characters. |
| `tags` | Vec<String> |  | Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with "#" and has a limit of 30 characters. |
| `parent` | String | ✅ | Required. The agent to create the test case for. Format: `projects//locations//agents/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | Output only. When the test was created. |
| `display_name` | String | Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters. |
| `test_case_conversation_turns` | Vec<String> | The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly. |
| `last_test_result` | String | The latest test result. |
| `test_config` | String | Config for the test case. |
| `name` | String | The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents//testCases/`. |
| `notes` | String | Additional freeform notes about the test case. Limit of 400 characters. |
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
test_case_creation_time = test_case.creation_time
test_case_display_name = test_case.display_name
test_case_test_case_conversation_turns = test_case.test_case_conversation_turns
test_case_last_test_result = test_case.last_test_result
test_case_test_config = test_case.test_config
test_case_name = test_case.name
test_case_notes = test_case.notes
test_case_tags = test_case.tags
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.dialogflow_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
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
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |


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
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple agent resources
agent_0 = provider.dialogflow_api.Agent {
    parent = "value-0"
}
agent_1 = provider.dialogflow_api.Agent {
    parent = "value-1"
}
agent_2 = provider.dialogflow_api.Agent {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    agent = provider.dialogflow_api.Agent {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Dialogflow_api Documentation](https://cloud.google.com/dialogflow_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
