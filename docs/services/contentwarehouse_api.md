# Contentwarehouse_api Service



**Resources**: 9

---

## Overview

The contentwarehouse_api service provides access to 9 resource types:

- [Rule_set](#rule_set) [CRUD]
- [Document](#document) [CRUD]
- [Document_schema](#document_schema) [CRUD]
- [Synonym_set](#synonym_set) [CRUD]
- [Operation](#operation) [R]
- [Location](#location) [CR]
- [Document_link](#document_link) [CD]
- [Reference_id](#reference_id) [RUD]
- [Project](#project) [C]

---

## Resources


### Rule_set

Creates a ruleset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name of the rule set. Managed internally. Format: projects/{project_number}/locations/{location}/ruleSet/{rule_set_id}. The name is ignored when creating a rule set. |
| `source` | String |  | Source of the rules i.e., customer name. |
| `description` | String |  | Short description of the rule-set. |
| `rules` | Vec<String> |  | List of rules given by the customer. |
| `parent` | String | ✅ | Required. The parent name. Format: projects/{project_number}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the rule set. Managed internally. Format: projects/{project_number}/locations/{location}/ruleSet/{rule_set_id}. The name is ignored when creating a rule set. |
| `source` | String | Source of the rules i.e., customer name. |
| `description` | String | Short description of the rule-set. |
| `rules` | Vec<String> | List of rules given by the customer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rule_set
rule_set = provider.contentwarehouse_api.Rule_set {
    parent = "value"  # Required. The parent name. Format: projects/{project_number}/locations/{location}.
}

# Access rule_set outputs
rule_set_id = rule_set.id
rule_set_name = rule_set.name
rule_set_source = rule_set.source
rule_set_description = rule_set.description
rule_set_rules = rule_set.rules
```

---


### Document

Creates a document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_ai_document_option` | String |  | Request Option for processing Cloud AI Document in Document Warehouse. This field offers limited support for mapping entities from Cloud AI Document to Warehouse Document. Please consult with product team before using this field and other available options. |
| `policy` | String |  | Default document policy during creation. This refers to an Identity and Access (IAM) policy, which specifies access controls for the Document. Conditions defined in the policy will be ignored. |
| `create_mask` | String |  | Field mask for creating Document fields. If mask path is empty, it means all fields are masked. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask. |
| `request_metadata` | String |  | The meta information collected about the end user, used to enforce access control for the service. |
| `document` | String |  | Required. The document to create. |
| `parent` | String | ✅ | Required. The parent name. Format: projects/{project_number}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `plain_text` | String | Other document format, such as PPTX, XLXS |
| `disposition_time` | String | Output only. If linked to a Collection with RetentionPolicy, the date when the document becomes mutable. |
| `inline_raw_document` | String | Raw document content. |
| `display_uri` | String | Uri to display the document, for example, in the UI. |
| `display_name` | String | Required. Display name of the document given by the user. This name will be displayed in the UI. Customer can populate this field with the name of the document. This differs from the 'title' field as 'title' is optional and stores the top heading in the document. |
| `text_extraction_enabled` | bool | If true, text extraction will be performed. |
| `raw_document_file_type` | String | This is used when DocAI was not used to load the document and parsing/ extracting is needed for the inline_raw_document. For example, if inline_raw_document is the byte representation of a PDF file, then this should be set to: RAW_DOCUMENT_FILE_TYPE_PDF. |
| `updater` | String | The user who lastly updates the document. |
| `creator` | String | The user who creates the document. |
| `name` | String | The resource name of the document. Format: projects/{project_number}/locations/{location}/documents/{document_id}. The name is ignored when creating a document. |
| `text_extraction_disabled` | bool | If true, text extraction will not be performed. |
| `create_time` | String | Output only. The time when the document is created. |
| `title` | String | Title that describes the document. This can be the top heading or text that describes the document. |
| `content_category` | String | Indicates the category (image, audio, video etc.) of the original content. |
| `legal_hold` | bool | Output only. Indicates if the document has a legal hold on it. |
| `reference_id` | String | The reference ID set by customers. Must be unique per project and location. |
| `raw_document_path` | String | Raw document file in Cloud Storage path. |
| `properties` | Vec<String> | List of values that are user supplied metadata. |
| `update_time` | String | Output only. The time when the document is last updated. |
| `cloud_ai_document` | String | Document AI format to save the structured content, including OCR. |
| `document_schema_name` | String | The Document schema name. Format: projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.contentwarehouse_api.Document {
    parent = "value"  # Required. The parent name. Format: projects/{project_number}/locations/{location}.
}

# Access document outputs
document_id = document.id
document_plain_text = document.plain_text
document_disposition_time = document.disposition_time
document_inline_raw_document = document.inline_raw_document
document_display_uri = document.display_uri
document_display_name = document.display_name
document_text_extraction_enabled = document.text_extraction_enabled
document_raw_document_file_type = document.raw_document_file_type
document_updater = document.updater
document_creator = document.creator
document_name = document.name
document_text_extraction_disabled = document.text_extraction_disabled
document_create_time = document.create_time
document_title = document.title
document_content_category = document.content_category
document_legal_hold = document.legal_hold
document_reference_id = document.reference_id
document_raw_document_path = document.raw_document_path
document_properties = document.properties
document_update_time = document.update_time
document_cloud_ai_document = document.cloud_ai_document
document_document_schema_name = document.document_schema_name
```

---


### Document_schema

Creates a document schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Schema description. |
| `name` | String |  | The resource name of the document schema. Format: projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}. The name is ignored when creating a document schema. |
| `property_definitions` | Vec<String> |  | Document details. |
| `document_is_folder` | bool |  | Document Type, true refers the document is a folder, otherwise it is a typical document. |
| `update_time` | String |  | Output only. The time when the document schema is last updated. |
| `create_time` | String |  | Output only. The time when the document schema is created. |
| `display_name` | String |  | Required. Name of the schema given by the user. Must be unique per project. |
| `parent` | String | ✅ | Required. The parent name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Schema description. |
| `name` | String | The resource name of the document schema. Format: projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}. The name is ignored when creating a document schema. |
| `property_definitions` | Vec<String> | Document details. |
| `document_is_folder` | bool | Document Type, true refers the document is a folder, otherwise it is a typical document. |
| `update_time` | String | Output only. The time when the document schema is last updated. |
| `create_time` | String | Output only. The time when the document schema is created. |
| `display_name` | String | Required. Name of the schema given by the user. Must be unique per project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document_schema
document_schema = provider.contentwarehouse_api.Document_schema {
    parent = "value"  # Required. The parent name.
}

# Access document_schema outputs
document_schema_id = document_schema.id
document_schema_description = document_schema.description
document_schema_name = document_schema.name
document_schema_property_definitions = document_schema.property_definitions
document_schema_document_is_folder = document_schema.document_is_folder
document_schema_update_time = document_schema.update_time
document_schema_create_time = document_schema.create_time
document_schema_display_name = document_schema.display_name
```

---


### Synonym_set

Creates a SynonymSet for a single context. Throws an ALREADY_EXISTS exception if a synonymset already exists for the context.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context` | String |  | This is a freeform field. Example contexts can be "sales," "engineering," "real estate," "accounting," etc. The context can be supplied during search requests. |
| `name` | String |  | The resource name of the SynonymSet This is mandatory for google.api.resource. Format: projects/{project_number}/locations/{location}/synonymSets/{context}. |
| `synonyms` | Vec<String> |  | List of Synonyms for the context. |
| `parent` | String | ✅ | Required. The parent name. Format: projects/{project_number}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context` | String | This is a freeform field. Example contexts can be "sales," "engineering," "real estate," "accounting," etc. The context can be supplied during search requests. |
| `name` | String | The resource name of the SynonymSet This is mandatory for google.api.resource. Format: projects/{project_number}/locations/{location}/synonymSets/{context}. |
| `synonyms` | Vec<String> | List of Synonyms for the context. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create synonym_set
synonym_set = provider.contentwarehouse_api.Synonym_set {
    parent = "value"  # Required. The parent name. Format: projects/{project_number}/locations/{location}.
}

# Access synonym_set outputs
synonym_set_id = synonym_set.id
synonym_set_context = synonym_set.context
synonym_set_name = synonym_set.name
synonym_set_synonyms = synonym_set.synonyms
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Location

Run a predefined pipeline.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_ingest_pipeline` | String |  | Cloud Storage ingestion pipeline. |
| `export_cdw_pipeline` | String |  | Export docuemnts from Document Warehouse to CDW for training purpose. |
| `gcs_ingest_with_doc_ai_processors_pipeline` | String |  | Use DocAI processors to process documents in Cloud Storage and ingest them to Document Warehouse. |
| `process_with_doc_ai_pipeline` | String |  | Use a DocAI processor to process documents in Document Warehouse, and re-ingest the updated results into Document Warehouse. |
| `request_metadata` | String |  | The meta information collected about the end user, used to enforce access control for the service. |
| `name` | String | ✅ | Required. The resource name which owns the resources of the pipeline. Format: projects/{project_number}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_creator_default_role` | String | The default role for the person who create a document. |
| `qa_enabled` | bool | If the qa is enabled on this project. |
| `location` | String | The location of the queried project. |
| `state` | String | State of the project. |
| `access_control_mode` | String | Access control mode. |
| `database_type` | String | Database type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.contentwarehouse_api.Location {
    name = "value"  # Required. The resource name which owns the resources of the pipeline. Format: projects/{project_number}/locations/{location}.
}

# Access location outputs
location_id = location.id
location_document_creator_default_role = location.document_creator_default_role
location_qa_enabled = location.qa_enabled
location_location = location.location
location_state = location.state
location_access_control_mode = location.access_control_mode
location_database_type = location.database_type
```

---


### Document_link

Create a link between a source document and a target document.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_metadata` | String |  | The meta information collected about the document creator, used to enforce access control for the service. |
| `document_link` | String |  | Required. Document links associated with the source documents (source_document_id). |
| `parent` | String | ✅ | Required. Parent of the document-link to be created. parent of document-link should be a document. Format: projects/{project_number}/locations/{location}/documents/{source_document_id}. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document_link
document_link = provider.contentwarehouse_api.Document_link {
    parent = "value"  # Required. Parent of the document-link to be created. parent of document-link should be a document. Format: projects/{project_number}/locations/{location}/documents/{source_document_id}.
}

```

---


### Reference_id

Gets a document. Returns NOT_FOUND if the document does not exist.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_options` | String |  | Options for the update operation. |
| `request_metadata` | String |  | The meta information collected about the end user, used to enforce access control for the service. |
| `cloud_ai_document_option` | String |  | Request Option for processing Cloud AI Document in Document Warehouse. This field offers limited support for mapping entities from Cloud AI Document to Warehouse Document. Please consult with product team before using this field and other available options. |
| `document` | String |  | Required. The document to update. |
| `name` | String | ✅ | Required. The name of the document to update. Format: projects/{project_number}/locations/{location}/documents/{document_id} or projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `plain_text` | String | Other document format, such as PPTX, XLXS |
| `disposition_time` | String | Output only. If linked to a Collection with RetentionPolicy, the date when the document becomes mutable. |
| `inline_raw_document` | String | Raw document content. |
| `display_uri` | String | Uri to display the document, for example, in the UI. |
| `display_name` | String | Required. Display name of the document given by the user. This name will be displayed in the UI. Customer can populate this field with the name of the document. This differs from the 'title' field as 'title' is optional and stores the top heading in the document. |
| `text_extraction_enabled` | bool | If true, text extraction will be performed. |
| `raw_document_file_type` | String | This is used when DocAI was not used to load the document and parsing/ extracting is needed for the inline_raw_document. For example, if inline_raw_document is the byte representation of a PDF file, then this should be set to: RAW_DOCUMENT_FILE_TYPE_PDF. |
| `updater` | String | The user who lastly updates the document. |
| `creator` | String | The user who creates the document. |
| `name` | String | The resource name of the document. Format: projects/{project_number}/locations/{location}/documents/{document_id}. The name is ignored when creating a document. |
| `text_extraction_disabled` | bool | If true, text extraction will not be performed. |
| `create_time` | String | Output only. The time when the document is created. |
| `title` | String | Title that describes the document. This can be the top heading or text that describes the document. |
| `content_category` | String | Indicates the category (image, audio, video etc.) of the original content. |
| `legal_hold` | bool | Output only. Indicates if the document has a legal hold on it. |
| `reference_id` | String | The reference ID set by customers. Must be unique per project and location. |
| `raw_document_path` | String | Raw document file in Cloud Storage path. |
| `properties` | Vec<String> | List of values that are user supplied metadata. |
| `update_time` | String | Output only. The time when the document is last updated. |
| `cloud_ai_document` | String | Document AI format to save the structured content, including OCR. |
| `document_schema_name` | String | The Document schema name. Format: projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access reference_id outputs
reference_id_id = reference_id.id
reference_id_plain_text = reference_id.plain_text
reference_id_disposition_time = reference_id.disposition_time
reference_id_inline_raw_document = reference_id.inline_raw_document
reference_id_display_uri = reference_id.display_uri
reference_id_display_name = reference_id.display_name
reference_id_text_extraction_enabled = reference_id.text_extraction_enabled
reference_id_raw_document_file_type = reference_id.raw_document_file_type
reference_id_updater = reference_id.updater
reference_id_creator = reference_id.creator
reference_id_name = reference_id.name
reference_id_text_extraction_disabled = reference_id.text_extraction_disabled
reference_id_create_time = reference_id.create_time
reference_id_title = reference_id.title
reference_id_content_category = reference_id.content_category
reference_id_legal_hold = reference_id.legal_hold
reference_id_reference_id = reference_id.reference_id
reference_id_raw_document_path = reference_id.raw_document_path
reference_id_properties = reference_id.properties
reference_id_update_time = reference_id.update_time
reference_id_cloud_ai_document = reference_id.cloud_ai_document
reference_id_document_schema_name = reference_id.document_schema_name
```

---


### Project

Sets the access control policy for a resource. Replaces any existing policy.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | Required. REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. This refers to an Identity and Access (IAM) policy, which specifies access controls for the Document. You can set ACL with condition for projects only. Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left of the operator is `DocumentSchemaId` or property name and the right of the operator is a number or a quoted string. You must escape backslash (\\) and quote (\") characters. Boolean expressions (AND/OR) are supported up to 3 levels of nesting (for example, "((A AND B AND C) OR D) AND E"), a maximum of 10 comparisons are allowed in the expression. The expression must be < 6000 bytes in length. Sample condition: `"DocumentSchemaId = \"some schema id\" OR SchemaId.floatPropertyName >= 10"` |
| `project_owner` | bool |  | For Set Project ACL only. Authorization check for end user will be ignored when project_owner=true. |
| `request_metadata` | String |  | The meta information collected about the end user, used to enforce access control for the service. |
| `resource` | String | ✅ | Required. REQUIRED: The resource for which the policy is being requested. Format for document: projects/{project_number}/locations/{location}/documents/{document_id}. Format for collection: projects/{project_number}/locations/{location}/collections/{collection_id}. Format for project: projects/{project_number}. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.contentwarehouse_api.Project {
    resource = "value"  # Required. REQUIRED: The resource for which the policy is being requested. Format for document: projects/{project_number}/locations/{location}/documents/{document_id}. Format for collection: projects/{project_number}/locations/{location}/collections/{collection_id}. Format for project: projects/{project_number}.
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

# Create multiple rule_set resources
rule_set_0 = provider.contentwarehouse_api.Rule_set {
    parent = "value-0"
}
rule_set_1 = provider.contentwarehouse_api.Rule_set {
    parent = "value-1"
}
rule_set_2 = provider.contentwarehouse_api.Rule_set {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rule_set = provider.contentwarehouse_api.Rule_set {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Contentwarehouse_api Documentation](https://cloud.google.com/contentwarehouse_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
