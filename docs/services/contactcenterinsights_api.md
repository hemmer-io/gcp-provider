# Contactcenterinsights_api Service



**Resources**: 23

---

## Overview

The contactcenterinsights_api service provides access to 23 resource types:

- [Operation](#operation) [CR]
- [View](#view) [CRUD]
- [Conversation](#conversation) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Phrase_matcher](#phrase_matcher) [CRUD]
- [Qa_question_tag](#qa_question_tag) [CRUD]
- [Feedback_label](#feedback_label) [CRUD]
- [Note](#note) [CRUD]
- [Authorized_view_set](#authorized_view_set) [CRUD]
- [Authorized_view](#authorized_view) [CRUD]
- [Qa_question](#qa_question) [CRUD]
- [Qa_scorecard](#qa_scorecard) [CRUD]
- [Assessment](#assessment) [CRD]
- [Issue](#issue) [CRUD]
- [Encryption_spec](#encryption_spec) [C]
- [Analysis_rule](#analysis_rule) [CRUD]
- [Segment](#segment) [C]
- [Assessment_rule](#assessment_rule) [CRUD]
- [Revision](#revision) [CRD]
- [Location](#location) [CRU]
- [Insightsdata](#insightsdata) [C]
- [Issue_model](#issue_model) [CRUD]
- [Analyse](#analyse) [CRD]

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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.contactcenterinsights_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
```

---


### View

Creates a view.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String |  | A filter to reduce conversation results to a specific subset. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `create_time` | String |  | Output only. The time at which this view was created. |
| `name` | String |  | Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view} |
| `display_name` | String |  | The human-readable display name of the view. |
| `update_time` | String |  | Output only. The most recent time at which the view was updated. |
| `parent` | String | ✅ | Required. The parent resource of the view. Required. The location to create a view for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | A filter to reduce conversation results to a specific subset. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `create_time` | String | Output only. The time at which this view was created. |
| `name` | String | Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view} |
| `display_name` | String | The human-readable display name of the view. |
| `update_time` | String | Output only. The most recent time at which the view was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create view
view = provider.contactcenterinsights_api.View {
    parent = "value"  # Required. The parent resource of the view. Required. The location to create a view for. Format: `projects//locations/` or `projects//locations/`
}

# Access view outputs
view_id = view.id
view_value = view.value
view_create_time = view.create_time
view_name = view.name
view_display_name = view.display_name
view_update_time = view.update_time
```

---


### Conversation

Creates a conversation. Note that this method does not support audio transcription or redaction. Use `conversations.upload` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The most recent time at which the conversation was updated. |
| `labels` | HashMap<String, String> |  | A map for the user to specify any custom fields. A maximum of 100 labels per conversation is allowed, with a maximum of 256 characters per entry. |
| `medium` | String |  | Immutable. The conversation medium, if unspecified will default to PHONE_CALL. |
| `ttl` | String |  | Input only. The TTL for this resource. If specified, then this TTL will be used to calculate the expire time. |
| `duration` | String |  | Output only. The duration of the conversation. |
| `language_code` | String |  | A user-specified language code for the conversation. |
| `turn_count` | i64 |  | Output only. The number of turns in the conversation. |
| `quality_metadata` | String |  | Conversation metadata related to quality management. |
| `call_metadata` | String |  | Call-specific metadata. |
| `create_time` | String |  | Output only. The time at which the conversation was created. |
| `transcript` | String |  | Output only. The conversation transcript. |
| `latest_summary` | String |  | Output only. Latest summary of the conversation. |
| `latest_analysis` | String |  | Output only. The conversation's latest analysis, if one exists. |
| `expire_time` | String |  | The time at which this conversation should expire. After this time, the conversation data and any associated analyses will be deleted. |
| `agent_id` | String |  | An opaque, user-specified string representing the human agent who handled the conversation. |
| `dialogflow_intents` | HashMap<String, String> |  | Output only. All the matched Dialogflow intents in the call. The key corresponds to a Dialogflow intent, format: projects/{project}/agent/{agent}/intents/{intent} |
| `runtime_annotations` | Vec<String> |  | Output only. The annotations that were generated during the customer and agent interaction. |
| `metadata_json` | String |  | Input only. JSON metadata encoded as a string. This field is primarily used by Insights integrations with various telephony systems and must be in one of Insight's supported formats. |
| `obfuscated_user_id` | String |  | Obfuscated user ID which the customer sent to us. |
| `start_time` | String |  | The time at which the conversation started. |
| `data_source` | String |  | The source of the audio and transcription for the conversation. |
| `name` | String |  | Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation} |
| `parent` | String | ✅ | Required. The parent resource of the conversation. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The most recent time at which the conversation was updated. |
| `labels` | HashMap<String, String> | A map for the user to specify any custom fields. A maximum of 100 labels per conversation is allowed, with a maximum of 256 characters per entry. |
| `medium` | String | Immutable. The conversation medium, if unspecified will default to PHONE_CALL. |
| `ttl` | String | Input only. The TTL for this resource. If specified, then this TTL will be used to calculate the expire time. |
| `duration` | String | Output only. The duration of the conversation. |
| `language_code` | String | A user-specified language code for the conversation. |
| `turn_count` | i64 | Output only. The number of turns in the conversation. |
| `quality_metadata` | String | Conversation metadata related to quality management. |
| `call_metadata` | String | Call-specific metadata. |
| `create_time` | String | Output only. The time at which the conversation was created. |
| `transcript` | String | Output only. The conversation transcript. |
| `latest_summary` | String | Output only. Latest summary of the conversation. |
| `latest_analysis` | String | Output only. The conversation's latest analysis, if one exists. |
| `expire_time` | String | The time at which this conversation should expire. After this time, the conversation data and any associated analyses will be deleted. |
| `agent_id` | String | An opaque, user-specified string representing the human agent who handled the conversation. |
| `dialogflow_intents` | HashMap<String, String> | Output only. All the matched Dialogflow intents in the call. The key corresponds to a Dialogflow intent, format: projects/{project}/agent/{agent}/intents/{intent} |
| `runtime_annotations` | Vec<String> | Output only. The annotations that were generated during the customer and agent interaction. |
| `metadata_json` | String | Input only. JSON metadata encoded as a string. This field is primarily used by Insights integrations with various telephony systems and must be in one of Insight's supported formats. |
| `obfuscated_user_id` | String | Obfuscated user ID which the customer sent to us. |
| `start_time` | String | The time at which the conversation started. |
| `data_source` | String | The source of the audio and transcription for the conversation. |
| `name` | String | Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.contactcenterinsights_api.Conversation {
    parent = "value"  # Required. The parent resource of the conversation.
}

# Access conversation outputs
conversation_id = conversation.id
conversation_update_time = conversation.update_time
conversation_labels = conversation.labels
conversation_medium = conversation.medium
conversation_ttl = conversation.ttl
conversation_duration = conversation.duration
conversation_language_code = conversation.language_code
conversation_turn_count = conversation.turn_count
conversation_quality_metadata = conversation.quality_metadata
conversation_call_metadata = conversation.call_metadata
conversation_create_time = conversation.create_time
conversation_transcript = conversation.transcript
conversation_latest_summary = conversation.latest_summary
conversation_latest_analysis = conversation.latest_analysis
conversation_expire_time = conversation.expire_time
conversation_agent_id = conversation.agent_id
conversation_dialogflow_intents = conversation.dialogflow_intents
conversation_runtime_annotations = conversation.runtime_annotations
conversation_metadata_json = conversation.metadata_json
conversation_obfuscated_user_id = conversation.obfuscated_user_id
conversation_start_time = conversation.start_time
conversation_data_source = conversation.data_source
conversation_name = conversation.name
```

---


### Dataset

Creates a dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Dataset description. |
| `display_name` | String |  | Display name for the dataaset |
| `type` | String |  | Dataset usage type. |
| `name` | String |  | Immutable. Identifier. Resource name of the dataset. Format: projects/{project}/locations/{location}/datasets/{dataset} |
| `ttl` | String |  | Optional. Option TTL for the dataset. |
| `create_time` | String |  | Output only. Dataset create time. |
| `update_time` | String |  | Output only. Dataset update time. |
| `parent` | String | ✅ | Required. The parent resource of the dataset. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Dataset description. |
| `display_name` | String | Display name for the dataaset |
| `type` | String | Dataset usage type. |
| `name` | String | Immutable. Identifier. Resource name of the dataset. Format: projects/{project}/locations/{location}/datasets/{dataset} |
| `ttl` | String | Optional. Option TTL for the dataset. |
| `create_time` | String | Output only. Dataset create time. |
| `update_time` | String | Output only. Dataset update time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.contactcenterinsights_api.Dataset {
    parent = "value"  # Required. The parent resource of the dataset.
}

# Access dataset outputs
dataset_id = dataset.id
dataset_description = dataset.description
dataset_display_name = dataset.display_name
dataset_type = dataset.type
dataset_name = dataset.name
dataset_ttl = dataset.ttl
dataset_create_time = dataset.create_time
dataset_update_time = dataset.update_time
```

---


### Phrase_matcher

Creates a phrase matcher.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_tag` | String |  | The customized version tag to use for the phrase matcher. If not specified, it will default to `revision_id`. |
| `role_match` | String |  | The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript. |
| `activation_update_time` | String |  | Output only. The most recent time at which the activation status was updated. |
| `type` | String |  | Required. The type of this phrase matcher. |
| `update_time` | String |  | Output only. The most recent time at which the phrase matcher was updated. |
| `phrase_match_rule_groups` | Vec<String> |  | A list of phase match rule groups that are included in this matcher. |
| `name` | String |  | The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher} |
| `revision_create_time` | String |  | Output only. The timestamp of when the revision was created. It is also the create time when a new matcher is added. |
| `display_name` | String |  | The human-readable name of the phrase matcher. |
| `active` | bool |  | Applies the phrase matcher only when it is active. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the phrase matcher. A new revision is committed whenever the matcher is changed, except when it is activated or deactivated. A server generated random ID will be used. Example: locations/global/phraseMatchers/my-first-matcher@1234567 |
| `parent` | String | ✅ | Required. The parent resource of the phrase matcher. Required. The location to create a phrase matcher for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_tag` | String | The customized version tag to use for the phrase matcher. If not specified, it will default to `revision_id`. |
| `role_match` | String | The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript. |
| `activation_update_time` | String | Output only. The most recent time at which the activation status was updated. |
| `type` | String | Required. The type of this phrase matcher. |
| `update_time` | String | Output only. The most recent time at which the phrase matcher was updated. |
| `phrase_match_rule_groups` | Vec<String> | A list of phase match rule groups that are included in this matcher. |
| `name` | String | The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher} |
| `revision_create_time` | String | Output only. The timestamp of when the revision was created. It is also the create time when a new matcher is added. |
| `display_name` | String | The human-readable name of the phrase matcher. |
| `active` | bool | Applies the phrase matcher only when it is active. |
| `revision_id` | String | Output only. Immutable. The revision ID of the phrase matcher. A new revision is committed whenever the matcher is changed, except when it is activated or deactivated. A server generated random ID will be used. Example: locations/global/phraseMatchers/my-first-matcher@1234567 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create phrase_matcher
phrase_matcher = provider.contactcenterinsights_api.Phrase_matcher {
    parent = "value"  # Required. The parent resource of the phrase matcher. Required. The location to create a phrase matcher for. Format: `projects//locations/` or `projects//locations/`
}

# Access phrase_matcher outputs
phrase_matcher_id = phrase_matcher.id
phrase_matcher_version_tag = phrase_matcher.version_tag
phrase_matcher_role_match = phrase_matcher.role_match
phrase_matcher_activation_update_time = phrase_matcher.activation_update_time
phrase_matcher_type = phrase_matcher.type
phrase_matcher_update_time = phrase_matcher.update_time
phrase_matcher_phrase_match_rule_groups = phrase_matcher.phrase_match_rule_groups
phrase_matcher_name = phrase_matcher.name
phrase_matcher_revision_create_time = phrase_matcher.revision_create_time
phrase_matcher_display_name = phrase_matcher.display_name
phrase_matcher_active = phrase_matcher.active
phrase_matcher_revision_id = phrase_matcher.revision_id
```

---


### Qa_question_tag

Creates a QaQuestionTag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. A user-specified display name for the tag. |
| `update_time` | String |  | Output only. The most recent time at which the question tag was updated. |
| `qa_question_ids` | Vec<String> |  | Optional. The list of Scorecard Question IDs that the tag applies to. Each QaQuestionId is represented as a full resource name containing the Question ID. Lastly, Since a tag may not necessarily be referenced by any Scorecard Questions, we treat this field as optional. |
| `name` | String |  | Identifier. Resource name for the QaQuestionTag Format projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag} In the above format, the last segment, i.e., qa_question_tag, is a server-generated ID corresponding to the tag resource. |
| `create_time` | String |  | Output only. The time at which the question tag was created. |
| `parent` | String | ✅ | Required. The parent resource of the QaQuestionTag. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. A user-specified display name for the tag. |
| `update_time` | String | Output only. The most recent time at which the question tag was updated. |
| `qa_question_ids` | Vec<String> | Optional. The list of Scorecard Question IDs that the tag applies to. Each QaQuestionId is represented as a full resource name containing the Question ID. Lastly, Since a tag may not necessarily be referenced by any Scorecard Questions, we treat this field as optional. |
| `name` | String | Identifier. Resource name for the QaQuestionTag Format projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag} In the above format, the last segment, i.e., qa_question_tag, is a server-generated ID corresponding to the tag resource. |
| `create_time` | String | Output only. The time at which the question tag was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create qa_question_tag
qa_question_tag = provider.contactcenterinsights_api.Qa_question_tag {
    parent = "value"  # Required. The parent resource of the QaQuestionTag.
}

# Access qa_question_tag outputs
qa_question_tag_id = qa_question_tag.id
qa_question_tag_display_name = qa_question_tag.display_name
qa_question_tag_update_time = qa_question_tag.update_time
qa_question_tag_qa_question_ids = qa_question_tag.qa_question_ids
qa_question_tag_name = qa_question_tag.name
qa_question_tag_create_time = qa_question_tag.create_time
```

---


### Feedback_label

Create feedback label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the label. |
| `labeled_resource` | String |  | Name of the resource to be labeled. Supported resources are: * `projects/{project}/locations/{location}/qaScorecards/{scorecard}/revisions/{revision}/qaQuestions/{question}` * `projects/{project}/locations/{location}/issueModels/{issue_model}` * `projects/{project}/locations/{location}/generators/{generator_id}` |
| `label` | String |  | String label used for Topic Modeling. |
| `name` | String |  | Immutable. Resource name of the FeedbackLabel. Format: projects/{project}/locations/{location}/conversations/{conversation}/feedbackLabels/{feedback_label} |
| `qa_answer_label` | String |  | QaAnswer label used for Quality AI example conversations. |
| `update_time` | String |  | Output only. Update time of the label. |
| `parent` | String | ✅ | Required. The parent resource of the feedback label. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the label. |
| `labeled_resource` | String | Name of the resource to be labeled. Supported resources are: * `projects/{project}/locations/{location}/qaScorecards/{scorecard}/revisions/{revision}/qaQuestions/{question}` * `projects/{project}/locations/{location}/issueModels/{issue_model}` * `projects/{project}/locations/{location}/generators/{generator_id}` |
| `label` | String | String label used for Topic Modeling. |
| `name` | String | Immutable. Resource name of the FeedbackLabel. Format: projects/{project}/locations/{location}/conversations/{conversation}/feedbackLabels/{feedback_label} |
| `qa_answer_label` | String | QaAnswer label used for Quality AI example conversations. |
| `update_time` | String | Output only. Update time of the label. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feedback_label
feedback_label = provider.contactcenterinsights_api.Feedback_label {
    parent = "value"  # Required. The parent resource of the feedback label.
}

# Access feedback_label outputs
feedback_label_id = feedback_label.id
feedback_label_create_time = feedback_label.create_time
feedback_label_labeled_resource = feedback_label.labeled_resource
feedback_label_label = feedback_label.label
feedback_label_name = feedback_label.name
feedback_label_qa_answer_label = feedback_label.qa_answer_label
feedback_label_update_time = feedback_label.update_time
```

---


### Note

Create Note.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assessment_note` | String |  | The note is associated to the entire parent assessment. |
| `create_time` | String |  | Output only. The time at which the note was created. |
| `note_creator` | String |  | Output only. The user that created the note. |
| `qa_question_note` | String |  | The note is associated with a QA question in one of the conversation's scorecard results. |
| `conversation_turn_note` | String |  | The note is associated with a conversation turn. |
| `name` | String |  | Identifier. The resource name of the note. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment}/notes/{note} |
| `content` | String |  | The note content. |
| `update_time` | String |  | Output only. The time at which the note was last updated. |
| `parent` | String | ✅ | Required. The parent resource of the note. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notes` | Vec<String> | The notes that match the request. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.contactcenterinsights_api.Note {
    parent = "value"  # Required. The parent resource of the note.
}

# Access note outputs
note_id = note.id
note_notes = note.notes
note_next_page_token = note.next_page_token
```

---


### Authorized_view_set

Create AuthorizedViewSet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time. |
| `name` | String |  | Identifier. The resource name of the AuthorizedViewSet. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set} |
| `update_time` | String |  | Output only. Update time. |
| `display_name` | String |  | Display Name. Limit 64 characters. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizedViewSet. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time. |
| `name` | String | Identifier. The resource name of the AuthorizedViewSet. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set} |
| `update_time` | String | Output only. Update time. |
| `display_name` | String | Display Name. Limit 64 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_view_set
authorized_view_set = provider.contactcenterinsights_api.Authorized_view_set {
    parent = "value"  # Required. The parent resource of the AuthorizedViewSet.
}

# Access authorized_view_set outputs
authorized_view_set_id = authorized_view_set.id
authorized_view_set_create_time = authorized_view_set.create_time
authorized_view_set_name = authorized_view_set.name
authorized_view_set_update_time = authorized_view_set.update_time
authorized_view_set_display_name = authorized_view_set.display_name
```

---


### Authorized_view

Create AuthorizedView

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conversation_filter` | String |  | A filter to reduce conversation results to a specific subset. The AuthorizedView's assigned permission (read/write) could be applied to the subset of conversations. If conversation_filter is empty, there is no restriction on the conversations that the AuthorizedView can access. Having *authorizedViews.get* access to the AuthorizedView means having the same read/write access to the Conversations (as well as metadata/annotations linked to the conversation) that this AuthorizedView has. |
| `create_time` | String |  | Output only. The time at which the authorized view was created. |
| `display_name` | String |  | Display Name. Limit 64 characters. |
| `name` | String |  | Identifier. The resource name of the AuthorizedView. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set}/authorizedViews/{authorized_view} |
| `update_time` | String |  | Output only. The most recent time at which the authorized view was updated. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizedView. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conversation_filter` | String | A filter to reduce conversation results to a specific subset. The AuthorizedView's assigned permission (read/write) could be applied to the subset of conversations. If conversation_filter is empty, there is no restriction on the conversations that the AuthorizedView can access. Having *authorizedViews.get* access to the AuthorizedView means having the same read/write access to the Conversations (as well as metadata/annotations linked to the conversation) that this AuthorizedView has. |
| `create_time` | String | Output only. The time at which the authorized view was created. |
| `display_name` | String | Display Name. Limit 64 characters. |
| `name` | String | Identifier. The resource name of the AuthorizedView. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set}/authorizedViews/{authorized_view} |
| `update_time` | String | Output only. The most recent time at which the authorized view was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_view
authorized_view = provider.contactcenterinsights_api.Authorized_view {
    parent = "value"  # Required. The parent resource of the AuthorizedView.
}

# Access authorized_view outputs
authorized_view_id = authorized_view.id
authorized_view_conversation_filter = authorized_view.conversation_filter
authorized_view_create_time = authorized_view.create_time
authorized_view_display_name = authorized_view.display_name
authorized_view_name = authorized_view.name
authorized_view_update_time = authorized_view.update_time
```

---


### Qa_question

Create a QaQuestion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the question. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question} |
| `answer_choices` | Vec<String> |  | A list of valid answers to the question, which the LLM must choose from. |
| `create_time` | String |  | Output only. The time at which this question was created. |
| `metrics` | String |  | Metrics of the underlying tuned LLM over a holdout/test set while fine tuning the underlying LLM for the given question. This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `predefined_question_config` | String |  | The configuration of the predefined question. This field will only be set if the Question Type is predefined. |
| `tuning_metadata` | String |  | Metadata about the tuning operation for the question.This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `question_body` | String |  | Question text. E.g., "Did the agent greet the customer?" |
| `update_time` | String |  | Output only. The most recent time at which the question was updated. |
| `answer_instructions` | String |  | Instructions describing how to determine the answer. |
| `question_type` | String |  | The type of question. |
| `abbreviation` | String |  | Short, descriptive string, used in the UI where it's not practical to display the full question body. E.g., "Greeting". |
| `order` | i64 |  | Defines the order of the question within its parent scorecard revision. |
| `tags` | Vec<String> |  | Questions are tagged for categorization and scoring. Tags can either be: - Default Tags: These are predefined categories. They are identified by their string value (e.g., "BUSINESS", "COMPLIANCE", and "CUSTOMER"). - Custom Tags: These are user-defined categories. They are identified by their full resource name (e.g., projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}). Both default and custom tags are used to group questions and to influence the scoring of each question. |
| `parent` | String | ✅ | Required. The parent resource of the QaQuestion. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the question. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question} |
| `answer_choices` | Vec<String> | A list of valid answers to the question, which the LLM must choose from. |
| `create_time` | String | Output only. The time at which this question was created. |
| `metrics` | String | Metrics of the underlying tuned LLM over a holdout/test set while fine tuning the underlying LLM for the given question. This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `predefined_question_config` | String | The configuration of the predefined question. This field will only be set if the Question Type is predefined. |
| `tuning_metadata` | String | Metadata about the tuning operation for the question.This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `question_body` | String | Question text. E.g., "Did the agent greet the customer?" |
| `update_time` | String | Output only. The most recent time at which the question was updated. |
| `answer_instructions` | String | Instructions describing how to determine the answer. |
| `question_type` | String | The type of question. |
| `abbreviation` | String | Short, descriptive string, used in the UI where it's not practical to display the full question body. E.g., "Greeting". |
| `order` | i64 | Defines the order of the question within its parent scorecard revision. |
| `tags` | Vec<String> | Questions are tagged for categorization and scoring. Tags can either be: - Default Tags: These are predefined categories. They are identified by their string value (e.g., "BUSINESS", "COMPLIANCE", and "CUSTOMER"). - Custom Tags: These are user-defined categories. They are identified by their full resource name (e.g., projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}). Both default and custom tags are used to group questions and to influence the scoring of each question. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create qa_question
qa_question = provider.contactcenterinsights_api.Qa_question {
    parent = "value"  # Required. The parent resource of the QaQuestion.
}

# Access qa_question outputs
qa_question_id = qa_question.id
qa_question_name = qa_question.name
qa_question_answer_choices = qa_question.answer_choices
qa_question_create_time = qa_question.create_time
qa_question_metrics = qa_question.metrics
qa_question_predefined_question_config = qa_question.predefined_question_config
qa_question_tuning_metadata = qa_question.tuning_metadata
qa_question_question_body = qa_question.question_body
qa_question_update_time = qa_question.update_time
qa_question_answer_instructions = qa_question.answer_instructions
qa_question_question_type = qa_question.question_type
qa_question_abbreviation = qa_question.abbreviation
qa_question_order = qa_question.order
qa_question_tags = qa_question.tags
```

---


### Qa_scorecard

Create a QaScorecard.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The user-specified display name of the scorecard. |
| `name` | String |  | Identifier. The scorecard name. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard} |
| `source` | String |  | Output only. The source of the scorecard. |
| `update_time` | String |  | Output only. The most recent time at which the scorecard was updated. |
| `create_time` | String |  | Output only. The time at which this scorecard was created. |
| `is_default` | bool |  | Whether the scorecard is the default one for the project. A default scorecard cannot be deleted and will always appear first in scorecard selector. |
| `description` | String |  | A text description explaining the intent of the scorecard. |
| `parent` | String | ✅ | Required. The parent resource of the QaScorecard. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The user-specified display name of the scorecard. |
| `name` | String | Identifier. The scorecard name. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard} |
| `source` | String | Output only. The source of the scorecard. |
| `update_time` | String | Output only. The most recent time at which the scorecard was updated. |
| `create_time` | String | Output only. The time at which this scorecard was created. |
| `is_default` | bool | Whether the scorecard is the default one for the project. A default scorecard cannot be deleted and will always appear first in scorecard selector. |
| `description` | String | A text description explaining the intent of the scorecard. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create qa_scorecard
qa_scorecard = provider.contactcenterinsights_api.Qa_scorecard {
    parent = "value"  # Required. The parent resource of the QaScorecard.
}

# Access qa_scorecard outputs
qa_scorecard_id = qa_scorecard.id
qa_scorecard_display_name = qa_scorecard.display_name
qa_scorecard_name = qa_scorecard.name
qa_scorecard_source = qa_scorecard.source
qa_scorecard_update_time = qa_scorecard.update_time
qa_scorecard_create_time = qa_scorecard.create_time
qa_scorecard_is_default = qa_scorecard.is_default
qa_scorecard_description = qa_scorecard.description
```

---


### Assessment

Create Assessment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time at which the assessment was last updated. |
| `name` | String |  | Identifier. The resource name of the assessment. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment} |
| `create_time` | String |  | Output only. The time at which the assessment was created. |
| `agent_info` | String |  | Information about the agent the assessment is for. |
| `state` | String |  | Output only. The state of the assessment. |
| `parent` | String | ✅ | Required. The parent resource of the assessment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which the assessment was last updated. |
| `name` | String | Identifier. The resource name of the assessment. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment} |
| `create_time` | String | Output only. The time at which the assessment was created. |
| `agent_info` | String | Information about the agent the assessment is for. |
| `state` | String | Output only. The state of the assessment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assessment
assessment = provider.contactcenterinsights_api.Assessment {
    parent = "value"  # Required. The parent resource of the assessment.
}

# Access assessment outputs
assessment_id = assessment.id
assessment_update_time = assessment.update_time
assessment_name = assessment.name
assessment_create_time = assessment.create_time
assessment_agent_info = assessment.agent_info
assessment_state = assessment.state
```

---


### Issue

Creates an issue.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue} |
| `create_time` | String |  | Output only. The time at which this issue was created. |
| `display_description` | String |  | Representative description of the issue. |
| `display_name` | String |  | The representative name for the issue. |
| `sample_utterances` | Vec<String> |  | Output only. Resource names of the sample representative utterances that match to this issue. |
| `update_time` | String |  | Output only. The most recent time that this issue was updated. |
| `parent` | String | ✅ | Required. The parent resource of the issue. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue} |
| `create_time` | String | Output only. The time at which this issue was created. |
| `display_description` | String | Representative description of the issue. |
| `display_name` | String | The representative name for the issue. |
| `sample_utterances` | Vec<String> | Output only. Resource names of the sample representative utterances that match to this issue. |
| `update_time` | String | Output only. The most recent time that this issue was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create issue
issue = provider.contactcenterinsights_api.Issue {
    parent = "value"  # Required. The parent resource of the issue.
}

# Access issue outputs
issue_id = issue.id
issue_name = issue.name
issue_create_time = issue.create_time
issue_display_description = issue.display_description
issue_display_name = issue.display_name
issue_sample_utterances = issue.sample_utterances
issue_update_time = issue.update_time
```

---


### Encryption_spec

Initializes a location-level encryption key specification. An error will result if the location has resources already created before the initialization. After the encryption specification is initialized at a location, it is immutable and all newly created resources under the location will be encrypted with the existing specification.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_spec` | String |  | Required. The encryption spec used for CMEK encryption. It is required that the kms key is in the same region as the endpoint. The same key will be used for all provisioned resources, if encryption is available. If the `kms_key_name` field is left empty, no encryption will be enforced. |
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
encryption_spec = provider.contactcenterinsights_api.Encryption_spec {
    name = "value"  # Immutable. The resource name of the encryption key specification resource. Format: projects/{project}/locations/{location}/encryptionSpec
}

```

---


### Analysis_rule

Creates a analysis rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which this analysis rule was created. |
| `active` | bool |  | If true, apply this rule to conversations. Otherwise, this rule is inactive and saved as a draft. |
| `analysis_percentage` | f64 |  | Percentage of conversations that we should apply this analysis setting automatically, between [0, 1]. For example, 0.1 means 10%. Conversations are sampled in a determenestic way. The original runtime_percentage & upload percentage will be replaced by defining filters on the conversation. |
| `conversation_filter` | String |  | Filter for the conversations that should apply this analysis rule. An empty filter means this analysis rule applies to all conversations. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `update_time` | String |  | Output only. The most recent time at which this analysis rule was updated. |
| `display_name` | String |  | Display Name of the analysis rule. |
| `name` | String |  | Identifier. The resource name of the analysis rule. Format: projects/{project}/locations/{location}/analysisRules/{analysis_rule} |
| `annotator_selector` | String |  | Selector of annotators to run and the phrase matchers to use for conversations that matches the conversation_filter. If not specified, NO annotators will be run. |
| `parent` | String | ✅ | Required. The parent resource of the analysis rule. Required. The location to create a analysis rule for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which this analysis rule was created. |
| `active` | bool | If true, apply this rule to conversations. Otherwise, this rule is inactive and saved as a draft. |
| `analysis_percentage` | f64 | Percentage of conversations that we should apply this analysis setting automatically, between [0, 1]. For example, 0.1 means 10%. Conversations are sampled in a determenestic way. The original runtime_percentage & upload percentage will be replaced by defining filters on the conversation. |
| `conversation_filter` | String | Filter for the conversations that should apply this analysis rule. An empty filter means this analysis rule applies to all conversations. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `update_time` | String | Output only. The most recent time at which this analysis rule was updated. |
| `display_name` | String | Display Name of the analysis rule. |
| `name` | String | Identifier. The resource name of the analysis rule. Format: projects/{project}/locations/{location}/analysisRules/{analysis_rule} |
| `annotator_selector` | String | Selector of annotators to run and the phrase matchers to use for conversations that matches the conversation_filter. If not specified, NO annotators will be run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create analysis_rule
analysis_rule = provider.contactcenterinsights_api.Analysis_rule {
    parent = "value"  # Required. The parent resource of the analysis rule. Required. The location to create a analysis rule for. Format: `projects//locations/` or `projects//locations/`
}

# Access analysis_rule outputs
analysis_rule_id = analysis_rule.id
analysis_rule_create_time = analysis_rule.create_time
analysis_rule_active = analysis_rule.active
analysis_rule_analysis_percentage = analysis_rule.analysis_percentage
analysis_rule_conversation_filter = analysis_rule.conversation_filter
analysis_rule_update_time = analysis_rule.update_time
analysis_rule_display_name = analysis_rule.display_name
analysis_rule_name = analysis_rule.name
analysis_rule_annotator_selector = analysis_rule.annotator_selector
```

---


### Segment

Analyzes multiple conversations in a single request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `analysis_percentage` | f64 |  | Required. Percentage of selected conversation to analyze, between [0, 100]. |
| `filter` | String |  | Required. Filter used to select the subset of conversations to analyze. |
| `parent` | String |  | Required. The parent resource to create analyses in. |
| `annotator_selector` | String |  | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `parent` | String | ✅ | Required. The parent resource to create analyses in. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create segment
segment = provider.contactcenterinsights_api.Segment {
    parent = "value"  # Required. The parent resource to create analyses in.
}

```

---


### Assessment_rule

Creates an assessment rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `active` | bool |  | If true, apply this rule to conversations. Otherwise, this rule is inactive. |
| `name` | String |  | Identifier. The resource name of the assessment rule. Format: projects/{project}/locations/{location}/assessmentRules/{assessment_rule} |
| `update_time` | String |  | Output only. The most recent time at which this assessment rule was updated. |
| `create_time` | String |  | Output only. The time at which this assessment rule was created. |
| `display_name` | String |  | Display Name of the assessment rule. |
| `schedule_info` | String |  | Schedule info for the assessment rule. |
| `sample_rule` | String |  | The sample rule for the assessment rule. |
| `parent` | String | ✅ | Required. The parent resource of the assessment rule. Required. The location to create a assessment rule for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active` | bool | If true, apply this rule to conversations. Otherwise, this rule is inactive. |
| `name` | String | Identifier. The resource name of the assessment rule. Format: projects/{project}/locations/{location}/assessmentRules/{assessment_rule} |
| `update_time` | String | Output only. The most recent time at which this assessment rule was updated. |
| `create_time` | String | Output only. The time at which this assessment rule was created. |
| `display_name` | String | Display Name of the assessment rule. |
| `schedule_info` | String | Schedule info for the assessment rule. |
| `sample_rule` | String | The sample rule for the assessment rule. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assessment_rule
assessment_rule = provider.contactcenterinsights_api.Assessment_rule {
    parent = "value"  # Required. The parent resource of the assessment rule. Required. The location to create a assessment rule for. Format: `projects//locations/` or `projects//locations/`
}

# Access assessment_rule outputs
assessment_rule_id = assessment_rule.id
assessment_rule_active = assessment_rule.active
assessment_rule_name = assessment_rule.name
assessment_rule_update_time = assessment_rule.update_time
assessment_rule_create_time = assessment_rule.create_time
assessment_rule_display_name = assessment_rule.display_name
assessment_rule_schedule_info = assessment_rule.schedule_info
assessment_rule_sample_rule = assessment_rule.sample_rule
```

---


### Revision

Creates a QaScorecardRevision.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alternate_ids` | Vec<String> |  | Output only. Alternative IDs for this revision of the scorecard, e.g., `latest`. |
| `create_time` | String |  | Output only. The timestamp that the revision was created. |
| `name` | String |  | Identifier. The name of the scorecard revision. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision} |
| `snapshot` | String |  | The snapshot of the scorecard at the time of this revision's creation. |
| `state` | String |  | Output only. State of the scorecard revision, indicating whether it's ready to be used in analysis. |
| `parent` | String | ✅ | Required. The parent resource of the QaScorecardRevision. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alternate_ids` | Vec<String> | Output only. Alternative IDs for this revision of the scorecard, e.g., `latest`. |
| `create_time` | String | Output only. The timestamp that the revision was created. |
| `name` | String | Identifier. The name of the scorecard revision. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision} |
| `snapshot` | String | The snapshot of the scorecard at the time of this revision's creation. |
| `state` | String | Output only. State of the scorecard revision, indicating whether it's ready to be used in analysis. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create revision
revision = provider.contactcenterinsights_api.Revision {
    parent = "value"  # Required. The parent resource of the QaScorecardRevision.
}

# Access revision outputs
revision_id = revision.id
revision_alternate_ids = revision.alternate_ids
revision_create_time = revision.create_time
revision_name = revision.name
revision_snapshot = revision.snapshot
revision_state = revision.state
```

---


### Location

Upload feedback labels from an external source in bulk. Currently supports labeling Quality AI example conversations.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_only` | bool |  | Optional. If set, upload will not happen and the labels will be validated. If not set, then default behavior will be to upload the labels after validation is complete. |
| `gcs_source` | String |  | A cloud storage bucket source. |
| `sheets_source` | String |  | A sheets document source. |
| `parent` | String | ✅ | Required. The parent resource for new feedback labels. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feedback_labels` | Vec<String> | The feedback labels that match the request. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.contactcenterinsights_api.Location {
    parent = "value"  # Required. The parent resource for new feedback labels.
}

# Access location outputs
location_id = location.id
location_feedback_labels = location.feedback_labels
location_next_page_token = location.next_page_token
```

---


### Insightsdata

Export insights data to a destination defined in the request body.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key` | String |  | A fully qualified KMS key name for BigQuery tables protected by CMEK. Format: projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version} |
| `parent` | String |  | Required. The parent resource to export data from. |
| `write_disposition` | String |  | Options for what to do if the destination table already exists. |
| `big_query_destination` | String |  | Specified if sink is a BigQuery table. |
| `export_schema_version` | String |  | Optional. Version of the export schema. |
| `filter` | String |  | A filter to reduce results to a specific subset. Useful for exporting conversations with specific properties. |
| `parent` | String | ✅ | Required. The parent resource to export data from. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create insightsdata
insightsdata = provider.contactcenterinsights_api.Insightsdata {
    parent = "value"  # Required. The parent resource to export data from.
}

```

---


### Issue_model

Creates an issue model.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `issue_count` | String |  | Output only. Number of issues in this issue model. |
| `name` | String |  | Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model} |
| `input_data_config` | String |  | Configs for the input data that used to create the issue model. |
| `model_type` | String |  | Type of the model. |
| `create_time` | String |  | Output only. The time at which this issue model was created. |
| `display_name` | String |  | The representative name for the issue model. |
| `training_stats` | String |  | Output only. Immutable. The issue model's label statistics on its training data. |
| `language_code` | String |  | Language of the model. |
| `update_time` | String |  | Output only. The most recent time at which the issue model was updated. |
| `state` | String |  | Output only. State of the model. |
| `parent` | String | ✅ | Required. The parent resource of the issue model. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issue_count` | String | Output only. Number of issues in this issue model. |
| `name` | String | Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model} |
| `input_data_config` | String | Configs for the input data that used to create the issue model. |
| `model_type` | String | Type of the model. |
| `create_time` | String | Output only. The time at which this issue model was created. |
| `display_name` | String | The representative name for the issue model. |
| `training_stats` | String | Output only. Immutable. The issue model's label statistics on its training data. |
| `language_code` | String | Language of the model. |
| `update_time` | String | Output only. The most recent time at which the issue model was updated. |
| `state` | String | Output only. State of the model. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create issue_model
issue_model = provider.contactcenterinsights_api.Issue_model {
    parent = "value"  # Required. The parent resource of the issue model.
}

# Access issue_model outputs
issue_model_id = issue_model.id
issue_model_issue_count = issue_model.issue_count
issue_model_name = issue_model.name
issue_model_input_data_config = issue_model.input_data_config
issue_model_model_type = issue_model.model_type
issue_model_create_time = issue_model.create_time
issue_model_display_name = issue_model.display_name
issue_model_training_stats = issue_model.training_stats
issue_model_language_code = issue_model.language_code
issue_model_update_time = issue_model.update_time
issue_model_state = issue_model.state
```

---


### Analyse

Creates an analysis. The long running operation is done when the analysis has completed.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_time` | String |  | Output only. The time at which the analysis was requested. |
| `analysis_result` | String |  | Output only. The result of the analysis, which is populated when the analysis finishes. |
| `name` | String |  | Immutable. The resource name of the analysis. Format: projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis} |
| `annotator_selector` | String |  | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `create_time` | String |  | Output only. The time at which the analysis was created, which occurs when the long-running operation completes. |
| `parent` | String | ✅ | Required. The parent resource of the analysis. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_time` | String | Output only. The time at which the analysis was requested. |
| `analysis_result` | String | Output only. The result of the analysis, which is populated when the analysis finishes. |
| `name` | String | Immutable. The resource name of the analysis. Format: projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis} |
| `annotator_selector` | String | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `create_time` | String | Output only. The time at which the analysis was created, which occurs when the long-running operation completes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create analyse
analyse = provider.contactcenterinsights_api.Analyse {
    parent = "value"  # Required. The parent resource of the analysis.
}

# Access analyse outputs
analyse_id = analyse.id
analyse_request_time = analyse.request_time
analyse_analysis_result = analyse.analysis_result
analyse_name = analyse.name
analyse_annotator_selector = analyse.annotator_selector
analyse_create_time = analyse.create_time
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
operation_0 = provider.contactcenterinsights_api.Operation {
    name = "value-0"
}
operation_1 = provider.contactcenterinsights_api.Operation {
    name = "value-1"
}
operation_2 = provider.contactcenterinsights_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.contactcenterinsights_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Contactcenterinsights_api Documentation](https://cloud.google.com/contactcenterinsights_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
