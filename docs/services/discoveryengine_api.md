# Discoveryengine_api Service



**Resources**: 112

---

## Overview

The discoveryengine_api service provides access to 112 resource types:

- [Cmek_config](#cmek_config) [RUD]
- [Answer](#answer) [R]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Document](#document) [CRUD]
- [Schema](#schema) [CRUD]
- [Operation](#operation) [CR]
- [Location](#location) [CRU]
- [Conversation](#conversation) [CRUD]
- [Control](#control) [CRUD]
- [Sitemap](#sitemap) [CRD]
- [Media](#media) [R]
- [User_license](#user_license) [R]
- [Collection](#collection) [RUD]
- [Completion_suggestion](#completion_suggestion) [C]
- [Session](#session) [CRUD]
- [License_config](#license_config) [CRU]
- [Serving_config](#serving_config) [CRU]
- [Project](#project) [C]
- [User_store](#user_store) [CRUD]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Grounding_config](#grounding_config) [C]
- [Branche](#branche) [R]
- [Completion_config](#completion_config) [C]
- [Engine](#engine) [CRUD]
- [Data_store](#data_store) [CRUD]
- [Custom_model](#custom_model) [R]
- [Target_site](#target_site) [CRUD]
- [Assistant](#assistant) [CRU]
- [Ranking_config](#ranking_config) [C]
- [User_event](#user_event) [CR]
- [Site_search_engine](#site_search_engine) [CR]
- [Site_search_engine](#site_search_engine) [CR]
- [Grounding_config](#grounding_config) [C]
- [Requirement](#requirement) [C]
- [Operation](#operation) [CR]
- [Connector_run](#connector_run) [R]
- [Engine](#engine) [CRUD]
- [Evaluation](#evaluation) [CR]
- [Sitemap](#sitemap) [CRD]
- [Completion_config](#completion_config) [C]
- [Session](#session) [CRUD]
- [Answer](#answer) [R]
- [Source](#source) [CR]
- [Ranking_config](#ranking_config) [C]
- [Serving_config](#serving_config) [CRU]
- [Control](#control) [CRUD]
- [Project](#project) [CRU]
- [Canned_querie](#canned_querie) [CRUD]
- [Sample_querie](#sample_querie) [CRUD]
- [Notebook](#notebook) [CR]
- [Branche](#branche) [R]
- [Collection](#collection) [RUD]
- [Agent](#agent) [CRUD]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Schema](#schema) [CRUD]
- [User_store](#user_store) [CRUD]
- [User_event](#user_event) [CR]
- [File](#file) [CR]
- [Cmek_config](#cmek_config) [RUD]
- [User_license](#user_license) [R]
- [Audio_overview](#audio_overview) [CD]
- [Location](#location) [CRU]
- [Target_site](#target_site) [CRUD]
- [Billing_account_license_config](#billing_account_license_config) [CR]
- [Authorization](#authorization) [CRUD]
- [Conversation](#conversation) [CRUD]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Media](#media) [CR]
- [Data_store](#data_store) [CRUD]
- [Data_connector](#data_connector) [CR]
- [Assistant](#assistant) [CRU]
- [Sample_query_set](#sample_query_set) [CRUD]
- [Analytic](#analytic) [C]
- [Widget_config](#widget_config) [RU]
- [Chunk](#chunk) [R]
- [License_config](#license_config) [CRU]
- [Completion_suggestion](#completion_suggestion) [C]
- [Document](#document) [CRUD]
- [Custom_model](#custom_model) [R]
- [Session](#session) [CRUD]
- [Evaluation](#evaluation) [CR]
- [Sitemap](#sitemap) [CRD]
- [Target_site](#target_site) [CRUD]
- [Sample_query_set](#sample_query_set) [CRUD]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Control](#control) [CRUD]
- [Data_store](#data_store) [CRUD]
- [Serving_config](#serving_config) [CRU]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Branche](#branche) [R]
- [User_license](#user_license) [R]
- [Cmek_config](#cmek_config) [RUD]
- [Operation](#operation) [CR]
- [Answer](#answer) [R]
- [Document](#document) [CRUD]
- [License_config](#license_config) [CRU]
- [Sample_querie](#sample_querie) [CRUD]
- [User_store](#user_store) [CRUD]
- [Conversation](#conversation) [CRUD]
- [Location](#location) [CRU]
- [Media](#media) [R]
- [Assistant](#assistant) [CRU]
- [Project](#project) [C]
- [Ranking_config](#ranking_config) [C]
- [Completion_config](#completion_config) [C]
- [Custom_model](#custom_model) [R]
- [Engine](#engine) [CRUD]
- [Site_search_engine](#site_search_engine) [CR]
- [User_event](#user_event) [CR]
- [Completion_suggestion](#completion_suggestion) [C]
- [Grounding_config](#grounding_config) [C]
- [Schema](#schema) [CRUD]

---

## Resources


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String |  | Output only. The states of the CmekConfig. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String | Output only. The states of the CmekConfig. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cmek_config outputs
cmek_config_id = cmek_config.id
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_state = cmek_config.state
cmek_config_is_default = cmek_config.is_default
cmek_config_name = cmek_config.name
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_kms_key = cmek_config.kms_key
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
```

---


### Answer

Gets a Answer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `steps` | Vec<String> | Answer generation steps. |
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `query_understanding_info` | String | Query understanding information. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |
| `references` | Vec<String> | References. |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `citations` | Vec<String> | Citations. |
| `state` | String | The state of the answer generation. |
| `related_questions` | Vec<String> | Suggested related questions. |
| `answer_text` | String | The textual answer. |
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |
| `create_time` | String | Output only. Answer creation timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access answer outputs
answer_id = answer.id
answer_safety_ratings = answer.safety_ratings
answer_steps = answer.steps
answer_grounding_score = answer.grounding_score
answer_query_understanding_info = answer.query_understanding_info
answer_name = answer.name
answer_references = answer.references
answer_complete_time = answer.complete_time
answer_citations = answer.citations
answer_state = answer.state
answer_related_questions = answer.related_questions
answer_answer_text = answer.answer_text
answer_answer_skipped_reasons = answer.answer_skipped_reasons
answer_grounding_supports = answer.grounding_supports
answer_create_time = answer.create_time
```

---


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identity_mapping_store
identity_mapping_store = provider.discoveryengine_api.Identity_mapping_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access identity_mapping_store outputs
identity_mapping_store_id = identity_mapping_store.id
identity_mapping_store_name = identity_mapping_store.name
identity_mapping_store_kms_key_name = identity_mapping_store.kms_key_name
identity_mapping_store_cmek_config = identity_mapping_store.cmek_config
```

---


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String |  | Access control information for the document. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String | Access control information for the document. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `schema_id` | String | The identifier of the schema located in the same data store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.discoveryengine_api.Document {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
}

# Access document outputs
document_id = document.id
document_struct_data = document.struct_data
document_id = document.id
document_name = document.name
document_content = document.content
document_index_status = document.index_status
document_json_data = document.json_data
document_acl_info = document.acl_info
document_index_time = document.index_time
document_parent_document_id = document.parent_document_id
document_derived_struct_data = document.derived_struct_data
document_schema_id = document.schema_id
```

---


### Schema

Creates a Schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `json_schema` | String |  | The JSON representation of the schema. |
| `name` | String |  | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> |  | The structured representation of the schema. |
| `parent` | String | ✅ | Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `json_schema` | String | The JSON representation of the schema. |
| `name` | String | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> | The structured representation of the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.discoveryengine_api.Schema {
    parent = "value"  # Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
}

# Access schema outputs
schema_id = schema.id
schema_json_schema = schema.json_schema
schema_name = schema.name
schema_struct_schema = schema.struct_schema
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.discoveryengine_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Location

Creates a Collection and sets up the DataConnector for it. To stop a DataConnector after setup, use the CollectionService.DeleteCollection method.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `incremental_refresh_interval` | String |  | Optional. The refresh interval specifically for incremental data syncs. If unset, incremental syncs will use the default from env, set to 3hrs. The minimum is 30 minutes and maximum is 7 days. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `auto_run_disabled` | bool |  | Optional. Indicates whether the connector is disabled for auto run. It can be used to pause periodical and real time sync. Update: with the introduction of incremental_sync_disabled, auto_run_disabled is used to pause/disable only full syncs |
| `private_connectivity_project_id` | String |  | Output only. The tenant project ID associated with private connectivity connectors. This project must be allowlisted by in order for the connector to function. |
| `realtime_sync_config` | String |  | Optional. The configuration for realtime sync. |
| `refresh_interval` | String |  | Required. The refresh interval for data sync. If duration is set to 0, the data will be synced in real time. The streaming feature is not supported yet. The minimum is 30 minutes and maximum is 7 days. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `static_ip_enabled` | bool |  | Optional. Whether customer has enabled static IP addresses for this connector. |
| `errors` | Vec<String> |  | Output only. The errors from initialization or from the latest connector run. |
| `action_config` | String |  | Optional. Action configurations to make the connector support actions. |
| `json_params` | String |  | Required data connector parameters in json string format. |
| `identity_refresh_interval` | String |  | The refresh interval to sync the Access Control List information for the documents ingested by this connector. If not set, the access control list will be refreshed at the default interval of 30 minutes. The identity refresh interval can be at least 30 minutes and at most 7 days. |
| `blocking_reasons` | Vec<String> |  | Output only. User actions that must be completed before the connector can start syncing data. |
| `alert_policy_configs` | Vec<String> |  | Optional. The connector level alert config. |
| `connector_modes` | Vec<String> |  | Optional. The modes enabled for this connector. Default state is CONNECTOR_MODE_UNSPECIFIED. |
| `acl_enabled` | bool |  | Optional. Whether the connector will be created with an ACL config. Currently this field only affects Cloud Storage and BigQuery connectors. |
| `create_time` | String |  | Output only. Timestamp the DataConnector was created at. |
| `hybrid_ingestion_disabled` | bool |  | Optional. If the connector is a hybrid connector, determines whether ingestion is enabled and appropriate resources are provisioned during connector creation. If the connector is not a hybrid connector, this field is ignored. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect the DataStores managed by this connector. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStores created by this connector will be protected by the KMS key. |
| `update_time` | String |  | Output only. Timestamp the DataConnector was last updated. |
| `name` | String |  | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |
| `bap_config` | String |  | Optional. The configuration for establishing a BAP connection. |
| `incremental_sync_disabled` | bool |  | Optional. Indicates whether incremental syncs are paused for this connector. This is independent of auto_run_disabled. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled, i.e. set to true. |
| `last_sync_time` | String |  | Output only. For periodic connectors only, the last time a data sync was completed. |
| `next_sync_time` | String |  | Defines the scheduled time for the next data synchronization. This field requires hour , minute, and time_zone from the [IANA Time Zone Database](https://www.iana.org/time-zones). This is utilized when the data connector has a refresh interval greater than 1 day. When the hours or minutes are not specified, we will assume a sync time of 0:00. The user must provide a time zone to avoid ambiguity. |
| `end_user_config` | String |  | Optional. Any params and credentials used specifically for EUA connectors. |
| `entities` | Vec<String> |  | List of entities from the connected data source to ingest. |
| `identity_schedule_config` | String |  | The configuration for the identity data synchronization runs. This contains the refresh interval to sync the Access Control List information for the documents ingested by this connector. |
| `params` | HashMap<String, String> |  | Required data connector parameters in structured json format. |
| `connector_type` | String |  | Output only. The type of connector. Each source can only map to one type. For example, salesforce, confluence and jira have THIRD_PARTY connector type. It is not mutable once set by system. |
| `destination_configs` | Vec<String> |  | Optional. Any target destinations used to connect to third-party services. |
| `sync_mode` | String |  | The data synchronization mode supported by the data connector. |
| `create_eua_saas` | bool |  | Optional. Whether the END USER AUTHENTICATION connector is created in SaaS. |
| `federated_config` | String |  | Optional. Any params and credentials used specifically for hybrid connectors supporting FEDERATED mode. This field should only be set if the connector is a hybrid connector and we want to enable FEDERATED mode. |
| `state` | String |  | Output only. State of the connector. |
| `static_ip_addresses` | Vec<String> |  | Output only. The static IP addresses used by this connector. |
| `remove_param_keys` | Vec<String> |  | Optional. Specifies keys to be removed from the 'params' field. This is only active when 'params' is included in the 'update_mask' in an UpdateDataConnectorRequest. Deletion takes precedence if a key is both in 'remove_param_keys' and present in the 'params' field of the request. |
| `realtime_state` | String |  | Output only. real-time sync state |
| `latest_pause_time` | String |  | Output only. The most recent timestamp when this DataConnector was paused, affecting all functionalities such as data synchronization. Pausing a connector has the following effects: - All functionalities, including data synchronization, are halted. - Any ongoing data synchronization job will be canceled. - No future data synchronization runs will be scheduled nor can be triggered. |
| `action_state` | String |  | Output only. State of the action connector. This reflects whether the action connector is initializing, active or has encountered errors. |
| `data_source` | String |  | Required. The name of the data source. Supported values: `salesforce`, `jira`, `confluence`, `bigquery`. |
| `parent` | String | ✅ | Required. The parent of Collection, in the format of `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `idp_config` | String | Identity provider config. |
| `name` | String | Immutable. The full resource name of the acl configuration. Format: `projects/{project}/locations/{location}/aclConfig`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.discoveryengine_api.Location {
    parent = "value"  # Required. The parent of Collection, in the format of `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_idp_config = location.idp_config
location_name = location.name
```

---


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `end_time` | String |  | Output only. The time the conversation finished. |
| `state` | String |  | The state of the Conversation. |
| `messages` | Vec<String> |  | Conversation messages. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `start_time` | String |  | Output only. The time the conversation started. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `end_time` | String | Output only. The time the conversation finished. |
| `state` | String | The state of the Conversation. |
| `messages` | Vec<String> | Conversation messages. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `start_time` | String | Output only. The time the conversation started. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.discoveryengine_api.Conversation {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access conversation outputs
conversation_id = conversation.id
conversation_name = conversation.name
conversation_end_time = conversation.end_time
conversation_state = conversation.state
conversation_messages = conversation.messages
conversation_user_pseudo_id = conversation.user_pseudo_id
conversation_start_time = conversation.start_time
```

---


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `boost_action` | String |  | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `redirect_action` | String |  | Defines a redirect-type control. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `boost_action` | String | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `solution_type` | String | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `redirect_action` | String | Defines a redirect-type control. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.discoveryengine_api.Control {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
}

# Access control outputs
control_id = control.id
control_name = control.name
control_conditions = control.conditions
control_display_name = control.display_name
control_filter_action = control.filter_action
control_synonyms_action = control.synonyms_action
control_boost_action = control.boost_action
control_associated_serving_config_ids = control.associated_serving_config_ids
control_solution_type = control.solution_type
control_redirect_action = control.redirect_action
control_promote_action = control.promote_action
control_use_cases = control.use_cases
```

---


### Sitemap

Creates a Sitemap.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | Public URI for the sitemap, e.g. `www.example.com/sitemap.xml`. |
| `name` | String |  | Output only. The fully qualified resource name of the sitemap. `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine/sitemaps/*` The `sitemap_id` suffix is system-generated. |
| `create_time` | String |  | Output only. The sitemap's creation time. |
| `parent` | String | ✅ | Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sitemaps_metadata` | Vec<String> | List of Sitemaps fetched. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sitemap
sitemap = provider.discoveryengine_api.Sitemap {
    parent = "value"  # Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
}

# Access sitemap outputs
sitemap_id = sitemap.id
sitemap_sitemaps_metadata = sitemap.sitemaps_metadata
```

---


### Media

Downloads a file from the session.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `content_type` | String | MIME type of the data |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `reference_type` | String | Describes what the field reference contains. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `download_parameters` | String | Parameters for a media download. |
| `filename` | String | Original file name |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `length` | String | Size of the data, in bytes |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `token` | String | A unique fingerprint/version id for the media data |
| `path` | String | Path to the data, set if reference_type is PATH |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access media outputs
media_id = media.id
media_hash_verified = media.hash_verified
media_composite_media = media.composite_media
media_diff_checksums_response = media.diff_checksums_response
media_content_type = media.content_type
media_diff_upload_request = media.diff_upload_request
media_md5_hash = media.md5_hash
media_media_id = media.media_id
media_reference_type = media.reference_type
media_algorithm = media.algorithm
media_diff_upload_response = media.diff_upload_response
media_cosmo_binary_reference = media.cosmo_binary_reference
media_bigstore_object_ref = media.bigstore_object_ref
media_download_parameters = media.download_parameters
media_filename = media.filename
media_content_type_info = media.content_type_info
media_sha1_hash = media.sha1_hash
media_inline = media.inline
media_diff_version_response = media.diff_version_response
media_length = media.length
media_sha256_hash = media.sha256_hash
media_blobstore2_info = media.blobstore2_info
media_token = media.token
media_path = media.path
media_diff_download_response = media.diff_download_response
media_hash = media.hash
media_object_id = media.object_id
media_blob_ref = media.blob_ref
media_crc32c_hash = media.crc32c_hash
media_is_potential_retry = media.is_potential_retry
media_timestamp = media.timestamp
```

---


### User_license

Lists the User Licenses.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `user_licenses` | Vec<String> | All the customer's UserLicenses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_license outputs
user_license_id = user_license.id
user_license_next_page_token = user_license.next_page_token
user_license_user_licenses = user_license.user_licenses
```

---


### Collection

Gets the DataConnector. DataConnector is a singleton resource for each Collection.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `incremental_refresh_interval` | String |  | Optional. The refresh interval specifically for incremental data syncs. If unset, incremental syncs will use the default from env, set to 3hrs. The minimum is 30 minutes and maximum is 7 days. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `auto_run_disabled` | bool |  | Optional. Indicates whether the connector is disabled for auto run. It can be used to pause periodical and real time sync. Update: with the introduction of incremental_sync_disabled, auto_run_disabled is used to pause/disable only full syncs |
| `private_connectivity_project_id` | String |  | Output only. The tenant project ID associated with private connectivity connectors. This project must be allowlisted by in order for the connector to function. |
| `realtime_sync_config` | String |  | Optional. The configuration for realtime sync. |
| `refresh_interval` | String |  | Required. The refresh interval for data sync. If duration is set to 0, the data will be synced in real time. The streaming feature is not supported yet. The minimum is 30 minutes and maximum is 7 days. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `static_ip_enabled` | bool |  | Optional. Whether customer has enabled static IP addresses for this connector. |
| `errors` | Vec<String> |  | Output only. The errors from initialization or from the latest connector run. |
| `action_config` | String |  | Optional. Action configurations to make the connector support actions. |
| `json_params` | String |  | Required data connector parameters in json string format. |
| `identity_refresh_interval` | String |  | The refresh interval to sync the Access Control List information for the documents ingested by this connector. If not set, the access control list will be refreshed at the default interval of 30 minutes. The identity refresh interval can be at least 30 minutes and at most 7 days. |
| `blocking_reasons` | Vec<String> |  | Output only. User actions that must be completed before the connector can start syncing data. |
| `alert_policy_configs` | Vec<String> |  | Optional. The connector level alert config. |
| `connector_modes` | Vec<String> |  | Optional. The modes enabled for this connector. Default state is CONNECTOR_MODE_UNSPECIFIED. |
| `acl_enabled` | bool |  | Optional. Whether the connector will be created with an ACL config. Currently this field only affects Cloud Storage and BigQuery connectors. |
| `create_time` | String |  | Output only. Timestamp the DataConnector was created at. |
| `hybrid_ingestion_disabled` | bool |  | Optional. If the connector is a hybrid connector, determines whether ingestion is enabled and appropriate resources are provisioned during connector creation. If the connector is not a hybrid connector, this field is ignored. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect the DataStores managed by this connector. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStores created by this connector will be protected by the KMS key. |
| `update_time` | String |  | Output only. Timestamp the DataConnector was last updated. |
| `name` | String |  | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |
| `bap_config` | String |  | Optional. The configuration for establishing a BAP connection. |
| `incremental_sync_disabled` | bool |  | Optional. Indicates whether incremental syncs are paused for this connector. This is independent of auto_run_disabled. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled, i.e. set to true. |
| `last_sync_time` | String |  | Output only. For periodic connectors only, the last time a data sync was completed. |
| `next_sync_time` | String |  | Defines the scheduled time for the next data synchronization. This field requires hour , minute, and time_zone from the [IANA Time Zone Database](https://www.iana.org/time-zones). This is utilized when the data connector has a refresh interval greater than 1 day. When the hours or minutes are not specified, we will assume a sync time of 0:00. The user must provide a time zone to avoid ambiguity. |
| `end_user_config` | String |  | Optional. Any params and credentials used specifically for EUA connectors. |
| `entities` | Vec<String> |  | List of entities from the connected data source to ingest. |
| `identity_schedule_config` | String |  | The configuration for the identity data synchronization runs. This contains the refresh interval to sync the Access Control List information for the documents ingested by this connector. |
| `params` | HashMap<String, String> |  | Required data connector parameters in structured json format. |
| `connector_type` | String |  | Output only. The type of connector. Each source can only map to one type. For example, salesforce, confluence and jira have THIRD_PARTY connector type. It is not mutable once set by system. |
| `destination_configs` | Vec<String> |  | Optional. Any target destinations used to connect to third-party services. |
| `sync_mode` | String |  | The data synchronization mode supported by the data connector. |
| `create_eua_saas` | bool |  | Optional. Whether the END USER AUTHENTICATION connector is created in SaaS. |
| `federated_config` | String |  | Optional. Any params and credentials used specifically for hybrid connectors supporting FEDERATED mode. This field should only be set if the connector is a hybrid connector and we want to enable FEDERATED mode. |
| `state` | String |  | Output only. State of the connector. |
| `static_ip_addresses` | Vec<String> |  | Output only. The static IP addresses used by this connector. |
| `remove_param_keys` | Vec<String> |  | Optional. Specifies keys to be removed from the 'params' field. This is only active when 'params' is included in the 'update_mask' in an UpdateDataConnectorRequest. Deletion takes precedence if a key is both in 'remove_param_keys' and present in the 'params' field of the request. |
| `realtime_state` | String |  | Output only. real-time sync state |
| `latest_pause_time` | String |  | Output only. The most recent timestamp when this DataConnector was paused, affecting all functionalities such as data synchronization. Pausing a connector has the following effects: - All functionalities, including data synchronization, are halted. - Any ongoing data synchronization job will be canceled. - No future data synchronization runs will be scheduled nor can be triggered. |
| `action_state` | String |  | Output only. State of the action connector. This reflects whether the action connector is initializing, active or has encountered errors. |
| `data_source` | String |  | Required. The name of the data source. Supported values: `salesforce`, `jira`, `confluence`, `bigquery`. |
| `name` | String | ✅ | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `incremental_refresh_interval` | String | Optional. The refresh interval specifically for incremental data syncs. If unset, incremental syncs will use the default from env, set to 3hrs. The minimum is 30 minutes and maximum is 7 days. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `auto_run_disabled` | bool | Optional. Indicates whether the connector is disabled for auto run. It can be used to pause periodical and real time sync. Update: with the introduction of incremental_sync_disabled, auto_run_disabled is used to pause/disable only full syncs |
| `private_connectivity_project_id` | String | Output only. The tenant project ID associated with private connectivity connectors. This project must be allowlisted by in order for the connector to function. |
| `realtime_sync_config` | String | Optional. The configuration for realtime sync. |
| `refresh_interval` | String | Required. The refresh interval for data sync. If duration is set to 0, the data will be synced in real time. The streaming feature is not supported yet. The minimum is 30 minutes and maximum is 7 days. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `static_ip_enabled` | bool | Optional. Whether customer has enabled static IP addresses for this connector. |
| `errors` | Vec<String> | Output only. The errors from initialization or from the latest connector run. |
| `action_config` | String | Optional. Action configurations to make the connector support actions. |
| `json_params` | String | Required data connector parameters in json string format. |
| `identity_refresh_interval` | String | The refresh interval to sync the Access Control List information for the documents ingested by this connector. If not set, the access control list will be refreshed at the default interval of 30 minutes. The identity refresh interval can be at least 30 minutes and at most 7 days. |
| `blocking_reasons` | Vec<String> | Output only. User actions that must be completed before the connector can start syncing data. |
| `alert_policy_configs` | Vec<String> | Optional. The connector level alert config. |
| `connector_modes` | Vec<String> | Optional. The modes enabled for this connector. Default state is CONNECTOR_MODE_UNSPECIFIED. |
| `acl_enabled` | bool | Optional. Whether the connector will be created with an ACL config. Currently this field only affects Cloud Storage and BigQuery connectors. |
| `create_time` | String | Output only. Timestamp the DataConnector was created at. |
| `hybrid_ingestion_disabled` | bool | Optional. If the connector is a hybrid connector, determines whether ingestion is enabled and appropriate resources are provisioned during connector creation. If the connector is not a hybrid connector, this field is ignored. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect the DataStores managed by this connector. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStores created by this connector will be protected by the KMS key. |
| `update_time` | String | Output only. Timestamp the DataConnector was last updated. |
| `name` | String | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |
| `bap_config` | String | Optional. The configuration for establishing a BAP connection. |
| `incremental_sync_disabled` | bool | Optional. Indicates whether incremental syncs are paused for this connector. This is independent of auto_run_disabled. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled, i.e. set to true. |
| `last_sync_time` | String | Output only. For periodic connectors only, the last time a data sync was completed. |
| `next_sync_time` | String | Defines the scheduled time for the next data synchronization. This field requires hour , minute, and time_zone from the [IANA Time Zone Database](https://www.iana.org/time-zones). This is utilized when the data connector has a refresh interval greater than 1 day. When the hours or minutes are not specified, we will assume a sync time of 0:00. The user must provide a time zone to avoid ambiguity. |
| `end_user_config` | String | Optional. Any params and credentials used specifically for EUA connectors. |
| `entities` | Vec<String> | List of entities from the connected data source to ingest. |
| `identity_schedule_config` | String | The configuration for the identity data synchronization runs. This contains the refresh interval to sync the Access Control List information for the documents ingested by this connector. |
| `params` | HashMap<String, String> | Required data connector parameters in structured json format. |
| `connector_type` | String | Output only. The type of connector. Each source can only map to one type. For example, salesforce, confluence and jira have THIRD_PARTY connector type. It is not mutable once set by system. |
| `destination_configs` | Vec<String> | Optional. Any target destinations used to connect to third-party services. |
| `sync_mode` | String | The data synchronization mode supported by the data connector. |
| `create_eua_saas` | bool | Optional. Whether the END USER AUTHENTICATION connector is created in SaaS. |
| `federated_config` | String | Optional. Any params and credentials used specifically for hybrid connectors supporting FEDERATED mode. This field should only be set if the connector is a hybrid connector and we want to enable FEDERATED mode. |
| `state` | String | Output only. State of the connector. |
| `static_ip_addresses` | Vec<String> | Output only. The static IP addresses used by this connector. |
| `remove_param_keys` | Vec<String> | Optional. Specifies keys to be removed from the 'params' field. This is only active when 'params' is included in the 'update_mask' in an UpdateDataConnectorRequest. Deletion takes precedence if a key is both in 'remove_param_keys' and present in the 'params' field of the request. |
| `realtime_state` | String | Output only. real-time sync state |
| `latest_pause_time` | String | Output only. The most recent timestamp when this DataConnector was paused, affecting all functionalities such as data synchronization. Pausing a connector has the following effects: - All functionalities, including data synchronization, are halted. - Any ongoing data synchronization job will be canceled. - No future data synchronization runs will be scheduled nor can be triggered. |
| `action_state` | String | Output only. State of the action connector. This reflects whether the action connector is initializing, active or has encountered errors. |
| `data_source` | String | Required. The name of the data source. Supported values: `salesforce`, `jira`, `confluence`, `bigquery`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access collection outputs
collection_id = collection.id
collection_incremental_refresh_interval = collection.incremental_refresh_interval
collection_auto_run_disabled = collection.auto_run_disabled
collection_private_connectivity_project_id = collection.private_connectivity_project_id
collection_realtime_sync_config = collection.realtime_sync_config
collection_refresh_interval = collection.refresh_interval
collection_static_ip_enabled = collection.static_ip_enabled
collection_errors = collection.errors
collection_action_config = collection.action_config
collection_json_params = collection.json_params
collection_identity_refresh_interval = collection.identity_refresh_interval
collection_blocking_reasons = collection.blocking_reasons
collection_alert_policy_configs = collection.alert_policy_configs
collection_connector_modes = collection.connector_modes
collection_acl_enabled = collection.acl_enabled
collection_create_time = collection.create_time
collection_hybrid_ingestion_disabled = collection.hybrid_ingestion_disabled
collection_kms_key_name = collection.kms_key_name
collection_update_time = collection.update_time
collection_name = collection.name
collection_bap_config = collection.bap_config
collection_incremental_sync_disabled = collection.incremental_sync_disabled
collection_last_sync_time = collection.last_sync_time
collection_next_sync_time = collection.next_sync_time
collection_end_user_config = collection.end_user_config
collection_entities = collection.entities
collection_identity_schedule_config = collection.identity_schedule_config
collection_params = collection.params
collection_connector_type = collection.connector_type
collection_destination_configs = collection.destination_configs
collection_sync_mode = collection.sync_mode
collection_create_eua_saas = collection.create_eua_saas
collection_federated_config = collection.federated_config
collection_state = collection.state
collection_static_ip_addresses = collection.static_ip_addresses
collection_remove_param_keys = collection.remove_param_keys
collection_realtime_state = collection.realtime_state
collection_latest_pause_time = collection.latest_pause_time
collection_action_state = collection.action_state
collection_data_source = collection.data_source
```

---


### Completion_suggestion

Imports CompletionSuggestions for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Cloud Storage location for the input content. |
| `error_config` | String |  | The desired location of errors incurred during the Import. |
| `bigquery_source` | String |  | BigQuery input source. |
| `inline_source` | String |  | The Inline source for suggestion entries. |
| `parent` | String | ✅ | Required. The parent data store resource name for which to import customer autocomplete suggestions. Follows pattern `projects/*/locations/*/collections/*/dataStores/*` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_suggestion
completion_suggestion = provider.discoveryengine_api.Completion_suggestion {
    parent = "value"  # Required. The parent data store resource name for which to import customer autocomplete suggestions. Follows pattern `projects/*/locations/*/collections/*/dataStores/*`
}

```

---


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Output only. The time the session started. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `state` | String |  | The state of the session. |
| `end_time` | String |  | Output only. The time the session finished. |
| `turns` | Vec<String> |  | Turns. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. The time the session started. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `state` | String | The state of the session. |
| `end_time` | String | Output only. The time the session finished. |
| `turns` | Vec<String> | Turns. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.discoveryengine_api.Session {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access session outputs
session_id = session.id
session_start_time = session.start_time
session_user_pseudo_id = session.user_pseudo_id
session_display_name = session.display_name
session_is_pinned = session.is_pinned
session_labels = session.labels
session_state = session.state
session_end_time = session.end_time
session_turns = session.turns
session_name = session.name
```

---


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `end_date` | String |  | Optional. The planed end date. |
| `subscription_term` | String |  | Required. Subscription term. |
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `state` | String |  | Output only. The state of the license config. |
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `start_date` | String |  | Required. The start date. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription_tier` | String | Required. Subscription tier information for the license config. |
| `end_date` | String | Optional. The planed end date. |
| `subscription_term` | String | Required. Subscription term. |
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `state` | String | Output only. The state of the license config. |
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |
| `start_date` | String | Required. The start date. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `license_count` | String | Required. Number of licenses purchased. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create license_config
license_config = provider.discoveryengine_api.License_config {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access license_config outputs
license_config_id = license_config.id
license_config_subscription_tier = license_config.subscription_tier
license_config_end_date = license_config.end_date
license_config_subscription_term = license_config.subscription_term
license_config_auto_renew = license_config.auto_renew
license_config_state = license_config.state
license_config_gemini_bundle = license_config.gemini_bundle
license_config_free_trial = license_config.free_trial
license_config_start_date = license_config.start_date
license_config_name = license_config.name
license_config_license_count = license_config.license_count
```

---


### Serving_config

Answer query method (streaming). It takes one AnswerQueryRequest and returns multiple AnswerQueryResponse messages in a stream.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pseudo_id` | String |  | A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `session` | String |  | The session resource name. Not required. When session field is not set, the API is in sessionless mode. We support auto session mode: users can use the wildcard symbol `-` as session ID. A new ID will be automatically generated and assigned. |
| `grounding_spec` | String |  | Optional. Grounding specification. |
| `asynchronous_mode` | bool |  | Deprecated: This field is deprecated. Streaming Answer API will be supported. Asynchronous mode control. If enabled, the response will be returned with answer/session resource name without final answer. The API users need to do the polling to get the latest status of answer/session by calling ConversationalSearchService.GetAnswer or ConversationalSearchService.GetSession method. |
| `answer_generation_spec` | String |  | Answer generation specification. |
| `end_user_spec` | String |  | Optional. End user specification. |
| `query` | String |  | Required. Current user query. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `safety_spec` | String |  | Model specification. |
| `query_understanding_spec` | String |  | Query understanding specification. |
| `related_questions_spec` | String |  | Related questions specification. |
| `search_spec` | String |  | Search specification. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `media_config` | String | The MediaConfig of the serving configuration. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `create_time` | String | Output only. ServingConfig created timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.discoveryengine_api.Serving_config {
    serving_config = "value"  # Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search.
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_model_id = serving_config.model_id
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_solution_type = serving_config.solution_type
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_display_name = serving_config.display_name
serving_config_media_config = serving_config.media_config
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_update_time = serving_config.update_time
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_name = serving_config.name
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_diversity_level = serving_config.diversity_level
serving_config_generic_config = serving_config.generic_config
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_create_time = serving_config.create_time
```

---


### Project

Provisions the project resource. During the process, related systems will get prepared and initialized. Caller must read the [Terms for data use](https://cloud.google.com/retail/data-use-terms), and optionally specify in request to provide consent to that service terms.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accept_data_use_terms` | bool |  | Required. Set to `true` to specify that caller has read and would like to give consent to the [Terms for data use](https://cloud.google.com/retail/data-use-terms). |
| `data_use_terms_version` | String |  | Required. The version of the [Terms for data use](https://cloud.google.com/retail/data-use-terms) that caller has read and would like to give consent to. Acceptable version is `2022-11-23`, and this may change over time. |
| `saas_params` | String |  | Optional. Parameters for Agentspace. |
| `name` | String | ✅ | Required. Full resource name of a Project, such as `projects/{project_id_or_number}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.discoveryengine_api.Project {
    name = "value"  # Required. Full resource name of a Project, such as `projects/{project_id_or_number}`.
}

```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `display_name` | String |  | The display name of the User Store. |
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `display_name` | String | The display name of the User Store. |
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_store
user_store = provider.discoveryengine_api.User_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access user_store outputs
user_store_id = user_store.id
user_store_default_license_config = user_store.default_license_config
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_display_name = user_store.display_name
user_store_enable_license_auto_register = user_store.enable_license_auto_register
user_store_name = user_store.name
```

---


### Suggestion_deny_list_entrie

Imports all SuggestionDenyListEntry for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_source` | String |  | The Inline source for the input content for suggestion deny list entries. |
| `gcs_source` | String |  | Cloud Storage location for the input content. Only 1 file can be specified that contains all entries to import. Supported values `gcs_source.schema` for autocomplete suggestion deny list entry imports: * `suggestion_deny_list` (default): One JSON [SuggestionDenyListEntry] per line. |
| `parent` | String | ✅ | Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suggestion_deny_list_entrie
suggestion_deny_list_entrie = provider.discoveryengine_api.Suggestion_deny_list_entrie {
    parent = "value"  # Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*.
}

```

---


### Grounding_config

Performs a grounding check.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `grounding_spec` | String |  | Configuration of the grounding check. |
| `facts` | Vec<String> |  | List of facts for the grounding check. We support up to 200 facts. |
| `grounding_config` | String | ✅ | Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grounding_config
grounding_config = provider.discoveryengine_api.Grounding_config {
    grounding_config = "value"  # Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`.
}

```

---


### Branche

Gets index freshness metadata for Documents. Supported for website search only.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents_metadata` | Vec<String> | The metadata of the Documents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access branche outputs
branche_id = branche.id
branche_documents_metadata = branche.documents_metadata
```

---


### Completion_config

Completes the user input with advanced keyword suggestions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suggestion_type_specs` | Vec<String> |  | Optional. Specification of each suggestion type. |
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128 |
| `experiment_ids` | Vec<String> |  | Optional. Experiment ids for this request. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `boost_spec` | String |  | Optional. Specification to boost suggestions matching the condition. |
| `query` | String |  | Required. The typeahead input used to fetch suggestions. Maximum length is 128 characters. The query can not be empty for most of the suggestion types. If it is empty, an `INVALID_ARGUMENT` error is returned. The exception is when the suggestion_types contains only the type `RECENT_SEARCH`, the query can be an empty string. The is called "zero prefix" feature, which returns user's recently searched queries given the empty query. |
| `suggestion_types` | Vec<String> |  | Optional. Suggestion types to return. If empty or unspecified, query suggestions are returned. Only one suggestion type is supported at the moment. |
| `query_model` | String |  | Specifies the autocomplete query model, which only applies to the QUERY SuggestionType. This overrides any model specified in the Configuration > Autocomplete section of the Cloud console. Currently supported values: * `document` - Using suggestions generated from user-imported documents. * `search-history` - Using suggestions generated from the past history of SearchService.Search API calls. Do not use it when there is no traffic for Search API. * `user-event` - Using suggestions generated from user-imported search events. * `document-completable` - Using suggestions taken directly from user-imported document fields marked as completable. Default values: * `document` is the default model for regular dataStores. * `search-history` is the default model for site search dataStores. |
| `include_tail_suggestions` | bool |  | Indicates if tail suggestions should be returned if there are no suggestions that match the full query. Even if set to true, if there are suggestions that match the full query, those are returned and no tail suggestions are returned. |
| `completion_config` | String | ✅ | Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_config
completion_config = provider.discoveryengine_api.Completion_config {
    completion_config = "value"  # Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`.
}

```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `solution_type` | String |  | Required. The solutions of the engine. |
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `chat_engine_metadata` | String | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |
| `solution_type` | String | Required. The solutions of the engine. |
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create engine
engine = provider.discoveryengine_api.Engine {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access engine outputs
engine_id = engine.id
engine_chat_engine_metadata = engine.chat_engine_metadata
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_search_engine_config = engine.search_engine_config
engine_name = engine.name
engine_create_time = engine.create_time
engine_industry_vertical = engine.industry_vertical
engine_update_time = engine.update_time
engine_solution_type = engine.solution_type
engine_display_name = engine.display_name
engine_chat_engine_config = engine.chat_engine_config
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_common_config = engine.common_config
engine_data_store_ids = engine.data_store_ids
engine_app_type = engine.app_type
engine_disable_analytics = engine.disable_analytics
engine_features = engine.features
```

---


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_store
data_store = provider.discoveryengine_api.Data_store {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access data_store outputs
data_store_id = data_store.id
data_store_cmek_config = data_store.cmek_config
data_store_acl_enabled = data_store.acl_enabled
data_store_content_config = data_store.content_config
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_billing_estimation = data_store.billing_estimation
data_store_default_schema_id = data_store.default_schema_id
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_solution_types = data_store.solution_types
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_industry_vertical = data_store.industry_vertical
data_store_kms_key_name = data_store.kms_key_name
data_store_advanced_site_search_config = data_store.advanced_site_search_config
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
data_store_starting_schema = data_store.starting_schema
data_store_workspace_config = data_store.workspace_config
data_store_create_time = data_store.create_time
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
data_store_display_name = data_store.display_name
data_store_document_processing_config = data_store.document_processing_config
data_store_name = data_store.name
```

---


### Custom_model

Gets a list of all the custom models.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `models` | Vec<String> | List of custom tuning models. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access custom_model outputs
custom_model_id = custom_model.id
custom_model_models = custom_model.models
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `indexing_status` | String |  | Output only. Indexing status. |
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `indexing_status` | String | Output only. Indexing status. |
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `update_time` | String | Output only. The target site's last updated time. |
| `failure_reason` | String | Output only. Failure reason. |
| `provided_uri_pattern` | String | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target_site
target_site = provider.discoveryengine_api.Target_site {
    parent = "value"  # Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
}

# Access target_site outputs
target_site_id = target_site.id
target_site_type = target_site.type
target_site_site_verification_info = target_site.site_verification_info
target_site_indexing_status = target_site.indexing_status
target_site_name = target_site.name
target_site_update_time = target_site.update_time
target_site_failure_reason = target_site.failure_reason
target_site_provided_uri_pattern = target_site.provided_uri_pattern
target_site_root_domain_uri = target_site.root_domain_uri
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_exact_match = target_site.exact_match
```

---


### Assistant

Assists the user with a query in a streaming fashion.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session` | String |  | Optional. The session to use for the request. If specified, the assistant has access to the session history, and the query and the answer are stored there. If `-` is specified as the session ID, or it is left empty, then a new session is created with an automatically generated ID. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `tools_spec` | String |  | Optional. Specification of tools that are used to serve the request. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |
| `customer_policy` | String | Optional. Customer policy for the assistant. |
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assistant
assistant = provider.discoveryengine_api.Assistant {
    name = "value"  # Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}`
}

# Access assistant outputs
assistant_id = assistant.id
assistant_name = assistant.name
assistant_generation_config = assistant.generation_config
assistant_customer_policy = assistant.customer_policy
assistant_web_grounding_type = assistant.web_grounding_type
assistant_enabled_tools = assistant.enabled_tools
```

---


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `query` | String |  | The query to use. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
| `top_n` | i64 |  | The number of results to return. If this is unset or no bigger than zero, returns all results. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `ignore_record_details_in_response` | bool |  | If true, the response will contain only record ID and score. By default, it is false, the response will contain record details. |
| `ranking_config` | String | ✅ | Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ranking_config
ranking_config = provider.discoveryengine_api.Ranking_config {
    ranking_config = "value"  # Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`.
}

```

---


### User_event

Bulk import of user events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Cloud Storage location for the input content. |
| `inline_source` | String |  | The Inline source for the input content for UserEvents. |
| `error_config` | String |  | The desired location of errors incurred during the Import. Cannot be set for inline user event imports. |
| `bigquery_source` | String |  | BigQuery input source. |
| `parent` | String | ✅ | Required. Parent DataStore resource name, of the form `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.discoveryengine_api.User_event {
    parent = "value"  # Required. Parent DataStore resource name, of the form `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`
}

# Access user_event outputs
user_event_id = user_event.id
user_event_content_type = user_event.content_type
user_event_data = user_event.data
user_event_extensions = user_event.extensions
```

---


### Site_search_engine

Request on-demand recrawl for a list of URIs.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uris` | Vec<String> |  | Required. List of URIs to crawl. At most 10K URIs are supported, otherwise an INVALID_ARGUMENT error is thrown. Each URI should match at least one TargetSite in `site_search_engine`. |
| `site_credential` | String |  | Optional. Credential id to use for crawling. |
| `site_search_engine` | String | ✅ | Required. Full resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |
| `total_size` | i64 | The total number of items matching the request. This will always be populated in the response. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site_search_engine
site_search_engine = provider.discoveryengine_api.Site_search_engine {
    site_search_engine = "value"  # Required. Full resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
}

# Access site_search_engine outputs
site_search_engine_id = site_search_engine.id
site_search_engine_target_sites = site_search_engine.target_sites
site_search_engine_total_size = site_search_engine.total_size
site_search_engine_next_page_token = site_search_engine.next_page_token
```

---


### Site_search_engine

Request on-demand recrawl for a list of URIs.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_credential` | String |  | Optional. Credential id to use for crawling. |
| `uris` | Vec<String> |  | Required. List of URIs to crawl. At most 10K URIs are supported, otherwise an INVALID_ARGUMENT error is thrown. Each URI should match at least one TargetSite in `site_search_engine`. |
| `site_search_engine` | String | ✅ | Required. Full resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |
| `total_size` | i64 | The total number of items matching the request. This will always be populated in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site_search_engine
site_search_engine = provider.discoveryengine_api.Site_search_engine {
    site_search_engine = "value"  # Required. Full resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
}

# Access site_search_engine outputs
site_search_engine_id = site_search_engine.id
site_search_engine_next_page_token = site_search_engine.next_page_token
site_search_engine_target_sites = site_search_engine.target_sites
site_search_engine_total_size = site_search_engine.total_size
```

---


### Grounding_config

Performs a grounding check.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grounding_spec` | String |  | Configuration of the grounding check. |
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `facts` | Vec<String> |  | List of facts for the grounding check. We support up to 200 facts. |
| `grounding_config` | String | ✅ | Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grounding_config
grounding_config = provider.discoveryengine_api.Grounding_config {
    grounding_config = "value"  # Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`.
}

```

---


### Requirement

Check a particular requirement.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requirement_type` | String |  | The type specifying the requirement to check. The supported types are: * `discoveryengine.googleapis.com/media_recs/general/all/warning` * `discoveryengine.googleapis.com/media_recs/oyml/cvr/warning` * `discoveryengine.googleapis.com/media_recs/rfy/cvr/warning` * `discoveryengine.googleapis.com/media_recs/mlt/cvr/warning` * `discoveryengine.googleapis.com/media_recs/mp/cvr/warning` * `discoveryengine.googleapis.com/media_recs/oyml/wdps/warning` * `discoveryengine.googleapis.com/media_recs/rfy/wdps/warning` * `discoveryengine.googleapis.com/media_recs/mlt/wdps/warning` |
| `resources` | Vec<String> |  | The resources to be checked for this requirement. The type needed for the monitored resources: * `discoveryengine.googleapis.com/Branch`. * The labels needed for this resource: * `project_number` * `location_id` * `collection_id` * `datastore_id` * `branch_id` * `discoveryengine.googleapis.com/DataStore` * The labels needed for this resource: * `project_number` * `location_id` * `collection_id` * `datastore_id` |
| `location` | String | ✅ | Required. Full resource name of the location. Format `projects/{project_number_or_id}/locations/{location}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create requirement
requirement = provider.discoveryengine_api.Requirement {
    location = "value"  # Required. Full resource name of the location. Format `projects/{project_number_or_id}/locations/{location}`
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
operation = provider.discoveryengine_api.Operation {
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


### Connector_run

Lists the ConnectorRuns of a DataConnector.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `connector_runs` | Vec<String> | List of ConnectorRuns. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connector_run outputs
connector_run_id = connector_run.id
connector_run_next_page_token = connector_run.next_page_token
connector_run_connector_runs = connector_run.connector_runs
```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `solution_type` | String |  | Required. The solutions of the engine. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `recommendation_metadata` | String |  | Output only. Additional information of a recommendation engine. Only applicable if solution_type is SOLUTION_TYPE_RECOMMENDATION. |
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `similar_documents_config` | String |  | Additional config specs for a `similar-items` engine. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solution_type` | String | Required. The solutions of the engine. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `chat_engine_metadata` | String | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `recommendation_metadata` | String | Output only. Additional information of a recommendation engine. Only applicable if solution_type is SOLUTION_TYPE_RECOMMENDATION. |
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `similar_documents_config` | String | Additional config specs for a `similar-items` engine. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create engine
engine = provider.discoveryengine_api.Engine {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access engine outputs
engine_id = engine.id
engine_solution_type = engine.solution_type
engine_app_type = engine.app_type
engine_features = engine.features
engine_update_time = engine.update_time
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_name = engine.name
engine_disable_analytics = engine.disable_analytics
engine_chat_engine_metadata = engine.chat_engine_metadata
engine_common_config = engine.common_config
engine_create_time = engine.create_time
engine_industry_vertical = engine.industry_vertical
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_data_store_ids = engine.data_store_ids
engine_display_name = engine.display_name
engine_recommendation_metadata = engine.recommendation_metadata
engine_search_engine_config = engine.search_engine_config
engine_chat_engine_config = engine.chat_engine_config
engine_similar_documents_config = engine.similar_documents_config
```

---


### Evaluation

Creates a Evaluation. Upon creation, the evaluation will be automatically triggered and begin execution.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `error_samples` | Vec<String> |  | Output only. A sample of errors encountered while processing the request. |
| `evaluation_spec` | String |  | Required. The specification of the evaluation. |
| `end_time` | String |  | Output only. Timestamp the Evaluation was completed at. |
| `error` | String |  | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String |  | Output only. The state of the evaluation. |
| `create_time` | String |  | Output only. Timestamp the Evaluation was created at. |
| `quality_metrics` | String |  | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `error_samples` | Vec<String> | Output only. A sample of errors encountered while processing the request. |
| `evaluation_spec` | String | Required. The specification of the evaluation. |
| `end_time` | String | Output only. Timestamp the Evaluation was completed at. |
| `error` | String | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String | Output only. The state of the evaluation. |
| `create_time` | String | Output only. Timestamp the Evaluation was created at. |
| `quality_metrics` | String | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.discoveryengine_api.Evaluation {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_name = evaluation.name
evaluation_error_samples = evaluation.error_samples
evaluation_evaluation_spec = evaluation.evaluation_spec
evaluation_end_time = evaluation.end_time
evaluation_error = evaluation.error
evaluation_state = evaluation.state
evaluation_create_time = evaluation.create_time
evaluation_quality_metrics = evaluation.quality_metrics
```

---


### Sitemap

Creates a Sitemap.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The fully qualified resource name of the sitemap. `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine/sitemaps/*` The `sitemap_id` suffix is system-generated. |
| `uri` | String |  | Public URI for the sitemap, e.g. `www.example.com/sitemap.xml`. |
| `create_time` | String |  | Output only. The sitemap's creation time. |
| `parent` | String | ✅ | Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sitemaps_metadata` | Vec<String> | List of Sitemaps fetched. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sitemap
sitemap = provider.discoveryengine_api.Sitemap {
    parent = "value"  # Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
}

# Access sitemap outputs
sitemap_id = sitemap.id
sitemap_sitemaps_metadata = sitemap.sitemaps_metadata
```

---


### Completion_config

Completes the user input with advanced keyword suggestions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_model` | String |  | Specifies the autocomplete query model, which only applies to the QUERY SuggestionType. This overrides any model specified in the Configuration > Autocomplete section of the Cloud console. Currently supported values: * `document` - Using suggestions generated from user-imported documents. * `search-history` - Using suggestions generated from the past history of SearchService.Search API calls. Do not use it when there is no traffic for Search API. * `user-event` - Using suggestions generated from user-imported search events. * `document-completable` - Using suggestions taken directly from user-imported document fields marked as completable. Default values: * `document` is the default model for regular dataStores. * `search-history` is the default model for site search dataStores. |
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128 |
| `suggestion_types` | Vec<String> |  | Optional. Suggestion types to return. If empty or unspecified, query suggestions are returned. Only one suggestion type is supported at the moment. |
| `experiment_ids` | Vec<String> |  | Optional. Experiment ids for this request. |
| `boost_spec` | String |  | Optional. Specification to boost suggestions matching the condition. |
| `include_tail_suggestions` | bool |  | Indicates if tail suggestions should be returned if there are no suggestions that match the full query. Even if set to true, if there are suggestions that match the full query, those are returned and no tail suggestions are returned. |
| `query` | String |  | Required. The typeahead input used to fetch suggestions. Maximum length is 128 characters. The query can not be empty for most of the suggestion types. If it is empty, an `INVALID_ARGUMENT` error is returned. The exception is when the suggestion_types contains only the type `RECENT_SEARCH`, the query can be an empty string. The is called "zero prefix" feature, which returns user's recently searched queries given the empty query. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `suggestion_type_specs` | Vec<String> |  | Optional. Specification of each suggestion type. |
| `completion_config` | String | ✅ | Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_config
completion_config = provider.discoveryengine_api.Completion_config {
    completion_config = "value"  # Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`.
}

```

---


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `end_time` | String |  | Output only. The time the session finished. |
| `start_time` | String |  | Output only. The time the session started. |
| `turns` | Vec<String> |  | Turns. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `state` | String |  | The state of the session. |
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `end_time` | String | Output only. The time the session finished. |
| `start_time` | String | Output only. The time the session started. |
| `turns` | Vec<String> | Turns. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `state` | String | The state of the session. |
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.discoveryengine_api.Session {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access session outputs
session_id = session.id
session_display_name = session.display_name
session_name = session.name
session_end_time = session.end_time
session_start_time = session.start_time
session_turns = session.turns
session_user_pseudo_id = session.user_pseudo_id
session_state = session.state
session_labels = session.labels
session_is_pinned = session.is_pinned
```

---


### Answer

Gets a Answer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |
| `blob_attachments` | Vec<String> | List of blob attachments in the answer. |
| `answer_text` | String | The textual answer. |
| `citations` | Vec<String> | Citations. |
| `related_questions` | Vec<String> | Suggested related questions. |
| `create_time` | String | Output only. Answer creation timestamp. |
| `state` | String | The state of the answer generation. |
| `steps` | Vec<String> | Answer generation steps. |
| `query_understanding_info` | String | Query understanding information. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `references` | Vec<String> | References. |
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access answer outputs
answer_id = answer.id
answer_grounding_score = answer.grounding_score
answer_grounding_supports = answer.grounding_supports
answer_blob_attachments = answer.blob_attachments
answer_answer_text = answer.answer_text
answer_citations = answer.citations
answer_related_questions = answer.related_questions
answer_create_time = answer.create_time
answer_state = answer.state
answer_steps = answer.steps
answer_query_understanding_info = answer.query_understanding_info
answer_name = answer.name
answer_complete_time = answer.complete_time
answer_safety_ratings = answer.safety_ratings
answer_references = answer.references
answer_answer_skipped_reasons = answer.answer_skipped_reasons
```

---


### Source

Uploads a file for Notebook LM to use. Creates a Source.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blob` | String |  | Information about the file being uploaded. |
| `media_request_info` | String |  | Media upload request metadata. |
| `source_id` | String |  | The source id of the associated file. If not set, a source id will be generated and a new tentative source will be created. |
| `source_id` | String | ✅ | The source id of the associated file. If not set, a source id will be generated and a new tentative source will be created. |
| `parent` | String | ✅ | Required. The parent resource where the sources will be created. Format: projects/{project}/locations/{location}/notebooks/{notebook} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_id` | String | Optional. Output only. Source id, which is the last segment of the source's resource name. |
| `title` | String | Optional. Title of the source. |
| `metadata` | String | Output only. Metadata about the source. |
| `name` | String | Identifier. The full resource name of the source. Format: `projects/{project}/locations/{location}/notebooks/{notebook}/sources/{source_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `settings` | String | Output only. Status of the source, and any failure reasons. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create source
source = provider.discoveryengine_api.Source {
    source_id = "value"  # The source id of the associated file. If not set, a source id will be generated and a new tentative source will be created.
    parent = "value"  # Required. The parent resource where the sources will be created. Format: projects/{project}/locations/{location}/notebooks/{notebook}
}

# Access source outputs
source_id = source.id
source_source_id = source.source_id
source_title = source.title
source_metadata = source.metadata
source_name = source.name
source_settings = source.settings
```

---


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ignore_record_details_in_response` | bool |  | If true, the response will contain only record ID and score. By default, it is false, the response will contain record details. |
| `query` | String |  | The query to use. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `top_n` | i64 |  | The number of results to return. If this is unset or no bigger than zero, returns all results. |
| `ranking_config` | String | ✅ | Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ranking_config
ranking_config = provider.discoveryengine_api.Ranking_config {
    ranking_config = "value"  # Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`.
}

```

---


### Serving_config

Performs a search. Similar to the SearchService.Search method, but a lite version that allows API key for authentication, where OAuth and IAM checks are not required. Only public website search is supported by this method. If data stores and engines not associated with public website search are specified, a `FAILED_PRECONDITION` error is returned. This method can be used for easy onboarding without having to implement an authentication backend. However, it is strongly recommended to use SearchService.Search instead with required OAuth and IAM checks to provide better data security.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_info` | String |  | Information about the end user. Highly recommended for analytics and personalization. UserInfo.user_agent is used to deduce `device_type` for analytics. |
| `content_search_spec` | String |  | A specification for configuring the behavior of content search. |
| `display_spec` | String |  | Optional. Config for display feature, like match highlighting on search results. |
| `relevance_threshold` | String |  | The relevance threshold of the search results. Default to Google defined threshold, leveraging a balance of precision and recall to deliver both highly accurate results and comprehensive coverage of relevant information. This feature is not supported for healthcare search. |
| `page_token` | String |  | A page token received from a previous SearchService.Search call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to SearchService.Search must match the call that provided the page token. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `custom_fine_tuning_spec` | String |  | Custom fine tuning configs. If set, it has higher priority than the configs set in ServingConfig.custom_fine_tuning_spec. |
| `data_store_specs` | Vec<String> |  | Specifications that define the specific DataStores to be searched, along with configurations for those data stores. This is only considered for Engines with multiple data stores. For engines with a single data store, the specs directly under SearchRequest should be used. |
| `natural_language_query_understanding_spec` | String |  | Optional. Config for natural language query understanding capabilities, such as extracting structured field filters from the query. Refer to [this documentation](https://cloud.google.com/generative-ai-app-builder/docs/natural-language-queries) for more information. If `naturalLanguageQueryUnderstandingSpec` is not specified, no additional natural language query understanding will be done. |
| `filter` | String |  | The filter syntax consists of an expression language for constructing a predicate from one or more fields of the documents being filtered. Filter expression is case-sensitive. If this field is unrecognizable, an `INVALID_ARGUMENT` is returned. Filtering in Vertex AI Search is done by mapping the LHS filter key to a key property defined in the Vertex AI Search backend -- this mapping is defined by the customer in their schema. For example a media customer might have a field 'name' in their schema. In this case the filter would look like this: filter --> name:'ANY("king kong")' For more information about filtering including syntax and filter operators, see [Filter](https://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata) |
| `embedding_spec` | String |  | Uses the provided embedding to do additional semantic document retrieval. The retrieval is based on the dot product of SearchRequest.EmbeddingSpec.EmbeddingVector.vector and the document embedding that is provided in SearchRequest.EmbeddingSpec.EmbeddingVector.field_path. If SearchRequest.EmbeddingSpec.EmbeddingVector.field_path is not provided, it will use ServingConfig.EmbeddingConfig.field_path. |
| `order_by` | String |  | The order in which documents are returned. Documents can be ordered by a field in an Document object. Leave it unset if ordered by relevance. `order_by` expression is case-sensitive. For more information on ordering the website search results, see [Order web search results](https://cloud.google.com/generative-ai-app-builder/docs/order-web-search-results). For more information on ordering the healthcare search results, see [Order healthcare search results](https://cloud.google.com/generative-ai-app-builder/docs/order-hc-results). If this field is unrecognizable, an `INVALID_ARGUMENT` is returned. |
| `session_spec` | String |  | Session specification. Can be used only when `session` is set. |
| `region_code` | String |  | The Unicode country/region code (CLDR) of a location, such as "US" and "419". For more information, see [Standard fields](https://cloud.google.com/apis/design/standard_fields). If set, then results will be boosted based on the region_code provided. |
| `serving_config` | String |  | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |
| `page_size` | i64 |  | Maximum number of Documents to return. The maximum allowed value depends on the data type. Values above the maximum value are coerced to the maximum value. * Websites with basic indexing: Default `10`, Maximum `25`. * Websites with advanced indexing: Default `25`, Maximum `50`. * Other: Default `50`, Maximum `100`. If this field is negative, an `INVALID_ARGUMENT` is returned. |
| `use_latest_data` | bool |  | Uses the Engine, ServingConfig and Control freshly read from the database. Note: this skips config cache and introduces dependency on databases, which could significantly increase the API latency. It should only be used for testing, but not serving end users. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `ranking_expression_backend` | String |  | Optional. The backend to use for the ranking expression evaluation. |
| `boost_spec` | String |  | Boost specification to boost certain documents. For more information on boosting, see [Boosting](https://cloud.google.com/generative-ai-app-builder/docs/boost-search-results) |
| `session` | String |  | The session resource name. Optional. Session allows users to do multi-turn /search API calls or coordination between /search API calls and /answer API calls. Example #1 (multi-turn /search API calls): Call /search API with the session ID generated in the first call. Here, the previous search query gets considered in query standing. I.e., if the first query is "How did Alphabet do in 2022?" and the current query is "How about 2023?", the current query will be interpreted as "How did Alphabet do in 2023?". Example #2 (coordination between /search API calls and /answer API calls): Call /answer API with the session ID generated in the first call. Here, the answer generation happens in the context of the search results from the first search call. Multi-turn Search feature is currently at private GA stage. Please use v1alpha or v1beta version instead before we launch this feature to public GA. Or ask for allowlisting through Google Support team. |
| `spell_correction_spec` | String |  | The spell correction specification that specifies the mode under which spell correction takes effect. |
| `branch` | String |  | The branch resource name, such as `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/branches/0`. Use `default_branch` as the branch ID or leave this field empty, to search documents under the default branch. |
| `search_addon_spec` | String |  | Optional. SearchAddonSpec is used to disable add-ons for search as per new repricing model. This field is only supported for search requests. |
| `canonical_filter` | String |  | The default filter that is applied when a user performs a search without checking any filters on the search page. The filter applied to every search request when quality improvement such as query expansion is needed. In the case a query does not have a sufficient amount of results this filter will be used to determine whether or not to enable the query expansion flow. The original filter will still be used for the query expanded search. This field is strongly recommended to achieve high search quality. For more information about filter syntax, see SearchRequest.filter. |
| `image_query` | String |  | Raw image query. |
| `facet_specs` | Vec<String> |  | Facet specifications for faceted search. If empty, no facets are returned. A maximum of 100 values are allowed. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `params` | HashMap<String, String> |  | Additional search parameters. For public website search only, supported values are: * `user_country_code`: string. Default empty. If set to non-empty, results are restricted or boosted based on the location provided. For example, `user_country_code: "au"` For available codes see [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) * `search_type`: double. Default empty. Enables non-webpage searching depending on the value. The only valid non-default value is 1, which enables image searching. For example, `search_type: 1` |
| `query` | String |  | Raw search query. |
| `offset` | i64 |  | A 0-indexed integer that specifies the current offset (that is, starting result location, amongst the Documents deemed by the API as relevant) in search results. This field is only considered if page_token is unset. If this field is negative, an `INVALID_ARGUMENT` is returned. A large offset may be capped to a reasonable threshold. |
| `relevance_score_spec` | String |  | Optional. The specification for returning the relevance score. |
| `language_code` | String |  | The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see [Standard fields](https://cloud.google.com/apis/design/standard_fields). This field helps to better interpret the query. If a value isn't specified, the query language code is automatically detected, which may not be accurate. |
| `query_expansion_spec` | String |  | The query expansion specification that specifies the conditions under which query expansion occurs. |
| `personalization_spec` | String |  | The specification for personalization. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set, SearchRequest.personalization_spec overrides ServingConfig.personalization_spec. |
| `one_box_page_size` | i64 |  | The maximum number of results to return for OneBox. This applies to each OneBox type individually. Default number is 10. |
| `ranking_expression` | String |  | Optional. The ranking expression controls the customized ranking on retrieval documents. This overrides ServingConfig.ranking_expression. The syntax and supported features depend on the `ranking_expression_backend` value. If `ranking_expression_backend` is not provided, it defaults to `RANK_BY_EMBEDDING`. If ranking_expression_backend is not provided or set to `RANK_BY_EMBEDDING`, it should be a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between `embedding_field_path` and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. If ranking_expression_backend is set to `RANK_BY_FORMULA`, the following expression types (and combinations of those chained using + or * operators) are supported: * `double` * `signal` * `log(signal)` * `exp(signal)` * `rr(signal, double > 0)` -- reciprocal rank transformation with second argument being a denominator constant. * `is_nan(signal)` -- returns 0 if signal is NaN, 1 otherwise. * `fill_nan(signal1, signal2 | double)` -- if signal1 is NaN, returns signal2 | double, else returns signal1. Here are a few examples of ranking formulas that use the supported ranking expression types: - `0.2 * semantic_similarity_score + 0.8 * log(keyword_similarity_score)` -- mostly rank by the logarithm of `keyword_similarity_score` with slight `semantic_smilarity_score` adjustment. - `0.2 * exp(fill_nan(semantic_similarity_score, 0)) + 0.3 * is_nan(keyword_similarity_score)` -- rank by the exponent of `semantic_similarity_score` filling the value with 0 if it's NaN, also add constant 0.3 adjustment to the final score if `semantic_similarity_score` is NaN. - `0.2 * rr(semantic_similarity_score, 16) + 0.8 * rr(keyword_similarity_score, 16)` -- mostly rank by the reciprocal rank of `keyword_similarity_score` with slight adjustment of reciprocal rank of `semantic_smilarity_score`. The following signals are supported: * `semantic_similarity_score`: semantic similarity adjustment that is calculated using the embeddings generated by a proprietary Google model. This score determines how semantically similar a search query is to a document. * `keyword_similarity_score`: keyword match adjustment uses the Best Match 25 (BM25) ranking function. This score is calculated using a probabilistic model to estimate the probability that a document is relevant to a given query. * `relevance_score`: semantic relevance adjustment that uses a proprietary Google model to determine the meaning and intent behind a user's query in context with the content in the documents. * `pctr_rank`: predicted conversion rate adjustment as a rank use predicted Click-through rate (pCTR) to gauge the relevance and attractiveness of a search result from a user's perspective. A higher pCTR suggests that the result is more likely to satisfy the user's query and intent, making it a valuable signal for ranking. * `freshness_rank`: freshness adjustment as a rank * `document_age`: The time in hours elapsed since the document was last updated, a floating-point number (e.g., 0.25 means 15 minutes). * `topicality_rank`: topicality adjustment as a rank. Uses proprietary Google model to determine the keyword-based overlap between the query and the document. * `base_rank`: the default rank of the result |
| `search_as_you_type_spec` | String |  | Search as you type configuration. Only supported for the IndustryVertical.MEDIA vertical. |
| `safe_search` | bool |  | Whether to turn on safe search. This is only supported for website search. |
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and CompleteQueryRequest.user_pseudo_id The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `create_time` | String | Output only. ServingConfig created timestamp. |
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `embedding_config` | String | Bring your own embedding config. The config is used for search semantic retrieval. The retrieval is based on the dot product of SearchRequest.EmbeddingSpec.EmbeddingVector.vector and the document embeddings that are provided by this EmbeddingConfig. If SearchRequest.EmbeddingSpec.EmbeddingVector.vector is provided, it overrides this ServingConfig.embedding_config. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `media_config` | String | The MediaConfig of the serving configuration. |
| `guided_search_spec` | String | Guided search configs. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `custom_fine_tuning_spec` | String | Custom fine tuning configs. If SearchRequest.custom_fine_tuning_spec is set, it has higher priority than the configs set here. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `personalization_spec` | String | The specification for personalization spec. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set, SearchRequest.personalization_spec overrides ServingConfig.personalization_spec. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.discoveryengine_api.Serving_config {
    serving_config = "value"  # Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search.
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_create_time = serving_config.create_time
serving_config_model_id = serving_config.model_id
serving_config_solution_type = serving_config.solution_type
serving_config_embedding_config = serving_config.embedding_config
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_media_config = serving_config.media_config
serving_config_guided_search_spec = serving_config.guided_search_spec
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_custom_fine_tuning_spec = serving_config.custom_fine_tuning_spec
serving_config_diversity_level = serving_config.diversity_level
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_generic_config = serving_config.generic_config
serving_config_name = serving_config.name
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_update_time = serving_config.update_time
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_display_name = serving_config.display_name
serving_config_ignore_control_ids = serving_config.ignore_control_ids
```

---


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redirect_action` | String |  | Defines a redirect-type control. |
| `boost_action` | String |  | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `redirect_action` | String | Defines a redirect-type control. |
| `boost_action` | String | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `solution_type` | String | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.discoveryengine_api.Control {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
}

# Access control outputs
control_id = control.id
control_redirect_action = control.redirect_action
control_boost_action = control.boost_action
control_associated_serving_config_ids = control.associated_serving_config_ids
control_conditions = control.conditions
control_name = control.name
control_synonyms_action = control.synonyms_action
control_display_name = control.display_name
control_filter_action = control.filter_action
control_use_cases = control.use_cases
control_promote_action = control.promote_action
control_solution_type = control.solution_type
```

---


### Project

Provisions the project resource. During the process, related systems will get prepared and initialized. Caller must read the [Terms for data use](https://cloud.google.com/retail/data-use-terms), and optionally specify in request to provide consent to that service terms.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_use_terms_version` | String |  | Required. The version of the [Terms for data use](https://cloud.google.com/retail/data-use-terms) that caller has read and would like to give consent to. Acceptable version is `2022-11-23`, and this may change over time. |
| `accept_data_use_terms` | bool |  | Required. Set to `true` to specify that caller has read and would like to give consent to the [Terms for data use](https://cloud.google.com/retail/data-use-terms). |
| `saas_params` | String |  | Optional. Parameters for Agentspace. |
| `name` | String | ✅ | Required. Full resource name of a Project, such as `projects/{project_id_or_number}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provision_completion_time` | String | Output only. The timestamp when this project is successfully provisioned. Empty value means this project is still provisioning and is not ready for use. |
| `name` | String | Output only. Full resource name of the project, for example `projects/{project}`. Note that when making requests, project number and project id are both acceptable, but the server will always respond in project number. |
| `service_terms_map` | HashMap<String, String> | Output only. A map of terms of services. The key is the `id` of ServiceTerms. |
| `create_time` | String | Output only. The timestamp when this project is created. |
| `customer_provided_config` | String | Optional. Customer provided configurations. |
| `configurable_billing_status` | String | Output only. The current status of the project's configurable billing. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.discoveryengine_api.Project {
    name = "value"  # Required. Full resource name of a Project, such as `projects/{project_id_or_number}`.
}

# Access project outputs
project_id = project.id
project_provision_completion_time = project.provision_completion_time
project_name = project.name
project_service_terms_map = project.service_terms_map
project_create_time = project.create_time
project_customer_provided_config = project.customer_provided_config
project_configurable_billing_status = project.configurable_billing_status
```

---


### Canned_querie

Creates a CannedQuery.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_texts` | String |  | Required. The default (non-localized) values for the text attributes. |
| `required_capabilities` | Vec<String> |  | Optional. The capabilities the Assistant needs to have to use this canned query. |
| `display_name` | String |  | The display name of the canned query. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `google_defined` | bool |  | Output only. Whether this is a Google-defined, read-only canned query. |
| `name` | String |  | Immutable. Resource name of the canned query. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/cannedQueries/{canned_query}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `localized_texts` | HashMap<String, String> |  | Optional. The translations of the text attributes. The keys should be BCP-47 language codes. |
| `enabled` | bool |  | Whether this canned query is enabled. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_texts` | String | Required. The default (non-localized) values for the text attributes. |
| `required_capabilities` | Vec<String> | Optional. The capabilities the Assistant needs to have to use this canned query. |
| `display_name` | String | The display name of the canned query. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `google_defined` | bool | Output only. Whether this is a Google-defined, read-only canned query. |
| `name` | String | Immutable. Resource name of the canned query. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/cannedQueries/{canned_query}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `localized_texts` | HashMap<String, String> | Optional. The translations of the text attributes. The keys should be BCP-47 language codes. |
| `enabled` | bool | Whether this canned query is enabled. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create canned_querie
canned_querie = provider.discoveryengine_api.Canned_querie {
    parent = "value"  # Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}`
}

# Access canned_querie outputs
canned_querie_id = canned_querie.id
canned_querie_default_texts = canned_querie.default_texts
canned_querie_required_capabilities = canned_querie.required_capabilities
canned_querie_display_name = canned_querie.display_name
canned_querie_google_defined = canned_querie.google_defined
canned_querie_name = canned_querie.name
canned_querie_localized_texts = canned_querie.localized_texts
canned_querie_enabled = canned_querie.enabled
```

---


### Sample_querie

Creates a SampleQuery

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp the SampleQuery was created at. |
| `name` | String |  | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `query_entry` | String |  | The query entry. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/sampleQuerySets/{sampleQuerySet}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp the SampleQuery was created at. |
| `name` | String | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `query_entry` | String | The query entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sample_querie
sample_querie = provider.discoveryengine_api.Sample_querie {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/sampleQuerySets/{sampleQuerySet}`.
}

# Access sample_querie outputs
sample_querie_id = sample_querie.id
sample_querie_create_time = sample_querie.create_time
sample_querie_name = sample_querie.name
sample_querie_query_entry = sample_querie.query_entry
```

---


### Notebook

Creates a notebook.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cmek_config` | String |  | Output only. CMEK-related information for the Notebook. |
| `name` | String |  | Identifier. The identifier of the notebook. Format: `projects/{project}/locations/{location}/notebooks/{notebook_id}`. This field must be a UTF-8 encoded string. |
| `title` | String |  | Optional. The title of the notebook. |
| `metadata` | String |  | Output only. The metadata of the notebook. |
| `sources` | Vec<String> |  | Output only. List of sources in the notebook. This is an output only field. |
| `emoji` | String |  | Output only. The emoji of the notebook. |
| `notebook_id` | String |  | Output only. Notebook id, which is the last segment of the notebook's resource name. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cmek_config` | String | Output only. CMEK-related information for the Notebook. |
| `name` | String | Identifier. The identifier of the notebook. Format: `projects/{project}/locations/{location}/notebooks/{notebook_id}`. This field must be a UTF-8 encoded string. |
| `title` | String | Optional. The title of the notebook. |
| `metadata` | String | Output only. The metadata of the notebook. |
| `sources` | Vec<String> | Output only. List of sources in the notebook. This is an output only field. |
| `emoji` | String | Output only. The emoji of the notebook. |
| `notebook_id` | String | Output only. Notebook id, which is the last segment of the notebook's resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook
notebook = provider.discoveryengine_api.Notebook {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access notebook outputs
notebook_id = notebook.id
notebook_cmek_config = notebook.cmek_config
notebook_name = notebook.name
notebook_title = notebook.title
notebook_metadata = notebook.metadata
notebook_sources = notebook.sources
notebook_emoji = notebook.emoji
notebook_notebook_id = notebook.notebook_id
```

---


### Branche

Retrieves a Branch.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_default` | bool | Output only. Indicates whether this branch is set as the default branch of its parent data store. |
| `last_document_import_time` | String | Output only. Timestamp of last import through DocumentService.ImportDocuments. Empty value means no import has been made to this branch. |
| `branch_stats` | String | Output only. Stistics describing a branch. This field is not populated in BranchView.BASIC view. |
| `display_name` | String | Output only. Human readable name of the branch to display in the UI. |
| `name` | String | Immutable. Full resource name of the branch, such as `projects/*/locations/global/dataStores/data_store/branches/branch_id`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access branche outputs
branche_id = branche.id
branche_is_default = branche.is_default
branche_last_document_import_time = branche.last_document_import_time
branche_branch_stats = branche.branch_stats
branche_display_name = branche.display_name
branche_name = branche.name
```

---


### Collection

Gets a Collection.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp the Collection was created at. |
| `data_connector` | String |  | Output only. The data connector, if present, manages the connection for data stores in the Collection. To set up the connector, use DataConnectorService.SetUpDataConnector method, which creates a new Collection while setting up the DataConnector singleton resource. Setting up connector on an existing Collection is not supported. This output only field contains a subset of the DataConnector fields, including `name`, `data_source`, `entities.entity_name` and `entities.data_store`. To get more details about a data connector, use the DataConnectorService.GetDataConnector method. |
| `display_name` | String |  | Required. The Collection display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `name` | String |  | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `name` | String | ✅ | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp the Collection was created at. |
| `data_connector` | String | Output only. The data connector, if present, manages the connection for data stores in the Collection. To set up the connector, use DataConnectorService.SetUpDataConnector method, which creates a new Collection while setting up the DataConnector singleton resource. Setting up connector on an existing Collection is not supported. This output only field contains a subset of the DataConnector fields, including `name`, `data_source`, `entities.entity_name` and `entities.data_store`. To get more details about a data connector, use the DataConnectorService.GetDataConnector method. |
| `display_name` | String | Required. The Collection display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `name` | String | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access collection outputs
collection_id = collection.id
collection_create_time = collection.create_time
collection_data_connector = collection.data_connector
collection_display_name = collection.display_name
collection_name = collection.name
```

---


### Agent

Creates an Agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Required. Human-readable description of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `a2a_agent_definition` | String |  | Optional. The behavior of the agent is defined as an A2A agent. |
| `starter_prompts` | Vec<String> |  | Optional. The starter prompt suggestions to show the user on the landing page of the agent. |
| `language_code` | String |  | Optional. The code of the language of the text in the description, display_name and starter_prompts fields. |
| `state` | String |  | Output only. The lifecycle state of the agent. |
| `icon` | String |  | Optional. The icon that represents the agent on the UI. |
| `suspension_reason` | String |  | Output only. The reason why the agent was suspended. Only set if the state is SUSPENDED. |
| `update_time` | String |  | Output only. Timestamp when this Agent was most recently updated. |
| `name` | String |  | Identifier. Resource name of the agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}` |
| `create_time` | String |  | Output only. Timestamp when this Agent was created. |
| `adk_agent_definition` | String |  | Optional. The behavior of the agent is defined as an ADK agent. |
| `authorization_config` | String |  | Optional. The authorizations that are required by the agent. |
| `deployment_failure_reason` | String |  | Output only. The reason why the agent deployment failed. Only set if the state is DEPLOYMENT_FAILED. |
| `rejection_reason` | String |  | Output only. The reason why the agent was rejected. Only set if the state is PRIVATE, and got there via rejection. |
| `custom_placeholder_text` | String |  | Optional. The custom placeholder text that appears in the text box before the user enters any text. |
| `display_name` | String |  | Required. Display name of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `dialogflow_agent_definition` | String |  | Optional. The behavior of the agent is defined as a Dialogflow agent. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Required. Human-readable description of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `a2a_agent_definition` | String | Optional. The behavior of the agent is defined as an A2A agent. |
| `starter_prompts` | Vec<String> | Optional. The starter prompt suggestions to show the user on the landing page of the agent. |
| `language_code` | String | Optional. The code of the language of the text in the description, display_name and starter_prompts fields. |
| `state` | String | Output only. The lifecycle state of the agent. |
| `icon` | String | Optional. The icon that represents the agent on the UI. |
| `suspension_reason` | String | Output only. The reason why the agent was suspended. Only set if the state is SUSPENDED. |
| `update_time` | String | Output only. Timestamp when this Agent was most recently updated. |
| `name` | String | Identifier. Resource name of the agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}` |
| `create_time` | String | Output only. Timestamp when this Agent was created. |
| `adk_agent_definition` | String | Optional. The behavior of the agent is defined as an ADK agent. |
| `authorization_config` | String | Optional. The authorizations that are required by the agent. |
| `deployment_failure_reason` | String | Output only. The reason why the agent deployment failed. Only set if the state is DEPLOYMENT_FAILED. |
| `rejection_reason` | String | Output only. The reason why the agent was rejected. Only set if the state is PRIVATE, and got there via rejection. |
| `custom_placeholder_text` | String | Optional. The custom placeholder text that appears in the text box before the user enters any text. |
| `display_name` | String | Required. Display name of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `dialogflow_agent_definition` | String | Optional. The behavior of the agent is defined as a Dialogflow agent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create agent
agent = provider.discoveryengine_api.Agent {
    parent = "value"  # Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}`
}

# Access agent outputs
agent_id = agent.id
agent_description = agent.description
agent_a2a_agent_definition = agent.a2a_agent_definition
agent_starter_prompts = agent.starter_prompts
agent_language_code = agent.language_code
agent_state = agent.state
agent_icon = agent.icon
agent_suspension_reason = agent.suspension_reason
agent_update_time = agent.update_time
agent_name = agent.name
agent_create_time = agent.create_time
agent_adk_agent_definition = agent.adk_agent_definition
agent_authorization_config = agent.authorization_config
agent_deployment_failure_reason = agent.deployment_failure_reason
agent_rejection_reason = agent.rejection_reason
agent_custom_placeholder_text = agent.custom_placeholder_text
agent_display_name = agent.display_name
agent_dialogflow_agent_definition = agent.dialogflow_agent_definition
```

---


### Suggestion_deny_list_entrie

Permanently deletes all SuggestionDenyListEntry for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suggestion_deny_list_entrie
suggestion_deny_list_entrie = provider.discoveryengine_api.Suggestion_deny_list_entrie {
    parent = "value"  # Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*.
}

```

---


### Schema

Creates a Schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `json_schema` | String |  | The JSON representation of the schema. |
| `name` | String |  | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `field_configs` | Vec<String> |  | Output only. Configurations for fields of the schema. |
| `struct_schema` | HashMap<String, String> |  | The structured representation of the schema. |
| `parent` | String | ✅ | Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `json_schema` | String | The JSON representation of the schema. |
| `name` | String | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `field_configs` | Vec<String> | Output only. Configurations for fields of the schema. |
| `struct_schema` | HashMap<String, String> | The structured representation of the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.discoveryengine_api.Schema {
    parent = "value"  # Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
}

# Access schema outputs
schema_id = schema.id
schema_json_schema = schema.json_schema
schema_name = schema.name
schema_field_configs = schema.field_configs
schema_struct_schema = schema.struct_schema
```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `display_name` | String |  | The display name of the User Store. |
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `display_name` | String | The display name of the User Store. |
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_store
user_store = provider.discoveryengine_api.User_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access user_store outputs
user_store_id = user_store.id
user_store_default_license_config = user_store.default_license_config
user_store_display_name = user_store.display_name
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_enable_license_auto_register = user_store.enable_license_auto_register
user_store_name = user_store.name
```

---


### User_event

Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Required. The filter string to specify the events to be deleted with a length limit of 5,000 characters. The eligible fields for filtering are: * `eventType`: Double quoted UserEvent.event_type string. * `eventTime`: in ISO 8601 "zulu" format. * `userPseudoId`: Double quoted string. Specifying this will delete all events associated with a visitor. * `userId`: Double quoted string. Specifying this will delete all events associated with a user. Note: This API only supports purging a max range of 30 days. Examples: * Deleting all events in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z"` * Deleting specific eventType in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z" eventType = "search"` * Deleting all events for a specific visitor in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z" userPseudoId = "visitor1024"` * Deleting the past 30 days of events inside a DataStore: `*` The filtering fields are assumed to have an implicit AND. |
| `force` | bool |  | The `force` field is currently not supported. Purge user event requests will permanently delete all purgeable events. Once the development is complete: If `force` is set to false, the method will return the expected purge count without deleting any user events. This field will default to false if not included in the request. |
| `parent` | String | ✅ | Required. The resource name of the catalog under which the events are created. The format is `projects/{project}/locations/global/collections/{collection}/dataStores/{dataStore}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.discoveryengine_api.User_event {
    parent = "value"  # Required. The resource name of the catalog under which the events are created. The format is `projects/{project}/locations/global/collections/{collection}/dataStores/{dataStore}`.
}

# Access user_event outputs
user_event_id = user_event.id
user_event_content_type = user_event.content_type
user_event_data = user_event.data
user_event_extensions = user_event.extensions
```

---


### File

Imports a file to an Agent. Currently only No-Code agents are supported.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mime_type` | String |  | Optional. The content type of the file, see https://www.iana.org/assignments/media-types/media-types.xhtml. This field is required when the data source does not provide the content type. |
| `file_name` | String |  | Required. The name of the file. |
| `parent` | String | ✅ | Required. The resource name of the Agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `files` | Vec<String> | The FileMetadatas. |
| `next_page_token` | String | A token to retrieve next page of results. Pass this value in the ListFilesRequest.page_token field in the subsequent call to `ListFiles` method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.discoveryengine_api.File {
    parent = "value"  # Required. The resource name of the Agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}`
}

# Access file outputs
file_id = file.id
file_files = file.files
file_next_page_token = file.next_page_token
```

---


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `state` | String |  | Output only. The states of the CmekConfig. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |
| `state` | String | Output only. The states of the CmekConfig. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cmek_config outputs
cmek_config_id = cmek_config.id
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
cmek_config_state = cmek_config.state
cmek_config_name = cmek_config.name
cmek_config_is_default = cmek_config.is_default
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_kms_key = cmek_config.kms_key
```

---


### User_license

Lists the User Licenses.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `user_licenses` | Vec<String> | All the customer's UserLicenses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_license outputs
user_license_id = user_license.id
user_license_next_page_token = user_license.next_page_token
user_license_user_licenses = user_license.user_licenses
```

---


### Audio_overview

Generates a new audio overview.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generation_options` | String |  | Options for the audio overview generation. |
| `parent` | String | ✅ | Required. The parent resource where this notebook will be created. Format: projects/{project}/locations/{location}/notebooks/{notebook} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create audio_overview
audio_overview = provider.discoveryengine_api.Audio_overview {
    parent = "value"  # Required. The parent resource where this notebook will be created. Format: projects/{project}/locations/{location}/notebooks/{notebook}
}

```

---


### Location

Estimates the data size to be used by a customer.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_data_source` | String |  | Structured or unstructured data. |
| `website_data_source` | String |  | Website data. |
| `location` | String | ✅ | Required. Full resource name of the location, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_usages` | Vec<String> | A list of metric usages, one for each requested resource type that has data in the requested time range. |
| `name` | String | Identifier. The name of the ConfigurablePricingUsageStats. Format: projects/{project}/locations/{location}/configurablePricingUsageStats |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.discoveryengine_api.Location {
    location = "value"  # Required. Full resource name of the location, such as `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_metric_usages = location.metric_usages
location_name = location.name
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `indexing_status` | String |  | Output only. Indexing status. |
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `update_time` | String | Output only. The target site's last updated time. |
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `failure_reason` | String | Output only. Failure reason. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `indexing_status` | String | Output only. Indexing status. |
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `provided_uri_pattern` | String | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target_site
target_site = provider.discoveryengine_api.Target_site {
    parent = "value"  # Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
}

# Access target_site outputs
target_site_id = target_site.id
target_site_exact_match = target_site.exact_match
target_site_type = target_site.type
target_site_update_time = target_site.update_time
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_name = target_site.name
target_site_failure_reason = target_site.failure_reason
target_site_root_domain_uri = target_site.root_domain_uri
target_site_indexing_status = target_site.indexing_status
target_site_site_verification_info = target_site.site_verification_info
target_site_provided_uri_pattern = target_site.provided_uri_pattern
```

---


### Billing_account_license_config

Distributes a LicenseConfig from billing account level to project level.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | Required. The target GCP project region to distribute the license config to. |
| `license_config_id` | String |  | Optional. Distribute seats to this license config instead of creating a new one. If not specified, a new license config will be created from the billing account license config. |
| `project_number` | String |  | Required. The target GCP project number to distribute the license config to. |
| `license_count` | String |  | Required. The number of licenses to distribute. |
| `billing_account_license_config` | String | ✅ | Required. Full resource name of BillingAccountLicenseConfig. Format: `billingAccounts/{billing_account}/billingAccountLicenseConfigs/{billing_account_license_config_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `license_count` | String | Required. The number of licenses purchased under this billing account license config. |
| `subscription_display_name` | String | The subscription display name. |
| `state` | String | Output only. The state of the BillingAccountLicenseConfig. |
| `end_date` | String | Optional. The planed subscription end date. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the billing account license config. Format: `billingAccounts/{billing_account}/billingAccountLicenseConfigs/{billing_account_license_config}`. |
| `start_date` | String | Required. The subscription start date. |
| `subscription_name` | String | Output only. The corresponding SubV3 subscription name. |
| `subscription_tier` | String | Required. The subscription tier. |
| `auto_renew` | bool | Whether the BillingAccountLicenseConfig is auto renewed when it reaches the end date. |
| `procurement_entitlement_id` | String | The procurement entitlement id of the subscription. |
| `subscription_term` | String | Required. The subscription term. |
| `gemini_bundle` | bool | Whether the license config is for Gemini bundle. |
| `license_config_distributions` | HashMap<String, String> | A map of LicenseConfig names to the number of licenses distributed to each. The key is the full resource name of the LicenseConfig, such as `projects/{project}/locations/{location}/licenseConfigs/{license_config}`. The value is the count of licenses allocated to it. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create billing_account_license_config
billing_account_license_config = provider.discoveryengine_api.Billing_account_license_config {
    billing_account_license_config = "value"  # Required. Full resource name of BillingAccountLicenseConfig. Format: `billingAccounts/{billing_account}/billingAccountLicenseConfigs/{billing_account_license_config_id}`.
}

# Access billing_account_license_config outputs
billing_account_license_config_id = billing_account_license_config.id
billing_account_license_config_license_count = billing_account_license_config.license_count
billing_account_license_config_subscription_display_name = billing_account_license_config.subscription_display_name
billing_account_license_config_state = billing_account_license_config.state
billing_account_license_config_end_date = billing_account_license_config.end_date
billing_account_license_config_name = billing_account_license_config.name
billing_account_license_config_start_date = billing_account_license_config.start_date
billing_account_license_config_subscription_name = billing_account_license_config.subscription_name
billing_account_license_config_subscription_tier = billing_account_license_config.subscription_tier
billing_account_license_config_auto_renew = billing_account_license_config.auto_renew
billing_account_license_config_procurement_entitlement_id = billing_account_license_config.procurement_entitlement_id
billing_account_license_config_subscription_term = billing_account_license_config.subscription_term
billing_account_license_config_gemini_bundle = billing_account_license_config.gemini_bundle
billing_account_license_config_license_config_distributions = billing_account_license_config.license_config_distributions
```

---


### Authorization

Creates an Authorization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the authorization. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String |  | Identifier. Resource name of the authorization. Format: `projects/{project}/locations/{location}/authorizations/{authorization}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `server_side_oauth2` | String |  | Server-side OAuth2 configuration. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the authorization. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String | Identifier. Resource name of the authorization. Format: `projects/{project}/locations/{location}/authorizations/{authorization}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `server_side_oauth2` | String | Server-side OAuth2 configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorization
authorization = provider.discoveryengine_api.Authorization {
    parent = "value"  # Required. The parent resource name. Format: `projects/{project}/locations/{location}`
}

# Access authorization outputs
authorization_id = authorization.id
authorization_display_name = authorization.display_name
authorization_name = authorization.name
authorization_server_side_oauth2 = authorization.server_side_oauth2
```

---


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time the conversation finished. |
| `messages` | Vec<String> |  | Conversation messages. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `start_time` | String |  | Output only. The time the conversation started. |
| `state` | String |  | The state of the Conversation. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time the conversation finished. |
| `messages` | Vec<String> | Conversation messages. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `start_time` | String | Output only. The time the conversation started. |
| `state` | String | The state of the Conversation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.discoveryengine_api.Conversation {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access conversation outputs
conversation_id = conversation.id
conversation_end_time = conversation.end_time
conversation_messages = conversation.messages
conversation_name = conversation.name
conversation_user_pseudo_id = conversation.user_pseudo_id
conversation_start_time = conversation.start_time
conversation_state = conversation.state
```

---


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `idp_config` | String |  | Output only. The identity provider configuration this is bound to translate the identity mapping entries within. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `idp_config` | String | Output only. The identity provider configuration this is bound to translate the identity mapping entries within. |
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identity_mapping_store
identity_mapping_store = provider.discoveryengine_api.Identity_mapping_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access identity_mapping_store outputs
identity_mapping_store_id = identity_mapping_store.id
identity_mapping_store_idp_config = identity_mapping_store.idp_config
identity_mapping_store_cmek_config = identity_mapping_store.cmek_config
identity_mapping_store_kms_key_name = identity_mapping_store.kms_key_name
identity_mapping_store_name = identity_mapping_store.name
```

---


### Media

Uploads a file for the assistant to use as a source of information within the session.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blob` | String |  | Information about the file being uploaded. |
| `media_request_info` | String |  | Media upload request metadata. |
| `name` | String | ✅ | Required. The resource name of the Session. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `token` | String | A unique fingerprint/version id for the media data |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `length` | String | Size of the data, in bytes |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `content_type` | String | MIME type of the data |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `inline` | String | Media data, set if reference_type is INLINE |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `download_parameters` | String | Parameters for a media download. |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `filename` | String | Original file name |
| `reference_type` | String | Describes what the field reference contains. |
| `bigstore_object_ref` | String | Use object_id instead. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.discoveryengine_api.Media {
    name = "value"  # Required. The resource name of the Session. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}`
}

# Access media outputs
media_id = media.id
media_blob_ref = media.blob_ref
media_path = media.path
media_composite_media = media.composite_media
media_token = media.token
media_hash = media.hash
media_length = media.length
media_crc32c_hash = media.crc32c_hash
media_diff_upload_response = media.diff_upload_response
media_hash_verified = media.hash_verified
media_content_type = media.content_type
media_md5_hash = media.md5_hash
media_diff_checksums_response = media.diff_checksums_response
media_blobstore2_info = media.blobstore2_info
media_is_potential_retry = media.is_potential_retry
media_sha1_hash = media.sha1_hash
media_timestamp = media.timestamp
media_inline = media.inline
media_algorithm = media.algorithm
media_cosmo_binary_reference = media.cosmo_binary_reference
media_diff_version_response = media.diff_version_response
media_object_id = media.object_id
media_diff_upload_request = media.diff_upload_request
media_sha256_hash = media.sha256_hash
media_media_id = media.media_id
media_diff_download_response = media.diff_download_response
media_download_parameters = media.download_parameters
media_content_type_info = media.content_type_info
media_filename = media.filename
media_reference_type = media.reference_type
media_bigstore_object_ref = media.bigstore_object_ref
```

---


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `language_info` | String |  | Language info for DataStore. |
| `natural_language_query_understanding_config` | String |  | Optional. Configuration for Natural Language Query Understanding. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `idp_config` | String |  | Output only. Data store level identity provider config. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `language_info` | String | Language info for DataStore. |
| `natural_language_query_understanding_config` | String | Optional. Configuration for Natural Language Query Understanding. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `idp_config` | String | Output only. Data store level identity provider config. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_store
data_store = provider.discoveryengine_api.Data_store {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access data_store outputs
data_store_id = data_store.id
data_store_starting_schema = data_store.starting_schema
data_store_create_time = data_store.create_time
data_store_industry_vertical = data_store.industry_vertical
data_store_billing_estimation = data_store.billing_estimation
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
data_store_document_processing_config = data_store.document_processing_config
data_store_acl_enabled = data_store.acl_enabled
data_store_language_info = data_store.language_info
data_store_natural_language_query_understanding_config = data_store.natural_language_query_understanding_config
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_workspace_config = data_store.workspace_config
data_store_kms_key_name = data_store.kms_key_name
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_display_name = data_store.display_name
data_store_name = data_store.name
data_store_advanced_site_search_config = data_store.advanced_site_search_config
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_cmek_config = data_store.cmek_config
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
data_store_default_schema_id = data_store.default_schema_id
data_store_content_config = data_store.content_config
data_store_idp_config = data_store.idp_config
data_store_solution_types = data_store.solution_types
```

---


### Data_connector

Starts an immediate synchronization process for a DataConnector. Third Party Connector Users must specify which entities should be synced. FHIR Connectors must provide a timestamp to indicate the point in time from which data should be synced.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force_refresh_content` | bool |  | Optional. Whether to force refresh the unstructured content of the documents. If set to `true`, the content part of the documents will be refreshed regardless of the update status of the referencing content. |
| `entities` | Vec<String> |  | Specifies which Third Party Connector entities should be synced. If not specified, all entities will be synced. |
| `healthcare_fhir_resource_types` | Vec<String> |  | The FHIR resource types to import. The resource types should be a subset of all [supported FHIR resource types](https://cloud.google.com/generative-ai-app-builder/docs/fhir-schema-reference#resource-level-specification). Default to all supported FHIR resource types if empty. |
| `sync_identity` | bool |  | If true, trigger Identity sync. |
| `sync_since_timestamp` | String |  | Timestamp to indicate the point in time from which data should be synced for Streaming/Batch Data Connectors. This field is only utilized for Healthcare Connectors. |
| `parent` | String | ✅ | Required. Connector name of the form projects/{project}/locations/{location}/collections/ {collection_id}/dataConnector |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `refresh_token_info` | String | Info about the stored refresh token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_connector
data_connector = provider.discoveryengine_api.Data_connector {
    parent = "value"  # Required. Connector name of the form projects/{project}/locations/{location}/collections/ {collection_id}/dataConnector
}

# Access data_connector outputs
data_connector_id = data_connector.id
data_connector_refresh_token_info = data_connector.refresh_token_info
```

---


### Assistant

Assists the user with a query in a streaming fashion.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session` | String |  | Optional. The session to use for the request. If specified, the assistant has access to the session history, and the query and the answer are stored there. If `-` is specified as the session ID, or it is left empty, then a new session is created with an automatically generated ID. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |
| `tools_spec` | String |  | Optional. Specification of tools that are used to serve the request. |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_policy` | String | Optional. Customer policy for the assistant. |
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assistant
assistant = provider.discoveryengine_api.Assistant {
    name = "value"  # Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}`
}

# Access assistant outputs
assistant_id = assistant.id
assistant_customer_policy = assistant.customer_policy
assistant_web_grounding_type = assistant.web_grounding_type
assistant_generation_config = assistant.generation_config
assistant_name = assistant.name
assistant_enabled_tools = assistant.enabled_tools
```

---


### Sample_query_set

Creates a SampleQuerySet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the SampleQuerySet. |
| `display_name` | String |  | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String |  | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String |  | Output only. Timestamp the SampleQuerySet was created at. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the SampleQuerySet. |
| `display_name` | String | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String | Output only. Timestamp the SampleQuerySet was created at. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sample_query_set
sample_query_set = provider.discoveryengine_api.Sample_query_set {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access sample_query_set outputs
sample_query_set_id = sample_query_set.id
sample_query_set_description = sample_query_set.description
sample_query_set_display_name = sample_query_set.display_name
sample_query_set_name = sample_query_set.name
sample_query_set_create_time = sample_query_set.create_time
```

---


### Analytic

Exports metrics.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_config` | String |  | Required. The output location of the data. |
| `analytics` | String | ✅ | Required. The analytics resource name under the engine where the metrics are created. The format is `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/analytics`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create analytic
analytic = provider.discoveryengine_api.Analytic {
    analytics = "value"  # Required. The analytics resource name under the engine where the metrics are created. The format is `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/analytics`.
}

```

---


### Widget_config

Gets a WidgetConfig.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_safe_search` | bool |  | Whether to enable safe search. |
| `enable_quality_feedback` | bool |  | Turn on or off collecting the search result quality feedback from end users. |
| `config_id` | String |  | Output only. Unique obfuscated identifier of a WidgetConfig. |
| `content_search_spec` | String |  | The content search spec that configs the desired behavior of content search. |
| `update_time` | String |  | Output only. Timestamp the WidgetConfig was updated. |
| `collection_components` | Vec<String> |  | Output only. Collection components that lists all collections and child data stores associated with the widget config, those data sources can be used for filtering in widget service APIs, users can return results that from selected data sources. |
| `enable_summarization` | bool |  | Turn on or off summarization for the search response. |
| `create_time` | String |  | Output only. Timestamp the WidgetConfig was created. |
| `minimum_data_term_accepted` | bool |  | Output only. Whether the customer accepted data use terms. |
| `result_display_type` | String |  | The type of snippet to display in UCS widget. - RESULT_DISPLAY_TYPE_UNSPECIFIED for existing users. - SNIPPET for new non-enterprise search users. - EXTRACTIVE_ANSWER for new enterprise search users. |
| `enable_web_app` | bool |  | Whether to enable standalone web app. |
| `access_settings` | String |  | Will be used for all widget access settings seen in cloud console integration page. Replaces top deprecated top level properties. |
| `name` | String |  | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `data_store_ui_configs` | Vec<String> |  | Configurable UI configurations per data store. |
| `enable_snippet_result_summary` | bool |  | Turn on or off summary for each snippets result. |
| `facet_field` | Vec<String> |  | The configuration and appearance of facets in the end user view. |
| `enable_private_knowledge_graph` | bool |  | Optional. Output only. Whether to enable private knowledge graph. |
| `ui_branding` | String |  | Describes search widget UI branding settings, such as the widget title, logo, favicons, and colors. |
| `solution_type` | String |  | Required. Immutable. Specifies the solution type that this WidgetConfig can be used for. |
| `customer_provided_config` | String |  | Optional. Output only. Describes the customer related configurations, currently only used for government customers. This field cannot be modified after project onboarding. |
| `homepage_setting` | String |  | Optional. Describes the homepage settings of the widget. |
| `default_search_request_order_by` | String |  | The default ordering for search results if specified. Used to set SearchRequest#order_by on applicable requests. https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1alpha/projects.locations.dataStores.servingConfigs/search#request-body |
| `allow_public_access` | bool |  | Whether allow no-auth integration with widget. If set true, public access to search or other solutions from widget is allowed without authenication token provided by customer hosted backend server. |
| `enable_result_score` | bool |  | Whether to show the result score. |
| `ui_settings` | String |  | Describes general widget search settings as seen in cloud console widget configuration page. Replaces top deprecated top level properties. |
| `llm_enabled` | bool |  | Output only. Whether LLM is enabled in the corresponding data store. |
| `assistant_settings` | String |  | Optional. Output only. Describes the assistant settings of the widget. |
| `data_store_type` | String |  | Output only. The type of the parent data store. |
| `enable_autocomplete` | bool |  | Whether or not to enable autocomplete. |
| `display_name` | String |  | Required. The human readable widget config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `fields_ui_components_map` | HashMap<String, String> |  | The key is the UI component. Mock. Currently supported `title`, `thumbnail`, `url`, `custom1`, `custom2`, `custom3`. The value is the name of the field along with its device visibility. The 3 custom fields are optional and can be added or removed. `title`, `thumbnail`, `url` are required UI components that cannot be removed. |
| `gemini_bundle` | bool |  | Output only. Whether the subscription is gemini bundle or not. |
| `enable_search_as_you_type` | bool |  | Whether to enable search-as-you-type behavior for the search widget |
| `enable_conversational_search` | bool |  | Whether to allow conversational search (LLM, multi-turn) or not (non-LLM, single-turn). |
| `industry_vertical` | String |  | Output only. The industry vertical that the WidgetConfig registers. The WidgetConfig industry vertical is based on the associated Engine. |
| `allowlisted_domains` | Vec<String> |  | Allowlisted domains that can load this widget. |
| `name` | String | ✅ | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_safe_search` | bool | Whether to enable safe search. |
| `enable_quality_feedback` | bool | Turn on or off collecting the search result quality feedback from end users. |
| `config_id` | String | Output only. Unique obfuscated identifier of a WidgetConfig. |
| `content_search_spec` | String | The content search spec that configs the desired behavior of content search. |
| `update_time` | String | Output only. Timestamp the WidgetConfig was updated. |
| `collection_components` | Vec<String> | Output only. Collection components that lists all collections and child data stores associated with the widget config, those data sources can be used for filtering in widget service APIs, users can return results that from selected data sources. |
| `enable_summarization` | bool | Turn on or off summarization for the search response. |
| `create_time` | String | Output only. Timestamp the WidgetConfig was created. |
| `minimum_data_term_accepted` | bool | Output only. Whether the customer accepted data use terms. |
| `result_display_type` | String | The type of snippet to display in UCS widget. - RESULT_DISPLAY_TYPE_UNSPECIFIED for existing users. - SNIPPET for new non-enterprise search users. - EXTRACTIVE_ANSWER for new enterprise search users. |
| `enable_web_app` | bool | Whether to enable standalone web app. |
| `access_settings` | String | Will be used for all widget access settings seen in cloud console integration page. Replaces top deprecated top level properties. |
| `name` | String | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `data_store_ui_configs` | Vec<String> | Configurable UI configurations per data store. |
| `enable_snippet_result_summary` | bool | Turn on or off summary for each snippets result. |
| `facet_field` | Vec<String> | The configuration and appearance of facets in the end user view. |
| `enable_private_knowledge_graph` | bool | Optional. Output only. Whether to enable private knowledge graph. |
| `ui_branding` | String | Describes search widget UI branding settings, such as the widget title, logo, favicons, and colors. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that this WidgetConfig can be used for. |
| `customer_provided_config` | String | Optional. Output only. Describes the customer related configurations, currently only used for government customers. This field cannot be modified after project onboarding. |
| `homepage_setting` | String | Optional. Describes the homepage settings of the widget. |
| `default_search_request_order_by` | String | The default ordering for search results if specified. Used to set SearchRequest#order_by on applicable requests. https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1alpha/projects.locations.dataStores.servingConfigs/search#request-body |
| `allow_public_access` | bool | Whether allow no-auth integration with widget. If set true, public access to search or other solutions from widget is allowed without authenication token provided by customer hosted backend server. |
| `enable_result_score` | bool | Whether to show the result score. |
| `ui_settings` | String | Describes general widget search settings as seen in cloud console widget configuration page. Replaces top deprecated top level properties. |
| `llm_enabled` | bool | Output only. Whether LLM is enabled in the corresponding data store. |
| `assistant_settings` | String | Optional. Output only. Describes the assistant settings of the widget. |
| `data_store_type` | String | Output only. The type of the parent data store. |
| `enable_autocomplete` | bool | Whether or not to enable autocomplete. |
| `display_name` | String | Required. The human readable widget config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `fields_ui_components_map` | HashMap<String, String> | The key is the UI component. Mock. Currently supported `title`, `thumbnail`, `url`, `custom1`, `custom2`, `custom3`. The value is the name of the field along with its device visibility. The 3 custom fields are optional and can be added or removed. `title`, `thumbnail`, `url` are required UI components that cannot be removed. |
| `gemini_bundle` | bool | Output only. Whether the subscription is gemini bundle or not. |
| `enable_search_as_you_type` | bool | Whether to enable search-as-you-type behavior for the search widget |
| `enable_conversational_search` | bool | Whether to allow conversational search (LLM, multi-turn) or not (non-LLM, single-turn). |
| `industry_vertical` | String | Output only. The industry vertical that the WidgetConfig registers. The WidgetConfig industry vertical is based on the associated Engine. |
| `allowlisted_domains` | Vec<String> | Allowlisted domains that can load this widget. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access widget_config outputs
widget_config_id = widget_config.id
widget_config_enable_safe_search = widget_config.enable_safe_search
widget_config_enable_quality_feedback = widget_config.enable_quality_feedback
widget_config_config_id = widget_config.config_id
widget_config_content_search_spec = widget_config.content_search_spec
widget_config_update_time = widget_config.update_time
widget_config_collection_components = widget_config.collection_components
widget_config_enable_summarization = widget_config.enable_summarization
widget_config_create_time = widget_config.create_time
widget_config_minimum_data_term_accepted = widget_config.minimum_data_term_accepted
widget_config_result_display_type = widget_config.result_display_type
widget_config_enable_web_app = widget_config.enable_web_app
widget_config_access_settings = widget_config.access_settings
widget_config_name = widget_config.name
widget_config_data_store_ui_configs = widget_config.data_store_ui_configs
widget_config_enable_snippet_result_summary = widget_config.enable_snippet_result_summary
widget_config_facet_field = widget_config.facet_field
widget_config_enable_private_knowledge_graph = widget_config.enable_private_knowledge_graph
widget_config_ui_branding = widget_config.ui_branding
widget_config_solution_type = widget_config.solution_type
widget_config_customer_provided_config = widget_config.customer_provided_config
widget_config_homepage_setting = widget_config.homepage_setting
widget_config_default_search_request_order_by = widget_config.default_search_request_order_by
widget_config_allow_public_access = widget_config.allow_public_access
widget_config_enable_result_score = widget_config.enable_result_score
widget_config_ui_settings = widget_config.ui_settings
widget_config_llm_enabled = widget_config.llm_enabled
widget_config_assistant_settings = widget_config.assistant_settings
widget_config_data_store_type = widget_config.data_store_type
widget_config_enable_autocomplete = widget_config.enable_autocomplete
widget_config_display_name = widget_config.display_name
widget_config_fields_ui_components_map = widget_config.fields_ui_components_map
widget_config_gemini_bundle = widget_config.gemini_bundle
widget_config_enable_search_as_you_type = widget_config.enable_search_as_you_type
widget_config_enable_conversational_search = widget_config.enable_conversational_search
widget_config_industry_vertical = widget_config.industry_vertical
widget_config_allowlisted_domains = widget_config.allowlisted_domains
```

---


### Chunk

Gets a Document.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotation_contents` | Vec<String> | Output only. Annotation contents if the current chunk contains annotations. |
| `annotation_metadata` | Vec<String> | Output only. The annotation metadata includes structured content in the current chunk. |
| `id` | String | Unique chunk ID of the current chunk. |
| `name` | String | The full resource name of the chunk. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}/chunks/{chunk_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `content` | String | Content is a string from a document (parsed content). |
| `relevance_score` | f64 | Output only. Represents the relevance score based on similarity. Higher score indicates higher chunk relevance. The score is in range [-1.0, 1.0]. Only populated on SearchResponse. |
| `chunk_metadata` | String | Output only. Metadata of the current chunk. |
| `data_urls` | Vec<String> | Output only. Image Data URLs if the current chunk contains images. Data URLs are composed of four parts: a prefix (data:), a MIME type indicating the type of data, an optional base64 token if non-textual, and the data itself: data:, |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `document_metadata` | String | Metadata of the document from the current chunk. |
| `page_span` | String | Page span of the chunk. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access chunk outputs
chunk_id = chunk.id
chunk_annotation_contents = chunk.annotation_contents
chunk_annotation_metadata = chunk.annotation_metadata
chunk_id = chunk.id
chunk_name = chunk.name
chunk_content = chunk.content
chunk_relevance_score = chunk.relevance_score
chunk_chunk_metadata = chunk.chunk_metadata
chunk_data_urls = chunk.data_urls
chunk_derived_struct_data = chunk.derived_struct_data
chunk_document_metadata = chunk.document_metadata
chunk_page_span = chunk.page_span
```

---


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_date` | String |  | Required. The start date. |
| `subscription_term` | String |  | Required. Subscription term. |
| `state` | String |  | Output only. The state of the license config. |
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `alert_policy_resource_config` | String |  | Optional. The alert policy config for this license config. |
| `end_date` | String |  | Optional. The planed end date. |
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_date` | String | Required. The start date. |
| `subscription_term` | String | Required. Subscription term. |
| `state` | String | Output only. The state of the license config. |
| `subscription_tier` | String | Required. Subscription tier information for the license config. |
| `license_count` | String | Required. Number of licenses purchased. |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `alert_policy_resource_config` | String | Optional. The alert policy config for this license config. |
| `end_date` | String | Optional. The planed end date. |
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create license_config
license_config = provider.discoveryengine_api.License_config {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access license_config outputs
license_config_id = license_config.id
license_config_start_date = license_config.start_date
license_config_subscription_term = license_config.subscription_term
license_config_state = license_config.state
license_config_subscription_tier = license_config.subscription_tier
license_config_license_count = license_config.license_count
license_config_free_trial = license_config.free_trial
license_config_auto_renew = license_config.auto_renew
license_config_name = license_config.name
license_config_alert_policy_resource_config = license_config.alert_policy_resource_config
license_config_end_date = license_config.end_date
license_config_gemini_bundle = license_config.gemini_bundle
```

---


### Completion_suggestion

Imports CompletionSuggestions for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Cloud Storage location for the input content. |
| `inline_source` | String |  | The Inline source for suggestion entries. |
| `bigquery_source` | String |  | BigQuery input source. |
| `error_config` | String |  | The desired location of errors incurred during the Import. |
| `parent` | String | ✅ | Required. The parent data store resource name for which to import customer autocomplete suggestions. Follows pattern `projects/*/locations/*/collections/*/dataStores/*` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_suggestion
completion_suggestion = provider.discoveryengine_api.Completion_suggestion {
    parent = "value"  # Required. The parent data store resource name for which to import customer autocomplete suggestions. Follows pattern `projects/*/locations/*/collections/*/dataStores/*`
}

```

---


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `acl_info` | String |  | Access control information for the document. |
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `acl_info` | String | Access control information for the document. |
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `schema_id` | String | The identifier of the schema located in the same data store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.discoveryengine_api.Document {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
}

# Access document outputs
document_id = document.id
document_derived_struct_data = document.derived_struct_data
document_content = document.content
document_name = document.name
document_json_data = document.json_data
document_index_status = document.index_status
document_index_time = document.index_time
document_acl_info = document.acl_info
document_struct_data = document.struct_data
document_parent_document_id = document.parent_document_id
document_id = document.id
document_schema_id = document.schema_id
```

---


### Custom_model

Gets a list of all the custom models.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `models` | Vec<String> | List of custom tuning models. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access custom_model outputs
custom_model_id = custom_model.id
custom_model_models = custom_model.models
```

---


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `end_time` | String |  | Output only. The time the session finished. |
| `state` | String |  | The state of the session. |
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `start_time` | String |  | Output only. The time the session started. |
| `turns` | Vec<String> |  | Turns. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `end_time` | String | Output only. The time the session finished. |
| `state` | String | The state of the session. |
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `start_time` | String | Output only. The time the session started. |
| `turns` | Vec<String> | Turns. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.discoveryengine_api.Session {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access session outputs
session_id = session.id
session_display_name = session.display_name
session_is_pinned = session.is_pinned
session_end_time = session.end_time
session_state = session.state
session_labels = session.labels
session_start_time = session.start_time
session_turns = session.turns
session_user_pseudo_id = session.user_pseudo_id
session_name = session.name
```

---


### Evaluation

Creates a Evaluation. Upon creation, the evaluation will be automatically triggered and begin execution.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quality_metrics` | String |  | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `error` | String |  | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String |  | Output only. The state of the evaluation. |
| `error_samples` | Vec<String> |  | Output only. A sample of errors encountered while processing the request. |
| `create_time` | String |  | Output only. Timestamp the Evaluation was created at. |
| `name` | String |  | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `end_time` | String |  | Output only. Timestamp the Evaluation was completed at. |
| `evaluation_spec` | String |  | Required. The specification of the evaluation. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quality_metrics` | String | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `error` | String | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String | Output only. The state of the evaluation. |
| `error_samples` | Vec<String> | Output only. A sample of errors encountered while processing the request. |
| `create_time` | String | Output only. Timestamp the Evaluation was created at. |
| `name` | String | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `end_time` | String | Output only. Timestamp the Evaluation was completed at. |
| `evaluation_spec` | String | Required. The specification of the evaluation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.discoveryengine_api.Evaluation {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_quality_metrics = evaluation.quality_metrics
evaluation_error = evaluation.error
evaluation_state = evaluation.state
evaluation_error_samples = evaluation.error_samples
evaluation_create_time = evaluation.create_time
evaluation_name = evaluation.name
evaluation_end_time = evaluation.end_time
evaluation_evaluation_spec = evaluation.evaluation_spec
```

---


### Sitemap

Creates a Sitemap.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The fully qualified resource name of the sitemap. `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine/sitemaps/*` The `sitemap_id` suffix is system-generated. |
| `create_time` | String |  | Output only. The sitemap's creation time. |
| `uri` | String |  | Public URI for the sitemap, e.g. `www.example.com/sitemap.xml`. |
| `parent` | String | ✅ | Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sitemaps_metadata` | Vec<String> | List of Sitemaps fetched. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sitemap
sitemap = provider.discoveryengine_api.Sitemap {
    parent = "value"  # Required. Parent resource name of the SiteSearchEngine, such as `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
}

# Access sitemap outputs
sitemap_id = sitemap.id
sitemap_sitemaps_metadata = sitemap.sitemaps_metadata
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `indexing_status` | String |  | Output only. Indexing status. |
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `indexing_status` | String | Output only. Indexing status. |
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `update_time` | String | Output only. The target site's last updated time. |
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
| `failure_reason` | String | Output only. Failure reason. |
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `provided_uri_pattern` | String | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create target_site
target_site = provider.discoveryengine_api.Target_site {
    parent = "value"  # Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
}

# Access target_site outputs
target_site_id = target_site.id
target_site_indexing_status = target_site.indexing_status
target_site_exact_match = target_site.exact_match
target_site_root_domain_uri = target_site.root_domain_uri
target_site_name = target_site.name
target_site_update_time = target_site.update_time
target_site_site_verification_info = target_site.site_verification_info
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_failure_reason = target_site.failure_reason
target_site_type = target_site.type
target_site_provided_uri_pattern = target_site.provided_uri_pattern
```

---


### Sample_query_set

Creates a SampleQuerySet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String |  | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `description` | String |  | The description of the SampleQuerySet. |
| `create_time` | String |  | Output only. Timestamp the SampleQuerySet was created at. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `description` | String | The description of the SampleQuerySet. |
| `create_time` | String | Output only. Timestamp the SampleQuerySet was created at. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sample_query_set
sample_query_set = provider.discoveryengine_api.Sample_query_set {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access sample_query_set outputs
sample_query_set_id = sample_query_set.id
sample_query_set_display_name = sample_query_set.display_name
sample_query_set_name = sample_query_set.name
sample_query_set_description = sample_query_set.description
sample_query_set_create_time = sample_query_set.create_time
```

---


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identity_mapping_store
identity_mapping_store = provider.discoveryengine_api.Identity_mapping_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access identity_mapping_store outputs
identity_mapping_store_id = identity_mapping_store.id
identity_mapping_store_cmek_config = identity_mapping_store.cmek_config
identity_mapping_store_kms_key_name = identity_mapping_store.kms_key_name
identity_mapping_store_name = identity_mapping_store.name
```

---


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `boost_action` | String |  | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `redirect_action` | String |  | Defines a redirect-type control. |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `boost_action` | String | Defines a boost-type control |
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `redirect_action` | String | Defines a redirect-type control. |
| `solution_type` | String | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.discoveryengine_api.Control {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
}

# Access control outputs
control_id = control.id
control_synonyms_action = control.synonyms_action
control_conditions = control.conditions
control_filter_action = control.filter_action
control_display_name = control.display_name
control_name = control.name
control_use_cases = control.use_cases
control_promote_action = control.promote_action
control_boost_action = control.boost_action
control_associated_serving_config_ids = control.associated_serving_config_ids
control_redirect_action = control.redirect_action
control_solution_type = control.solution_type
```

---


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `language_info` | String |  | Language info for DataStore. |
| `natural_language_query_understanding_config` | String |  | Optional. Configuration for Natural Language Query Understanding. |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `language_info` | String | Language info for DataStore. |
| `natural_language_query_understanding_config` | String | Optional. Configuration for Natural Language Query Understanding. |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_store
data_store = provider.discoveryengine_api.Data_store {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access data_store outputs
data_store_id = data_store.id
data_store_display_name = data_store.display_name
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_create_time = data_store.create_time
data_store_cmek_config = data_store.cmek_config
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_industry_vertical = data_store.industry_vertical
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_billing_estimation = data_store.billing_estimation
data_store_default_schema_id = data_store.default_schema_id
data_store_name = data_store.name
data_store_acl_enabled = data_store.acl_enabled
data_store_language_info = data_store.language_info
data_store_natural_language_query_understanding_config = data_store.natural_language_query_understanding_config
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_solution_types = data_store.solution_types
data_store_document_processing_config = data_store.document_processing_config
data_store_workspace_config = data_store.workspace_config
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
data_store_content_config = data_store.content_config
data_store_kms_key_name = data_store.kms_key_name
data_store_starting_schema = data_store.starting_schema
data_store_advanced_site_search_config = data_store.advanced_site_search_config
```

---


### Serving_config

Answer query method.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `search_spec` | String |  | Search specification. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `asynchronous_mode` | bool |  | Deprecated: This field is deprecated. Streaming Answer API will be supported. Asynchronous mode control. If enabled, the response will be returned with answer/session resource name without final answer. The API users need to do the polling to get the latest status of answer/session by calling ConversationalSearchService.GetAnswer or ConversationalSearchService.GetSession method. |
| `end_user_spec` | String |  | Optional. End user specification. |
| `query_understanding_spec` | String |  | Query understanding specification. |
| `query` | String |  | Required. Current user query. |
| `answer_generation_spec` | String |  | Answer generation specification. |
| `grounding_spec` | String |  | Optional. Grounding specification. |
| `safety_spec` | String |  | Model specification. |
| `session` | String |  | The session resource name. Not required. When session field is not set, the API is in sessionless mode. We support auto session mode: users can use the wildcard symbol `-` as session ID. A new ID will be automatically generated and assigned. |
| `related_questions_spec` | String |  | Related questions specification. |
| `user_pseudo_id` | String |  | A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `embedding_config` | String | Bring your own embedding config. The config is used for search semantic retrieval. The retrieval is based on the dot product of SearchRequest.EmbeddingSpec.EmbeddingVector.vector and the document embeddings that are provided by this EmbeddingConfig. If SearchRequest.EmbeddingSpec.EmbeddingVector.vector is provided, it overrides this ServingConfig.embedding_config. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `personalization_spec` | String | The specification for personalization spec. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set, SearchRequest.personalization_spec overrides ServingConfig.personalization_spec. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |
| `media_config` | String | The MediaConfig of the serving configuration. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `create_time` | String | Output only. ServingConfig created timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.discoveryengine_api.Serving_config {
    serving_config = "value"  # Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search.
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_name = serving_config.name
serving_config_embedding_config = serving_config.embedding_config
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_generic_config = serving_config.generic_config
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_solution_type = serving_config.solution_type
serving_config_update_time = serving_config.update_time
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_model_id = serving_config.model_id
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_media_config = serving_config.media_config
serving_config_diversity_level = serving_config.diversity_level
serving_config_display_name = serving_config.display_name
serving_config_create_time = serving_config.create_time
```

---


### Suggestion_deny_list_entrie

Permanently deletes all SuggestionDenyListEntry for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create suggestion_deny_list_entrie
suggestion_deny_list_entrie = provider.discoveryengine_api.Suggestion_deny_list_entrie {
    parent = "value"  # Required. The parent data store resource name for which to import denylist entries. Follows pattern projects/*/locations/*/collections/*/dataStores/*.
}

```

---


### Branche

Gets index freshness metadata for Documents. Supported for website search only.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents_metadata` | Vec<String> | The metadata of the Documents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access branche outputs
branche_id = branche.id
branche_documents_metadata = branche.documents_metadata
```

---


### User_license

Lists the User Licenses.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_licenses` | Vec<String> | All the customer's UserLicenses. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_license outputs
user_license_id = user_license.id
user_license_user_licenses = user_license.user_licenses
user_license_next_page_token = user_license.next_page_token
```

---


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String |  | Output only. The states of the CmekConfig. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String | Output only. The states of the CmekConfig. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cmek_config outputs
cmek_config_id = cmek_config.id
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_state = cmek_config.state
cmek_config_name = cmek_config.name
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_kms_key = cmek_config.kms_key
cmek_config_is_default = cmek_config.is_default
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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

# Create operation
operation = provider.discoveryengine_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Answer

Gets a Answer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The state of the answer generation. |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `citations` | Vec<String> | Citations. |
| `related_questions` | Vec<String> | Suggested related questions. |
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `query_understanding_info` | String | Query understanding information. |
| `answer_text` | String | The textual answer. |
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `steps` | Vec<String> | Answer generation steps. |
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |
| `create_time` | String | Output only. Answer creation timestamp. |
| `references` | Vec<String> | References. |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |
| `blob_attachments` | Vec<String> | List of blob attachments in the answer. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access answer outputs
answer_id = answer.id
answer_state = answer.state
answer_complete_time = answer.complete_time
answer_citations = answer.citations
answer_related_questions = answer.related_questions
answer_safety_ratings = answer.safety_ratings
answer_query_understanding_info = answer.query_understanding_info
answer_answer_text = answer.answer_text
answer_grounding_score = answer.grounding_score
answer_steps = answer.steps
answer_answer_skipped_reasons = answer.answer_skipped_reasons
answer_create_time = answer.create_time
answer_references = answer.references
answer_grounding_supports = answer.grounding_supports
answer_blob_attachments = answer.blob_attachments
answer_name = answer.name
```

---


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `acl_info` | String |  | Access control information for the document. |
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_id` | String | The identifier of the schema located in the same data store. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `acl_info` | String | Access control information for the document. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.discoveryengine_api.Document {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
}

# Access document outputs
document_id = document.id
document_schema_id = document.schema_id
document_content = document.content
document_id = document.id
document_index_status = document.index_status
document_struct_data = document.struct_data
document_index_time = document.index_time
document_acl_info = document.acl_info
document_derived_struct_data = document.derived_struct_data
document_name = document.name
document_parent_document_id = document.parent_document_id
document_json_data = document.json_data
```

---


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `subscription_term` | String |  | Required. Subscription term. |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `start_date` | String |  | Required. The start date. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `state` | String |  | Output only. The state of the license config. |
| `end_date` | String |  | Optional. The planed end date. |
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |
| `subscription_term` | String | Required. Subscription term. |
| `license_count` | String | Required. Number of licenses purchased. |
| `subscription_tier` | String | Required. Subscription tier information for the license config. |
| `start_date` | String | Required. The start date. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |
| `state` | String | Output only. The state of the license config. |
| `end_date` | String | Optional. The planed end date. |
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create license_config
license_config = provider.discoveryengine_api.License_config {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}`.
}

# Access license_config outputs
license_config_id = license_config.id
license_config_gemini_bundle = license_config.gemini_bundle
license_config_subscription_term = license_config.subscription_term
license_config_license_count = license_config.license_count
license_config_subscription_tier = license_config.subscription_tier
license_config_start_date = license_config.start_date
license_config_name = license_config.name
license_config_free_trial = license_config.free_trial
license_config_state = license_config.state
license_config_end_date = license_config.end_date
license_config_auto_renew = license_config.auto_renew
```

---


### Sample_querie

Creates a SampleQuery

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp the SampleQuery was created at. |
| `name` | String |  | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `query_entry` | String |  | The query entry. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/sampleQuerySets/{sampleQuerySet}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp the SampleQuery was created at. |
| `name` | String | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `query_entry` | String | The query entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sample_querie
sample_querie = provider.discoveryengine_api.Sample_querie {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/sampleQuerySets/{sampleQuerySet}`.
}

# Access sample_querie outputs
sample_querie_id = sample_querie.id
sample_querie_create_time = sample_querie.create_time
sample_querie_name = sample_querie.name
sample_querie_query_entry = sample_querie.query_entry
```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The display name of the User Store. |
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The display name of the User Store. |
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_store
user_store = provider.discoveryengine_api.User_store {
    parent = "value"  # Required. The parent collection resource name, such as `projects/{project}/locations/{location}`.
}

# Access user_store outputs
user_store_id = user_store.id
user_store_display_name = user_store.display_name
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_name = user_store.name
user_store_default_license_config = user_store.default_license_config
user_store_enable_license_auto_register = user_store.enable_license_auto_register
```

---


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Output only. The time the conversation started. |
| `state` | String |  | The state of the Conversation. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `messages` | Vec<String> |  | Conversation messages. |
| `end_time` | String |  | Output only. The time the conversation finished. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. The time the conversation started. |
| `state` | String | The state of the Conversation. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `messages` | Vec<String> | Conversation messages. |
| `end_time` | String | Output only. The time the conversation finished. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversation
conversation = provider.discoveryengine_api.Conversation {
    parent = "value"  # Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}`
}

# Access conversation outputs
conversation_id = conversation.id
conversation_start_time = conversation.start_time
conversation_state = conversation.state
conversation_user_pseudo_id = conversation.user_pseudo_id
conversation_messages = conversation.messages
conversation_end_time = conversation.end_time
conversation_name = conversation.name
```

---


### Location

Removes the dedicated crawl rate for a craw_rate_scope. If the dedicated crawl rate was set, this will disable vertex AI's crawl bot from using the dedicated crawl rate for crawling. If the dedicated crawl rate was not set, this is a no-op.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `crawl_rate_scope` | String |  | Required. The scope of the crawl rate change. Currently, only domain and host name are supported. A domain name example: `example.com`. A host name example: `www.example.com`. Please do not include `/` in the domain or host name. |
| `location` | String | ✅ | Required. The location resource where crawl rate management will be performed. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String | Output only. The states of the CmekConfig. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.discoveryengine_api.Location {
    location = "value"  # Required. The location resource where crawl rate management will be performed. Format: `projects/{project}/locations/{location}`
}

# Access location outputs
location_id = location.id
location_single_region_keys = location.single_region_keys
location_state = location.state
location_name = location.name
location_last_rotation_timestamp_micros = location.last_rotation_timestamp_micros
location_kms_key_version = location.kms_key_version
location_notebooklm_state = location.notebooklm_state
location_kms_key = location.kms_key
location_is_default = location.is_default
```

---


### Media

Downloads a file from the session.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | Path to the data, set if reference_type is PATH |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `filename` | String | Original file name |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `download_parameters` | String | Parameters for a media download. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `reference_type` | String | Describes what the field reference contains. |
| `content_type` | String | MIME type of the data |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `length` | String | Size of the data, in bytes |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `token` | String | A unique fingerprint/version id for the media data |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access media outputs
media_id = media.id
media_path = media.path
media_object_id = media.object_id
media_diff_upload_response = media.diff_upload_response
media_timestamp = media.timestamp
media_composite_media = media.composite_media
media_is_potential_retry = media.is_potential_retry
media_bigstore_object_ref = media.bigstore_object_ref
media_filename = media.filename
media_hash = media.hash
media_diff_checksums_response = media.diff_checksums_response
media_hash_verified = media.hash_verified
media_md5_hash = media.md5_hash
media_cosmo_binary_reference = media.cosmo_binary_reference
media_diff_upload_request = media.diff_upload_request
media_download_parameters = media.download_parameters
media_blob_ref = media.blob_ref
media_reference_type = media.reference_type
media_content_type = media.content_type
media_sha256_hash = media.sha256_hash
media_content_type_info = media.content_type_info
media_diff_version_response = media.diff_version_response
media_crc32c_hash = media.crc32c_hash
media_inline = media.inline
media_length = media.length
media_diff_download_response = media.diff_download_response
media_algorithm = media.algorithm
media_media_id = media.media_id
media_token = media.token
media_blobstore2_info = media.blobstore2_info
media_sha1_hash = media.sha1_hash
```

---


### Assistant

Assists the user with a query in a streaming fashion.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `tools_spec` | String |  | Optional. Specification of tools that are used to serve the request. |
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `session` | String |  | Optional. The session to use for the request. If specified, the assistant has access to the session history, and the query and the answer are stored there. If `-` is specified as the session ID, or it is left empty, then a new session is created with an automatically generated ID. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |
| `customer_policy` | String | Optional. Customer policy for the assistant. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assistant
assistant = provider.discoveryengine_api.Assistant {
    name = "value"  # Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}`
}

# Access assistant outputs
assistant_id = assistant.id
assistant_name = assistant.name
assistant_enabled_tools = assistant.enabled_tools
assistant_web_grounding_type = assistant.web_grounding_type
assistant_generation_config = assistant.generation_config
assistant_customer_policy = assistant.customer_policy
```

---


### Project

Provisions the project resource. During the process, related systems will get prepared and initialized. Caller must read the [Terms for data use](https://cloud.google.com/retail/data-use-terms), and optionally specify in request to provide consent to that service terms.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_use_terms_version` | String |  | Required. The version of the [Terms for data use](https://cloud.google.com/retail/data-use-terms) that caller has read and would like to give consent to. Acceptable version is `2022-11-23`, and this may change over time. |
| `saas_params` | String |  | Optional. Parameters for Agentspace. |
| `accept_data_use_terms` | bool |  | Required. Set to `true` to specify that caller has read and would like to give consent to the [Terms for data use](https://cloud.google.com/retail/data-use-terms). |
| `name` | String | ✅ | Required. Full resource name of a Project, such as `projects/{project_id_or_number}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.discoveryengine_api.Project {
    name = "value"  # Required. Full resource name of a Project, such as `projects/{project_id_or_number}`.
}

```

---


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ignore_record_details_in_response` | bool |  | If true, the response will contain only record ID and score. By default, it is false, the response will contain record details. |
| `top_n` | i64 |  | The number of results to return. If this is unset or no bigger than zero, returns all results. |
| `query` | String |  | The query to use. |
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `ranking_config` | String | ✅ | Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ranking_config
ranking_config = provider.discoveryengine_api.Ranking_config {
    ranking_config = "value"  # Required. The resource name of the rank service config, such as `projects/{project_num}/locations/{location}/rankingConfigs/default_ranking_config`.
}

```

---


### Completion_config

Completes the user input with advanced keyword suggestions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_model` | String |  | Specifies the autocomplete query model, which only applies to the QUERY SuggestionType. This overrides any model specified in the Configuration > Autocomplete section of the Cloud console. Currently supported values: * `document` - Using suggestions generated from user-imported documents. * `search-history` - Using suggestions generated from the past history of SearchService.Search API calls. Do not use it when there is no traffic for Search API. * `user-event` - Using suggestions generated from user-imported search events. * `document-completable` - Using suggestions taken directly from user-imported document fields marked as completable. Default values: * `document` is the default model for regular dataStores. * `search-history` is the default model for site search dataStores. |
| `suggestion_types` | Vec<String> |  | Optional. Suggestion types to return. If empty or unspecified, query suggestions are returned. Only one suggestion type is supported at the moment. |
| `experiment_ids` | Vec<String> |  | Optional. Experiment ids for this request. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `query` | String |  | Required. The typeahead input used to fetch suggestions. Maximum length is 128 characters. The query can not be empty for most of the suggestion types. If it is empty, an `INVALID_ARGUMENT` error is returned. The exception is when the suggestion_types contains only the type `RECENT_SEARCH`, the query can be an empty string. The is called "zero prefix" feature, which returns user's recently searched queries given the empty query. |
| `include_tail_suggestions` | bool |  | Indicates if tail suggestions should be returned if there are no suggestions that match the full query. Even if set to true, if there are suggestions that match the full query, those are returned and no tail suggestions are returned. |
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128 |
| `suggestion_type_specs` | Vec<String> |  | Optional. Specification of each suggestion type. |
| `boost_spec` | String |  | Optional. Specification to boost suggestions matching the condition. |
| `completion_config` | String | ✅ | Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_config
completion_config = provider.discoveryengine_api.Completion_config {
    completion_config = "value"  # Required. The completion_config of the parent dataStore or engine resource name for which the completion is performed, such as `projects/*/locations/global/collections/default_collection/dataStores/*/completionConfig` `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`.
}

```

---


### Custom_model

Gets a list of all the custom models.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `models` | Vec<String> | List of custom tuning models. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access custom_model outputs
custom_model_id = custom_model.id
custom_model_models = custom_model.models
```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `solution_type` | String |  | Required. The solutions of the engine. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `solution_type` | String | Required. The solutions of the engine. |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_metadata` | String | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create engine
engine = provider.discoveryengine_api.Engine {
    parent = "value"  # Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`.
}

# Access engine outputs
engine_id = engine.id
engine_display_name = engine.display_name
engine_disable_analytics = engine.disable_analytics
engine_common_config = engine.common_config
engine_create_time = engine.create_time
engine_data_store_ids = engine.data_store_ids
engine_app_type = engine.app_type
engine_name = engine.name
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_features = engine.features
engine_solution_type = engine.solution_type
engine_update_time = engine.update_time
engine_industry_vertical = engine.industry_vertical
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_chat_engine_config = engine.chat_engine_config
engine_search_engine_config = engine.search_engine_config
engine_chat_engine_metadata = engine.chat_engine_metadata
```

---


### Site_search_engine

Upgrade from basic site search to advanced site search.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_search_engine` | String | ✅ | Required. Full resource name of the SiteSearchEngine, such as `projects/{project}/locations/{location}/dataStores/{data_store_id}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |
| `total_size` | i64 | The total number of items matching the request. This will always be populated in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site_search_engine
site_search_engine = provider.discoveryengine_api.Site_search_engine {
    site_search_engine = "value"  # Required. Full resource name of the SiteSearchEngine, such as `projects/{project}/locations/{location}/dataStores/{data_store_id}/siteSearchEngine`.
}

# Access site_search_engine outputs
site_search_engine_id = site_search_engine.id
site_search_engine_next_page_token = site_search_engine.next_page_token
site_search_engine_target_sites = site_search_engine.target_sites
site_search_engine_total_size = site_search_engine.total_size
```

---


### User_event

Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Required. The filter string to specify the events to be deleted with a length limit of 5,000 characters. The eligible fields for filtering are: * `eventType`: Double quoted UserEvent.event_type string. * `eventTime`: in ISO 8601 "zulu" format. * `userPseudoId`: Double quoted string. Specifying this will delete all events associated with a visitor. * `userId`: Double quoted string. Specifying this will delete all events associated with a user. Note: This API only supports purging a max range of 30 days. Examples: * Deleting all events in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z"` * Deleting specific eventType in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z" eventType = "search"` * Deleting all events for a specific visitor in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z" userPseudoId = "visitor1024"` * Deleting the past 30 days of events inside a DataStore: `*` The filtering fields are assumed to have an implicit AND. |
| `force` | bool |  | The `force` field is currently not supported. Purge user event requests will permanently delete all purgeable events. Once the development is complete: If `force` is set to false, the method will return the expected purge count without deleting any user events. This field will default to false if not included in the request. |
| `parent` | String | ✅ | Required. The resource name of the catalog under which the events are created. The format is `projects/{project}/locations/global/collections/{collection}/dataStores/{dataStore}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.discoveryengine_api.User_event {
    parent = "value"  # Required. The resource name of the catalog under which the events are created. The format is `projects/{project}/locations/global/collections/{collection}/dataStores/{dataStore}`.
}

# Access user_event outputs
user_event_id = user_event.id
user_event_content_type = user_event.content_type
user_event_extensions = user_event.extensions
user_event_data = user_event.data
```

---


### Completion_suggestion

Permanently deletes all CompletionSuggestions for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent data store resource name for which to purge completion suggestions. Follows pattern projects/*/locations/*/collections/*/dataStores/*. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_suggestion
completion_suggestion = provider.discoveryengine_api.Completion_suggestion {
    parent = "value"  # Required. The parent data store resource name for which to purge completion suggestions. Follows pattern projects/*/locations/*/collections/*/dataStores/*.
}

```

---


### Grounding_config

Performs a grounding check.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grounding_spec` | String |  | Configuration of the grounding check. |
| `facts` | Vec<String> |  | List of facts for the grounding check. We support up to 200 facts. |
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `grounding_config` | String | ✅ | Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grounding_config
grounding_config = provider.discoveryengine_api.Grounding_config {
    grounding_config = "value"  # Required. The resource name of the grounding config, such as `projects/*/locations/global/groundingConfigs/default_grounding_config`.
}

```

---


### Schema

Creates a Schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `json_schema` | String |  | The JSON representation of the schema. |
| `name` | String |  | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> |  | The structured representation of the schema. |
| `parent` | String | ✅ | Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `json_schema` | String | The JSON representation of the schema. |
| `name` | String | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> | The structured representation of the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.discoveryengine_api.Schema {
    parent = "value"  # Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
}

# Access schema outputs
schema_id = schema.id
schema_json_schema = schema.json_schema
schema_name = schema.name
schema_struct_schema = schema.struct_schema
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple cmek_config resources
cmek_config_0 = provider.discoveryengine_api.Cmek_config {
    name = "value-0"
}
cmek_config_1 = provider.discoveryengine_api.Cmek_config {
    name = "value-1"
}
cmek_config_2 = provider.discoveryengine_api.Cmek_config {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cmek_config = provider.discoveryengine_api.Cmek_config {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Discoveryengine_api Documentation](https://cloud.google.com/discoveryengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
