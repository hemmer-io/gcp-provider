# Documentai_api Service



**Resources**: 21

---

## Overview

The documentai_api service provides access to 21 resource types:

- [Operation](#operation) [CR]
- [Evaluation](#evaluation) [R]
- [Location](#location) [R]
- [Processor_type](#processor_type) [R]
- [Human_review_config](#human_review_config) [C]
- [Processor](#processor) [CRUD]
- [Schema_version](#schema_version) [CRUD]
- [Schema](#schema) [CRUD]
- [Processor_version](#processor_version) [CRD]
- [Dataset](#dataset) [CRU]
- [Processor](#processor) [CRD]
- [Processor_type](#processor_type) [R]
- [Human_review_config](#human_review_config) [C]
- [Location](#location) [R]
- [Processor_version](#processor_version) [CRD]
- [Schema_version](#schema_version) [CRUD]
- [Operation](#operation) [CRD]
- [Evaluation](#evaluation) [R]
- [Schema](#schema) [CRUD]
- [Operation](#operation) [R]
- [Document](#document) [C]

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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.documentai_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Evaluation

Retrieves a specific evaluation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version_name` | String | The KMS key version with which data is encrypted. |
| `all_entities_metrics` | String | Metrics for all the entities in aggregate. |
| `create_time` | String | The time that the evaluation was created. |
| `entity_metrics` | HashMap<String, String> | Metrics across confidence levels, for different entities. |
| `kms_key_name` | String | The KMS key name used for encryption. |
| `name` | String | The resource name of the evaluation. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}/evaluations/{evaluation}` |
| `document_counters` | String | Counters for the documents used in the evaluation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_kms_key_version_name = evaluation.kms_key_version_name
evaluation_all_entities_metrics = evaluation.all_entities_metrics
evaluation_create_time = evaluation.create_time
evaluation_entity_metrics = evaluation.entity_metrics
evaluation_kms_key_name = evaluation.kms_key_name
evaluation_name = evaluation.name
evaluation_document_counters = evaluation.document_counters
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_name = location.name
location_location_id = location.location_id
location_labels = location.labels
```

---


### Processor_type

Gets a processor type detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `allow_creation` | bool | Whether the processor type allows creation. If true, users can create a processor of this processor type. Otherwise, users need to request access. |
| `category` | String | The processor category, used by UI to group processor types. |
| `available_locations` | Vec<String> | The locations in which this processor is available. |
| `launch_stage` | String | Launch stage of the processor type |
| `type` | String | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. |
| `sample_document_uris` | Vec<String> | A set of Cloud Storage URIs of sample documents for this processor. |
| `name` | String | The resource name of the processor type. Format: `projects/{project}/processorTypes/{processor_type}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access processor_type outputs
processor_type_id = processor_type.id
processor_type_allow_creation = processor_type.allow_creation
processor_type_category = processor_type.category
processor_type_available_locations = processor_type.available_locations
processor_type_launch_stage = processor_type.launch_stage
processor_type_type = processor_type.type
processor_type_sample_document_uris = processor_type.sample_document_uris
processor_type_name = processor_type.name
```

---


### Human_review_config

Send a document for Human Review. The input document should be processed by the specified processor.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | String |  | The priority of the human review task. |
| `document_schema` | String |  | The document schema of the human review task. |
| `document` | String |  | The document that needs human review. |
| `inline_document` | String |  | An inline document proto. |
| `enable_schema_validation` | bool |  | Whether the validation should be performed on the ad-hoc review request. |
| `human_review_config` | String | ✅ | Required. The resource name of the HumanReviewConfig that the document will be reviewed with. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create human_review_config
human_review_config = provider.documentai_api.Human_review_config {
    human_review_config = "value"  # Required. The resource name of the HumanReviewConfig that the document will be reviewed with.
}

```

---


### Processor

Creates a processor from the ProcessorType provided. The processor will be at `ENABLED` state by default after its creation. Note that this method requires the `documentai.processors.create` permission on the project, which is highly privileged. A user or service account with this permission can create new processors that can interact with any gcs bucket in your project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The display name of the processor. |
| `name` | String |  | Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}` |
| `create_time` | String |  | Output only. The time the processor was created. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `process_endpoint` | String |  | Output only. Immutable. The http endpoint that can be called to invoke processing. |
| `state` | String |  | Output only. The state of the processor. |
| `active_schema_version` | String |  | Optional. SchemaVersion used by the Processor. It is the same as Processor's DatasetSchema.schema_version Format is `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version} |
| `default_processor_version` | String |  | The default processor version. |
| `processor_version_aliases` | Vec<String> |  | Output only. The processor version aliases. |
| `kms_key_name` | String |  | The [KMS key](https://cloud.google.com/security-key-management) used for encryption and decryption in CMEK scenarios. |
| `type` | String |  | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. To get a list of processor types, see FetchProcessorTypes. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The display name of the processor. |
| `name` | String | Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}` |
| `create_time` | String | Output only. The time the processor was created. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `process_endpoint` | String | Output only. Immutable. The http endpoint that can be called to invoke processing. |
| `state` | String | Output only. The state of the processor. |
| `active_schema_version` | String | Optional. SchemaVersion used by the Processor. It is the same as Processor's DatasetSchema.schema_version Format is `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version} |
| `default_processor_version` | String | The default processor version. |
| `processor_version_aliases` | Vec<String> | Output only. The processor version aliases. |
| `kms_key_name` | String | The [KMS key](https://cloud.google.com/security-key-management) used for encryption and decryption in CMEK scenarios. |
| `type` | String | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. To get a list of processor types, see FetchProcessorTypes. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create processor
processor = provider.documentai_api.Processor {
    parent = "value"  # Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}`
}

# Access processor outputs
processor_id = processor.id
processor_display_name = processor.display_name
processor_name = processor.name
processor_create_time = processor.create_time
processor_satisfies_pzi = processor.satisfies_pzi
processor_process_endpoint = processor.process_endpoint
processor_state = processor.state
processor_active_schema_version = processor.active_schema_version
processor_default_processor_version = processor.default_processor_version
processor_processor_version_aliases = processor.processor_version_aliases
processor_kms_key_name = processor.kms_key_name
processor_type = processor.type
processor_satisfies_pzs = processor.satisfies_pzs
```

---


### Schema_version

Creates a schema version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. The user-defined name of the SchemaVersion. |
| `labels` | HashMap<String, String> |  | Optional. The GCP labels for the SchemaVersion. |
| `create_time` | String |  | Output only. The time when the SchemaVersion was created. |
| `name` | String |  | Identifier. The resource name of the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version}` |
| `schema` | String |  | Required. The schema of the SchemaVersion. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. The user-defined name of the SchemaVersion. |
| `labels` | HashMap<String, String> | Optional. The GCP labels for the SchemaVersion. |
| `create_time` | String | Output only. The time when the SchemaVersion was created. |
| `name` | String | Identifier. The resource name of the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version}` |
| `schema` | String | Required. The schema of the SchemaVersion. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema_version
schema_version = provider.documentai_api.Schema_version {
    parent = "value"  # Required. The parent (project and location) under which to create the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}`
}

# Access schema_version outputs
schema_version_id = schema_version.id
schema_version_display_name = schema_version.display_name
schema_version_labels = schema_version.labels
schema_version_create_time = schema_version.create_time
schema_version_name = schema_version.name
schema_version_schema = schema_version.schema
```

---


### Schema

Creates a schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The GCP labels for the Schema. |
| `name` | String |  | Identifier. The resource name of the Schema. Format: `projects/{project}/locations/{location}/schemas/{schema}` |
| `display_name` | String |  | Optional. The user-defined name of the Schema. |
| `update_time` | String |  | Output only. The time when the Schema was last updated. |
| `create_time` | String |  | Output only. The time when the Schema was created. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the Schema. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. The GCP labels for the Schema. |
| `name` | String | Identifier. The resource name of the Schema. Format: `projects/{project}/locations/{location}/schemas/{schema}` |
| `display_name` | String | Optional. The user-defined name of the Schema. |
| `update_time` | String | Output only. The time when the Schema was last updated. |
| `create_time` | String | Output only. The time when the Schema was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.documentai_api.Schema {
    parent = "value"  # Required. The parent (project and location) under which to create the Schema. Format: `projects/{project}/locations/{location}`
}

# Access schema outputs
schema_id = schema.id
schema_labels = schema.labels
schema_name = schema.name
schema_display_name = schema.display_name
schema_update_time = schema.update_time
schema_create_time = schema.create_time
```

---


### Processor_version

LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints) and can only contain lowercase letters, numeric characters, underscores, and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `input_configs` | Vec<String> |  | The input config for each single document in the batch process. |
| `process_options` | String |  | Inference-time options for the process API |
| `document_output_config` | String |  | The output configuration for the BatchProcessDocuments method. |
| `input_documents` | String |  | The input documents for the BatchProcessDocuments method. |
| `output_config` | String |  | The overall output config for batch process. |
| `skip_human_review` | bool |  | Whether human review should be skipped for this request. Default to `false`. |
| `name` | String | ✅ | Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The display name of the processor version. |
| `gen_ai_model_info` | String | Output only. Information about Generative AI model-based processor versions. |
| `name` | String | Identifier. The resource name of the processor version. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}` |
| `model_type` | String | Output only. The model type of this processor version. |
| `latest_evaluation` | String | Output only. The most recently invoked evaluation for the processor version. |
| `google_managed` | bool | Output only. Denotes that this `ProcessorVersion` is managed by Google. |
| `document_schema` | String | Output only. The schema of the processor version. Describes the output. |
| `kms_key_version_name` | String | Output only. The KMS key version with which data is encrypted. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. The time the processor version was created. |
| `state` | String | Output only. The state of the processor version. |
| `deprecation_info` | String | Output only. If set, information about the eventual deprecation of this version. |
| `kms_key_name` | String | Output only. The KMS key name used for encryption. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create processor_version
processor_version = provider.documentai_api.Processor_version {
    name = "value"  # Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
}

# Access processor_version outputs
processor_version_id = processor_version.id
processor_version_display_name = processor_version.display_name
processor_version_gen_ai_model_info = processor_version.gen_ai_model_info
processor_version_name = processor_version.name
processor_version_model_type = processor_version.model_type
processor_version_latest_evaluation = processor_version.latest_evaluation
processor_version_google_managed = processor_version.google_managed
processor_version_document_schema = processor_version.document_schema
processor_version_kms_key_version_name = processor_version.kms_key_version_name
processor_version_satisfies_pzi = processor_version.satisfies_pzi
processor_version_satisfies_pzs = processor_version.satisfies_pzs
processor_version_create_time = processor_version.create_time
processor_version_state = processor_version.state
processor_version_deprecation_info = processor_version.deprecation_info
processor_version_kms_key_name = processor_version.kms_key_name
```

---


### Dataset

Deletes a set of documents.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_documents` | String |  | Required. Dataset documents input. If given `filter`, all documents satisfying the filter will be deleted. If given documentIds, a maximum of 50 documents can be deleted in a batch. The request will be rejected if more than 50 document_ids are provided. |
| `dataset` | String | ✅ | Required. The dataset resource name. Format: projects/{project}/locations/{location}/processors/{processor}/dataset |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `name` | String | Dataset schema resource name. Format: `projects/{project}/locations/{location}/processors/{processor}/dataset/datasetSchema` |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `document_schema` | String | Optional. Schema of the dataset. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.documentai_api.Dataset {
    dataset = "value"  # Required. The dataset resource name. Format: projects/{project}/locations/{location}/processors/{processor}/dataset
}

# Access dataset outputs
dataset_id = dataset.id
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_name = dataset.name
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_document_schema = dataset.document_schema
```

---


### Processor

Creates a processor from the ProcessorType provided. The processor will be at `ENABLED` state by default after its creation. Note that this method requires the `documentai.processors.create` permission on the project, which is highly privileged. A user or service account with this permission can create new processors that can interact with any gcs bucket in your project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. The time the processor was created. |
| `name` | String |  | Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}` |
| `processor_version_aliases` | Vec<String> |  | Output only. The processor version aliases. |
| `state` | String |  | Output only. The state of the processor. |
| `default_processor_version` | String |  | The default processor version. |
| `process_endpoint` | String |  | Output only. Immutable. The http endpoint that can be called to invoke processing. |
| `active_schema_version` | String |  | Optional. SchemaVersion used by the Processor. It is the same as Processor's DatasetSchema.schema_version Format is `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version} |
| `display_name` | String |  | The display name of the processor. |
| `type` | String |  | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. To get a list of processor types, see FetchProcessorTypes. |
| `kms_key_name` | String |  | The [KMS key](https://cloud.google.com/security-key-management) used for encryption and decryption in CMEK scenarios. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. The time the processor was created. |
| `name` | String | Output only. Immutable. The resource name of the processor. Format: `projects/{project}/locations/{location}/processors/{processor}` |
| `processor_version_aliases` | Vec<String> | Output only. The processor version aliases. |
| `state` | String | Output only. The state of the processor. |
| `default_processor_version` | String | The default processor version. |
| `process_endpoint` | String | Output only. Immutable. The http endpoint that can be called to invoke processing. |
| `active_schema_version` | String | Optional. SchemaVersion used by the Processor. It is the same as Processor's DatasetSchema.schema_version Format is `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version} |
| `display_name` | String | The display name of the processor. |
| `type` | String | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. To get a list of processor types, see FetchProcessorTypes. |
| `kms_key_name` | String | The [KMS key](https://cloud.google.com/security-key-management) used for encryption and decryption in CMEK scenarios. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create processor
processor = provider.documentai_api.Processor {
    parent = "value"  # Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}`
}

# Access processor outputs
processor_id = processor.id
processor_satisfies_pzs = processor.satisfies_pzs
processor_create_time = processor.create_time
processor_name = processor.name
processor_processor_version_aliases = processor.processor_version_aliases
processor_state = processor.state
processor_default_processor_version = processor.default_processor_version
processor_process_endpoint = processor.process_endpoint
processor_active_schema_version = processor.active_schema_version
processor_display_name = processor.display_name
processor_type = processor.type
processor_kms_key_name = processor.kms_key_name
processor_satisfies_pzi = processor.satisfies_pzi
```

---


### Processor_type

Gets a processor type detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the processor type. Format: `projects/{project}/processorTypes/{processor_type}` |
| `type` | String | The processor type, such as: `OCR_PROCESSOR`, `INVOICE_PROCESSOR`. |
| `sample_document_uris` | Vec<String> | A set of Cloud Storage URIs of sample documents for this processor. |
| `category` | String | The processor category, used by UI to group processor types. |
| `allow_creation` | bool | Whether the processor type allows creation. If true, users can create a processor of this processor type. Otherwise, users need to request access. |
| `available_locations` | Vec<String> | The locations in which this processor is available. |
| `launch_stage` | String | Launch stage of the processor type |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access processor_type outputs
processor_type_id = processor_type.id
processor_type_name = processor_type.name
processor_type_type = processor_type.type
processor_type_sample_document_uris = processor_type.sample_document_uris
processor_type_category = processor_type.category
processor_type_allow_creation = processor_type.allow_creation
processor_type_available_locations = processor_type.available_locations
processor_type_launch_stage = processor_type.launch_stage
```

---


### Human_review_config

Send a document for Human Review. The input document should be processed by the specified processor.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | String |  | The priority of the human review task. |
| `inline_document` | String |  | An inline document proto. |
| `enable_schema_validation` | bool |  | Whether the validation should be performed on the ad-hoc review request. |
| `document_schema` | String |  | The document schema of the human review task. |
| `human_review_config` | String | ✅ | Required. The resource name of the HumanReviewConfig that the document will be reviewed with. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create human_review_config
human_review_config = provider.documentai_api.Human_review_config {
    human_review_config = "value"  # Required. The resource name of the HumanReviewConfig that the document will be reviewed with.
}

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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
```

---


### Processor_version

Evaluates a ProcessorVersion against annotated documents, producing an Evaluation.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_documents` | String |  | Optional. The documents used in the evaluation. If unspecified, use the processor's dataset as evaluation input. |
| `processor_version` | String | ✅ | Required. The resource name of the ProcessorVersion to evaluate. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gen_ai_model_info` | String | Output only. Information about Generative AI model-based processor versions. |
| `document_schema` | String | Output only. The schema of the processor version. Describes the output. |
| `latest_evaluation` | String | Output only. The most recently invoked evaluation for the processor version. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `deprecation_info` | String | Output only. If set, information about the eventual deprecation of this version. |
| `name` | String | Identifier. The resource name of the processor version. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}` |
| `create_time` | String | Output only. The time the processor version was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `google_managed` | bool | Output only. Denotes that this `ProcessorVersion` is managed by Google. |
| `kms_key_name` | String | Output only. The KMS key name used for encryption. |
| `kms_key_version_name` | String | Output only. The KMS key version with which data is encrypted. |
| `state` | String | Output only. The state of the processor version. |
| `display_name` | String | The display name of the processor version. |
| `model_type` | String | Output only. The model type of this processor version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create processor_version
processor_version = provider.documentai_api.Processor_version {
    processor_version = "value"  # Required. The resource name of the ProcessorVersion to evaluate. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
}

# Access processor_version outputs
processor_version_id = processor_version.id
processor_version_gen_ai_model_info = processor_version.gen_ai_model_info
processor_version_document_schema = processor_version.document_schema
processor_version_latest_evaluation = processor_version.latest_evaluation
processor_version_satisfies_pzi = processor_version.satisfies_pzi
processor_version_deprecation_info = processor_version.deprecation_info
processor_version_name = processor_version.name
processor_version_create_time = processor_version.create_time
processor_version_satisfies_pzs = processor_version.satisfies_pzs
processor_version_google_managed = processor_version.google_managed
processor_version_kms_key_name = processor_version.kms_key_name
processor_version_kms_key_version_name = processor_version.kms_key_version_name
processor_version_state = processor_version.state
processor_version_display_name = processor_version.display_name
processor_version_model_type = processor_version.model_type
```

---


### Schema_version

Creates a schema version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the SchemaVersion was created. |
| `name` | String |  | Identifier. The resource name of the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version}` |
| `display_name` | String |  | Optional. The user-defined name of the SchemaVersion. |
| `schema` | String |  | Required. The schema of the SchemaVersion. |
| `labels` | HashMap<String, String> |  | Optional. The GCP labels for the SchemaVersion. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the SchemaVersion was created. |
| `name` | String | Identifier. The resource name of the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}/schemaVersions/{schema_version}` |
| `display_name` | String | Optional. The user-defined name of the SchemaVersion. |
| `schema` | String | Required. The schema of the SchemaVersion. |
| `labels` | HashMap<String, String> | Optional. The GCP labels for the SchemaVersion. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema_version
schema_version = provider.documentai_api.Schema_version {
    parent = "value"  # Required. The parent (project and location) under which to create the SchemaVersion. Format: `projects/{project}/locations/{location}/schemas/{schema}`
}

# Access schema_version outputs
schema_version_id = schema_version.id
schema_version_create_time = schema_version.create_time
schema_version_name = schema_version.name
schema_version_display_name = schema_version.display_name
schema_version_schema = schema_version.schema
schema_version_labels = schema_version.labels
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.documentai_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Evaluation

Retrieves a specific evaluation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `all_entities_metrics` | String | Metrics for all the entities in aggregate. |
| `entity_metrics` | HashMap<String, String> | Metrics across confidence levels, for different entities. |
| `create_time` | String | The time that the evaluation was created. |
| `kms_key_version_name` | String | The KMS key version with which data is encrypted. |
| `name` | String | The resource name of the evaluation. Format: `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}/evaluations/{evaluation}` |
| `kms_key_name` | String | The KMS key name used for encryption. |
| `document_counters` | String | Counters for the documents used in the evaluation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_all_entities_metrics = evaluation.all_entities_metrics
evaluation_entity_metrics = evaluation.entity_metrics
evaluation_create_time = evaluation.create_time
evaluation_kms_key_version_name = evaluation.kms_key_version_name
evaluation_name = evaluation.name
evaluation_kms_key_name = evaluation.kms_key_name
evaluation_document_counters = evaluation.document_counters
```

---


### Schema

Creates a schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. The user-defined name of the Schema. |
| `name` | String |  | Identifier. The resource name of the Schema. Format: `projects/{project}/locations/{location}/schemas/{schema}` |
| `update_time` | String |  | Output only. The time when the Schema was last updated. |
| `create_time` | String |  | Output only. The time when the Schema was created. |
| `labels` | HashMap<String, String> |  | Optional. The GCP labels for the Schema. |
| `parent` | String | ✅ | Required. The parent (project and location) under which to create the Schema. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. The user-defined name of the Schema. |
| `name` | String | Identifier. The resource name of the Schema. Format: `projects/{project}/locations/{location}/schemas/{schema}` |
| `update_time` | String | Output only. The time when the Schema was last updated. |
| `create_time` | String | Output only. The time when the Schema was created. |
| `labels` | HashMap<String, String> | Optional. The GCP labels for the Schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.documentai_api.Schema {
    parent = "value"  # Required. The parent (project and location) under which to create the Schema. Format: `projects/{project}/locations/{location}`
}

# Access schema outputs
schema_id = schema.id
schema_display_name = schema.display_name
schema_name = schema.name
schema_update_time = schema.update_time
schema_create_time = schema.create_time
schema_labels = schema.labels
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Document

Processes a single document.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ocr_params` | String |  | Controls OCR behavior. If not specified, the system will decide reasonable defaults. |
| `output_config` | String |  | The desired output location. This field is only needed in BatchProcessDocumentsRequest. |
| `input_config` | String |  | Required. Information about the input file. |
| `parent` | String |  | Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method. |
| `table_extraction_params` | String |  | Controls table extraction behavior. If not specified, the system will decide reasonable defaults. |
| `document_type` | String |  | Specifies a known document type for deeper structure detection. Valid values are currently "general" and "invoice". If not provided, "general"\ is used as default. If any other value is given, the request is rejected. |
| `automl_params` | String |  | Controls AutoML model prediction behavior. AutoMlParams cannot be used together with other Params. |
| `entity_extraction_params` | String |  | Controls entity extraction behavior. If not specified, the system will decide reasonable defaults. |
| `form_extraction_params` | String |  | Controls form extraction behavior. If not specified, the system will decide reasonable defaults. |
| `parent` | String | ✅ | Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.documentai_api.Document {
    parent = "value"  # Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
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

# Create multiple operation resources
operation_0 = provider.documentai_api.Operation {
    name = "value-0"
}
operation_1 = provider.documentai_api.Operation {
    name = "value-1"
}
operation_2 = provider.documentai_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.documentai_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Documentai_api Documentation](https://cloud.google.com/documentai_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
