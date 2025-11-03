# Contactcenterinsights_api Service



**Resources**: 23

---

## Overview

The contactcenterinsights_api service provides access to 23 resource types:

- [Issue_model](#issue_model) [CRUD]
- [Issue](#issue) [CRUD]
- [Authorized_view_set](#authorized_view_set) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Assessment_rule](#assessment_rule) [CRUD]
- [Qa_scorecard](#qa_scorecard) [CRUD]
- [Analyse](#analyse) [CRD]
- [Operation](#operation) [CR]
- [Qa_question](#qa_question) [CRUD]
- [Qa_question_tag](#qa_question_tag) [CRUD]
- [Authorized_view](#authorized_view) [CRUD]
- [Note](#note) [CRUD]
- [Analysis_rule](#analysis_rule) [CRUD]
- [Revision](#revision) [CRD]
- [Location](#location) [CRU]
- [Encryption_spec](#encryption_spec) [C]
- [Feedback_label](#feedback_label) [CRUD]
- [Segment](#segment) [C]
- [Assessment](#assessment) [CRD]
- [View](#view) [CRUD]
- [Insightsdata](#insightsdata) [C]
- [Phrase_matcher](#phrase_matcher) [CRUD]
- [Conversation](#conversation) [CRUD]

---

## Resources


### Issue_model

Creates an issue model.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_data_config` | String |  | Configs for the input data that used to create the issue model. |
| `display_name` | String |  | The representative name for the issue model. |
| `training_stats` | String |  | Output only. Immutable. The issue model's label statistics on its training data. |
| `language_code` | String |  | Language of the model. |
| `update_time` | String |  | Output only. The most recent time at which the issue model was updated. |
| `name` | String |  | Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model} |
| `model_type` | String |  | Type of the model. |
| `state` | String |  | Output only. State of the model. |
| `create_time` | String |  | Output only. The time at which this issue model was created. |
| `issue_count` | String |  | Output only. Number of issues in this issue model. |
| `parent` | String | ✅ | Required. The parent resource of the issue model. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input_data_config` | String | Configs for the input data that used to create the issue model. |
| `display_name` | String | The representative name for the issue model. |
| `training_stats` | String | Output only. Immutable. The issue model's label statistics on its training data. |
| `language_code` | String | Language of the model. |
| `update_time` | String | Output only. The most recent time at which the issue model was updated. |
| `name` | String | Immutable. The resource name of the issue model. Format: projects/{project}/locations/{location}/issueModels/{issue_model} |
| `model_type` | String | Type of the model. |
| `state` | String | Output only. State of the model. |
| `create_time` | String | Output only. The time at which this issue model was created. |
| `issue_count` | String | Output only. Number of issues in this issue model. |


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
issue_model_input_data_config = issue_model.input_data_config
issue_model_display_name = issue_model.display_name
issue_model_training_stats = issue_model.training_stats
issue_model_language_code = issue_model.language_code
issue_model_update_time = issue_model.update_time
issue_model_name = issue_model.name
issue_model_model_type = issue_model.model_type
issue_model_state = issue_model.state
issue_model_create_time = issue_model.create_time
issue_model_issue_count = issue_model.issue_count
```

---


### Issue

Creates an issue.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The most recent time that this issue was updated. |
| `display_name` | String |  | The representative name for the issue. |
| `create_time` | String |  | Output only. The time at which this issue was created. |
| `display_description` | String |  | Representative description of the issue. |
| `name` | String |  | Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue} |
| `sample_utterances` | Vec<String> |  | Output only. Resource names of the sample representative utterances that match to this issue. |
| `parent` | String | ✅ | Required. The parent resource of the issue. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The most recent time that this issue was updated. |
| `display_name` | String | The representative name for the issue. |
| `create_time` | String | Output only. The time at which this issue was created. |
| `display_description` | String | Representative description of the issue. |
| `name` | String | Immutable. The resource name of the issue. Format: projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue} |
| `sample_utterances` | Vec<String> | Output only. Resource names of the sample representative utterances that match to this issue. |


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
issue_update_time = issue.update_time
issue_display_name = issue.display_name
issue_create_time = issue.create_time
issue_display_description = issue.display_description
issue_name = issue.name
issue_sample_utterances = issue.sample_utterances
```

---


### Authorized_view_set

Create AuthorizedViewSet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Display Name. Limit 64 characters. |
| `name` | String |  | Identifier. The resource name of the AuthorizedViewSet. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set} |
| `create_time` | String |  | Output only. Create time. |
| `update_time` | String |  | Output only. Update time. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizedViewSet. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Display Name. Limit 64 characters. |
| `name` | String | Identifier. The resource name of the AuthorizedViewSet. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set} |
| `create_time` | String | Output only. Create time. |
| `update_time` | String | Output only. Update time. |


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
authorized_view_set_display_name = authorized_view_set.display_name
authorized_view_set_name = authorized_view_set.name
authorized_view_set_create_time = authorized_view_set.create_time
authorized_view_set_update_time = authorized_view_set.update_time
```

---


### Dataset

Creates a dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ttl` | String |  | Optional. Option TTL for the dataset. |
| `type` | String |  | Dataset usage type. |
| `update_time` | String |  | Output only. Dataset update time. |
| `display_name` | String |  | Display name for the dataaset |
| `description` | String |  | Dataset description. |
| `create_time` | String |  | Output only. Dataset create time. |
| `name` | String |  | Immutable. Identifier. Resource name of the dataset. Format: projects/{project}/locations/{location}/datasets/{dataset} |
| `parent` | String | ✅ | Required. The parent resource of the dataset. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ttl` | String | Optional. Option TTL for the dataset. |
| `type` | String | Dataset usage type. |
| `update_time` | String | Output only. Dataset update time. |
| `display_name` | String | Display name for the dataaset |
| `description` | String | Dataset description. |
| `create_time` | String | Output only. Dataset create time. |
| `name` | String | Immutable. Identifier. Resource name of the dataset. Format: projects/{project}/locations/{location}/datasets/{dataset} |


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
dataset_ttl = dataset.ttl
dataset_type = dataset.type
dataset_update_time = dataset.update_time
dataset_display_name = dataset.display_name
dataset_description = dataset.description
dataset_create_time = dataset.create_time
dataset_name = dataset.name
```

---


### Assessment_rule

Creates an assessment rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schedule_info` | String |  | Schedule info for the assessment rule. |
| `update_time` | String |  | Output only. The most recent time at which this assessment rule was updated. |
| `create_time` | String |  | Output only. The time at which this assessment rule was created. |
| `active` | bool |  | If true, apply this rule to conversations. Otherwise, this rule is inactive. |
| `display_name` | String |  | Display Name of the assessment rule. |
| `sample_rule` | String |  | The sample rule for the assessment rule. |
| `name` | String |  | Identifier. The resource name of the assessment rule. Format: projects/{project}/locations/{location}/assessmentRules/{assessment_rule} |
| `parent` | String | ✅ | Required. The parent resource of the assessment rule. Required. The location to create a assessment rule for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schedule_info` | String | Schedule info for the assessment rule. |
| `update_time` | String | Output only. The most recent time at which this assessment rule was updated. |
| `create_time` | String | Output only. The time at which this assessment rule was created. |
| `active` | bool | If true, apply this rule to conversations. Otherwise, this rule is inactive. |
| `display_name` | String | Display Name of the assessment rule. |
| `sample_rule` | String | The sample rule for the assessment rule. |
| `name` | String | Identifier. The resource name of the assessment rule. Format: projects/{project}/locations/{location}/assessmentRules/{assessment_rule} |


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
assessment_rule_schedule_info = assessment_rule.schedule_info
assessment_rule_update_time = assessment_rule.update_time
assessment_rule_create_time = assessment_rule.create_time
assessment_rule_active = assessment_rule.active
assessment_rule_display_name = assessment_rule.display_name
assessment_rule_sample_rule = assessment_rule.sample_rule
assessment_rule_name = assessment_rule.name
```

---


### Qa_scorecard

Create a QaScorecard.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A text description explaining the intent of the scorecard. |
| `source` | String |  | Output only. The source of the scorecard. |
| `create_time` | String |  | Output only. The time at which this scorecard was created. |
| `display_name` | String |  | The user-specified display name of the scorecard. |
| `is_default` | bool |  | Whether the scorecard is the default one for the project. A default scorecard cannot be deleted and will always appear first in scorecard selector. |
| `update_time` | String |  | Output only. The most recent time at which the scorecard was updated. |
| `name` | String |  | Identifier. The scorecard name. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard} |
| `parent` | String | ✅ | Required. The parent resource of the QaScorecard. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A text description explaining the intent of the scorecard. |
| `source` | String | Output only. The source of the scorecard. |
| `create_time` | String | Output only. The time at which this scorecard was created. |
| `display_name` | String | The user-specified display name of the scorecard. |
| `is_default` | bool | Whether the scorecard is the default one for the project. A default scorecard cannot be deleted and will always appear first in scorecard selector. |
| `update_time` | String | Output only. The most recent time at which the scorecard was updated. |
| `name` | String | Identifier. The scorecard name. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard} |


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
qa_scorecard_description = qa_scorecard.description
qa_scorecard_source = qa_scorecard.source
qa_scorecard_create_time = qa_scorecard.create_time
qa_scorecard_display_name = qa_scorecard.display_name
qa_scorecard_is_default = qa_scorecard.is_default
qa_scorecard_update_time = qa_scorecard.update_time
qa_scorecard_name = qa_scorecard.name
```

---


### Analyse

Creates an analysis. The long running operation is done when the analysis has completed.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which the analysis was created, which occurs when the long-running operation completes. |
| `analysis_result` | String |  | Output only. The result of the analysis, which is populated when the analysis finishes. |
| `annotator_selector` | String |  | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `name` | String |  | Immutable. The resource name of the analysis. Format: projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis} |
| `request_time` | String |  | Output only. The time at which the analysis was requested. |
| `parent` | String | ✅ | Required. The parent resource of the analysis. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the analysis was created, which occurs when the long-running operation completes. |
| `analysis_result` | String | Output only. The result of the analysis, which is populated when the analysis finishes. |
| `annotator_selector` | String | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `name` | String | Immutable. The resource name of the analysis. Format: projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis} |
| `request_time` | String | Output only. The time at which the analysis was requested. |


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
analyse_create_time = analyse.create_time
analyse_analysis_result = analyse.analysis_result
analyse_annotator_selector = analyse.annotator_selector
analyse_name = analyse.name
analyse_request_time = analyse.request_time
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Qa_question

Create a QaQuestion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metrics` | String |  | Metrics of the underlying tuned LLM over a holdout/test set while fine tuning the underlying LLM for the given question. This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `tags` | Vec<String> |  | Questions are tagged for categorization and scoring. Tags can either be: - Default Tags: These are predefined categories. They are identified by their string value (e.g., "BUSINESS", "COMPLIANCE", and "CUSTOMER"). - Custom Tags: These are user-defined categories. They are identified by their full resource name (e.g., projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}). Both default and custom tags are used to group questions and to influence the scoring of each question. |
| `order` | i64 |  | Defines the order of the question within its parent scorecard revision. |
| `answer_instructions` | String |  | Instructions describing how to determine the answer. |
| `question_body` | String |  | Question text. E.g., "Did the agent greet the customer?" |
| `question_type` | String |  | The type of question. |
| `predefined_question_config` | String |  | The configuration of the predefined question. This field will only be set if the Question Type is predefined. |
| `create_time` | String |  | Output only. The time at which this question was created. |
| `update_time` | String |  | Output only. The most recent time at which the question was updated. |
| `tuning_metadata` | String |  | Metadata about the tuning operation for the question.This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `name` | String |  | Identifier. The resource name of the question. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question} |
| `answer_choices` | Vec<String> |  | A list of valid answers to the question, which the LLM must choose from. |
| `abbreviation` | String |  | Short, descriptive string, used in the UI where it's not practical to display the full question body. E.g., "Greeting". |
| `parent` | String | ✅ | Required. The parent resource of the QaQuestion. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | String | Metrics of the underlying tuned LLM over a holdout/test set while fine tuning the underlying LLM for the given question. This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `tags` | Vec<String> | Questions are tagged for categorization and scoring. Tags can either be: - Default Tags: These are predefined categories. They are identified by their string value (e.g., "BUSINESS", "COMPLIANCE", and "CUSTOMER"). - Custom Tags: These are user-defined categories. They are identified by their full resource name (e.g., projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}). Both default and custom tags are used to group questions and to influence the scoring of each question. |
| `order` | i64 | Defines the order of the question within its parent scorecard revision. |
| `answer_instructions` | String | Instructions describing how to determine the answer. |
| `question_body` | String | Question text. E.g., "Did the agent greet the customer?" |
| `question_type` | String | The type of question. |
| `predefined_question_config` | String | The configuration of the predefined question. This field will only be set if the Question Type is predefined. |
| `create_time` | String | Output only. The time at which this question was created. |
| `update_time` | String | Output only. The most recent time at which the question was updated. |
| `tuning_metadata` | String | Metadata about the tuning operation for the question.This field will only be populated if and only if the question is part of a scorecard revision that has been tuned. |
| `name` | String | Identifier. The resource name of the question. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question} |
| `answer_choices` | Vec<String> | A list of valid answers to the question, which the LLM must choose from. |
| `abbreviation` | String | Short, descriptive string, used in the UI where it's not practical to display the full question body. E.g., "Greeting". |


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
qa_question_metrics = qa_question.metrics
qa_question_tags = qa_question.tags
qa_question_order = qa_question.order
qa_question_answer_instructions = qa_question.answer_instructions
qa_question_question_body = qa_question.question_body
qa_question_question_type = qa_question.question_type
qa_question_predefined_question_config = qa_question.predefined_question_config
qa_question_create_time = qa_question.create_time
qa_question_update_time = qa_question.update_time
qa_question_tuning_metadata = qa_question.tuning_metadata
qa_question_name = qa_question.name
qa_question_answer_choices = qa_question.answer_choices
qa_question_abbreviation = qa_question.abbreviation
```

---


### Qa_question_tag

Creates a QaQuestionTag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The most recent time at which the question tag was updated. |
| `name` | String |  | Identifier. Resource name for the QaQuestionTag Format projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag} In the above format, the last segment, i.e., qa_question_tag, is a server-generated ID corresponding to the tag resource. |
| `display_name` | String |  | Required. A user-specified display name for the tag. |
| `create_time` | String |  | Output only. The time at which the question tag was created. |
| `qa_question_ids` | Vec<String> |  | Optional. The list of Scorecard Question IDs that the tag applies to. Each QaQuestionId is represented as a full resource name containing the Question ID. Lastly, Since a tag may not necessarily be referenced by any Scorecard Questions, we treat this field as optional. |
| `parent` | String | ✅ | Required. The parent resource of the QaQuestionTag. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The most recent time at which the question tag was updated. |
| `name` | String | Identifier. Resource name for the QaQuestionTag Format projects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag} In the above format, the last segment, i.e., qa_question_tag, is a server-generated ID corresponding to the tag resource. |
| `display_name` | String | Required. A user-specified display name for the tag. |
| `create_time` | String | Output only. The time at which the question tag was created. |
| `qa_question_ids` | Vec<String> | Optional. The list of Scorecard Question IDs that the tag applies to. Each QaQuestionId is represented as a full resource name containing the Question ID. Lastly, Since a tag may not necessarily be referenced by any Scorecard Questions, we treat this field as optional. |


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
qa_question_tag_update_time = qa_question_tag.update_time
qa_question_tag_name = qa_question_tag.name
qa_question_tag_display_name = qa_question_tag.display_name
qa_question_tag_create_time = qa_question_tag.create_time
qa_question_tag_qa_question_ids = qa_question_tag.qa_question_ids
```

---


### Authorized_view

Create AuthorizedView

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the AuthorizedView. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set}/authorizedViews/{authorized_view} |
| `update_time` | String |  | Output only. The most recent time at which the authorized view was updated. |
| `display_name` | String |  | Display Name. Limit 64 characters. |
| `conversation_filter` | String |  | A filter to reduce conversation results to a specific subset. The AuthorizedView's assigned permission (read/write) could be applied to the subset of conversations. If conversation_filter is empty, there is no restriction on the conversations that the AuthorizedView can access. Having *authorizedViews.get* access to the AuthorizedView means having the same read/write access to the Conversations (as well as metadata/annotations linked to the conversation) that this AuthorizedView has. |
| `create_time` | String |  | Output only. The time at which the authorized view was created. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizedView. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the AuthorizedView. Format: projects/{project}/locations/{location}/authorizedViewSets/{authorized_view_set}/authorizedViews/{authorized_view} |
| `update_time` | String | Output only. The most recent time at which the authorized view was updated. |
| `display_name` | String | Display Name. Limit 64 characters. |
| `conversation_filter` | String | A filter to reduce conversation results to a specific subset. The AuthorizedView's assigned permission (read/write) could be applied to the subset of conversations. If conversation_filter is empty, there is no restriction on the conversations that the AuthorizedView can access. Having *authorizedViews.get* access to the AuthorizedView means having the same read/write access to the Conversations (as well as metadata/annotations linked to the conversation) that this AuthorizedView has. |
| `create_time` | String | Output only. The time at which the authorized view was created. |


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
authorized_view_name = authorized_view.name
authorized_view_update_time = authorized_view.update_time
authorized_view_display_name = authorized_view.display_name
authorized_view_conversation_filter = authorized_view.conversation_filter
authorized_view_create_time = authorized_view.create_time
```

---


### Note

Create Note.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assessment_note` | String |  | The note is associated to the entire parent assessment. |
| `content` | String |  | The note content. |
| `conversation_turn_note` | String |  | The note is associated with a conversation turn. |
| `create_time` | String |  | Output only. The time at which the note was created. |
| `qa_question_note` | String |  | The note is associated with a QA question in one of the conversation's scorecard results. |
| `name` | String |  | Identifier. The resource name of the note. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment}/notes/{note} |
| `note_creator` | String |  | Output only. The user that created the note. |
| `update_time` | String |  | Output only. The time at which the note was last updated. |
| `parent` | String | ✅ | Required. The parent resource of the note. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `notes` | Vec<String> | The notes that match the request. |


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
note_next_page_token = note.next_page_token
note_notes = note.notes
```

---


### Analysis_rule

Creates a analysis rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which this analysis rule was created. |
| `update_time` | String |  | Output only. The most recent time at which this analysis rule was updated. |
| `analysis_percentage` | f64 |  | Percentage of conversations that we should apply this analysis setting automatically, between [0, 1]. For example, 0.1 means 10%. Conversations are sampled in a determenestic way. The original runtime_percentage & upload percentage will be replaced by defining filters on the conversation. |
| `active` | bool |  | If true, apply this rule to conversations. Otherwise, this rule is inactive and saved as a draft. |
| `display_name` | String |  | Display Name of the analysis rule. |
| `name` | String |  | Identifier. The resource name of the analysis rule. Format: projects/{project}/locations/{location}/analysisRules/{analysis_rule} |
| `conversation_filter` | String |  | Filter for the conversations that should apply this analysis rule. An empty filter means this analysis rule applies to all conversations. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `annotator_selector` | String |  | Selector of annotators to run and the phrase matchers to use for conversations that matches the conversation_filter. If not specified, NO annotators will be run. |
| `parent` | String | ✅ | Required. The parent resource of the analysis rule. Required. The location to create a analysis rule for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which this analysis rule was created. |
| `update_time` | String | Output only. The most recent time at which this analysis rule was updated. |
| `analysis_percentage` | f64 | Percentage of conversations that we should apply this analysis setting automatically, between [0, 1]. For example, 0.1 means 10%. Conversations are sampled in a determenestic way. The original runtime_percentage & upload percentage will be replaced by defining filters on the conversation. |
| `active` | bool | If true, apply this rule to conversations. Otherwise, this rule is inactive and saved as a draft. |
| `display_name` | String | Display Name of the analysis rule. |
| `name` | String | Identifier. The resource name of the analysis rule. Format: projects/{project}/locations/{location}/analysisRules/{analysis_rule} |
| `conversation_filter` | String | Filter for the conversations that should apply this analysis rule. An empty filter means this analysis rule applies to all conversations. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
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
analysis_rule_update_time = analysis_rule.update_time
analysis_rule_analysis_percentage = analysis_rule.analysis_percentage
analysis_rule_active = analysis_rule.active
analysis_rule_display_name = analysis_rule.display_name
analysis_rule_name = analysis_rule.name
analysis_rule_conversation_filter = analysis_rule.conversation_filter
analysis_rule_annotator_selector = analysis_rule.annotator_selector
```

---


### Revision

Creates a QaScorecardRevision.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the scorecard revision. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision} |
| `alternate_ids` | Vec<String> |  | Output only. Alternative IDs for this revision of the scorecard, e.g., `latest`. |
| `state` | String |  | Output only. State of the scorecard revision, indicating whether it's ready to be used in analysis. |
| `snapshot` | String |  | The snapshot of the scorecard at the time of this revision's creation. |
| `create_time` | String |  | Output only. The timestamp that the revision was created. |
| `parent` | String | ✅ | Required. The parent resource of the QaScorecardRevision. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the scorecard revision. Format: projects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision} |
| `alternate_ids` | Vec<String> | Output only. Alternative IDs for this revision of the scorecard, e.g., `latest`. |
| `state` | String | Output only. State of the scorecard revision, indicating whether it's ready to be used in analysis. |
| `snapshot` | String | The snapshot of the scorecard at the time of this revision's creation. |
| `create_time` | String | Output only. The timestamp that the revision was created. |


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
revision_name = revision.name
revision_alternate_ids = revision.alternate_ids
revision_state = revision.state
revision_snapshot = revision.snapshot
revision_create_time = revision.create_time
```

---


### Location

Delete feedback labels in bulk using a filter.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Required. The parent resource for new feedback labels. |
| `filter` | String |  | Optional. A filter to reduce results to a specific subset. Supports disjunctions (OR) and conjunctions (AND). Supported fields: * `issue_model_id` * `qa_question_id` * `qa_scorecard_id` * `min_create_time` * `max_create_time` * `min_update_time` * `max_update_time` * `feedback_label_type`: QUALITY_AI, TOPIC_MODELING |
| `parent` | String | ✅ | Required. The parent resource for new feedback labels. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `feedback_labels` | Vec<String> | The feedback labels that match the request. |


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
location_next_page_token = location.next_page_token
location_feedback_labels = location.feedback_labels
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


### Feedback_label

Create feedback label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time of the label. |
| `label` | String |  | String label used for Topic Modeling. |
| `update_time` | String |  | Output only. Update time of the label. |
| `name` | String |  | Immutable. Resource name of the FeedbackLabel. Format: projects/{project}/locations/{location}/conversations/{conversation}/feedbackLabels/{feedback_label} |
| `qa_answer_label` | String |  | QaAnswer label used for Quality AI example conversations. |
| `labeled_resource` | String |  | Name of the resource to be labeled. Supported resources are: * `projects/{project}/locations/{location}/qaScorecards/{scorecard}/revisions/{revision}/qaQuestions/{question}` * `projects/{project}/locations/{location}/issueModels/{issue_model}` * `projects/{project}/locations/{location}/generators/{generator_id}` |
| `parent` | String | ✅ | Required. The parent resource of the feedback label. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time of the label. |
| `label` | String | String label used for Topic Modeling. |
| `update_time` | String | Output only. Update time of the label. |
| `name` | String | Immutable. Resource name of the FeedbackLabel. Format: projects/{project}/locations/{location}/conversations/{conversation}/feedbackLabels/{feedback_label} |
| `qa_answer_label` | String | QaAnswer label used for Quality AI example conversations. |
| `labeled_resource` | String | Name of the resource to be labeled. Supported resources are: * `projects/{project}/locations/{location}/qaScorecards/{scorecard}/revisions/{revision}/qaQuestions/{question}` * `projects/{project}/locations/{location}/issueModels/{issue_model}` * `projects/{project}/locations/{location}/generators/{generator_id}` |


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
feedback_label_label = feedback_label.label
feedback_label_update_time = feedback_label.update_time
feedback_label_name = feedback_label.name
feedback_label_qa_answer_label = feedback_label.qa_answer_label
feedback_label_labeled_resource = feedback_label.labeled_resource
```

---


### Segment

Analyzes multiple conversations in a single request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotator_selector` | String |  | To select the annotators to run and the phrase matchers to use (if any). If not specified, all annotators will be run. |
| `filter` | String |  | Required. Filter used to select the subset of conversations to analyze. |
| `parent` | String |  | Required. The parent resource to create analyses in. |
| `analysis_percentage` | f64 |  | Required. Percentage of selected conversation to analyze, between [0, 100]. |
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


### Assessment

Create Assessment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time at which the assessment was last updated. |
| `agent_info` | String |  | Information about the agent the assessment is for. |
| `create_time` | String |  | Output only. The time at which the assessment was created. |
| `name` | String |  | Identifier. The resource name of the assessment. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment} |
| `state` | String |  | Output only. The state of the assessment. |
| `parent` | String | ✅ | Required. The parent resource of the assessment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which the assessment was last updated. |
| `agent_info` | String | Information about the agent the assessment is for. |
| `create_time` | String | Output only. The time at which the assessment was created. |
| `name` | String | Identifier. The resource name of the assessment. Format: projects/{project}/locations/{location}/conversations/{conversation}/assessments/{assessment} |
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
assessment_agent_info = assessment.agent_info
assessment_create_time = assessment.create_time
assessment_name = assessment.name
assessment_state = assessment.state
```

---


### View

Creates a view.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The human-readable display name of the view. |
| `value` | String |  | A filter to reduce conversation results to a specific subset. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `update_time` | String |  | Output only. The most recent time at which the view was updated. |
| `create_time` | String |  | Output only. The time at which this view was created. |
| `name` | String |  | Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view} |
| `parent` | String | ✅ | Required. The parent resource of the view. Required. The location to create a view for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The human-readable display name of the view. |
| `value` | String | A filter to reduce conversation results to a specific subset. Refer to https://cloud.google.com/contact-center/insights/docs/filtering for details. |
| `update_time` | String | Output only. The most recent time at which the view was updated. |
| `create_time` | String | Output only. The time at which this view was created. |
| `name` | String | Immutable. The resource name of the view. Format: projects/{project}/locations/{location}/views/{view} |


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
view_display_name = view.display_name
view_value = view.value
view_update_time = view.update_time
view_create_time = view.create_time
view_name = view.name
```

---


### Insightsdata

Export insights data to a destination defined in the request body.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key` | String |  | A fully qualified KMS key name for BigQuery tables protected by CMEK. Format: projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version} |
| `filter` | String |  | A filter to reduce results to a specific subset. Useful for exporting conversations with specific properties. |
| `export_schema_version` | String |  | Optional. Version of the export schema. |
| `big_query_destination` | String |  | Specified if sink is a BigQuery table. |
| `parent` | String |  | Required. The parent resource to export data from. |
| `write_disposition` | String |  | Options for what to do if the destination table already exists. |
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


### Phrase_matcher

Creates a phrase matcher.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The most recent time at which the phrase matcher was updated. |
| `version_tag` | String |  | The customized version tag to use for the phrase matcher. If not specified, it will default to `revision_id`. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the phrase matcher. A new revision is committed whenever the matcher is changed, except when it is activated or deactivated. A server generated random ID will be used. Example: locations/global/phraseMatchers/my-first-matcher@1234567 |
| `phrase_match_rule_groups` | Vec<String> |  | A list of phase match rule groups that are included in this matcher. |
| `active` | bool |  | Applies the phrase matcher only when it is active. |
| `name` | String |  | The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher} |
| `activation_update_time` | String |  | Output only. The most recent time at which the activation status was updated. |
| `display_name` | String |  | The human-readable name of the phrase matcher. |
| `revision_create_time` | String |  | Output only. The timestamp of when the revision was created. It is also the create time when a new matcher is added. |
| `role_match` | String |  | The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript. |
| `type` | String |  | Required. The type of this phrase matcher. |
| `parent` | String | ✅ | Required. The parent resource of the phrase matcher. Required. The location to create a phrase matcher for. Format: `projects//locations/` or `projects//locations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The most recent time at which the phrase matcher was updated. |
| `version_tag` | String | The customized version tag to use for the phrase matcher. If not specified, it will default to `revision_id`. |
| `revision_id` | String | Output only. Immutable. The revision ID of the phrase matcher. A new revision is committed whenever the matcher is changed, except when it is activated or deactivated. A server generated random ID will be used. Example: locations/global/phraseMatchers/my-first-matcher@1234567 |
| `phrase_match_rule_groups` | Vec<String> | A list of phase match rule groups that are included in this matcher. |
| `active` | bool | Applies the phrase matcher only when it is active. |
| `name` | String | The resource name of the phrase matcher. Format: projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher} |
| `activation_update_time` | String | Output only. The most recent time at which the activation status was updated. |
| `display_name` | String | The human-readable name of the phrase matcher. |
| `revision_create_time` | String | Output only. The timestamp of when the revision was created. It is also the create time when a new matcher is added. |
| `role_match` | String | The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript. |
| `type` | String | Required. The type of this phrase matcher. |


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
phrase_matcher_update_time = phrase_matcher.update_time
phrase_matcher_version_tag = phrase_matcher.version_tag
phrase_matcher_revision_id = phrase_matcher.revision_id
phrase_matcher_phrase_match_rule_groups = phrase_matcher.phrase_match_rule_groups
phrase_matcher_active = phrase_matcher.active
phrase_matcher_name = phrase_matcher.name
phrase_matcher_activation_update_time = phrase_matcher.activation_update_time
phrase_matcher_display_name = phrase_matcher.display_name
phrase_matcher_revision_create_time = phrase_matcher.revision_create_time
phrase_matcher_role_match = phrase_matcher.role_match
phrase_matcher_type = phrase_matcher.type
```

---


### Conversation

Creates a conversation. Note that this method does not support audio transcription or redaction. Use `conversations.upload` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `obfuscated_user_id` | String |  | Obfuscated user ID which the customer sent to us. |
| `transcript` | String |  | Output only. The conversation transcript. |
| `update_time` | String |  | Output only. The most recent time at which the conversation was updated. |
| `name` | String |  | Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation} |
| `metadata_json` | String |  | Input only. JSON metadata encoded as a string. This field is primarily used by Insights integrations with various telephony systems and must be in one of Insight's supported formats. |
| `expire_time` | String |  | The time at which this conversation should expire. After this time, the conversation data and any associated analyses will be deleted. |
| `latest_analysis` | String |  | Output only. The conversation's latest analysis, if one exists. |
| `labels` | HashMap<String, String> |  | A map for the user to specify any custom fields. A maximum of 100 labels per conversation is allowed, with a maximum of 256 characters per entry. |
| `language_code` | String |  | A user-specified language code for the conversation. |
| `duration` | String |  | Output only. The duration of the conversation. |
| `agent_id` | String |  | An opaque, user-specified string representing the human agent who handled the conversation. |
| `data_source` | String |  | The source of the audio and transcription for the conversation. |
| `quality_metadata` | String |  | Conversation metadata related to quality management. |
| `medium` | String |  | Immutable. The conversation medium, if unspecified will default to PHONE_CALL. |
| `runtime_annotations` | Vec<String> |  | Output only. The annotations that were generated during the customer and agent interaction. |
| `turn_count` | i64 |  | Output only. The number of turns in the conversation. |
| `ttl` | String |  | Input only. The TTL for this resource. If specified, then this TTL will be used to calculate the expire time. |
| `start_time` | String |  | The time at which the conversation started. |
| `dialogflow_intents` | HashMap<String, String> |  | Output only. All the matched Dialogflow intents in the call. The key corresponds to a Dialogflow intent, format: projects/{project}/agent/{agent}/intents/{intent} |
| `latest_summary` | String |  | Output only. Latest summary of the conversation. |
| `call_metadata` | String |  | Call-specific metadata. |
| `create_time` | String |  | Output only. The time at which the conversation was created. |
| `parent` | String | ✅ | Required. The parent resource of the conversation. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `obfuscated_user_id` | String | Obfuscated user ID which the customer sent to us. |
| `transcript` | String | Output only. The conversation transcript. |
| `update_time` | String | Output only. The most recent time at which the conversation was updated. |
| `name` | String | Immutable. The resource name of the conversation. Format: projects/{project}/locations/{location}/conversations/{conversation} |
| `metadata_json` | String | Input only. JSON metadata encoded as a string. This field is primarily used by Insights integrations with various telephony systems and must be in one of Insight's supported formats. |
| `expire_time` | String | The time at which this conversation should expire. After this time, the conversation data and any associated analyses will be deleted. |
| `latest_analysis` | String | Output only. The conversation's latest analysis, if one exists. |
| `labels` | HashMap<String, String> | A map for the user to specify any custom fields. A maximum of 100 labels per conversation is allowed, with a maximum of 256 characters per entry. |
| `language_code` | String | A user-specified language code for the conversation. |
| `duration` | String | Output only. The duration of the conversation. |
| `agent_id` | String | An opaque, user-specified string representing the human agent who handled the conversation. |
| `data_source` | String | The source of the audio and transcription for the conversation. |
| `quality_metadata` | String | Conversation metadata related to quality management. |
| `medium` | String | Immutable. The conversation medium, if unspecified will default to PHONE_CALL. |
| `runtime_annotations` | Vec<String> | Output only. The annotations that were generated during the customer and agent interaction. |
| `turn_count` | i64 | Output only. The number of turns in the conversation. |
| `ttl` | String | Input only. The TTL for this resource. If specified, then this TTL will be used to calculate the expire time. |
| `start_time` | String | The time at which the conversation started. |
| `dialogflow_intents` | HashMap<String, String> | Output only. All the matched Dialogflow intents in the call. The key corresponds to a Dialogflow intent, format: projects/{project}/agent/{agent}/intents/{intent} |
| `latest_summary` | String | Output only. Latest summary of the conversation. |
| `call_metadata` | String | Call-specific metadata. |
| `create_time` | String | Output only. The time at which the conversation was created. |


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
conversation_obfuscated_user_id = conversation.obfuscated_user_id
conversation_transcript = conversation.transcript
conversation_update_time = conversation.update_time
conversation_name = conversation.name
conversation_metadata_json = conversation.metadata_json
conversation_expire_time = conversation.expire_time
conversation_latest_analysis = conversation.latest_analysis
conversation_labels = conversation.labels
conversation_language_code = conversation.language_code
conversation_duration = conversation.duration
conversation_agent_id = conversation.agent_id
conversation_data_source = conversation.data_source
conversation_quality_metadata = conversation.quality_metadata
conversation_medium = conversation.medium
conversation_runtime_annotations = conversation.runtime_annotations
conversation_turn_count = conversation.turn_count
conversation_ttl = conversation.ttl
conversation_start_time = conversation.start_time
conversation_dialogflow_intents = conversation.dialogflow_intents
conversation_latest_summary = conversation.latest_summary
conversation_call_metadata = conversation.call_metadata
conversation_create_time = conversation.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple issue_model resources
issue_model_0 = provider.contactcenterinsights_api.Issue_model {
    parent = "value-0"
}
issue_model_1 = provider.contactcenterinsights_api.Issue_model {
    parent = "value-1"
}
issue_model_2 = provider.contactcenterinsights_api.Issue_model {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    issue_model = provider.contactcenterinsights_api.Issue_model {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Contactcenterinsights_api Documentation](https://cloud.google.com/contactcenterinsights_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
