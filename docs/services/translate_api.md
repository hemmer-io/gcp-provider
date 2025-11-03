# Translate_api Service



**Resources**: 18

---

## Overview

The translate_api service provides access to 18 resource types:

- [Glossarie](#glossarie) [CRD]
- [Location](#location) [CR]
- [Operation](#operation) [CRD]
- [Project](#project) [CR]
- [Detection](#detection) [CR]
- [Translation](#translation) [CR]
- [Language](#language) [R]
- [Project](#project) [CR]
- [Adaptive_mt_sentence](#adaptive_mt_sentence) [R]
- [Dataset](#dataset) [CRD]
- [Example](#example) [R]
- [Glossary_entrie](#glossary_entrie) [CRUD]
- [Operation](#operation) [CRD]
- [Adaptive_mt_dataset](#adaptive_mt_dataset) [CRD]
- [Glossarie](#glossarie) [CRUD]
- [Location](#location) [CR]
- [Adaptive_mt_file](#adaptive_mt_file) [RD]
- [Model](#model) [CRD]

---

## Resources


### Glossarie

Creates a glossary and returns the long-running operation. Returns NOT_FOUND, if the project doesn't exist.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. When the glossary creation was finished. |
| `entry_count` | i64 |  | Output only. The number of entries defined in the glossary. |
| `input_config` | String |  | Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints. |
| `language_codes_set` | String |  | Used with equivalent term set glossaries. |
| `name` | String |  | Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`. |
| `language_pair` | String |  | Used with unidirectional glossaries. |
| `submit_time` | String |  | Output only. When CreateGlossary was called. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. When the glossary creation was finished. |
| `entry_count` | i64 | Output only. The number of entries defined in the glossary. |
| `input_config` | String | Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints. |
| `language_codes_set` | String | Used with equivalent term set glossaries. |
| `name` | String | Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`. |
| `language_pair` | String | Used with unidirectional glossaries. |
| `submit_time` | String | Output only. When CreateGlossary was called. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create glossarie
glossarie = provider.translate_api.Glossarie {
    parent = "value"  # Required. The project name.
}

# Access glossarie outputs
glossarie_id = glossarie.id
glossarie_end_time = glossarie.end_time
glossarie_entry_count = glossarie.entry_count
glossarie_input_config = glossarie.input_config
glossarie_language_codes_set = glossarie.language_codes_set
glossarie_name = glossarie.name
glossarie_language_pair = glossarie.language_pair
glossarie_submit_time = glossarie.submit_time
```

---


### Location

Translates input text and returns translated text.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/labels for more information. |
| `contents` | Vec<String> |  | Required. The content of the input in string format. We recommend the total content be less than 30k codepoints. The max length of this field is 1024. Use BatchTranslateText for larger text. |
| `glossary_config` | String |  | Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned. |
| `mime_type` | String |  | Optional. The format of the source text, for example, "text/html", "text/plain". If left blank, the MIME type defaults to "text/html". |
| `source_language_code` | String |  | Optional. The BCP-47 language code of the input text if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response. |
| `target_language_code` | String |  | Required. The BCP-47 language code to use for translation of the input text, set to one of the language codes listed in Language Support. |
| `model` | String |  | Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, For global (non-regionalized) requests, use `location-id` `global`. For example, `projects/{project-number-or-id}/locations/global/models/general/nmt`. If not provided, the default Google model (NMT) will be used |
| `parent` | String | ✅ | Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location = provider.translate_api.Location {
    parent = "value"  # Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
}

# Access location outputs
location_id = location.id
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
location_location_id = location.location_id
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.translate_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
```

---


### Project

Translates input text and returns translated text.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/labels for more information. |
| `contents` | Vec<String> |  | Required. The content of the input in string format. We recommend the total content be less than 30k codepoints. The max length of this field is 1024. Use BatchTranslateText for larger text. |
| `glossary_config` | String |  | Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned. |
| `mime_type` | String |  | Optional. The format of the source text, for example, "text/html", "text/plain". If left blank, the MIME type defaults to "text/html". |
| `source_language_code` | String |  | Optional. The BCP-47 language code of the input text if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response. |
| `target_language_code` | String |  | Required. The BCP-47 language code to use for translation of the input text, set to one of the language codes listed in Language Support. |
| `model` | String |  | Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, For global (non-regionalized) requests, use `location-id` `global`. For example, `projects/{project-number-or-id}/locations/global/models/general/nmt`. If not provided, the default Google model (NMT) will be used |
| `parent` | String | ✅ | Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `languages` | Vec<String> | A list of supported language responses. This list contains an entry for each language the Translation API supports. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.translate_api.Project {
    parent = "value"  # Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
}

# Access project outputs
project_id = project.id
project_languages = project.languages
```

---


### Detection

Detects the language of text within a request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `q` | Vec<String> |  | The input text upon which to perform language detection. Repeat this
parameter to perform language detection on multiple text inputs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detections` | Vec<Vec<String>> | A detections contains detection results of several text |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create detection
detection = provider.translate_api.Detection {
}

# Access detection outputs
detection_id = detection.id
detection_detections = detection.detections
```

---


### Translation

Translates input text, returning translated text.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String |  | The format of the source text, in either HTML (default) or plain-text. A
value of "html" indicates HTML and a value of "text" indicates plain-text. |
| `model` | String |  | The `model` type requested for this translation. Valid values are
listed in public documentation. |
| `target` | String |  | The language to use for translation of the input text, set to one of the
language codes listed in Language Support. |
| `q` | Vec<String> |  | The input text to translate. Repeat this parameter to perform translation
operations on multiple text inputs. |
| `source` | String |  | The language of the source text, set to one of the language codes listed in
Language Support. If the source language is not specified, the API will
attempt to identify the source language automatically and return it within
the response. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `translations` | Vec<String> | Translations contains list of translation results of given text |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create translation
translation = provider.translate_api.Translation {
}

# Access translation outputs
translation_id = translation.id
translation_translations = translation.translations
```

---


### Language

Returns a list of supported languages for translation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `languages` | Vec<String> | List of source/target languages supported by the translation API. If target parameter is unspecified, the list is sorted by the ASCII code point order of the language code. If target parameter is specified, the list is sorted by the collation order of the language name in the target language. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access language outputs
language_id = language.id
language_languages = language.languages
```

---


### Project

Romanize input text written in non-Latin scripts to Latin text.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_language_code` | String |  | Optional. The ISO-639 language code of the input text if known, for example, "hi" or "zh". If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language for each content in the response. |
| `contents` | Vec<String> |  | Required. The content of the input in string format. |
| `parent` | String | ✅ | Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}/locations/{location-id}` or `projects/{project-number-or-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `languages` | Vec<String> | A list of supported language responses. This list contains an entry for each language the Translation API supports. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.translate_api.Project {
    parent = "value"  # Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}/locations/{location-id}` or `projects/{project-number-or-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`.
}

# Access project outputs
project_id = project.id
project_languages = project.languages
```

---


### Adaptive_mt_sentence

Lists all AdaptiveMtSentences under a given file/dataset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `adaptive_mt_sentences` | Vec<String> | Output only. The list of AdaptiveMtSentences. |
| `next_page_token` | String | Optional.  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adaptive_mt_sentence outputs
adaptive_mt_sentence_id = adaptive_mt_sentence.id
adaptive_mt_sentence_adaptive_mt_sentences = adaptive_mt_sentence.adaptive_mt_sentences
adaptive_mt_sentence_next_page_token = adaptive_mt_sentence.next_page_token
```

---


### Dataset

Creates a Dataset.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `example_count` | i64 |  | Output only. The number of examples in the dataset. |
| `update_time` | String |  | Output only. Timestamp when this dataset was last updated. |
| `display_name` | String |  | The name of the dataset to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |
| `create_time` | String |  | Output only. Timestamp when this dataset was created. |
| `target_language_code` | String |  | The BCP-47 language code of the target language. |
| `name` | String |  | The resource name of the dataset, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}` |
| `train_example_count` | i64 |  | Output only. Number of training examples (sentence pairs). |
| `validate_example_count` | i64 |  | Output only. Number of validation examples (sentence pairs). |
| `source_language_code` | String |  | The BCP-47 language code of the source language. |
| `test_example_count` | i64 |  | Output only. Number of test examples (sentence pairs). |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `example_count` | i64 | Output only. The number of examples in the dataset. |
| `update_time` | String | Output only. Timestamp when this dataset was last updated. |
| `display_name` | String | The name of the dataset to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |
| `create_time` | String | Output only. Timestamp when this dataset was created. |
| `target_language_code` | String | The BCP-47 language code of the target language. |
| `name` | String | The resource name of the dataset, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}` |
| `train_example_count` | i64 | Output only. Number of training examples (sentence pairs). |
| `validate_example_count` | i64 | Output only. Number of validation examples (sentence pairs). |
| `source_language_code` | String | The BCP-47 language code of the source language. |
| `test_example_count` | i64 | Output only. Number of test examples (sentence pairs). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.translate_api.Dataset {
    parent = "value"  # Required. The project name.
}

# Access dataset outputs
dataset_id = dataset.id
dataset_example_count = dataset.example_count
dataset_update_time = dataset.update_time
dataset_display_name = dataset.display_name
dataset_create_time = dataset.create_time
dataset_target_language_code = dataset.target_language_code
dataset_name = dataset.name
dataset_train_example_count = dataset.train_example_count
dataset_validate_example_count = dataset.validate_example_count
dataset_source_language_code = dataset.source_language_code
dataset_test_example_count = dataset.test_example_count
```

---


### Example

Lists sentence pairs in the dataset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `examples` | Vec<String> | The sentence pairs. |
| `next_page_token` | String | A token to retrieve next page of results. Pass this token to the page_token field in the ListExamplesRequest to obtain the corresponding page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access example outputs
example_id = example.id
example_examples = example.examples
example_next_page_token = example.next_page_token
```

---


### Glossary_entrie

Creates a glossary entry.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the entry. Format: `projects/*/locations/*/glossaries/*/glossaryEntries/*` |
| `description` | String |  | Describes the glossary entry. |
| `terms_pair` | String |  | Used for an unidirectional glossary. |
| `terms_set` | String |  | Used for an equivalent term sets glossary. |
| `parent` | String | ✅ | Required. The resource name of the glossary to create the entry under. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the entry. Format: `projects/*/locations/*/glossaries/*/glossaryEntries/*` |
| `description` | String | Describes the glossary entry. |
| `terms_pair` | String | Used for an unidirectional glossary. |
| `terms_set` | String | Used for an equivalent term sets glossary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create glossary_entrie
glossary_entrie = provider.translate_api.Glossary_entrie {
    parent = "value"  # Required. The resource name of the glossary to create the entry under.
}

# Access glossary_entrie outputs
glossary_entrie_id = glossary_entrie.id
glossary_entrie_name = glossary_entrie.name
glossary_entrie_description = glossary_entrie.description
glossary_entrie_terms_pair = glossary_entrie.terms_pair
glossary_entrie_terms_set = glossary_entrie.terms_set
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.translate_api.Operation {
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Adaptive_mt_dataset

Creates an Adaptive MT dataset.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The resource name of the dataset, in form of `projects/{project-number-or-id}/locations/{location_id}/adaptiveMtDatasets/{dataset_id}` |
| `update_time` | String |  | Output only. Timestamp when this dataset was last updated. |
| `source_language_code` | String |  | The BCP-47 language code of the source language. |
| `target_language_code` | String |  | The BCP-47 language code of the target language. |
| `create_time` | String |  | Output only. Timestamp when this dataset was created. |
| `example_count` | i64 |  | The number of examples in the dataset. |
| `display_name` | String |  | The name of the dataset to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |
| `parent` | String | ✅ | Required. Name of the parent project. In form of `projects/{project-number-or-id}/locations/{location-id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The resource name of the dataset, in form of `projects/{project-number-or-id}/locations/{location_id}/adaptiveMtDatasets/{dataset_id}` |
| `update_time` | String | Output only. Timestamp when this dataset was last updated. |
| `source_language_code` | String | The BCP-47 language code of the source language. |
| `target_language_code` | String | The BCP-47 language code of the target language. |
| `create_time` | String | Output only. Timestamp when this dataset was created. |
| `example_count` | i64 | The number of examples in the dataset. |
| `display_name` | String | The name of the dataset to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create adaptive_mt_dataset
adaptive_mt_dataset = provider.translate_api.Adaptive_mt_dataset {
    parent = "value"  # Required. Name of the parent project. In form of `projects/{project-number-or-id}/locations/{location-id}`
}

# Access adaptive_mt_dataset outputs
adaptive_mt_dataset_id = adaptive_mt_dataset.id
adaptive_mt_dataset_name = adaptive_mt_dataset.name
adaptive_mt_dataset_update_time = adaptive_mt_dataset.update_time
adaptive_mt_dataset_source_language_code = adaptive_mt_dataset.source_language_code
adaptive_mt_dataset_target_language_code = adaptive_mt_dataset.target_language_code
adaptive_mt_dataset_create_time = adaptive_mt_dataset.create_time
adaptive_mt_dataset_example_count = adaptive_mt_dataset.example_count
adaptive_mt_dataset_display_name = adaptive_mt_dataset.display_name
```

---


### Glossarie

Creates a glossary and returns the long-running operation. Returns NOT_FOUND, if the project doesn't exist.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. When the glossary creation was finished. |
| `input_config` | String |  | Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints. |
| `language_codes_set` | String |  | Used with equivalent term set glossaries. |
| `language_pair` | String |  | Used with unidirectional glossaries. |
| `display_name` | String |  | Optional. The display name of the glossary. |
| `name` | String |  | Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`. |
| `entry_count` | i64 |  | Output only. The number of entries defined in the glossary. |
| `submit_time` | String |  | Output only. When CreateGlossary was called. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. When the glossary creation was finished. |
| `input_config` | String | Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints. |
| `language_codes_set` | String | Used with equivalent term set glossaries. |
| `language_pair` | String | Used with unidirectional glossaries. |
| `display_name` | String | Optional. The display name of the glossary. |
| `name` | String | Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`. |
| `entry_count` | i64 | Output only. The number of entries defined in the glossary. |
| `submit_time` | String | Output only. When CreateGlossary was called. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create glossarie
glossarie = provider.translate_api.Glossarie {
    parent = "value"  # Required. The project name.
}

# Access glossarie outputs
glossarie_id = glossarie.id
glossarie_end_time = glossarie.end_time
glossarie_input_config = glossarie.input_config
glossarie_language_codes_set = glossarie.language_codes_set
glossarie_language_pair = glossarie.language_pair
glossarie_display_name = glossarie.display_name
glossarie_name = glossarie.name
glossarie_entry_count = glossarie.entry_count
glossarie_submit_time = glossarie.submit_time
```

---


### Location

Translates input text and returns translated text.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mime_type` | String |  | Optional. The format of the source text, for example, "text/html", "text/plain". If left blank, the MIME type defaults to "text/html". |
| `source_language_code` | String |  | Optional. The ISO-639 language code of the input text if known, for example, "en-US" or "sr-Latn". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response. |
| `glossary_config` | String |  | Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned. |
| `target_language_code` | String |  | Required. The ISO-639 language code to use for translation of the input text, set to one of the language codes listed in Language Support. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/advanced/labels for more information. |
| `model` | String |  | Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, - Translation LLM models: `projects/{project-number-or-id}/locations/{location-id}/models/general/translation-llm`, For global (non-regionalized) requests, use `location-id` `global`. For example, `projects/{project-number-or-id}/locations/global/models/general/nmt`. If not provided, the default Google model (NMT) will be used |
| `contents` | Vec<String> |  | Required. The content of the input in string format. We recommend the total content be less than 30,000 codepoints. The max length of this field is 1024. Use BatchTranslateText for larger text. |
| `transliteration_config` | String |  | Optional. Transliteration to be applied. |
| `parent` | String | ✅ | Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location = provider.translate_api.Location {
    parent = "value"  # Required. Project or location to make a call. Must refer to a caller's project. Format: `projects/{project-number-or-id}` or `projects/{project-number-or-id}/locations/{location-id}`. For global calls, use `projects/{project-number-or-id}/locations/global` or `projects/{project-number-or-id}`. Non-global location is required for requests using AutoML models or custom glossaries. Models and glossaries must be within the same region (have same location-id), otherwise an INVALID_ARGUMENT (400) error is returned.
}

# Access location outputs
location_id = location.id
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Adaptive_mt_file

Gets and AdaptiveMtFile

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The file's display name. |
| `update_time` | String | Output only. Timestamp when this file was last updated. |
| `name` | String | Required. The resource name of the file, in form of `projects/{project-number-or-id}/locations/{location_id}/adaptiveMtDatasets/{dataset}/adaptiveMtFiles/{file}` |
| `create_time` | String | Output only. Timestamp when this file was created. |
| `entry_count` | i64 | The number of entries that the file contains. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adaptive_mt_file outputs
adaptive_mt_file_id = adaptive_mt_file.id
adaptive_mt_file_display_name = adaptive_mt_file.display_name
adaptive_mt_file_update_time = adaptive_mt_file.update_time
adaptive_mt_file_name = adaptive_mt_file.name
adaptive_mt_file_create_time = adaptive_mt_file.create_time
adaptive_mt_file_entry_count = adaptive_mt_file.entry_count
```

---


### Model

Creates a Model.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_language_code` | String |  | Output only. The BCP-47 language code of the target language. |
| `name` | String |  | The resource name of the model, in form of `projects/{project-number-or-id}/locations/{location_id}/models/{model_id}` |
| `source_language_code` | String |  | Output only. The BCP-47 language code of the source language. |
| `train_example_count` | i64 |  | Output only. Number of examples (sentence pairs) used to train the model. |
| `update_time` | String |  | Output only. Timestamp when this model was last updated. |
| `display_name` | String |  | The name of the model to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |
| `create_time` | String |  | Output only. Timestamp when the model resource was created, which is also when the training started. |
| `validate_example_count` | i64 |  | Output only. Number of examples (sentence pairs) used to validate the model. |
| `test_example_count` | i64 |  | Output only. Number of examples (sentence pairs) used to test the model. |
| `dataset` | String |  | The dataset from which the model is trained, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}` |
| `parent` | String | ✅ | Required. The project name, in form of `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_language_code` | String | Output only. The BCP-47 language code of the target language. |
| `name` | String | The resource name of the model, in form of `projects/{project-number-or-id}/locations/{location_id}/models/{model_id}` |
| `source_language_code` | String | Output only. The BCP-47 language code of the source language. |
| `train_example_count` | i64 | Output only. Number of examples (sentence pairs) used to train the model. |
| `update_time` | String | Output only. Timestamp when this model was last updated. |
| `display_name` | String | The name of the model to show in the interface. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores (_), and ASCII digits 0-9. |
| `create_time` | String | Output only. Timestamp when the model resource was created, which is also when the training started. |
| `validate_example_count` | i64 | Output only. Number of examples (sentence pairs) used to validate the model. |
| `test_example_count` | i64 | Output only. Number of examples (sentence pairs) used to test the model. |
| `dataset` | String | The dataset from which the model is trained, in form of `projects/{project-number-or-id}/locations/{location_id}/datasets/{dataset_id}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.translate_api.Model {
    parent = "value"  # Required. The project name, in form of `projects/{project}/locations/{location}`
}

# Access model outputs
model_id = model.id
model_target_language_code = model.target_language_code
model_name = model.name
model_source_language_code = model.source_language_code
model_train_example_count = model.train_example_count
model_update_time = model.update_time
model_display_name = model.display_name
model_create_time = model.create_time
model_validate_example_count = model.validate_example_count
model_test_example_count = model.test_example_count
model_dataset = model.dataset
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple glossarie resources
glossarie_0 = provider.translate_api.Glossarie {
    parent = "value-0"
}
glossarie_1 = provider.translate_api.Glossarie {
    parent = "value-1"
}
glossarie_2 = provider.translate_api.Glossarie {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    glossarie = provider.translate_api.Glossarie {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Translate_api Documentation](https://cloud.google.com/translate_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
