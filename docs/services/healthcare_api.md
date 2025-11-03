# Healthcare_api Service



**Resources**: 60

---

## Overview

The healthcare_api service provides access to 60 resource types:

- [Consent](#consent) [CRUD]
- [Dicom_store](#dicom_store) [CRUD]
- [Message](#message) [CRUD]
- [Nlp](#nlp) [C]
- [Fhir](#fhir) [CRUD]
- [User_data_mapping](#user_data_mapping) [CRUD]
- [Instance](#instance) [RD]
- [Fhir_store](#fhir_store) [CRUD]
- [Attribute_definition](#attribute_definition) [CRUD]
- [Operation](#operation) [CRD]
- [Bulkdata](#bulkdata) [R]
- [Data_mapper_workspace](#data_mapper_workspace) [CR]
- [Studie](#studie) [CRD]
- [Location](#location) [R]
- [Serie](#serie) [RD]
- [Consent_artifact](#consent_artifact) [CRD]
- [Hl7_v2_store](#hl7_v2_store) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Frame](#frame) [R]
- [Consent_store](#consent_store) [CRUD]
- [Operation](#operation) [R]
- [Hl7_v2_store](#hl7_v2_store) [CR]
- [Dataset](#dataset) [CR]
- [Location](#location) [R]
- [Dicom_store](#dicom_store) [CR]
- [Annotation](#annotation) [CRUD]
- [Operation](#operation) [R]
- [Hl7_v2_store](#hl7_v2_store) [CRUD]
- [Serie](#serie) [RD]
- [Dataset](#dataset) [CRUD]
- [Dicom_store](#dicom_store) [CRUD]
- [Fhir_store](#fhir_store) [CRUD]
- [Fhir](#fhir) [CRUD]
- [Location](#location) [R]
- [Studie](#studie) [CRD]
- [Instance](#instance) [RD]
- [Annotation_store](#annotation_store) [CRUD]
- [Dicom_web](#dicom_web) [CR]
- [Message](#message) [CRUD]
- [Frame](#frame) [R]
- [Attribute_definition](#attribute_definition) [CRUD]
- [Studie](#studie) [CRUD]
- [User_data_mapping](#user_data_mapping) [CRUD]
- [Consent_artifact](#consent_artifact) [CRD]
- [Message](#message) [CRUD]
- [Serie](#serie) [RUD]
- [Instance](#instance) [RUD]
- [Frame](#frame) [R]
- [Consent_store](#consent_store) [CRUD]
- [Location](#location) [R]
- [Data_mapper_workspace](#data_mapper_workspace) [CR]
- [Fhir_store](#fhir_store) [CRUD]
- [Hl7_v2_store](#hl7_v2_store) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Consent](#consent) [CRUD]
- [Dicom_store](#dicom_store) [CRUD]
- [Fhir](#fhir) [CRUD]
- [Bulkdata](#bulkdata) [R]
- [Nlp](#nlp) [C]
- [Operation](#operation) [CRD]

---

## Resources


### Consent

Creates a new Consent in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation. |
| `policies` | Vec<String> |  | Optional. Represents a user's consent in terms of the resources that can be accessed and under what conditions. |
| `consent_artifact` | String |  | Required. The resource name of the Consent artifact that contains proof of the end user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. |
| `revision_id` | String |  | Output only. The revision ID of the Consent. The format is an 8-character hexadecimal string. Refer to a specific revision of a Consent by appending `@{revision_id}` to the Consent's resource name. |
| `expire_time` | String |  | Timestamp in UTC of when this Consent is considered expired. |
| `revision_create_time` | String |  | Output only. The timestamp that the revision was created. |
| `ttl` | String |  | Input only. The time to live for this Consent from when it is created. |
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `metadata` | HashMap<String, String> |  | Optional. User-supplied key-value pairs used to organize Consent resources. Metadata keys must: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - begin with a letter - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes Metadata values must be: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes No more than 64 metadata entries can be associated with a given consent. |
| `state` | String |  | Required. Indicates the current state of this Consent. |
| `parent` | String | ✅ | Required. Name of the consent store. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation. |
| `policies` | Vec<String> | Optional. Represents a user's consent in terms of the resources that can be accessed and under what conditions. |
| `consent_artifact` | String | Required. The resource name of the Consent artifact that contains proof of the end user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. |
| `revision_id` | String | Output only. The revision ID of the Consent. The format is an 8-character hexadecimal string. Refer to a specific revision of a Consent by appending `@{revision_id}` to the Consent's resource name. |
| `expire_time` | String | Timestamp in UTC of when this Consent is considered expired. |
| `revision_create_time` | String | Output only. The timestamp that the revision was created. |
| `ttl` | String | Input only. The time to live for this Consent from when it is created. |
| `user_id` | String | Required. User's UUID provided by the client. |
| `metadata` | HashMap<String, String> | Optional. User-supplied key-value pairs used to organize Consent resources. Metadata keys must: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - begin with a letter - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes Metadata values must be: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes No more than 64 metadata entries can be associated with a given consent. |
| `state` | String | Required. Indicates the current state of this Consent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent
consent = provider.healthcare_api.Consent {
    parent = "value"  # Required. Name of the consent store.
}

# Access consent outputs
consent_id = consent.id
consent_name = consent.name
consent_policies = consent.policies
consent_consent_artifact = consent.consent_artifact
consent_revision_id = consent.revision_id
consent_expire_time = consent.expire_time
consent_revision_create_time = consent.revision_create_time
consent_ttl = consent.ttl
consent_user_id = consent.user_id
consent_metadata = consent.metadata
consent_state = consent.state
```

---


### Dicom_store

Creates a new DICOM store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_configs` | Vec<String> |  | Optional. Specifies where and whether to send notifications upon changes to a DICOM store. |
| `notification_config` | String |  | Optional. Notification destination for new DICOM instances. Supplied by the client. |
| `stream_configs` | Vec<String> |  | Optional. A list of streaming configs used to configure the destination of streaming exports for every DICOM instance insertion in this DICOM store. After a new config is added to `stream_configs`, DICOM instance insertions are streamed to the new destination. When a config is removed from `stream_configs`, the server stops streaming to that destination. Each config must contain a unique destination. |
| `name` | String |  | Identifier. Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `parent` | String | ✅ | Required. The name of the dataset this DICOM store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_configs` | Vec<String> | Optional. Specifies where and whether to send notifications upon changes to a DICOM store. |
| `notification_config` | String | Optional. Notification destination for new DICOM instances. Supplied by the client. |
| `stream_configs` | Vec<String> | Optional. A list of streaming configs used to configure the destination of streaming exports for every DICOM instance insertion in this DICOM store. After a new config is added to `stream_configs`, DICOM instance insertions are streamed to the new destination. When a config is removed from `stream_configs`, the server stops streaming to that destination. Each config must contain a unique destination. |
| `name` | String | Identifier. Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dicom_store
dicom_store = provider.healthcare_api.Dicom_store {
    parent = "value"  # Required. The name of the dataset this DICOM store belongs to.
}

# Access dicom_store outputs
dicom_store_id = dicom_store.id
dicom_store_notification_configs = dicom_store.notification_configs
dicom_store_notification_config = dicom_store.notification_config
dicom_store_stream_configs = dicom_store.stream_configs
dicom_store_name = dicom_store.name
dicom_store_labels = dicom_store.labels
```

---


### Message

Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Pub/Sub topic configured in Hl7V2Store.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Pub/Sub topic, the adapter transmits the message when a notification is received.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message` | String |  | Required. HL7v2 message. |
| `parent` | String | ✅ | Required. The name of the HL7v2 store this message belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message_type` | String | Output only. The message type for this message. MSH-9.1. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `parsed_data` | String | Output only. The parsed version of the raw message data. |
| `schematized_data` | String | Output only. The parsed version of the raw message data schematized according to this store's schemas and type definitions. |
| `name` | String | Output only. Resource name of the Message, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`. |
| `create_time` | String | Output only. The datetime when the message was created. Set by the server. |
| `send_time` | String | Output only. The datetime the sending application sent this message. MSH-7. |
| `send_facility` | String | Output only. The hospital that this message came from. MSH-4. |
| `data` | String | Required. Raw message bytes. |
| `patient_ids` | Vec<String> | Output only. All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this message. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.healthcare_api.Message {
    parent = "value"  # Required. The name of the HL7v2 store this message belongs to.
}

# Access message outputs
message_id = message.id
message_message_type = message.message_type
message_labels = message.labels
message_parsed_data = message.parsed_data
message_schematized_data = message.schematized_data
message_name = message.name
message_create_time = message.create_time
message_send_time = message.send_time
message_send_facility = message.send_facility
message_data = message.data
message_patient_ids = message.patient_ids
```

---


### Nlp

Analyze heathcare entity in a document. Its response includes the recognized entity mentions and the relationships between them. AnalyzeEntities uses context aware models to detect entities.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_content` | String |  | document_content is a document to be annotated. |
| `licensed_vocabularies` | Vec<String> |  | A list of licensed vocabularies to use in the request, in addition to the default unlicensed vocabularies. |
| `alternative_output_format` | String |  | Optional. Alternative output format to be generated based on the results of analysis. |
| `nlp_service` | String | ✅ | The resource name of the service of the form: "projects/{project_id}/locations/{location_id}/services/nlp". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create nlp
nlp = provider.healthcare_api.Nlp {
    nlp_service = "value"  # The resource name of the service of the form: "projects/{project_id}/locations/{location_id}/services/nlp".
}

```

---


### Fhir

Creates a FHIR resource. Implements the FHIR standard create interaction ([DSTU2](https://hl7.org/fhir/DSTU2/http.html#create), [STU3](https://hl7.org/fhir/STU3/http.html#create), [R4](https://hl7.org/fhir/R4/http.html#create), [R5](https://hl7.org/fhir/R5/http.html#create)), which creates a new resource with a server-assigned resource ID. Also supports the FHIR standard conditional create interaction ([DSTU2](https://hl7.org/fhir/DSTU2/http.html#ccreate), [STU3](https://hl7.org/fhir/STU3/http.html#ccreate), [R4](https://hl7.org/fhir/R4/http.html#ccreate), [R5](https://hl7.org/fhir/R5/http.html#ccreate)), specified by supplying an `If-None-Exist` header containing a FHIR search query, limited to searching by resource identifier. If no resources match this search query, the server processes the create operation as normal. When using conditional create, the search term for identifier should be in the pattern `identifier=system|value` or `identifier=value` - similar to the `search` method on resources with a specific identifier. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. On success, the response body contains a JSON-encoded representation of the resource as it was created on the server, including the server-assigned resource ID and version ID. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `create`, see [Creating a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#creating_a_fhir_resource).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `parent` | String | ✅ | Required. The name of the FHIR store this resource belongs to. |
| `type` | String | ✅ | Required. The FHIR resource type to create, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](https://hl7.org/fhir/DSTU2/resourcelist.html), [STU3](https://hl7.org/fhir/STU3/resourcelist.html), [R4](https://hl7.org/fhir/R4/resourcelist.html), [R5](https://hl7.org/fhir/R5/resourcelist.html)). Must match the resource type in the provided content. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir
fhir = provider.healthcare_api.Fhir {
    parent = "value"  # Required. The name of the FHIR store this resource belongs to.
    type = "value"  # Required. The FHIR resource type to create, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](https://hl7.org/fhir/DSTU2/resourcelist.html), [STU3](https://hl7.org/fhir/STU3/resourcelist.html), [R4](https://hl7.org/fhir/R4/resourcelist.html), [R5](https://hl7.org/fhir/R5/resourcelist.html)). Must match the resource type in the provided content.
}

# Access fhir outputs
fhir_id = fhir.id
fhir_data = fhir.data
fhir_content_type = fhir.content_type
fhir_extensions = fhir.extensions
```

---


### User_data_mapping

Creates a new User data mapping in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `archived` | bool |  | Output only. Indicates whether this mapping is archived. |
| `archive_time` | String |  | Output only. Indicates the time when this mapping was archived. |
| `name` | String |  | Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`. |
| `resource_attributes` | Vec<String> |  | Attributes of the resource. Only explicitly set attributes are displayed here. Attribute definitions with defaults set implicitly apply to these User data mappings. Attributes listed here must be single valued, that is, exactly one value is specified for the field "values" in each Attribute. |
| `data_id` | String |  | Required. A unique identifier for the mapped resource. |
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `parent` | String | ✅ | Required. Name of the consent store. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `archived` | bool | Output only. Indicates whether this mapping is archived. |
| `archive_time` | String | Output only. Indicates the time when this mapping was archived. |
| `name` | String | Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`. |
| `resource_attributes` | Vec<String> | Attributes of the resource. Only explicitly set attributes are displayed here. Attribute definitions with defaults set implicitly apply to these User data mappings. Attributes listed here must be single valued, that is, exactly one value is specified for the field "values" in each Attribute. |
| `data_id` | String | Required. A unique identifier for the mapped resource. |
| `user_id` | String | Required. User's UUID provided by the client. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_data_mapping
user_data_mapping = provider.healthcare_api.User_data_mapping {
    parent = "value"  # Required. Name of the consent store.
}

# Access user_data_mapping outputs
user_data_mapping_id = user_data_mapping.id
user_data_mapping_archived = user_data_mapping.archived
user_data_mapping_archive_time = user_data_mapping.archive_time
user_data_mapping_name = user_data_mapping.name
user_data_mapping_resource_attributes = user_data_mapping.resource_attributes
user_data_mapping_data_id = user_data_mapping.data_id
user_data_mapping_user_id = user_data_mapping.user_id
```

---


### Instance

RetrieveInstanceMetadata returns instance associated with the given study, series, and SOP Instance UID presented as metadata. See [RetrieveTransaction] (https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveInstanceMetadata, see [Metadata resources](https://cloud.google.com/healthcare/docs/dicom#metadata_resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveInstanceMetadata, see [Retrieve metadata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-metadata).

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance outputs
instance_id = instance.id
instance_data = instance.data
instance_content_type = instance.content_type
instance_extensions = instance.extensions
```

---


### Fhir_store

Creates a new FHIR store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_config` | String |  | Deprecated. Use `notification_configs` instead. If non-empty, publish all resource modifications of this FHIR store to this destination. The Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, "action":"CreateResource". Not supported in R5. Use `notification_configs` instead. |
| `notification_configs` | Vec<String> |  | Optional. Specifies where and whether to send notifications upon changes to a FHIR store. |
| `default_search_handling_strict` | bool |  | Optional. If true, overrides the default search behavior for this FHIR store to `handling=strict` which returns an error for unrecognized search parameters. If false, uses the FHIR specification default `handling=lenient` which ignores unrecognized search parameters. The handling can always be changed from the default on an individual API call by setting the HTTP header `Prefer: handling=strict` or `Prefer: handling=lenient`. Defaults to false. |
| `name` | String |  | Output only. Identifier. Resource name of the FHIR store, of the form `projects/{project_id}/locations/{location}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `disable_resource_versioning` | bool |  | Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions. Defaults to false. |
| `enable_update_create` | bool |  | Optional. Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. Defaults to false. |
| `complex_data_type_reference_parsing` | String |  | Optional. Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Cannot be disabled in R5. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `validation_config` | String |  | Optional. Configuration for how to validate incoming FHIR resources against configured profiles. |
| `bulk_export_gcs_destination` | String |  | Optional. FHIR bulk export exports resources to the specified Cloud Storage destination. A Cloud Storage destination is a URI for a Cloud Storage directory where result files will be written. Only used in the spec-defined bulk $export methods. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM role on the destination. |
| `version` | String |  | Required. Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store. |
| `disable_referential_integrity` | bool |  | Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist. |
| `stream_configs` | Vec<String> |  | Optional. A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination. |
| `consent_config` | String |  | Optional. Specifies whether this store has consent enforcement. Not available for DSTU2 FHIR version due to absence of Consent resources. Not supported for R5 FHIR version. |
| `parent` | String | ✅ | Required. The name of the dataset this FHIR store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_config` | String | Deprecated. Use `notification_configs` instead. If non-empty, publish all resource modifications of this FHIR store to this destination. The Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, "action":"CreateResource". Not supported in R5. Use `notification_configs` instead. |
| `notification_configs` | Vec<String> | Optional. Specifies where and whether to send notifications upon changes to a FHIR store. |
| `default_search_handling_strict` | bool | Optional. If true, overrides the default search behavior for this FHIR store to `handling=strict` which returns an error for unrecognized search parameters. If false, uses the FHIR specification default `handling=lenient` which ignores unrecognized search parameters. The handling can always be changed from the default on an individual API call by setting the HTTP header `Prefer: handling=strict` or `Prefer: handling=lenient`. Defaults to false. |
| `name` | String | Output only. Identifier. Resource name of the FHIR store, of the form `projects/{project_id}/locations/{location}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `disable_resource_versioning` | bool | Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions. Defaults to false. |
| `enable_update_create` | bool | Optional. Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. Defaults to false. |
| `complex_data_type_reference_parsing` | String | Optional. Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Cannot be disabled in R5. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `validation_config` | String | Optional. Configuration for how to validate incoming FHIR resources against configured profiles. |
| `bulk_export_gcs_destination` | String | Optional. FHIR bulk export exports resources to the specified Cloud Storage destination. A Cloud Storage destination is a URI for a Cloud Storage directory where result files will be written. Only used in the spec-defined bulk $export methods. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM role on the destination. |
| `version` | String | Required. Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store. |
| `disable_referential_integrity` | bool | Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist. |
| `stream_configs` | Vec<String> | Optional. A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination. |
| `consent_config` | String | Optional. Specifies whether this store has consent enforcement. Not available for DSTU2 FHIR version due to absence of Consent resources. Not supported for R5 FHIR version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir_store
fhir_store = provider.healthcare_api.Fhir_store {
    parent = "value"  # Required. The name of the dataset this FHIR store belongs to.
}

# Access fhir_store outputs
fhir_store_id = fhir_store.id
fhir_store_notification_config = fhir_store.notification_config
fhir_store_notification_configs = fhir_store.notification_configs
fhir_store_default_search_handling_strict = fhir_store.default_search_handling_strict
fhir_store_name = fhir_store.name
fhir_store_disable_resource_versioning = fhir_store.disable_resource_versioning
fhir_store_enable_update_create = fhir_store.enable_update_create
fhir_store_complex_data_type_reference_parsing = fhir_store.complex_data_type_reference_parsing
fhir_store_labels = fhir_store.labels
fhir_store_validation_config = fhir_store.validation_config
fhir_store_bulk_export_gcs_destination = fhir_store.bulk_export_gcs_destination
fhir_store_version = fhir_store.version
fhir_store_disable_referential_integrity = fhir_store.disable_referential_integrity
fhir_store_stream_configs = fhir_store.stream_configs
fhir_store_consent_config = fhir_store.consent_config
```

---


### Attribute_definition

Creates a new Attribute definition in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_mapping_default_value` | String |  | Optional. Default value of the attribute in User data mappings. If no default value is specified, it defaults to an empty value. This field is only applicable to attributes of the category `RESOURCE`. |
| `description` | String |  | Optional. A description of the attribute. |
| `allowed_values` | Vec<String> |  | Required. Possible values for the attribute. The number of allowed values must not exceed 500. An empty list is invalid. The list can only be expanded after creation. |
| `name` | String |  | Identifier. Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation. |
| `consent_default_values` | Vec<String> |  | Optional. Default values of the attribute in Consents. If no default values are specified, it defaults to an empty value. |
| `category` | String |  | Required. The category of the attribute. The value of this field cannot be changed after creation. |
| `parent` | String | ✅ | Required. The name of the consent store that this Attribute definition belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_mapping_default_value` | String | Optional. Default value of the attribute in User data mappings. If no default value is specified, it defaults to an empty value. This field is only applicable to attributes of the category `RESOURCE`. |
| `description` | String | Optional. A description of the attribute. |
| `allowed_values` | Vec<String> | Required. Possible values for the attribute. The number of allowed values must not exceed 500. An empty list is invalid. The list can only be expanded after creation. |
| `name` | String | Identifier. Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation. |
| `consent_default_values` | Vec<String> | Optional. Default values of the attribute in Consents. If no default values are specified, it defaults to an empty value. |
| `category` | String | Required. The category of the attribute. The value of this field cannot be changed after creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attribute_definition
attribute_definition = provider.healthcare_api.Attribute_definition {
    parent = "value"  # Required. The name of the consent store that this Attribute definition belongs to.
}

# Access attribute_definition outputs
attribute_definition_id = attribute_definition.id
attribute_definition_data_mapping_default_value = attribute_definition.data_mapping_default_value
attribute_definition_description = attribute_definition.description
attribute_definition_allowed_values = attribute_definition.allowed_values
attribute_definition_name = attribute_definition.name
attribute_definition_consent_default_values = attribute_definition.consent_default_values
attribute_definition_category = attribute_definition.category
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.healthcare_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Bulkdata

Returns uncompressed, unencoded bytes representing the referenced bulkdata tag from an instance. See [Retrieve Transaction](https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveBulkdata, see [Bulkdata resources](https://cloud.google.com/healthcare/docs/dicom#bulkdata-resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveBulkdata, see [Retrieve bulkdata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-bulkdata).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bulkdata outputs
bulkdata_id = bulkdata.id
bulkdata_data = bulkdata.data
bulkdata_content_type = bulkdata.content_type
bulkdata_extensions = bulkdata.extensions
```

---


### Data_mapper_workspace

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_mapper_workspace
data_mapper_workspace = provider.healthcare_api.Data_mapper_workspace {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access data_mapper_workspace outputs
data_mapper_workspace_id = data_mapper_workspace.id
data_mapper_workspace_etag = data_mapper_workspace.etag
data_mapper_workspace_version = data_mapper_workspace.version
data_mapper_workspace_bindings = data_mapper_workspace.bindings
data_mapper_workspace_audit_configs = data_mapper_workspace.audit_configs
```

---


### Studie

StoreInstances stores DICOM instances associated with study instance unique identifiers (SUID). See [Store Transaction] (https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5). For details on the implementation of StoreInstances, see [Store transaction](https://cloud.google.com/healthcare/docs/dicom#store_transaction) in the Cloud Healthcare API conformance statement. For samples that show how to call StoreInstances, see [Store DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#store-dicom).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `dicom_web_path` | String | ✅ | Required. The path of the StoreInstances DICOMweb request. For example, `studies/[{study_uid}]`. Note that the `study_uid` is optional. |
| `parent` | String | ✅ | Required. The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.healthcare_api.Studie {
    dicom_web_path = "value"  # Required. The path of the StoreInstances DICOMweb request. For example, `studies/[{study_uid}]`. Note that the `study_uid` is optional.
    parent = "value"  # Required. The name of the DICOM store that is being accessed. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
}

# Access studie outputs
studie_id = studie.id
studie_data = studie.data
studie_content_type = studie.content_type
studie_extensions = studie.extensions
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


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
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
location_display_name = location.display_name
```

---


### Serie

RetrieveSeries returns all instances within the given study and series. See [RetrieveTransaction] (https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveSeries, see [DICOM study/series/instances](https://cloud.google.com/healthcare/docs/dicom#dicom_studyseriesinstances) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveSeries, see [Retrieve DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-dicom).

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access serie outputs
serie_id = serie.id
serie_data = serie.data
serie_content_type = serie.content_type
serie_extensions = serie.extensions
```

---


### Consent_artifact

Creates a new Consent artifact in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `name` | String |  | Identifier. Resource name of the Consent artifact, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. Cannot be changed after creation. |
| `guardian_signature` | String |  | Optional. A signature from a guardian. |
| `user_signature` | String |  | Optional. User's signature. |
| `consent_content_version` | String |  | Optional. An string indicating the version of the consent information shown to the user. |
| `witness_signature` | String |  | Optional. A signature from a witness. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata associated with the Consent artifact. For example, the consent locale or user agent version. |
| `consent_content_screenshots` | Vec<String> |  | Optional. Screenshots, PDFs, or other binary information documenting the user's consent. |
| `parent` | String | ✅ | Required. The name of the consent store this Consent artifact belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_id` | String | Required. User's UUID provided by the client. |
| `name` | String | Identifier. Resource name of the Consent artifact, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. Cannot be changed after creation. |
| `guardian_signature` | String | Optional. A signature from a guardian. |
| `user_signature` | String | Optional. User's signature. |
| `consent_content_version` | String | Optional. An string indicating the version of the consent information shown to the user. |
| `witness_signature` | String | Optional. A signature from a witness. |
| `metadata` | HashMap<String, String> | Optional. Metadata associated with the Consent artifact. For example, the consent locale or user agent version. |
| `consent_content_screenshots` | Vec<String> | Optional. Screenshots, PDFs, or other binary information documenting the user's consent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent_artifact
consent_artifact = provider.healthcare_api.Consent_artifact {
    parent = "value"  # Required. The name of the consent store this Consent artifact belongs to.
}

# Access consent_artifact outputs
consent_artifact_id = consent_artifact.id
consent_artifact_user_id = consent_artifact.user_id
consent_artifact_name = consent_artifact.name
consent_artifact_guardian_signature = consent_artifact.guardian_signature
consent_artifact_user_signature = consent_artifact.user_signature
consent_artifact_consent_content_version = consent_artifact.consent_content_version
consent_artifact_witness_signature = consent_artifact.witness_signature
consent_artifact_metadata = consent_artifact.metadata
consent_artifact_consent_content_screenshots = consent_artifact.consent_content_screenshots
```

---


### Hl7_v2_store

Creates a new HL7v2 store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `notification_configs` | Vec<String> |  | Optional. A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client. |
| `parser_config` | String |  | Optional. The configuration for the parser. It determines how the server parses the messages. |
| `reject_duplicate_message` | bool |  | Optional. Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection. |
| `parent` | String | ✅ | Required. The name of the dataset this HL7v2 store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `notification_configs` | Vec<String> | Optional. A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client. |
| `parser_config` | String | Optional. The configuration for the parser. It determines how the server parses the messages. |
| `reject_duplicate_message` | bool | Optional. Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hl7_v2_store
hl7_v2_store = provider.healthcare_api.Hl7_v2_store {
    parent = "value"  # Required. The name of the dataset this HL7v2 store belongs to.
}

# Access hl7_v2_store outputs
hl7_v2_store_id = hl7_v2_store.id
hl7_v2_store_name = hl7_v2_store.name
hl7_v2_store_labels = hl7_v2_store.labels
hl7_v2_store_notification_configs = hl7_v2_store.notification_configs
hl7_v2_store_parser_config = hl7_v2_store.parser_config
hl7_v2_store_reject_duplicate_message = hl7_v2_store.reject_duplicate_message
```

---


### Dataset

Creates a new health dataset. Results are returned through the Operation interface which returns either an `Operation.response` which contains a Dataset or `Operation.error`. The metadata field type is OperationMetadata.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `satisfies_pzs` | bool |  | Output only. Whether the dataset satisfies zone separation. |
| `satisfies_pzi` | bool |  | Output only. Whether the dataset satisfies zone isolation. |
| `time_zone` | String |  | Optional. The default timezone used by this dataset. Must be a either a valid IANA time zone name such as "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified. |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for a Dataset. If set, this Dataset and all of its sub-resources will be secured by this key. If empty, the Dataset is secured by the default Google encryption key. |
| `parent` | String | ✅ | Required. The name of the project where the server creates the dataset. For example, `projects/{project_id}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `satisfies_pzs` | bool | Output only. Whether the dataset satisfies zone separation. |
| `satisfies_pzi` | bool | Output only. Whether the dataset satisfies zone isolation. |
| `time_zone` | String | Optional. The default timezone used by this dataset. Must be a either a valid IANA time zone name such as "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified. |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for a Dataset. If set, this Dataset and all of its sub-resources will be secured by this key. If empty, the Dataset is secured by the default Google encryption key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.healthcare_api.Dataset {
    parent = "value"  # Required. The name of the project where the server creates the dataset. For example, `projects/{project_id}/locations/{location_id}`.
}

# Access dataset outputs
dataset_id = dataset.id
dataset_name = dataset.name
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_time_zone = dataset.time_zone
dataset_encryption_spec = dataset.encryption_spec
```

---


### Frame

RetrieveFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers. See [RetrieveTransaction] (https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4}. For details on the implementation of RetrieveFrames, see [DICOM frames](https://cloud.google.com/healthcare/docs/dicom#dicom_frames) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveFrames, see [Retrieve DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-dicom).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access frame outputs
frame_id = frame.id
frame_data = frame.data
frame_content_type = frame.content_type
frame_extensions = frame.extensions
```

---


### Consent_store

Creates a new consent store in the parent dataset. Attempting to create a consent store with the same ID as an existing store fails with an ALREADY_EXISTS error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_consent_create_on_update` | bool |  | Optional. If `true`, UpdateConsent creates the Consent if it does not already exist. If unspecified, defaults to `false`. |
| `name` | String |  | Identifier. Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation. |
| `default_consent_ttl` | String |  | Optional. Default time to live for Consents created in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents. |
| `labels` | HashMap<String, String> |  | Optional. User-supplied key-value pairs used to organize consent stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}. Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}. No more than 64 labels can be associated with a given store. For more information: https://cloud.google.com/healthcare/docs/how-tos/labeling-resources |
| `parent` | String | ✅ | Required. The name of the dataset this consent store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_consent_create_on_update` | bool | Optional. If `true`, UpdateConsent creates the Consent if it does not already exist. If unspecified, defaults to `false`. |
| `name` | String | Identifier. Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation. |
| `default_consent_ttl` | String | Optional. Default time to live for Consents created in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents. |
| `labels` | HashMap<String, String> | Optional. User-supplied key-value pairs used to organize consent stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}. Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}. No more than 64 labels can be associated with a given store. For more information: https://cloud.google.com/healthcare/docs/how-tos/labeling-resources |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent_store
consent_store = provider.healthcare_api.Consent_store {
    parent = "value"  # Required. The name of the dataset this consent store belongs to.
}

# Access consent_store outputs
consent_store_id = consent_store.id
consent_store_enable_consent_create_on_update = consent_store.enable_consent_create_on_update
consent_store_name = consent_store.name
consent_store_default_consent_ttl = consent_store.default_consent_ttl
consent_store_labels = consent_store.labels
```

---


### Operation

Gets the latest state of a long-running operation.  Clients can use this
method to poll the operation result at intervals as recommended by the API
service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should have the format of `operations/some/unique/name`. |
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
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Hl7_v2_store

Returns permissions that a caller has on the specified resource.
If the resource does not exist, this will return an empty set of
permissions, not a NOT_FOUND error.

Note: This operation is designed to be used for building permission-aware
UIs and command-line tools, not for authorization checking. This operation
may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with
wildcards (such as '*' or 'storage.*') are not allowed. For more
information see
[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Deprecated. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

If no `etag` is provided in the call to `setIamPolicy`, then the existing
policy is overwritten blindly. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`.
`bindings` with no members will result in an error. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hl7_v2_store
hl7_v2_store = provider.healthcare_api.Hl7_v2_store {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field.
}

# Access hl7_v2_store outputs
hl7_v2_store_id = hl7_v2_store.id
hl7_v2_store_audit_configs = hl7_v2_store.audit_configs
hl7_v2_store_version = hl7_v2_store.version
hl7_v2_store_etag = hl7_v2_store.etag
hl7_v2_store_bindings = hl7_v2_store.bindings
```

---


### Dataset

Sets the access control policy on the specified resource. Replaces any
existing policy.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of
the policy is limited to a few 10s of KB. An empty policy is a
valid policy but certain Cloud Platform services (such as Projects)
might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only
the fields in the mask will be modified. If no mask is provided, the
following default mask is used:
paths: "bindings, etag"
This field is only used by Cloud IAM. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Deprecated. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

If no `etag` is provided in the call to `setIamPolicy`, then the existing
policy is overwritten blindly. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`.
`bindings` with no members will result in an error. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.healthcare_api.Dataset {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified.
See the operation documentation for the appropriate value for this field.
}

# Access dataset outputs
dataset_id = dataset.id
dataset_audit_configs = dataset.audit_configs
dataset_version = dataset.version
dataset_etag = dataset.etag
dataset_bindings = dataset.bindings
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
| `display_name` | String | The friendly name for this location, typically a nearby city name.
For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given
location. |
| `name` | String | Resource name for the location, which may vary between implementations.
For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example

    {"cloud.googleapis.com/region": "us-east1"} |
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
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Dicom_store

Returns permissions that a caller has on the specified resource.
If the resource does not exist, this will return an empty set of
permissions, not a NOT_FOUND error.

Note: This operation is designed to be used for building permission-aware
UIs and command-line tools, not for authorization checking. This operation
may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with
wildcards (such as '*' or 'storage.*') are not allowed. For more
information see
[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Deprecated. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

If no `etag` is provided in the call to `setIamPolicy`, then the existing
policy is overwritten blindly. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`.
`bindings` with no members will result in an error. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dicom_store
dicom_store = provider.healthcare_api.Dicom_store {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field.
}

# Access dicom_store outputs
dicom_store_id = dicom_store.id
dicom_store_audit_configs = dicom_store.audit_configs
dicom_store_version = dicom_store.version
dicom_store_etag = dicom_store.etag
dicom_store_bindings = dicom_store.bindings
```

---


### Annotation

Creates a new Annotation record. It is
valid to create Annotation objects for the same source more than once since
a unique ID is assigned to each record by this service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_annotation` | String |  | Annotations for resource, e.g., classification tags. |
| `text_annotation` | String |  | Annotations for sensitive texts, e.g., range of such texts. |
| `annotation_source` | String |  | Details of the source. |
| `image_annotation` | String |  | Annotations for images, e.g., bounding polygons. |
| `name` | String |  | Output only. Resource name of the Annotation, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{annotation_store_id}/annotations/{annotation_id}`. |
| `parent` | String | ✅ | The name of the Annotation store this annotation belongs to. For example,
`projects/my-project/locations/us-central1/datasets/mydataset/annotationStores/myannotationstore`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_annotation` | String | Annotations for resource, e.g., classification tags. |
| `text_annotation` | String | Annotations for sensitive texts, e.g., range of such texts. |
| `annotation_source` | String | Details of the source. |
| `image_annotation` | String | Annotations for images, e.g., bounding polygons. |
| `name` | String | Output only. Resource name of the Annotation, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{annotation_store_id}/annotations/{annotation_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create annotation
annotation = provider.healthcare_api.Annotation {
    parent = "value"  # The name of the Annotation store this annotation belongs to. For example,
`projects/my-project/locations/us-central1/datasets/mydataset/annotationStores/myannotationstore`.
}

# Access annotation outputs
annotation_id = annotation.id
annotation_resource_annotation = annotation.resource_annotation
annotation_text_annotation = annotation.text_annotation
annotation_annotation_source = annotation.annotation_source
annotation_image_annotation = annotation.image_annotation
annotation_name = annotation.name
```

---


### Operation

Gets the latest state of a long-running operation.  Clients can use this
method to poll the operation result at intervals as recommended by the API
service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |


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
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Hl7_v2_store

Creates a new HL7v2 store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_config` | String |  | The notification destination all messages (both Ingest & Create) are
published on. Only the message name is sent as part of the notification. If
this is unset, no notifications will be sent. Supplied by the client. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize HL7v2 stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `name` | String |  | Output only. Resource name of the HL7v2 store, of the form
`projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |
| `parser_config` | String |  | The configuration for the parser. It determines how the server parses the
messages. |
| `parent` | String | ✅ | The name of the dataset this HL7v2 store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_config` | String | The notification destination all messages (both Ingest & Create) are
published on. Only the message name is sent as part of the notification. If
this is unset, no notifications will be sent. Supplied by the client. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `name` | String | Output only. Resource name of the HL7v2 store, of the form
`projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |
| `parser_config` | String | The configuration for the parser. It determines how the server parses the
messages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hl7_v2_store
hl7_v2_store = provider.healthcare_api.Hl7_v2_store {
    parent = "value"  # The name of the dataset this HL7v2 store belongs to.
}

# Access hl7_v2_store outputs
hl7_v2_store_id = hl7_v2_store.id
hl7_v2_store_notification_config = hl7_v2_store.notification_config
hl7_v2_store_labels = hl7_v2_store.labels
hl7_v2_store_name = hl7_v2_store.name
hl7_v2_store_parser_config = hl7_v2_store.parser_config
```

---


### Serie

SearchForInstances returns a list of matching instances. See
http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access serie outputs
serie_id = serie.id
serie_content_type = serie.content_type
serie_extensions = serie.extensions
serie_data = serie.data
```

---


### Dataset

Creates a new health dataset. Results are returned through the
Operation interface which returns either an
`Operation.response` which contains a Dataset or
`Operation.error`. The metadata
field type is OperationMetadata.
A Google Cloud Platform project can contain up to 500 datasets across all
regions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the dataset, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `time_zone` | String |  | The default timezone used by this dataset. Must be a either a valid IANA
time zone name such as "America/New_York" or empty, which defaults to UTC.
This is used for parsing times in resources (e.g., HL7 messages) where no
explicit timezone is specified. |
| `parent` | String | ✅ | The name of the project in which the dataset should be created (e.g.,
`projects/{project_id}/locations/{location_id}`). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the dataset, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `time_zone` | String | The default timezone used by this dataset. Must be a either a valid IANA
time zone name such as "America/New_York" or empty, which defaults to UTC.
This is used for parsing times in resources (e.g., HL7 messages) where no
explicit timezone is specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.healthcare_api.Dataset {
    parent = "value"  # The name of the project in which the dataset should be created (e.g.,
`projects/{project_id}/locations/{location_id}`).
}

# Access dataset outputs
dataset_id = dataset.id
dataset_name = dataset.name
dataset_time_zone = dataset.time_zone
```

---


### Dicom_store

Creates a new DICOM store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_config` | String |  | Notification destination for new DICOM instances.
Supplied by the client. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize DICOM stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `name` | String |  | Output only. Resource name of the DICOM store, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |
| `parent` | String | ✅ | The name of the dataset this DICOM store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_config` | String | Notification destination for new DICOM instances.
Supplied by the client. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize DICOM stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `name` | String | Output only. Resource name of the DICOM store, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dicom_store
dicom_store = provider.healthcare_api.Dicom_store {
    parent = "value"  # The name of the dataset this DICOM store belongs to.
}

# Access dicom_store outputs
dicom_store_id = dicom_store.id
dicom_store_notification_config = dicom_store.notification_config
dicom_store_labels = dicom_store.labels
dicom_store_name = dicom_store.name
```

---


### Fhir_store

Creates a new FHIR store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validation_config` | String |  | Configuration for how incoming FHIR resources will be validated against
configured profiles. |
| `disable_referential_integrity` | bool |  | Whether to disable referential integrity in this FHIR store. This field is
immutable after FHIR store creation.
The default value is false, meaning that the API will enforce referential
integrity and fail the requests that will result in inconsistent state in
the FHIR store.
When this field is set to true, the API will skip referential integrity
check. Consequently, operations that rely on references, such as
GetPatientEverything, will not return all the results if broken references
exist. |
| `subscription_config` | String |  | Configuration of FHIR Subscription:
https://www.hl7.org/fhir/subscription.html. |
| `enable_history_import` | bool |  | Whether to allow the bulk import API to accept history bundles and directly
insert historical resource versions into the FHIR store. Importing resource
histories creates resource interactions that appear to have occurred in the
past, which clients may not want to allow. If set to false, history bundles
within an import will fail with an error. |
| `disable_resource_versioning` | bool |  | Whether to disable resource versioning for this FHIR store. This field can
not be changed after the creation of FHIR store.
If set to false, which is the default behavior, all write operations will
cause historical versions to be recorded automatically. The historical
versions can be fetched through the history APIs, but cannot be updated.
If set to true, no historical versions will be kept. The server will send
back errors for attempts to read the historical versions. |
| `name` | String |  | Output only. Resource name of the FHIR store, of the form
`projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize FHIR stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `notification_config` | String |  | If non-empty, publish all resource modifications of this FHIR store to
this destination. The Cloud Pub/Sub message attributes will contain a map
with a string describing the action that has triggered the notification,
e.g. "action":"CreateResource". |
| `stream_configs` | Vec<String> |  | A list of streaming configs that configure the destinations of streaming
export for every resource mutation in this FHIR store. Each store is
allowed to have up to 10 streaming configs.
After a new config is added, the next resource mutation will be streamed to
the new location in addition to the existing ones.
When a location is removed from the list, the server will simply stop
streaming to that location. Before adding a new config, you must add the
required
[`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor)
role to your project's **Cloud Healthcare Service Agent**
[service account](https://cloud.google.com/iam/docs/service-accounts).
Some lag (typically on the order of dozens of seconds) is expected before
the results show up in the streaming destination. |
| `enable_update_create` | bool |  | Whether this FHIR store has the [updateCreate
capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate).
This determines if the client can use an Update operation to create a new
resource with a client-specified ID. If false, all IDs are server-assigned
through the Create operation and attempts to Update a non-existent resource
will return errors. Please treat the audit logs with appropriate levels of
care if client-specified resource IDs contain sensitive data such as
patient identifiers, those IDs will be part of the FHIR resource path
recorded in Cloud audit logs and Cloud Pub/Sub notifications. |
| `parent` | String | ✅ | The name of the dataset this FHIR store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validation_config` | String | Configuration for how incoming FHIR resources will be validated against
configured profiles. |
| `disable_referential_integrity` | bool | Whether to disable referential integrity in this FHIR store. This field is
immutable after FHIR store creation.
The default value is false, meaning that the API will enforce referential
integrity and fail the requests that will result in inconsistent state in
the FHIR store.
When this field is set to true, the API will skip referential integrity
check. Consequently, operations that rely on references, such as
GetPatientEverything, will not return all the results if broken references
exist. |
| `subscription_config` | String | Configuration of FHIR Subscription:
https://www.hl7.org/fhir/subscription.html. |
| `enable_history_import` | bool | Whether to allow the bulk import API to accept history bundles and directly
insert historical resource versions into the FHIR store. Importing resource
histories creates resource interactions that appear to have occurred in the
past, which clients may not want to allow. If set to false, history bundles
within an import will fail with an error. |
| `disable_resource_versioning` | bool | Whether to disable resource versioning for this FHIR store. This field can
not be changed after the creation of FHIR store.
If set to false, which is the default behavior, all write operations will
cause historical versions to be recorded automatically. The historical
versions can be fetched through the history APIs, but cannot be updated.
If set to true, no historical versions will be kept. The server will send
back errors for attempts to read the historical versions. |
| `name` | String | Output only. Resource name of the FHIR store, of the form
`projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize FHIR stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `notification_config` | String | If non-empty, publish all resource modifications of this FHIR store to
this destination. The Cloud Pub/Sub message attributes will contain a map
with a string describing the action that has triggered the notification,
e.g. "action":"CreateResource". |
| `stream_configs` | Vec<String> | A list of streaming configs that configure the destinations of streaming
export for every resource mutation in this FHIR store. Each store is
allowed to have up to 10 streaming configs.
After a new config is added, the next resource mutation will be streamed to
the new location in addition to the existing ones.
When a location is removed from the list, the server will simply stop
streaming to that location. Before adding a new config, you must add the
required
[`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor)
role to your project's **Cloud Healthcare Service Agent**
[service account](https://cloud.google.com/iam/docs/service-accounts).
Some lag (typically on the order of dozens of seconds) is expected before
the results show up in the streaming destination. |
| `enable_update_create` | bool | Whether this FHIR store has the [updateCreate
capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate).
This determines if the client can use an Update operation to create a new
resource with a client-specified ID. If false, all IDs are server-assigned
through the Create operation and attempts to Update a non-existent resource
will return errors. Please treat the audit logs with appropriate levels of
care if client-specified resource IDs contain sensitive data such as
patient identifiers, those IDs will be part of the FHIR resource path
recorded in Cloud audit logs and Cloud Pub/Sub notifications. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir_store
fhir_store = provider.healthcare_api.Fhir_store {
    parent = "value"  # The name of the dataset this FHIR store belongs to.
}

# Access fhir_store outputs
fhir_store_id = fhir_store.id
fhir_store_validation_config = fhir_store.validation_config
fhir_store_disable_referential_integrity = fhir_store.disable_referential_integrity
fhir_store_subscription_config = fhir_store.subscription_config
fhir_store_enable_history_import = fhir_store.enable_history_import
fhir_store_disable_resource_versioning = fhir_store.disable_resource_versioning
fhir_store_name = fhir_store.name
fhir_store_labels = fhir_store.labels
fhir_store_notification_config = fhir_store.notification_config
fhir_store_stream_configs = fhir_store.stream_configs
fhir_store_enable_update_create = fhir_store.enable_update_create
```

---


### Fhir

Creates a FHIR resource.

Implements the FHIR standard create interaction
([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#create),
[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#create)),
which creates a new resource with a server-assigned resource ID.

Also supports the FHIR standard conditional create interaction
([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/http.html#ccreate),
[STU3](http://hl7.org/implement/standards/fhir/STU3/http.html#ccreate)),
specified by supplying an `If-None-Exist` header containing a FHIR search
query. If no resources match this search query, the server processes the
create operation as normal.

The request body must contain a JSON-encoded FHIR resource, and the request
headers must contain `Content-Type: application/fhir+json`.

On success, the response body will contain a JSON-encoded representation
of the resource as it was created on the server, including the
server-assigned resource ID and version ID.
Errors generated by the FHIR store will contain a JSON-encoded
`OperationOutcome` resource describing the reason for the error. If the
request cannot be mapped to a valid API method on a FHIR store, a generic
GCP error might be returned instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `parent` | String | ✅ | The name of the FHIR store this resource belongs to. |
| `type` | String | ✅ | The FHIR resource type to create, such as Patient or Observation. For a
complete list, see the FHIR Resource Index
([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html),
[STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html)).
Must match the resource type in the provided content. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir
fhir = provider.healthcare_api.Fhir {
    parent = "value"  # The name of the FHIR store this resource belongs to.
    type = "value"  # The FHIR resource type to create, such as Patient or Observation. For a
complete list, see the FHIR Resource Index
([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html),
[STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html)).
Must match the resource type in the provided content.
}

# Access fhir outputs
fhir_id = fhir.id
fhir_content_type = fhir.content_type
fhir_extensions = fhir.extensions
fhir_data = fhir.data
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given
location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example

    {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name.
For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations.
For example: `"projects/example-project/locations/us-east1"` |


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
location_labels = location.labels
location_display_name = location.display_name
location_name = location.name
```

---


### Studie

StoreInstances stores DICOM instances associated with study instance unique
identifiers (SUID). See
http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `parent` | String | ✅ | The name of the DICOM store that is being accessed (e.g.,
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`). |
| `dicom_web_path` | String | ✅ | The path of the StoreInstances DICOMweb request (e.g.,
`studies/[{study_id}]`). Note that the `study_uid` is optional. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.healthcare_api.Studie {
    parent = "value"  # The name of the DICOM store that is being accessed (e.g.,
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`).
    dicom_web_path = "value"  # The path of the StoreInstances DICOMweb request (e.g.,
`studies/[{study_id}]`). Note that the `study_uid` is optional.
}

# Access studie outputs
studie_id = studie.id
studie_content_type = studie.content_type
studie_extensions = studie.extensions
studie_data = studie.data
```

---


### Instance

RetrieveInstance returns instance associated with the given study, series,
and SOP Instance UID. See
http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance outputs
instance_id = instance.id
instance_content_type = instance.content_type
instance_extensions = instance.extensions
instance_data = instance.data
```

---


### Annotation_store

Creates a new Annotation store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the Annotation store, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{annotation_store_id}`. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize Annotation stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `parent` | String | ✅ | The name of the dataset this Annotation store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the Annotation store, of the form
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationStores/{annotation_store_id}`. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize Annotation stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create annotation_store
annotation_store = provider.healthcare_api.Annotation_store {
    parent = "value"  # The name of the dataset this Annotation store belongs to.
}

# Access annotation_store outputs
annotation_store_id = annotation_store.id
annotation_store_name = annotation_store.name
annotation_store_labels = annotation_store.labels
```

---


### Dicom_web

StoreInstances stores DICOM instances associated with study instance unique
identifiers (SUID). See
http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `dicom_web_path` | String | ✅ | The path of the StoreInstances DICOMweb request (e.g.,
`studies/[{study_id}]`). Note that the `study_uid` is optional. |
| `parent` | String | ✅ | The name of the DICOM store that is being accessed (e.g.,
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dicom_web
dicom_web = provider.healthcare_api.Dicom_web {
    dicom_web_path = "value"  # The path of the StoreInstances DICOMweb request (e.g.,
`studies/[{study_id}]`). Note that the `study_uid` is optional.
    parent = "value"  # The name of the DICOM store that is being accessed (e.g.,
`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`).
}

# Access dicom_web outputs
dicom_web_id = dicom_web.id
dicom_web_content_type = dicom_web.content_type
dicom_web_extensions = dicom_web.extensions
dicom_web_data = dicom_web.data
```

---


### Message

Creates a message and sends a notification to the Cloud Pub/Sub topic. If
configured, the MLLP adapter listens to messages created by this method and
sends those back to the hospital. A successful response indicates the
message has been persisted to storage and a Cloud Pub/Sub notification has
been sent. Sending to the hospital by the MLLP adapter happens
asynchronously.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message` | String |  | HL7v2 message. |
| `parent` | String | ✅ | The name of the dataset this message belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | Raw message bytes. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores.

Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
of maximum 128 bytes, and must conform to the
following PCRE regular expression:
\p{Ll}\p{Lo}{0,62}

Label values are optional, must be between 1 and 63 characters long, have
a UTF-8 encoding of maximum 128 bytes, and must conform to the
following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}

No more than 64 labels can be associated with a given store. |
| `create_time` | String | Output only. The datetime when the message was created. Set by the server. |
| `message_type` | String | The message type and trigger event for this message. MSH-9. |
| `send_facility` | String | The hospital that this message came from. MSH-4. |
| `send_time` | String | The datetime the sending application sent this message. MSH-7. |
| `patient_ids` | Vec<String> | All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this
message. |
| `name` | String | Resource name of the Message, of the form
`projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`.
Assigned by the server. |
| `parsed_data` | String | Output only. The parsed version of the raw message data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.healthcare_api.Message {
    parent = "value"  # The name of the dataset this message belongs to.
}

# Access message outputs
message_id = message.id
message_data = message.data
message_labels = message.labels
message_create_time = message.create_time
message_message_type = message.message_type
message_send_facility = message.send_facility
message_send_time = message.send_time
message_patient_ids = message.patient_ids
message_name = message.name
message_parsed_data = message.parsed_data
```

---


### Frame

RetrieveFrames returns instances associated with the given study, series,
SOP Instance UID and frame numbers. See
http://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access frame outputs
frame_id = frame.id
frame_content_type = frame.content_type
frame_extensions = frame.extensions
frame_data = frame.data
```

---


### Attribute_definition

Creates a new Attribute definition in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_mapping_default_value` | String |  | Optional. Default value of the attribute in User data mappings. If no default value is specified, it defaults to an empty value. This field is only applicable to attributes of the category `RESOURCE`. |
| `description` | String |  | Optional. A description of the attribute. |
| `consent_default_values` | Vec<String> |  | Optional. Default values of the attribute in Consents. If no default values are specified, it defaults to an empty value. |
| `allowed_values` | Vec<String> |  | Required. Possible values for the attribute. The number of allowed values must not exceed 500. An empty list is invalid. The list can only be expanded after creation. |
| `name` | String |  | Identifier. Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation. |
| `category` | String |  | Required. The category of the attribute. The value of this field cannot be changed after creation. |
| `parent` | String | ✅ | Required. The name of the consent store that this Attribute definition belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_mapping_default_value` | String | Optional. Default value of the attribute in User data mappings. If no default value is specified, it defaults to an empty value. This field is only applicable to attributes of the category `RESOURCE`. |
| `description` | String | Optional. A description of the attribute. |
| `consent_default_values` | Vec<String> | Optional. Default values of the attribute in Consents. If no default values are specified, it defaults to an empty value. |
| `allowed_values` | Vec<String> | Required. Possible values for the attribute. The number of allowed values must not exceed 500. An empty list is invalid. The list can only be expanded after creation. |
| `name` | String | Identifier. Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation. |
| `category` | String | Required. The category of the attribute. The value of this field cannot be changed after creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attribute_definition
attribute_definition = provider.healthcare_api.Attribute_definition {
    parent = "value"  # Required. The name of the consent store that this Attribute definition belongs to.
}

# Access attribute_definition outputs
attribute_definition_id = attribute_definition.id
attribute_definition_data_mapping_default_value = attribute_definition.data_mapping_default_value
attribute_definition_description = attribute_definition.description
attribute_definition_consent_default_values = attribute_definition.consent_default_values
attribute_definition_allowed_values = attribute_definition.allowed_values
attribute_definition_name = attribute_definition.name
attribute_definition_category = attribute_definition.category
```

---


### Studie

SetBlobStorageSettings sets the blob storage settings of the specified resources.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blob_storage_settings` | String |  | The blob storage settings to update for the specified resources. Only fields listed in `update_mask` are applied. |
| `filter_config` | String |  | Optional. A filter configuration. If `filter_config` is specified, set the value of `resource` to the resource name of a DICOM store in the format `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}`. |
| `resource` | String | ✅ | Required. The path of the resource to update the blob storage settings in the format of `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}`, `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}/series/{seriesUID}/`, or `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}/series/{seriesUID}/instances/{instanceUID}`. If `filter_config` is specified, set the value of `resource` to the resource name of a DICOM store in the format `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `series_count` | String | Number of series in the study. |
| `study` | String | The study resource path. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}/dicomWeb/studies/{study_uid}`. |
| `blob_storage_size_bytes` | String | Total blob storage bytes for all instances in the study. |
| `instance_count` | String | Number of instances in the study. |
| `structured_storage_size_bytes` | String | Total structured storage bytes for all instances in the study. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.healthcare_api.Studie {
    resource = "value"  # Required. The path of the resource to update the blob storage settings in the format of `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}`, `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}/series/{seriesUID}/`, or `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}/series/{seriesUID}/instances/{instanceUID}`. If `filter_config` is specified, set the value of `resource` to the resource name of a DICOM store in the format `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}`.
}

# Access studie outputs
studie_id = studie.id
studie_series_count = studie.series_count
studie_study = studie.study
studie_blob_storage_size_bytes = studie.blob_storage_size_bytes
studie_instance_count = studie.instance_count
studie_structured_storage_size_bytes = studie.structured_storage_size_bytes
```

---


### User_data_mapping

Creates a new User data mapping in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `archive_time` | String |  | Output only. Indicates the time when this mapping was archived. |
| `archived` | bool |  | Output only. Indicates whether this mapping is archived. |
| `resource_attributes` | Vec<String> |  | Attributes of the resource. Only explicitly set attributes are displayed here. Attribute definitions with defaults set implicitly apply to these User data mappings. Attributes listed here must be single valued, that is, exactly one value is specified for the field "values" in each Attribute. |
| `data_id` | String |  | Required. A unique identifier for the mapped resource. |
| `name` | String |  | Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`. |
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `parent` | String | ✅ | Required. Name of the consent store. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `archive_time` | String | Output only. Indicates the time when this mapping was archived. |
| `archived` | bool | Output only. Indicates whether this mapping is archived. |
| `resource_attributes` | Vec<String> | Attributes of the resource. Only explicitly set attributes are displayed here. Attribute definitions with defaults set implicitly apply to these User data mappings. Attributes listed here must be single valued, that is, exactly one value is specified for the field "values" in each Attribute. |
| `data_id` | String | Required. A unique identifier for the mapped resource. |
| `name` | String | Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`. |
| `user_id` | String | Required. User's UUID provided by the client. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_data_mapping
user_data_mapping = provider.healthcare_api.User_data_mapping {
    parent = "value"  # Required. Name of the consent store.
}

# Access user_data_mapping outputs
user_data_mapping_id = user_data_mapping.id
user_data_mapping_archive_time = user_data_mapping.archive_time
user_data_mapping_archived = user_data_mapping.archived
user_data_mapping_resource_attributes = user_data_mapping.resource_attributes
user_data_mapping_data_id = user_data_mapping.data_id
user_data_mapping_name = user_data_mapping.name
user_data_mapping_user_id = user_data_mapping.user_id
```

---


### Consent_artifact

Creates a new Consent artifact in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_signature` | String |  | Optional. User's signature. |
| `consent_content_version` | String |  | Optional. An string indicating the version of the consent information shown to the user. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata associated with the Consent artifact. For example, the consent locale or user agent version. |
| `consent_content_screenshots` | Vec<String> |  | Optional. Screenshots, PDFs, or other binary information documenting the user's consent. |
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `witness_signature` | String |  | Optional. A signature from a witness. |
| `guardian_signature` | String |  | Optional. A signature from a guardian. |
| `name` | String |  | Identifier. Resource name of the Consent artifact, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. Cannot be changed after creation. |
| `parent` | String | ✅ | Required. The name of the consent store this Consent artifact belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_signature` | String | Optional. User's signature. |
| `consent_content_version` | String | Optional. An string indicating the version of the consent information shown to the user. |
| `metadata` | HashMap<String, String> | Optional. Metadata associated with the Consent artifact. For example, the consent locale or user agent version. |
| `consent_content_screenshots` | Vec<String> | Optional. Screenshots, PDFs, or other binary information documenting the user's consent. |
| `user_id` | String | Required. User's UUID provided by the client. |
| `witness_signature` | String | Optional. A signature from a witness. |
| `guardian_signature` | String | Optional. A signature from a guardian. |
| `name` | String | Identifier. Resource name of the Consent artifact, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. Cannot be changed after creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent_artifact
consent_artifact = provider.healthcare_api.Consent_artifact {
    parent = "value"  # Required. The name of the consent store this Consent artifact belongs to.
}

# Access consent_artifact outputs
consent_artifact_id = consent_artifact.id
consent_artifact_user_signature = consent_artifact.user_signature
consent_artifact_consent_content_version = consent_artifact.consent_content_version
consent_artifact_metadata = consent_artifact.metadata
consent_artifact_consent_content_screenshots = consent_artifact.consent_content_screenshots
consent_artifact_user_id = consent_artifact.user_id
consent_artifact_witness_signature = consent_artifact.witness_signature
consent_artifact_guardian_signature = consent_artifact.guardian_signature
consent_artifact_name = consent_artifact.name
```

---


### Message

Parses and stores an HL7v2 message. This method triggers an asynchronous notification to any Pub/Sub topic configured in Hl7V2Store.Hl7V2NotificationConfig, if the filtering matches the message. If an MLLP adapter is configured to listen to a Pub/Sub topic, the adapter transmits the message when a notification is received.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message` | String |  | Required. HL7v2 message. |
| `parent` | String | ✅ | Required. The name of the HL7v2 store this message belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message_type` | String | The message type for this message. MSH-9.1. |
| `parsed_data` | String | Output only. The parsed version of the raw message data. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `name` | String | Output only. Resource name of the Message, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`. Assigned by the server. |
| `data` | String | Required. Raw message bytes. |
| `send_time` | String | The datetime the sending application sent this message. MSH-7. |
| `schematized_data` | String | The parsed version of the raw message data schematized according to this store's schemas and type definitions. |
| `send_facility` | String | The hospital that this message came from. MSH-4. |
| `patient_ids` | Vec<String> | All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this message. |
| `create_time` | String | Output only. The datetime when the message was created. Set by the server. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.healthcare_api.Message {
    parent = "value"  # Required. The name of the HL7v2 store this message belongs to.
}

# Access message outputs
message_id = message.id
message_message_type = message.message_type
message_parsed_data = message.parsed_data
message_labels = message.labels
message_name = message.name
message_data = message.data
message_send_time = message.send_time
message_schematized_data = message.schematized_data
message_send_facility = message.send_facility
message_patient_ids = message.patient_ids
message_create_time = message.create_time
```

---


### Serie

GetSeriesMetrics returns metrics for a series.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. The name of the DICOM store that is being accessed (for example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`). |
| `dicom_web_path` | String | ✅ | Required. The path of the UpdateSeriesMetadata request (for example, `studies/{study_uid}/series/{series_uid}`). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blob_storage_size_bytes` | String | Total blob storage bytes for all instances in the series. |
| `instance_count` | String | Number of instances in the series. |
| `series` | String | The series resource path. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}/dicomWeb/studies/{study_uid}/series/{series_uid}`. |
| `structured_storage_size_bytes` | String | Total structured storage bytes for all instances in the series. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access serie outputs
serie_id = serie.id
serie_blob_storage_size_bytes = serie.blob_storage_size_bytes
serie_instance_count = serie.instance_count
serie_series = serie.series
serie_structured_storage_size_bytes = serie.structured_storage_size_bytes
```

---


### Instance

GetStorageInfo returns the storage info of the specified resource.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. The name of the DICOM store that is being accessed (for example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`). |
| `dicom_web_path` | String | ✅ | Required. The path of the UpdateInstanceMetadata request (for example, `studies/{study_uid}/series/{series_uid}/instances/{instance_uid}`). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `referenced_resource` | String | The resource whose storage info is returned. For example: `projects/{projectID}/locations/{locationID}/datasets/{datasetID}/dicomStores/{dicomStoreID}/dicomWeb/studies/{studyUID}/series/{seriesUID}/instances/{instanceUID}` |
| `structured_storage_info` | String | Info about the data stored in structured storage for the resource. |
| `blob_storage_info` | String | Info about the data stored in blob storage for the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance outputs
instance_id = instance.id
instance_referenced_resource = instance.referenced_resource
instance_structured_storage_info = instance.structured_storage_info
instance_blob_storage_info = instance.blob_storage_info
```

---


### Frame

RetrieveFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers. See [RetrieveTransaction](https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveFrames, see [DICOM frames](https://cloud.google.com/healthcare/docs/dicom#dicom_frames) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveFrames, see [Retrieve DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-dicom).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access frame outputs
frame_id = frame.id
frame_data = frame.data
frame_extensions = frame.extensions
frame_content_type = frame.content_type
```

---


### Consent_store

Creates a new consent store in the parent dataset. Attempting to create a consent store with the same ID as an existing store fails with an ALREADY_EXISTS error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. User-supplied key-value pairs used to organize consent stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}. Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}. No more than 64 labels can be associated with a given store. For more information: https://cloud.google.com/healthcare/docs/how-tos/labeling-resources |
| `default_consent_ttl` | String |  | Optional. Default time to live for Consents created in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents. |
| `enable_consent_create_on_update` | bool |  | Optional. If `true`, UpdateConsent creates the Consent if it does not already exist. If unspecified, defaults to `false`. |
| `name` | String |  | Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation. |
| `parent` | String | ✅ | Required. The name of the dataset this consent store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. User-supplied key-value pairs used to organize consent stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}. Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}. No more than 64 labels can be associated with a given store. For more information: https://cloud.google.com/healthcare/docs/how-tos/labeling-resources |
| `default_consent_ttl` | String | Optional. Default time to live for Consents created in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents. |
| `enable_consent_create_on_update` | bool | Optional. If `true`, UpdateConsent creates the Consent if it does not already exist. If unspecified, defaults to `false`. |
| `name` | String | Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent_store
consent_store = provider.healthcare_api.Consent_store {
    parent = "value"  # Required. The name of the dataset this consent store belongs to.
}

# Access consent_store outputs
consent_store_id = consent_store.id
consent_store_labels = consent_store.labels
consent_store_default_consent_ttl = consent_store.default_consent_ttl
consent_store_enable_consent_create_on_update = consent_store.enable_consent_create_on_update
consent_store_name = consent_store.name
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_metadata = location.metadata
location_labels = location.labels
location_display_name = location.display_name
location_name = location.name
```

---


### Data_mapper_workspace

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_mapper_workspace
data_mapper_workspace = provider.healthcare_api.Data_mapper_workspace {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access data_mapper_workspace outputs
data_mapper_workspace_id = data_mapper_workspace.id
data_mapper_workspace_etag = data_mapper_workspace.etag
data_mapper_workspace_version = data_mapper_workspace.version
data_mapper_workspace_audit_configs = data_mapper_workspace.audit_configs
data_mapper_workspace_bindings = data_mapper_workspace.bindings
```

---


### Fhir_store

Creates a new FHIR store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_referential_integrity` | bool |  | Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist. |
| `notification_configs` | Vec<String> |  | Specifies where and whether to send notifications upon changes to a Fhir store. |
| `validation_config` | String |  | Configuration for how to validate incoming FHIR resources against configured profiles. |
| `notification_config` | String |  | Deprecated. Use `notification_configs` instead. If non-empty, publish all resource modifications of this FHIR store to this destination. The Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, "action":"CreateResource". Not supported in R5. Use `notification_configs` instead. |
| `stream_configs` | Vec<String> |  | A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination. |
| `default_search_handling_strict` | bool |  | If true, overrides the default search behavior for this FHIR store to `handling=strict` which returns an error for unrecognized search parameters. If false, uses the FHIR specification default `handling=lenient` which ignores unrecognized search parameters. The handling can always be changed from the default on an individual API call by setting the HTTP header `Prefer: handling=strict` or `Prefer: handling=lenient`. Defaults to false. |
| `disable_resource_versioning` | bool |  | Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions. Defaults to false. |
| `complex_data_type_reference_parsing` | String |  | Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Cannot be disabled in R5. |
| `bulk_export_gcs_destination` | String |  | Optional. FHIR bulk export exports resources to the specified Cloud Storage destination. A Cloud Storage destination is a URI for a Cloud Storage directory where result files will be written. Only used in the spec-defined bulk $export methods. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM role on the destination. |
| `enable_history_modifications` | bool |  | Optional. Whether to allow the [ImportResourcesHistory] and [ExecuteBundle] APIs to accept history bundles, and directly insert and overwrite historical resource versions into the FHIR store. Changing resource histories creates resource interactions that have occurred in the past which clients might not allow. If set to false, [ImportResourcesHistory] and [ExecuteBundle] requests will return errors. |
| `name` | String |  | Output only. Identifier. Resource name of the FHIR store, of the form `projects/{project_id}/locations/{location}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `search_config` | String |  | Configuration for how FHIR resources can be searched. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `consent_config` | String |  | Optional. Specifies whether this store has consent enforcement. Not available for DSTU2 FHIR version due to absence of Consent resources. Not supported for R5 FHIR version. |
| `version` | String |  | Required. Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store. |
| `enable_update_create` | bool |  | Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. Defaults to false. |
| `parent` | String | ✅ | Required. The name of the dataset this FHIR store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_referential_integrity` | bool | Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist. |
| `notification_configs` | Vec<String> | Specifies where and whether to send notifications upon changes to a Fhir store. |
| `validation_config` | String | Configuration for how to validate incoming FHIR resources against configured profiles. |
| `notification_config` | String | Deprecated. Use `notification_configs` instead. If non-empty, publish all resource modifications of this FHIR store to this destination. The Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, "action":"CreateResource". Not supported in R5. Use `notification_configs` instead. |
| `stream_configs` | Vec<String> | A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination. |
| `default_search_handling_strict` | bool | If true, overrides the default search behavior for this FHIR store to `handling=strict` which returns an error for unrecognized search parameters. If false, uses the FHIR specification default `handling=lenient` which ignores unrecognized search parameters. The handling can always be changed from the default on an individual API call by setting the HTTP header `Prefer: handling=strict` or `Prefer: handling=lenient`. Defaults to false. |
| `disable_resource_versioning` | bool | Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions. Defaults to false. |
| `complex_data_type_reference_parsing` | String | Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Cannot be disabled in R5. |
| `bulk_export_gcs_destination` | String | Optional. FHIR bulk export exports resources to the specified Cloud Storage destination. A Cloud Storage destination is a URI for a Cloud Storage directory where result files will be written. Only used in the spec-defined bulk $export methods. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM role on the destination. |
| `enable_history_modifications` | bool | Optional. Whether to allow the [ImportResourcesHistory] and [ExecuteBundle] APIs to accept history bundles, and directly insert and overwrite historical resource versions into the FHIR store. Changing resource histories creates resource interactions that have occurred in the past which clients might not allow. If set to false, [ImportResourcesHistory] and [ExecuteBundle] requests will return errors. |
| `name` | String | Output only. Identifier. Resource name of the FHIR store, of the form `projects/{project_id}/locations/{location}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. |
| `search_config` | String | Configuration for how FHIR resources can be searched. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `consent_config` | String | Optional. Specifies whether this store has consent enforcement. Not available for DSTU2 FHIR version due to absence of Consent resources. Not supported for R5 FHIR version. |
| `version` | String | Required. Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store. |
| `enable_update_create` | bool | Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources. Defaults to false. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir_store
fhir_store = provider.healthcare_api.Fhir_store {
    parent = "value"  # Required. The name of the dataset this FHIR store belongs to.
}

# Access fhir_store outputs
fhir_store_id = fhir_store.id
fhir_store_disable_referential_integrity = fhir_store.disable_referential_integrity
fhir_store_notification_configs = fhir_store.notification_configs
fhir_store_validation_config = fhir_store.validation_config
fhir_store_notification_config = fhir_store.notification_config
fhir_store_stream_configs = fhir_store.stream_configs
fhir_store_default_search_handling_strict = fhir_store.default_search_handling_strict
fhir_store_disable_resource_versioning = fhir_store.disable_resource_versioning
fhir_store_complex_data_type_reference_parsing = fhir_store.complex_data_type_reference_parsing
fhir_store_bulk_export_gcs_destination = fhir_store.bulk_export_gcs_destination
fhir_store_enable_history_modifications = fhir_store.enable_history_modifications
fhir_store_name = fhir_store.name
fhir_store_search_config = fhir_store.search_config
fhir_store_labels = fhir_store.labels
fhir_store_consent_config = fhir_store.consent_config
fhir_store_version = fhir_store.version
fhir_store_enable_update_create = fhir_store.enable_update_create
```

---


### Hl7_v2_store

Creates a new HL7v2 store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parser_config` | String |  | The configuration for the parser. It determines how the server parses the messages. |
| `notification_config` | String |  | The notification destination all messages (both Ingest & Create) are published on. Only the message name is sent as part of the notification. If this is unset, no notifications are sent. Supplied by the client. |
| `notification_configs` | Vec<String> |  | A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `reject_duplicate_message` | bool |  | Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection. |
| `name` | String |  | Identifier. Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |
| `parent` | String | ✅ | Required. The name of the dataset this HL7v2 store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parser_config` | String | The configuration for the parser. It determines how the server parses the messages. |
| `notification_config` | String | The notification destination all messages (both Ingest & Create) are published on. Only the message name is sent as part of the notification. If this is unset, no notifications are sent. Supplied by the client. |
| `notification_configs` | Vec<String> | A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `reject_duplicate_message` | bool | Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection. |
| `name` | String | Identifier. Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hl7_v2_store
hl7_v2_store = provider.healthcare_api.Hl7_v2_store {
    parent = "value"  # Required. The name of the dataset this HL7v2 store belongs to.
}

# Access hl7_v2_store outputs
hl7_v2_store_id = hl7_v2_store.id
hl7_v2_store_parser_config = hl7_v2_store.parser_config
hl7_v2_store_notification_config = hl7_v2_store.notification_config
hl7_v2_store_notification_configs = hl7_v2_store.notification_configs
hl7_v2_store_labels = hl7_v2_store.labels
hl7_v2_store_reject_duplicate_message = hl7_v2_store.reject_duplicate_message
hl7_v2_store_name = hl7_v2_store.name
```

---


### Dataset

Creates a new health dataset. Results are returned through the Operation interface which returns either an `Operation.response` which contains a Dataset or `Operation.error`. The metadata field type is OperationMetadata.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_zone` | String |  | The default timezone used by this dataset. Must be a either a valid IANA time zone name such as "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified. |
| `satisfies_pzi` | bool |  | Output only. Whether the dataset satisfies zone isolation. |
| `satisfies_pzs` | bool |  | Output only. Whether the dataset satisfies zone separation. |
| `name` | String |  | Identifier. Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all of its sub-resources will be secured by this key. If empty, the Dataset is secured by the default Google encryption key. |
| `parent` | String | ✅ | Required. The name of the project where the server creates the dataset. For example, `projects/{project_id}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_zone` | String | The default timezone used by this dataset. Must be a either a valid IANA time zone name such as "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified. |
| `satisfies_pzi` | bool | Output only. Whether the dataset satisfies zone isolation. |
| `satisfies_pzs` | bool | Output only. Whether the dataset satisfies zone separation. |
| `name` | String | Identifier. Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all of its sub-resources will be secured by this key. If empty, the Dataset is secured by the default Google encryption key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.healthcare_api.Dataset {
    parent = "value"  # Required. The name of the project where the server creates the dataset. For example, `projects/{project_id}/locations/{location_id}`.
}

# Access dataset outputs
dataset_id = dataset.id
dataset_time_zone = dataset.time_zone
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_name = dataset.name
dataset_encryption_spec = dataset.encryption_spec
```

---


### Consent

Creates a new Consent in the parent consent store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policies` | Vec<String> |  | Optional. Represents a user's consent in terms of the resources that can be accessed and under what conditions. |
| `expire_time` | String |  | Timestamp in UTC of when this Consent is considered expired. |
| `metadata` | HashMap<String, String> |  | Optional. User-supplied key-value pairs used to organize Consent resources. Metadata keys must: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - begin with a letter - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes Metadata values must be: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes No more than 64 metadata entries can be associated with a given consent. |
| `revision_create_time` | String |  | Output only. The timestamp that the revision was created. |
| `ttl` | String |  | Input only. The time to live for this Consent from when it is created. |
| `state` | String |  | Required. Indicates the current state of this Consent. |
| `revision_id` | String |  | Output only. The revision ID of the Consent. The format is an 8-character hexadecimal string. Refer to a specific revision of a Consent by appending `@{revision_id}` to the Consent's resource name. |
| `consent_artifact` | String |  | Required. The resource name of the Consent artifact that contains proof of the end user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. |
| `name` | String |  | Identifier. Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation. |
| `user_id` | String |  | Required. User's UUID provided by the client. |
| `parent` | String | ✅ | Required. Name of the consent store. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | Vec<String> | Optional. Represents a user's consent in terms of the resources that can be accessed and under what conditions. |
| `expire_time` | String | Timestamp in UTC of when this Consent is considered expired. |
| `metadata` | HashMap<String, String> | Optional. User-supplied key-value pairs used to organize Consent resources. Metadata keys must: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - begin with a letter - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes Metadata values must be: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes No more than 64 metadata entries can be associated with a given consent. |
| `revision_create_time` | String | Output only. The timestamp that the revision was created. |
| `ttl` | String | Input only. The time to live for this Consent from when it is created. |
| `state` | String | Required. Indicates the current state of this Consent. |
| `revision_id` | String | Output only. The revision ID of the Consent. The format is an 8-character hexadecimal string. Refer to a specific revision of a Consent by appending `@{revision_id}` to the Consent's resource name. |
| `consent_artifact` | String | Required. The resource name of the Consent artifact that contains proof of the end user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. |
| `name` | String | Identifier. Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation. |
| `user_id` | String | Required. User's UUID provided by the client. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consent
consent = provider.healthcare_api.Consent {
    parent = "value"  # Required. Name of the consent store.
}

# Access consent outputs
consent_id = consent.id
consent_policies = consent.policies
consent_expire_time = consent.expire_time
consent_metadata = consent.metadata
consent_revision_create_time = consent.revision_create_time
consent_ttl = consent.ttl
consent_state = consent.state
consent_revision_id = consent.revision_id
consent_consent_artifact = consent.consent_artifact
consent_name = consent.name
consent_user_id = consent.user_id
```

---


### Dicom_store

Creates a new DICOM store within the parent dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_configs` | Vec<String> |  | Optional. Specifies where and whether to send notifications upon changes to a DICOM store. |
| `notification_config` | String |  | Notification destination for new DICOM instances. Supplied by the client. |
| `stream_configs` | Vec<String> |  | Optional. A list of streaming configs used to configure the destination of streaming exports for every DICOM instance insertion in this DICOM store. After a new config is added to `stream_configs`, DICOM instance insertions are streamed to the new destination. When a config is removed from `stream_configs`, the server stops streaming to that destination. Each config must contain a unique destination. |
| `name` | String |  | Identifier. Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `parent` | String | ✅ | Required. The name of the dataset this DICOM store belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_configs` | Vec<String> | Optional. Specifies where and whether to send notifications upon changes to a DICOM store. |
| `notification_config` | String | Notification destination for new DICOM instances. Supplied by the client. |
| `stream_configs` | Vec<String> | Optional. A list of streaming configs used to configure the destination of streaming exports for every DICOM instance insertion in this DICOM store. After a new config is added to `stream_configs`, DICOM instance insertions are streamed to the new destination. When a config is removed from `stream_configs`, the server stops streaming to that destination. Each config must contain a unique destination. |
| `name` | String | Identifier. Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dicom_store
dicom_store = provider.healthcare_api.Dicom_store {
    parent = "value"  # Required. The name of the dataset this DICOM store belongs to.
}

# Access dicom_store outputs
dicom_store_id = dicom_store.id
dicom_store_notification_configs = dicom_store.notification_configs
dicom_store_notification_config = dicom_store.notification_config
dicom_store_stream_configs = dicom_store.stream_configs
dicom_store_name = dicom_store.name
dicom_store_labels = dicom_store.labels
```

---


### Fhir

Creates a FHIR resource. Implements the FHIR standard create interaction ([DSTU2](https://hl7.org/fhir/DSTU2/http.html#create), [STU3](https://hl7.org/fhir/STU3/http.html#create), [R4](https://hl7.org/fhir/R4/http.html#create)), [R5](https://hl7.org/fhir/R5/http.html#create)), which creates a new resource with a server-assigned resource ID. Also supports the FHIR standard conditional create interaction ([DSTU2](https://hl7.org/fhir/DSTU2/http.html#ccreate), [STU3](https://hl7.org/fhir/STU3/http.html#ccreate), [R4](https://hl7.org/fhir/R4/http.html#ccreate)), [R5](https://hl7.org/fhir/R5/http.html#ccreate)), specified by supplying an `If-None-Exist` header containing a FHIR search query. If no resources match this search query, the server processes the create operation as normal. The request body must contain a JSON-encoded FHIR resource, and the request headers must contain `Content-Type: application/fhir+json`. On success, the response body contains a JSON-encoded representation of the resource as it was created on the server, including the server-assigned resource ID and version ID. Errors generated by the FHIR store contain a JSON-encoded `OperationOutcome` resource describing the reason for the error. If the request cannot be mapped to a valid API method on a FHIR store, a generic GCP error might be returned instead. For samples that show how to call `create`, see [Creating a FHIR resource](https://cloud.google.com/healthcare/docs/how-tos/fhir-resources#creating_a_fhir_resource).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `type` | String | ✅ | Required. The FHIR resource type to create, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](https://hl7.org/fhir/DSTU2/resourcelist.html), [STU3](https://hl7.org/fhir/STU3/resourcelist.html), [R4](https://hl7.org/fhir/R4/resourcelist.html), [R5](https://hl7.org/fhir/R5/resourcelist.html)). Must match the resource type in the provided content. |
| `parent` | String | ✅ | Required. The name of the FHIR store this resource belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fhir
fhir = provider.healthcare_api.Fhir {
    type = "value"  # Required. The FHIR resource type to create, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](https://hl7.org/fhir/DSTU2/resourcelist.html), [STU3](https://hl7.org/fhir/STU3/resourcelist.html), [R4](https://hl7.org/fhir/R4/resourcelist.html), [R5](https://hl7.org/fhir/R5/resourcelist.html)). Must match the resource type in the provided content.
    parent = "value"  # Required. The name of the FHIR store this resource belongs to.
}

# Access fhir outputs
fhir_id = fhir.id
fhir_data = fhir.data
fhir_extensions = fhir.extensions
fhir_content_type = fhir.content_type
```

---


### Bulkdata

Returns uncompressed, unencoded bytes representing the referenced bulkdata tag from an instance. See [Retrieve Transaction](https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveBulkdata, see [Bulkdata resources](https://cloud.google.com/healthcare/docs/dicom#bulkdata-resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveBulkdata, see [Retrieve bulkdata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-bulkdata).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bulkdata outputs
bulkdata_id = bulkdata.id
bulkdata_data = bulkdata.data
bulkdata_extensions = bulkdata.extensions
bulkdata_content_type = bulkdata.content_type
```

---


### Nlp

Analyze heathcare entity in a document. Its response includes the recognized entity mentions and the relationships between them. AnalyzeEntities uses context aware models to detect entities. This method can only analyze documents written in English.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alternative_output_format` | String |  | Optional. Alternative output format to be generated based on the results of analysis. |
| `document_content` | String |  | document_content is a document to be annotated. |
| `licensed_vocabularies` | Vec<String> |  | A list of licensed vocabularies to use in the request, in addition to the default unlicensed vocabularies. |
| `nlp_service` | String | ✅ | The resource name of the service of the form: "projects/{project_id}/locations/{location_id}/services/nlp". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create nlp
nlp = provider.healthcare_api.Nlp {
    nlp_service = "value"  # The resource name of the service of the form: "projects/{project_id}/locations/{location_id}/services/nlp".
}

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

# Create operation
operation = provider.healthcare_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple consent resources
consent_0 = provider.healthcare_api.Consent {
    parent = "value-0"
}
consent_1 = provider.healthcare_api.Consent {
    parent = "value-1"
}
consent_2 = provider.healthcare_api.Consent {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    consent = provider.healthcare_api.Consent {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Healthcare_api Documentation](https://cloud.google.com/healthcare_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
