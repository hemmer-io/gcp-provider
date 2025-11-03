# Discoveryengine_api Service



**Resources**: 112

---

## Overview

The discoveryengine_api service provides access to 112 resource types:

- [Sitemap](#sitemap) [CRD]
- [Collection](#collection) [RUD]
- [User_event](#user_event) [CR]
- [Project](#project) [C]
- [Location](#location) [CRU]
- [Serving_config](#serving_config) [CRU]
- [Ranking_config](#ranking_config) [C]
- [Branche](#branche) [R]
- [Completion_suggestion](#completion_suggestion) [C]
- [Answer](#answer) [R]
- [Schema](#schema) [CRUD]
- [Custom_model](#custom_model) [R]
- [License_config](#license_config) [CRU]
- [User_license](#user_license) [R]
- [Session](#session) [CRUD]
- [Conversation](#conversation) [CRUD]
- [Operation](#operation) [CR]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Grounding_config](#grounding_config) [C]
- [Completion_config](#completion_config) [C]
- [Cmek_config](#cmek_config) [RUD]
- [Site_search_engine](#site_search_engine) [CR]
- [Media](#media) [R]
- [Engine](#engine) [CRUD]
- [Data_store](#data_store) [CRUD]
- [Control](#control) [CRUD]
- [User_store](#user_store) [CRUD]
- [Document](#document) [CRUD]
- [Target_site](#target_site) [CRUD]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Assistant](#assistant) [CRU]
- [Source](#source) [CR]
- [Engine](#engine) [CRUD]
- [Assistant](#assistant) [CRU]
- [Sitemap](#sitemap) [CRD]
- [Cmek_config](#cmek_config) [RUD]
- [Location](#location) [CRU]
- [Sample_querie](#sample_querie) [CRUD]
- [Evaluation](#evaluation) [CR]
- [Grounding_config](#grounding_config) [C]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Branche](#branche) [R]
- [Target_site](#target_site) [CRUD]
- [Site_search_engine](#site_search_engine) [CR]
- [Completion_config](#completion_config) [C]
- [Custom_model](#custom_model) [R]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Completion_suggestion](#completion_suggestion) [C]
- [Schema](#schema) [CRUD]
- [User_store](#user_store) [CRUD]
- [Requirement](#requirement) [C]
- [Project](#project) [CRU]
- [Canned_querie](#canned_querie) [CRUD]
- [Chunk](#chunk) [R]
- [Sample_query_set](#sample_query_set) [CRUD]
- [Control](#control) [CRUD]
- [Data_store](#data_store) [CRUD]
- [Notebook](#notebook) [CR]
- [File](#file) [CR]
- [Billing_account_license_config](#billing_account_license_config) [CR]
- [Conversation](#conversation) [CRUD]
- [Data_connector](#data_connector) [CR]
- [Document](#document) [CRUD]
- [Widget_config](#widget_config) [RU]
- [Session](#session) [CRUD]
- [User_license](#user_license) [R]
- [Ranking_config](#ranking_config) [C]
- [Media](#media) [CR]
- [Audio_overview](#audio_overview) [CD]
- [Agent](#agent) [CRUD]
- [Connector_run](#connector_run) [R]
- [User_event](#user_event) [CR]
- [License_config](#license_config) [CRU]
- [Serving_config](#serving_config) [CRU]
- [Authorization](#authorization) [CRUD]
- [Answer](#answer) [R]
- [Analytic](#analytic) [C]
- [Collection](#collection) [RUD]
- [Operation](#operation) [CR]
- [Site_search_engine](#site_search_engine) [CR]
- [User_store](#user_store) [CRUD]
- [User_event](#user_event) [CR]
- [Cmek_config](#cmek_config) [RUD]
- [Schema](#schema) [CRUD]
- [License_config](#license_config) [CRU]
- [Target_site](#target_site) [CRUD]
- [Engine](#engine) [CRUD]
- [Operation](#operation) [CR]
- [Custom_model](#custom_model) [R]
- [Sample_query_set](#sample_query_set) [CRUD]
- [Project](#project) [C]
- [Document](#document) [CRUD]
- [Branche](#branche) [R]
- [Control](#control) [CRUD]
- [Evaluation](#evaluation) [CR]
- [Completion_config](#completion_config) [C]
- [Grounding_config](#grounding_config) [C]
- [User_license](#user_license) [R]
- [Ranking_config](#ranking_config) [C]
- [Conversation](#conversation) [CRUD]
- [Completion_suggestion](#completion_suggestion) [C]
- [Identity_mapping_store](#identity_mapping_store) [CRD]
- [Sample_querie](#sample_querie) [CRUD]
- [Sitemap](#sitemap) [CRD]
- [Suggestion_deny_list_entrie](#suggestion_deny_list_entrie) [C]
- [Data_store](#data_store) [CRUD]
- [Session](#session) [CRUD]
- [Media](#media) [R]
- [Answer](#answer) [R]
- [Location](#location) [CRU]
- [Assistant](#assistant) [CRU]
- [Serving_config](#serving_config) [CRU]

---

## Resources


### Sitemap

Creates a Sitemap.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | Public URI for the sitemap, e.g. `www.example.com/sitemap.xml`. |
| `create_time` | String |  | Output only. The sitemap's creation time. |
| `name` | String |  | Output only. The fully qualified resource name of the sitemap. `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine/sitemaps/*` The `sitemap_id` suffix is system-generated. |
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


### Collection

Gets the DataConnector. DataConnector is a singleton resource for each Collection.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the connector. |
| `connector_modes` | Vec<String> |  | Optional. The modes enabled for this connector. Default state is CONNECTOR_MODE_UNSPECIFIED. |
| `private_connectivity_project_id` | String |  | Output only. The tenant project ID associated with private connectivity connectors. This project must be allowlisted by in order for the connector to function. |
| `alert_policy_configs` | Vec<String> |  | Optional. The connector level alert config. |
| `errors` | Vec<String> |  | Output only. The errors from initialization or from the latest connector run. |
| `identity_schedule_config` | String |  | The configuration for the identity data synchronization runs. This contains the refresh interval to sync the Access Control List information for the documents ingested by this connector. |
| `refresh_interval` | String |  | Required. The refresh interval for data sync. If duration is set to 0, the data will be synced in real time. The streaming feature is not supported yet. The minimum is 30 minutes and maximum is 7 days. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `static_ip_addresses` | Vec<String> |  | Output only. The static IP addresses used by this connector. |
| `hybrid_ingestion_disabled` | bool |  | Optional. If the connector is a hybrid connector, determines whether ingestion is enabled and appropriate resources are provisioned during connector creation. If the connector is not a hybrid connector, this field is ignored. |
| `last_sync_time` | String |  | Output only. For periodic connectors only, the last time a data sync was completed. |
| `static_ip_enabled` | bool |  | Optional. Whether customer has enabled static IP addresses for this connector. |
| `create_time` | String |  | Output only. Timestamp the DataConnector was created at. |
| `federated_config` | String |  | Optional. Any params and credentials used specifically for hybrid connectors supporting FEDERATED mode. This field should only be set if the connector is a hybrid connector and we want to enable FEDERATED mode. |
| `auto_run_disabled` | bool |  | Optional. Indicates whether the connector is disabled for auto run. It can be used to pause periodical and real time sync. Update: with the introduction of incremental_sync_disabled, auto_run_disabled is used to pause/disable only full syncs |
| `destination_configs` | Vec<String> |  | Optional. Any target destinations used to connect to third-party services. |
| `incremental_sync_disabled` | bool |  | Optional. Indicates whether incremental syncs are paused for this connector. This is independent of auto_run_disabled. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled, i.e. set to true. |
| `identity_refresh_interval` | String |  | The refresh interval to sync the Access Control List information for the documents ingested by this connector. If not set, the access control list will be refreshed at the default interval of 30 minutes. The identity refresh interval can be at least 30 minutes and at most 7 days. |
| `realtime_state` | String |  | Output only. real-time sync state |
| `incremental_refresh_interval` | String |  | Optional. The refresh interval specifically for incremental data syncs. If unset, incremental syncs will use the default from env, set to 3hrs. The minimum is 30 minutes and maximum is 7 days. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `data_source` | String |  | Required. The name of the data source. Supported values: `salesforce`, `jira`, `confluence`, `bigquery`. |
| `bap_config` | String |  | Optional. The configuration for establishing a BAP connection. |
| `latest_pause_time` | String |  | Output only. The most recent timestamp when this DataConnector was paused, affecting all functionalities such as data synchronization. Pausing a connector has the following effects: - All functionalities, including data synchronization, are halted. - Any ongoing data synchronization job will be canceled. - No future data synchronization runs will be scheduled nor can be triggered. |
| `name` | String |  | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |
| `end_user_config` | String |  | Optional. Any params and credentials used specifically for EUA connectors. |
| `realtime_sync_config` | String |  | Optional. The configuration for realtime sync. |
| `next_sync_time` | String |  | Defines the scheduled time for the next data synchronization. This field requires hour , minute, and time_zone from the [IANA Time Zone Database](https://www.iana.org/time-zones). This is utilized when the data connector has a refresh interval greater than 1 day. When the hours or minutes are not specified, we will assume a sync time of 0:00. The user must provide a time zone to avoid ambiguity. |
| `params` | HashMap<String, String> |  | Required data connector parameters in structured json format. |
| `create_eua_saas` | bool |  | Optional. Whether the END USER AUTHENTICATION connector is created in SaaS. |
| `entities` | Vec<String> |  | List of entities from the connected data source to ingest. |
| `update_time` | String |  | Output only. Timestamp the DataConnector was last updated. |
| `remove_param_keys` | Vec<String> |  | Optional. Specifies keys to be removed from the 'params' field. This is only active when 'params' is included in the 'update_mask' in an UpdateDataConnectorRequest. Deletion takes precedence if a key is both in 'remove_param_keys' and present in the 'params' field of the request. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect the DataStores managed by this connector. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStores created by this connector will be protected by the KMS key. |
| `action_state` | String |  | Output only. State of the action connector. This reflects whether the action connector is initializing, active or has encountered errors. |
| `acl_enabled` | bool |  | Optional. Whether the connector will be created with an ACL config. Currently this field only affects Cloud Storage and BigQuery connectors. |
| `action_config` | String |  | Optional. Action configurations to make the connector support actions. |
| `json_params` | String |  | Required data connector parameters in json string format. |
| `connector_type` | String |  | Output only. The type of connector. Each source can only map to one type. For example, salesforce, confluence and jira have THIRD_PARTY connector type. It is not mutable once set by system. |
| `blocking_reasons` | Vec<String> |  | Output only. User actions that must be completed before the connector can start syncing data. |
| `sync_mode` | String |  | The data synchronization mode supported by the data connector. |
| `name` | String | ✅ | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the connector. |
| `connector_modes` | Vec<String> | Optional. The modes enabled for this connector. Default state is CONNECTOR_MODE_UNSPECIFIED. |
| `private_connectivity_project_id` | String | Output only. The tenant project ID associated with private connectivity connectors. This project must be allowlisted by in order for the connector to function. |
| `alert_policy_configs` | Vec<String> | Optional. The connector level alert config. |
| `errors` | Vec<String> | Output only. The errors from initialization or from the latest connector run. |
| `identity_schedule_config` | String | The configuration for the identity data synchronization runs. This contains the refresh interval to sync the Access Control List information for the documents ingested by this connector. |
| `refresh_interval` | String | Required. The refresh interval for data sync. If duration is set to 0, the data will be synced in real time. The streaming feature is not supported yet. The minimum is 30 minutes and maximum is 7 days. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `static_ip_addresses` | Vec<String> | Output only. The static IP addresses used by this connector. |
| `hybrid_ingestion_disabled` | bool | Optional. If the connector is a hybrid connector, determines whether ingestion is enabled and appropriate resources are provisioned during connector creation. If the connector is not a hybrid connector, this field is ignored. |
| `last_sync_time` | String | Output only. For periodic connectors only, the last time a data sync was completed. |
| `static_ip_enabled` | bool | Optional. Whether customer has enabled static IP addresses for this connector. |
| `create_time` | String | Output only. Timestamp the DataConnector was created at. |
| `federated_config` | String | Optional. Any params and credentials used specifically for hybrid connectors supporting FEDERATED mode. This field should only be set if the connector is a hybrid connector and we want to enable FEDERATED mode. |
| `auto_run_disabled` | bool | Optional. Indicates whether the connector is disabled for auto run. It can be used to pause periodical and real time sync. Update: with the introduction of incremental_sync_disabled, auto_run_disabled is used to pause/disable only full syncs |
| `destination_configs` | Vec<String> | Optional. Any target destinations used to connect to third-party services. |
| `incremental_sync_disabled` | bool | Optional. Indicates whether incremental syncs are paused for this connector. This is independent of auto_run_disabled. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled, i.e. set to true. |
| `identity_refresh_interval` | String | The refresh interval to sync the Access Control List information for the documents ingested by this connector. If not set, the access control list will be refreshed at the default interval of 30 minutes. The identity refresh interval can be at least 30 minutes and at most 7 days. |
| `realtime_state` | String | Output only. real-time sync state |
| `incremental_refresh_interval` | String | Optional. The refresh interval specifically for incremental data syncs. If unset, incremental syncs will use the default from env, set to 3hrs. The minimum is 30 minutes and maximum is 7 days. Applicable to only 3P connectors. When the refresh interval is set to the same value as the incremental refresh interval, incremental sync will be disabled. |
| `data_source` | String | Required. The name of the data source. Supported values: `salesforce`, `jira`, `confluence`, `bigquery`. |
| `bap_config` | String | Optional. The configuration for establishing a BAP connection. |
| `latest_pause_time` | String | Output only. The most recent timestamp when this DataConnector was paused, affecting all functionalities such as data synchronization. Pausing a connector has the following effects: - All functionalities, including data synchronization, are halted. - Any ongoing data synchronization job will be canceled. - No future data synchronization runs will be scheduled nor can be triggered. |
| `name` | String | Output only. The full resource name of the Data Connector. Format: `projects/*/locations/*/collections/*/dataConnector`. |
| `end_user_config` | String | Optional. Any params and credentials used specifically for EUA connectors. |
| `realtime_sync_config` | String | Optional. The configuration for realtime sync. |
| `next_sync_time` | String | Defines the scheduled time for the next data synchronization. This field requires hour , minute, and time_zone from the [IANA Time Zone Database](https://www.iana.org/time-zones). This is utilized when the data connector has a refresh interval greater than 1 day. When the hours or minutes are not specified, we will assume a sync time of 0:00. The user must provide a time zone to avoid ambiguity. |
| `params` | HashMap<String, String> | Required data connector parameters in structured json format. |
| `create_eua_saas` | bool | Optional. Whether the END USER AUTHENTICATION connector is created in SaaS. |
| `entities` | Vec<String> | List of entities from the connected data source to ingest. |
| `update_time` | String | Output only. Timestamp the DataConnector was last updated. |
| `remove_param_keys` | Vec<String> | Optional. Specifies keys to be removed from the 'params' field. This is only active when 'params' is included in the 'update_mask' in an UpdateDataConnectorRequest. Deletion takes precedence if a key is both in 'remove_param_keys' and present in the 'params' field of the request. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect the DataStores managed by this connector. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStores created by this connector will be protected by the KMS key. |
| `action_state` | String | Output only. State of the action connector. This reflects whether the action connector is initializing, active or has encountered errors. |
| `acl_enabled` | bool | Optional. Whether the connector will be created with an ACL config. Currently this field only affects Cloud Storage and BigQuery connectors. |
| `action_config` | String | Optional. Action configurations to make the connector support actions. |
| `json_params` | String | Required data connector parameters in json string format. |
| `connector_type` | String | Output only. The type of connector. Each source can only map to one type. For example, salesforce, confluence and jira have THIRD_PARTY connector type. It is not mutable once set by system. |
| `blocking_reasons` | Vec<String> | Output only. User actions that must be completed before the connector can start syncing data. |
| `sync_mode` | String | The data synchronization mode supported by the data connector. |


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
collection_state = collection.state
collection_connector_modes = collection.connector_modes
collection_private_connectivity_project_id = collection.private_connectivity_project_id
collection_alert_policy_configs = collection.alert_policy_configs
collection_errors = collection.errors
collection_identity_schedule_config = collection.identity_schedule_config
collection_refresh_interval = collection.refresh_interval
collection_static_ip_addresses = collection.static_ip_addresses
collection_hybrid_ingestion_disabled = collection.hybrid_ingestion_disabled
collection_last_sync_time = collection.last_sync_time
collection_static_ip_enabled = collection.static_ip_enabled
collection_create_time = collection.create_time
collection_federated_config = collection.federated_config
collection_auto_run_disabled = collection.auto_run_disabled
collection_destination_configs = collection.destination_configs
collection_incremental_sync_disabled = collection.incremental_sync_disabled
collection_identity_refresh_interval = collection.identity_refresh_interval
collection_realtime_state = collection.realtime_state
collection_incremental_refresh_interval = collection.incremental_refresh_interval
collection_data_source = collection.data_source
collection_bap_config = collection.bap_config
collection_latest_pause_time = collection.latest_pause_time
collection_name = collection.name
collection_end_user_config = collection.end_user_config
collection_realtime_sync_config = collection.realtime_sync_config
collection_next_sync_time = collection.next_sync_time
collection_params = collection.params
collection_create_eua_saas = collection.create_eua_saas
collection_entities = collection.entities
collection_update_time = collection.update_time
collection_remove_param_keys = collection.remove_param_keys
collection_kms_key_name = collection.kms_key_name
collection_action_state = collection.action_state
collection_acl_enabled = collection.acl_enabled
collection_action_config = collection.action_config
collection_json_params = collection.json_params
collection_connector_type = collection.connector_type
collection_blocking_reasons = collection.blocking_reasons
collection_sync_mode = collection.sync_mode
```

---


### User_event

Bulk import of user events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_source` | String |  | The Inline source for the input content for UserEvents. |
| `error_config` | String |  | The desired location of errors incurred during the Import. Cannot be set for inline user event imports. |
| `bigquery_source` | String |  | BigQuery input source. |
| `gcs_source` | String |  | Cloud Storage location for the input content. |
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


### Project

Provisions the project resource. During the process, related systems will get prepared and initialized. Caller must read the [Terms for data use](https://cloud.google.com/retail/data-use-terms), and optionally specify in request to provide consent to that service terms.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accept_data_use_terms` | bool |  | Required. Set to `true` to specify that caller has read and would like to give consent to the [Terms for data use](https://cloud.google.com/retail/data-use-terms). |
| `saas_params` | String |  | Optional. Parameters for Agentspace. |
| `data_use_terms_version` | String |  | Required. The version of the [Terms for data use](https://cloud.google.com/retail/data-use-terms) that caller has read and would like to give consent to. Acceptable version is `2022-11-23`, and this may change over time. |
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


### Location

Creates a Collection and sets up the DataConnector for it. To stop a DataConnector after setup, use the CollectionService.DeleteCollection method.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `collection_display_name` | String |  | Required. The display name of the Collection. Should be human readable, used to display collections in the Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `collection_id` | String |  | Required. The ID to use for the Collection, which will become the final component of the Collection's resource name. A new Collection is created as part of the DataConnector setup. DataConnector is a singleton resource under Collection, managing all DataStores of the Collection. This field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `data_connector` | String |  | Required. The DataConnector to initialize in the newly created Collection. |
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


### Serving_config

Answer query method (streaming). It takes one AnswerQueryRequest and returns multiple AnswerQueryResponse messages in a stream.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grounding_spec` | String |  | Optional. Grounding specification. |
| `session` | String |  | The session resource name. Not required. When session field is not set, the API is in sessionless mode. We support auto session mode: users can use the wildcard symbol `-` as session ID. A new ID will be automatically generated and assigned. |
| `user_pseudo_id` | String |  | A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `related_questions_spec` | String |  | Related questions specification. |
| `answer_generation_spec` | String |  | Answer generation specification. |
| `end_user_spec` | String |  | Optional. End user specification. |
| `asynchronous_mode` | bool |  | Deprecated: This field is deprecated. Streaming Answer API will be supported. Asynchronous mode control. If enabled, the response will be returned with answer/session resource name without final answer. The API users need to do the polling to get the latest status of answer/session by calling ConversationalSearchService.GetAnswer or ConversationalSearchService.GetSession method. |
| `safety_spec` | String |  | Model specification. |
| `query_understanding_spec` | String |  | Query understanding specification. |
| `search_spec` | String |  | Search specification. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `query` | String |  | Required. Current user query. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `media_config` | String | The MediaConfig of the serving configuration. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `create_time` | String | Output only. ServingConfig created timestamp. |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |


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
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_name = serving_config.name
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_update_time = serving_config.update_time
serving_config_display_name = serving_config.display_name
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_generic_config = serving_config.generic_config
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_diversity_level = serving_config.diversity_level
serving_config_model_id = serving_config.model_id
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_media_config = serving_config.media_config
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_create_time = serving_config.create_time
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_solution_type = serving_config.solution_type
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
```

---


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `top_n` | i64 |  | The number of results to return. If this is unset or no bigger than zero, returns all results. |
| `query` | String |  | The query to use. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `ignore_record_details_in_response` | bool |  | If true, the response will contain only record ID and score. By default, it is false, the response will contain record details. |
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
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


### Completion_suggestion

Imports CompletionSuggestions for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_source` | String |  | The Inline source for suggestion entries. |
| `gcs_source` | String |  | Cloud Storage location for the input content. |
| `error_config` | String |  | The desired location of errors incurred during the Import. |
| `bigquery_source` | String |  | BigQuery input source. |
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


### Answer

Gets a Answer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |
| `citations` | Vec<String> | Citations. |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `create_time` | String | Output only. Answer creation timestamp. |
| `state` | String | The state of the answer generation. |
| `references` | Vec<String> | References. |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `answer_text` | String | The textual answer. |
| `query_understanding_info` | String | Query understanding information. |
| `steps` | Vec<String> | Answer generation steps. |
| `related_questions` | Vec<String> | Suggested related questions. |


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
answer_name = answer.name
answer_grounding_score = answer.grounding_score
answer_answer_skipped_reasons = answer.answer_skipped_reasons
answer_citations = answer.citations
answer_grounding_supports = answer.grounding_supports
answer_safety_ratings = answer.safety_ratings
answer_create_time = answer.create_time
answer_state = answer.state
answer_references = answer.references
answer_complete_time = answer.complete_time
answer_answer_text = answer.answer_text
answer_query_understanding_info = answer.query_understanding_info
answer_steps = answer.steps
answer_related_questions = answer.related_questions
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


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `end_date` | String |  | Optional. The planed end date. |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `state` | String |  | Output only. The state of the license config. |
| `subscription_term` | String |  | Required. Subscription term. |
| `start_date` | String |  | Required. The start date. |
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `license_count` | String | Required. Number of licenses purchased. |
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |
| `end_date` | String | Optional. The planed end date. |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `state` | String | Output only. The state of the license config. |
| `subscription_term` | String | Required. Subscription term. |
| `start_date` | String | Required. The start date. |
| `subscription_tier` | String | Required. Subscription tier information for the license config. |


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
license_config_auto_renew = license_config.auto_renew
license_config_license_count = license_config.license_count
license_config_gemini_bundle = license_config.gemini_bundle
license_config_end_date = license_config.end_date
license_config_free_trial = license_config.free_trial
license_config_name = license_config.name
license_config_state = license_config.state
license_config_subscription_term = license_config.subscription_term
license_config_start_date = license_config.start_date
license_config_subscription_tier = license_config.subscription_tier
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


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `start_time` | String |  | Output only. The time the session started. |
| `turns` | Vec<String> |  | Turns. |
| `state` | String |  | The state of the session. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `end_time` | String |  | Output only. The time the session finished. |
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `start_time` | String | Output only. The time the session started. |
| `turns` | Vec<String> | Turns. |
| `state` | String | The state of the session. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `end_time` | String | Output only. The time the session finished. |
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |


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
session_user_pseudo_id = session.user_pseudo_id
session_labels = session.labels
session_start_time = session.start_time
session_turns = session.turns
session_state = session.state
session_is_pinned = session.is_pinned
session_name = session.name
session_end_time = session.end_time
session_display_name = session.display_name
```

---


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `messages` | Vec<String> |  | Conversation messages. |
| `end_time` | String |  | Output only. The time the conversation finished. |
| `start_time` | String |  | Output only. The time the conversation started. |
| `state` | String |  | The state of the Conversation. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | Conversation messages. |
| `end_time` | String | Output only. The time the conversation finished. |
| `start_time` | String | Output only. The time the conversation started. |
| `state` | String | The state of the Conversation. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |


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
conversation_messages = conversation.messages
conversation_end_time = conversation.end_time
conversation_start_time = conversation.start_time
conversation_state = conversation.state
conversation_name = conversation.name
conversation_user_pseudo_id = conversation.user_pseudo_id
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
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
| `grounding_spec` | String |  | Configuration of the grounding check. |
| `facts` | Vec<String> |  | List of facts for the grounding check. We support up to 200 facts. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
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


### Completion_config

Completes the user input with advanced keyword suggestions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suggestion_types` | Vec<String> |  | Optional. Suggestion types to return. If empty or unspecified, query suggestions are returned. Only one suggestion type is supported at the moment. |
| `boost_spec` | String |  | Optional. Specification to boost suggestions matching the condition. |
| `experiment_ids` | Vec<String> |  | Optional. Experiment ids for this request. |
| `include_tail_suggestions` | bool |  | Indicates if tail suggestions should be returned if there are no suggestions that match the full query. Even if set to true, if there are suggestions that match the full query, those are returned and no tail suggestions are returned. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `query_model` | String |  | Specifies the autocomplete query model, which only applies to the QUERY SuggestionType. This overrides any model specified in the Configuration > Autocomplete section of the Cloud console. Currently supported values: * `document` - Using suggestions generated from user-imported documents. * `search-history` - Using suggestions generated from the past history of SearchService.Search API calls. Do not use it when there is no traffic for Search API. * `user-event` - Using suggestions generated from user-imported search events. * `document-completable` - Using suggestions taken directly from user-imported document fields marked as completable. Default values: * `document` is the default model for regular dataStores. * `search-history` is the default model for site search dataStores. |
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128 |
| `suggestion_type_specs` | Vec<String> |  | Optional. Specification of each suggestion type. |
| `query` | String |  | Required. The typeahead input used to fetch suggestions. Maximum length is 128 characters. The query can not be empty for most of the suggestion types. If it is empty, an `INVALID_ARGUMENT` error is returned. The exception is when the suggestion_types contains only the type `RECENT_SEARCH`, the query can be an empty string. The is called "zero prefix" feature, which returns user's recently searched queries given the empty query. |
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


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The states of the CmekConfig. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The states of the CmekConfig. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
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
cmek_config_state = cmek_config.state
cmek_config_name = cmek_config.name
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_kms_key = cmek_config.kms_key
cmek_config_is_default = cmek_config.is_default
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
```

---


### Site_search_engine

Verify target sites' ownership and validity. This API sends all the target sites under site search engine for verification.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The parent resource shared by all TargetSites being verified. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `total_size` | i64 | The total number of items matching the request. This will always be populated in the response. |
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |


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
    parent = "value"  # Required. The parent resource shared by all TargetSites being verified. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
}

# Access site_search_engine outputs
site_search_engine_id = site_search_engine.id
site_search_engine_next_page_token = site_search_engine.next_page_token
site_search_engine_total_size = site_search_engine.total_size
site_search_engine_target_sites = site_search_engine.target_sites
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
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `filename` | String | Original file name |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `reference_type` | String | Describes what the field reference contains. |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `download_parameters` | String | Parameters for a media download. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `token` | String | A unique fingerprint/version id for the media data |
| `length` | String | Size of the data, in bytes |
| `content_type` | String | MIME type of the data |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |


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
media_timestamp = media.timestamp
media_media_id = media.media_id
media_sha1_hash = media.sha1_hash
media_composite_media = media.composite_media
media_diff_version_response = media.diff_version_response
media_filename = media.filename
media_md5_hash = media.md5_hash
media_reference_type = media.reference_type
media_diff_upload_request = media.diff_upload_request
media_download_parameters = media.download_parameters
media_inline = media.inline
media_diff_checksums_response = media.diff_checksums_response
media_hash_verified = media.hash_verified
media_hash = media.hash
media_cosmo_binary_reference = media.cosmo_binary_reference
media_path = media.path
media_content_type_info = media.content_type_info
media_algorithm = media.algorithm
media_is_potential_retry = media.is_potential_retry
media_blobstore2_info = media.blobstore2_info
media_object_id = media.object_id
media_diff_upload_response = media.diff_upload_response
media_bigstore_object_ref = media.bigstore_object_ref
media_sha256_hash = media.sha256_hash
media_crc32c_hash = media.crc32c_hash
media_token = media.token
media_length = media.length
media_content_type = media.content_type
media_diff_download_response = media.diff_download_response
media_blob_ref = media.blob_ref
```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `solution_type` | String |  | Required. The solutions of the engine. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `solution_type` | String | Required. The solutions of the engine. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
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
engine_search_engine_config = engine.search_engine_config
engine_solution_type = engine.solution_type
engine_app_type = engine.app_type
engine_chat_engine_config = engine.chat_engine_config
engine_disable_analytics = engine.disable_analytics
engine_display_name = engine.display_name
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_create_time = engine.create_time
engine_common_config = engine.common_config
engine_industry_vertical = engine.industry_vertical
engine_features = engine.features
engine_update_time = engine.update_time
engine_name = engine.name
engine_data_store_ids = engine.data_store_ids
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_chat_engine_metadata = engine.chat_engine_metadata
```

---


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |


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
data_store_industry_vertical = data_store.industry_vertical
data_store_create_time = data_store.create_time
data_store_document_processing_config = data_store.document_processing_config
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_kms_key_name = data_store.kms_key_name
data_store_content_config = data_store.content_config
data_store_starting_schema = data_store.starting_schema
data_store_cmek_config = data_store.cmek_config
data_store_workspace_config = data_store.workspace_config
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_acl_enabled = data_store.acl_enabled
data_store_advanced_site_search_config = data_store.advanced_site_search_config
data_store_name = data_store.name
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_display_name = data_store.display_name
data_store_solution_types = data_store.solution_types
data_store_billing_estimation = data_store.billing_estimation
data_store_default_schema_id = data_store.default_schema_id
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
```

---


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `boost_action` | String |  | Defines a boost-type control |
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `redirect_action` | String |  | Defines a redirect-type control. |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `boost_action` | String | Defines a boost-type control |
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `redirect_action` | String | Defines a redirect-type control. |
| `solution_type` | String | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |


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
control_associated_serving_config_ids = control.associated_serving_config_ids
control_boost_action = control.boost_action
control_display_name = control.display_name
control_filter_action = control.filter_action
control_name = control.name
control_redirect_action = control.redirect_action
control_solution_type = control.solution_type
control_promote_action = control.promote_action
control_synonyms_action = control.synonyms_action
control_use_cases = control.use_cases
control_conditions = control.conditions
```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `display_name` | String |  | The display name of the User Store. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `display_name` | String | The display name of the User Store. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |


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
user_store_enable_license_auto_register = user_store.enable_license_auto_register
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_display_name = user_store.display_name
user_store_name = user_store.name
user_store_default_license_config = user_store.default_license_config
```

---


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String |  | Access control information for the document. |
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String | Access control information for the document. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
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
document_index_status = document.index_status
document_id = document.id
document_name = document.name
document_struct_data = document.struct_data
document_acl_info = document.acl_info
document_derived_struct_data = document.derived_struct_data
document_json_data = document.json_data
document_index_time = document.index_time
document_parent_document_id = document.parent_document_id
document_content = document.content
document_schema_id = document.schema_id
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `indexing_status` | String |  | Output only. Indexing status. |
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `indexing_status` | String | Output only. Indexing status. |
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `failure_reason` | String | Output only. Failure reason. |
| `provided_uri_pattern` | String | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `update_time` | String | Output only. The target site's last updated time. |


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
target_site_site_verification_info = target_site.site_verification_info
target_site_type = target_site.type
target_site_exact_match = target_site.exact_match
target_site_indexing_status = target_site.indexing_status
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_root_domain_uri = target_site.root_domain_uri
target_site_name = target_site.name
target_site_failure_reason = target_site.failure_reason
target_site_provided_uri_pattern = target_site.provided_uri_pattern
target_site_update_time = target_site.update_time
```

---


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |


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
identity_mapping_store_cmek_config = identity_mapping_store.cmek_config
identity_mapping_store_kms_key_name = identity_mapping_store.kms_key_name
```

---


### Assistant

Assists the user with a query in a streaming fashion.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tools_spec` | String |  | Optional. Specification of tools that are used to serve the request. |
| `session` | String |  | Optional. The session to use for the request. If specified, the assistant has access to the session history, and the query and the answer are stored there. If `-` is specified as the session ID, or it is left empty, then a new session is created with an automatically generated ID. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |
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
assistant_web_grounding_type = assistant.web_grounding_type
assistant_generation_config = assistant.generation_config
assistant_enabled_tools = assistant.enabled_tools
assistant_customer_policy = assistant.customer_policy
```

---


### Source

Deletes multiple sources

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `names` | Vec<String> |  | Required. Names of sources to be deleted. Format: projects/{project}/locations/{location}/notebooks/{notebook}/sources/{source} |
| `parent` | String | ✅ | Required. The parent resource where the sources will be deleted. Format: projects/{project}/locations/{location}/notebooks/{notebook} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_id` | String | Optional. Output only. Source id, which is the last segment of the source's resource name. |
| `metadata` | String | Output only. Metadata about the source. |
| `name` | String | Identifier. The full resource name of the source. Format: `projects/{project}/locations/{location}/notebooks/{notebook}/sources/{source_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `settings` | String | Output only. Status of the source, and any failure reasons. |
| `title` | String | Optional. Title of the source. |


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
    parent = "value"  # Required. The parent resource where the sources will be deleted. Format: projects/{project}/locations/{location}/notebooks/{notebook}
}

# Access source outputs
source_id = source.id
source_source_id = source.source_id
source_metadata = source.metadata
source_name = source.name
source_settings = source.settings
source_title = source.title
```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `similar_documents_config` | String |  | Additional config specs for a `similar-items` engine. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `recommendation_metadata` | String |  | Output only. Additional information of a recommendation engine. Only applicable if solution_type is SOLUTION_TYPE_RECOMMENDATION. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `solution_type` | String |  | Required. The solutions of the engine. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `similar_documents_config` | String | Additional config specs for a `similar-items` engine. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `recommendation_metadata` | String | Output only. Additional information of a recommendation engine. Only applicable if solution_type is SOLUTION_TYPE_RECOMMENDATION. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `chat_engine_metadata` | String | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `solution_type` | String | Required. The solutions of the engine. |


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
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_similar_documents_config = engine.similar_documents_config
engine_disable_analytics = engine.disable_analytics
engine_display_name = engine.display_name
engine_recommendation_metadata = engine.recommendation_metadata
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_search_engine_config = engine.search_engine_config
engine_update_time = engine.update_time
engine_industry_vertical = engine.industry_vertical
engine_name = engine.name
engine_data_store_ids = engine.data_store_ids
engine_chat_engine_metadata = engine.chat_engine_metadata
engine_features = engine.features
engine_create_time = engine.create_time
engine_chat_engine_config = engine.chat_engine_config
engine_common_config = engine.common_config
engine_app_type = engine.app_type
engine_solution_type = engine.solution_type
```

---


### Assistant

Assists the user with a query in a streaming fashion.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tools_spec` | String |  | Optional. Specification of tools that are used to serve the request. |
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `session` | String |  | Optional. The session to use for the request. If specified, the assistant has access to the session history, and the query and the answer are stored there. If `-` is specified as the session ID, or it is left empty, then a new session is created with an automatically generated ID. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |
| `customer_policy` | String | Optional. Customer policy for the assistant. |
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |


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
assistant_web_grounding_type = assistant.web_grounding_type
assistant_generation_config = assistant.generation_config
assistant_enabled_tools = assistant.enabled_tools
assistant_customer_policy = assistant.customer_policy
assistant_name = assistant.name
```

---


### Sitemap

Creates a Sitemap.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The sitemap's creation time. |
| `name` | String |  | Output only. The fully qualified resource name of the sitemap. `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine/sitemaps/*` The `sitemap_id` suffix is system-generated. |
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


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `state` | String |  | Output only. The states of the CmekConfig. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `state` | String | Output only. The states of the CmekConfig. |


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
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_name = cmek_config.name
cmek_config_is_default = cmek_config.is_default
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
cmek_config_kms_key = cmek_config.kms_key
cmek_config_state = cmek_config.state
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
| `name` | String | Identifier. The name of the ConfigurablePricingUsageStats. Format: projects/{project}/locations/{location}/configurablePricingUsageStats |
| `metric_usages` | Vec<String> | A list of metric usages, one for each requested resource type that has data in the requested time range. |


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
location_name = location.name
location_metric_usages = location.metric_usages
```

---


### Sample_querie

Creates a SampleQuery

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_entry` | String |  | The query entry. |
| `name` | String |  | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String |  | Output only. Timestamp the SampleQuery was created at. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/sampleQuerySets/{sampleQuerySet}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_entry` | String | The query entry. |
| `name` | String | Identifier. The full resource name of the sample query, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}/sampleQueries/{sample_query}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String | Output only. Timestamp the SampleQuery was created at. |


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
sample_querie_query_entry = sample_querie.query_entry
sample_querie_name = sample_querie.name
sample_querie_create_time = sample_querie.create_time
```

---


### Evaluation

Creates a Evaluation. Upon creation, the evaluation will be automatically triggered and begin execution.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quality_metrics` | String |  | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `create_time` | String |  | Output only. Timestamp the Evaluation was created at. |
| `error_samples` | Vec<String> |  | Output only. A sample of errors encountered while processing the request. |
| `evaluation_spec` | String |  | Required. The specification of the evaluation. |
| `name` | String |  | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `error` | String |  | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `end_time` | String |  | Output only. Timestamp the Evaluation was completed at. |
| `state` | String |  | Output only. The state of the evaluation. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quality_metrics` | String | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `create_time` | String | Output only. Timestamp the Evaluation was created at. |
| `error_samples` | Vec<String> | Output only. A sample of errors encountered while processing the request. |
| `evaluation_spec` | String | Required. The specification of the evaluation. |
| `name` | String | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `error` | String | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `end_time` | String | Output only. Timestamp the Evaluation was completed at. |
| `state` | String | Output only. The state of the evaluation. |


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
evaluation_create_time = evaluation.create_time
evaluation_error_samples = evaluation.error_samples
evaluation_evaluation_spec = evaluation.evaluation_spec
evaluation_name = evaluation.name
evaluation_error = evaluation.error
evaluation_end_time = evaluation.end_time
evaluation_state = evaluation.state
```

---


### Grounding_config

Performs a grounding check.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
| `grounding_spec` | String |  | Configuration of the grounding check. |
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


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `idp_config` | String |  | Output only. The identity provider configuration this is bound to translate the identity mapping entries within. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |
| `idp_config` | String | Output only. The identity provider configuration this is bound to translate the identity mapping entries within. |


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
identity_mapping_store_idp_config = identity_mapping_store.idp_config
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
| `display_name` | String | Output only. Human readable name of the branch to display in the UI. |
| `is_default` | bool | Output only. Indicates whether this branch is set as the default branch of its parent data store. |
| `last_document_import_time` | String | Output only. Timestamp of last import through DocumentService.ImportDocuments. Empty value means no import has been made to this branch. |
| `name` | String | Immutable. Full resource name of the branch, such as `projects/*/locations/global/dataStores/data_store/branches/branch_id`. |
| `branch_stats` | String | Output only. Stistics describing a branch. This field is not populated in BranchView.BASIC view. |


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
branche_display_name = branche.display_name
branche_is_default = branche.is_default
branche_last_document_import_time = branche.last_document_import_time
branche_name = branche.name
branche_branch_stats = branche.branch_stats
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `indexing_status` | String |  | Output only. Indexing status. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `failure_reason` | String | Output only. Failure reason. |
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `indexing_status` | String | Output only. Indexing status. |
| `update_time` | String | Output only. The target site's last updated time. |
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
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
target_site_name = target_site.name
target_site_type = target_site.type
target_site_failure_reason = target_site.failure_reason
target_site_site_verification_info = target_site.site_verification_info
target_site_indexing_status = target_site.indexing_status
target_site_update_time = target_site.update_time
target_site_exact_match = target_site.exact_match
target_site_root_domain_uri = target_site.root_domain_uri
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_provided_uri_pattern = target_site.provided_uri_pattern
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
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
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
site_search_engine_target_sites = site_search_engine.target_sites
site_search_engine_next_page_token = site_search_engine.next_page_token
site_search_engine_total_size = site_search_engine.total_size
```

---


### Completion_config

Removes the search history suggestion in an engine for a user. This will remove the suggestion from being returned in the AdvancedCompleteQueryResponse.recent_search_suggestions for this user. If the user searches the same suggestion again, the new history will override and suggest this suggestion again.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remove_time` | String |  | Required. Time at which the suggestion was removed. If not set, the current time will be used. |
| `remove_all_search_history_suggestions` | bool |  | Remove all search history suggestions for the user. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `user_pseudo_id` | String |  | Required. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128. |
| `search_history_suggestion` | String |  | The search history suggestion to be removed. |
| `completion_config` | String | ✅ | Required. The completion_config of the parent engine resource name for which the search history suggestion is to be removed, such as `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`. |



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
    completion_config = "value"  # Required. The completion_config of the parent engine resource name for which the search history suggestion is to be removed, such as `projects/*/locations/global/collections/default_collection/engines/*/completionConfig`.
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


### Suggestion_deny_list_entrie

Imports all SuggestionDenyListEntry for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Cloud Storage location for the input content. Only 1 file can be specified that contains all entries to import. Supported values `gcs_source.schema` for autocomplete suggestion deny list entry imports: * `suggestion_deny_list` (default): One JSON [SuggestionDenyListEntry] per line. |
| `inline_source` | String |  | The Inline source for the input content for suggestion deny list entries. |
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


### Completion_suggestion

Imports CompletionSuggestions for a DataStore.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Cloud Storage location for the input content. |
| `error_config` | String |  | The desired location of errors incurred during the Import. |
| `inline_source` | String |  | The Inline source for suggestion entries. |
| `bigquery_source` | String |  | BigQuery input source. |
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


### Schema

Creates a Schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `struct_schema` | HashMap<String, String> |  | The structured representation of the schema. |
| `name` | String |  | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `field_configs` | Vec<String> |  | Output only. Configurations for fields of the schema. |
| `json_schema` | String |  | The JSON representation of the schema. |
| `parent` | String | ✅ | Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `struct_schema` | HashMap<String, String> | The structured representation of the schema. |
| `name` | String | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `field_configs` | Vec<String> | Output only. Configurations for fields of the schema. |
| `json_schema` | String | The JSON representation of the schema. |


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
schema_struct_schema = schema.struct_schema
schema_name = schema.name
schema_field_configs = schema.field_configs
schema_json_schema = schema.json_schema
```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `display_name` | String |  | The display name of the User Store. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `display_name` | String | The display name of the User Store. |


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
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_enable_license_auto_register = user_store.enable_license_auto_register
user_store_name = user_store.name
user_store_default_license_config = user_store.default_license_config
user_store_display_name = user_store.display_name
```

---


### Requirement

Check a particular requirement.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> |  | The resources to be checked for this requirement. The type needed for the monitored resources: * `discoveryengine.googleapis.com/Branch`. * The labels needed for this resource: * `project_number` * `location_id` * `collection_id` * `datastore_id` * `branch_id` * `discoveryengine.googleapis.com/DataStore` * The labels needed for this resource: * `project_number` * `location_id` * `collection_id` * `datastore_id` |
| `requirement_type` | String |  | The type specifying the requirement to check. The supported types are: * `discoveryengine.googleapis.com/media_recs/general/all/warning` * `discoveryengine.googleapis.com/media_recs/oyml/cvr/warning` * `discoveryengine.googleapis.com/media_recs/rfy/cvr/warning` * `discoveryengine.googleapis.com/media_recs/mlt/cvr/warning` * `discoveryengine.googleapis.com/media_recs/mp/cvr/warning` * `discoveryengine.googleapis.com/media_recs/oyml/wdps/warning` * `discoveryengine.googleapis.com/media_recs/rfy/wdps/warning` * `discoveryengine.googleapis.com/media_recs/mlt/wdps/warning` |
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


### Project

Updates service terms for this project. This method can be used to retroactively accept the latest terms. Terms available for update: * [Terms for data use](https://cloud.google.com/retail/data-use-terms)

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_term_version` | String |  | Required. The version string of the terms of service to update. |
| `service_term_id` | String |  | Required. The unique identifier of the terms of service to update. Available term ids: * `GA_DATA_USE_TERMS`: [Terms for data use](https://cloud.google.com/retail/data-use-terms). When using this service term id, the acceptable service_term_version to provide is `2022-11-23`. |
| `consent_change_action` | String |  | Required. Whether customer decides to accept or decline service term. At this moment, only accept action is supported. |
| `project` | String | ✅ | Required. Full resource name of a Project, such as `projects/{project_id_or_number}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_provided_config` | String | Optional. Customer provided configurations. |
| `provision_completion_time` | String | Output only. The timestamp when this project is successfully provisioned. Empty value means this project is still provisioning and is not ready for use. |
| `create_time` | String | Output only. The timestamp when this project is created. |
| `configurable_billing_status` | String | Output only. The current status of the project's configurable billing. |
| `name` | String | Output only. Full resource name of the project, for example `projects/{project}`. Note that when making requests, project number and project id are both acceptable, but the server will always respond in project number. |
| `service_terms_map` | HashMap<String, String> | Output only. A map of terms of services. The key is the `id` of ServiceTerms. |


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
    project = "value"  # Required. Full resource name of a Project, such as `projects/{project_id_or_number}`.
}

# Access project outputs
project_id = project.id
project_customer_provided_config = project.customer_provided_config
project_provision_completion_time = project.provision_completion_time
project_create_time = project.create_time
project_configurable_billing_status = project.configurable_billing_status
project_name = project.name
project_service_terms_map = project.service_terms_map
```

---


### Canned_querie

Creates a CannedQuery.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `required_capabilities` | Vec<String> |  | Optional. The capabilities the Assistant needs to have to use this canned query. |
| `name` | String |  | Immutable. Resource name of the canned query. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/cannedQueries/{canned_query}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `google_defined` | bool |  | Output only. Whether this is a Google-defined, read-only canned query. |
| `default_texts` | String |  | Required. The default (non-localized) values for the text attributes. |
| `enabled` | bool |  | Whether this canned query is enabled. |
| `localized_texts` | HashMap<String, String> |  | Optional. The translations of the text attributes. The keys should be BCP-47 language codes. |
| `display_name` | String |  | The display name of the canned query. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `required_capabilities` | Vec<String> | Optional. The capabilities the Assistant needs to have to use this canned query. |
| `name` | String | Immutable. Resource name of the canned query. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/cannedQueries/{canned_query}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `google_defined` | bool | Output only. Whether this is a Google-defined, read-only canned query. |
| `default_texts` | String | Required. The default (non-localized) values for the text attributes. |
| `enabled` | bool | Whether this canned query is enabled. |
| `localized_texts` | HashMap<String, String> | Optional. The translations of the text attributes. The keys should be BCP-47 language codes. |
| `display_name` | String | The display name of the canned query. It must be a UTF-8 encoded string with a length limit of 128 characters. |


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
canned_querie_required_capabilities = canned_querie.required_capabilities
canned_querie_name = canned_querie.name
canned_querie_google_defined = canned_querie.google_defined
canned_querie_default_texts = canned_querie.default_texts
canned_querie_enabled = canned_querie.enabled
canned_querie_localized_texts = canned_querie.localized_texts
canned_querie_display_name = canned_querie.display_name
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
| `data_urls` | Vec<String> | Output only. Image Data URLs if the current chunk contains images. Data URLs are composed of four parts: a prefix (data:), a MIME type indicating the type of data, an optional base64 token if non-textual, and the data itself: data:, |
| `annotation_metadata` | Vec<String> | Output only. The annotation metadata includes structured content in the current chunk. |
| `chunk_metadata` | String | Output only. Metadata of the current chunk. |
| `document_metadata` | String | Metadata of the document from the current chunk. |
| `name` | String | The full resource name of the chunk. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}/chunks/{chunk_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `content` | String | Content is a string from a document (parsed content). |
| `page_span` | String | Page span of the chunk. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `relevance_score` | f64 | Output only. Represents the relevance score based on similarity. Higher score indicates higher chunk relevance. The score is in range [-1.0, 1.0]. Only populated on SearchResponse. |
| `annotation_contents` | Vec<String> | Output only. Annotation contents if the current chunk contains annotations. |
| `id` | String | Unique chunk ID of the current chunk. |


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
chunk_data_urls = chunk.data_urls
chunk_annotation_metadata = chunk.annotation_metadata
chunk_chunk_metadata = chunk.chunk_metadata
chunk_document_metadata = chunk.document_metadata
chunk_name = chunk.name
chunk_content = chunk.content
chunk_page_span = chunk.page_span
chunk_derived_struct_data = chunk.derived_struct_data
chunk_relevance_score = chunk.relevance_score
chunk_annotation_contents = chunk.annotation_contents
chunk_id = chunk.id
```

---


### Sample_query_set

Creates a SampleQuerySet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the SampleQuerySet. |
| `create_time` | String |  | Output only. Timestamp the SampleQuerySet was created at. |
| `display_name` | String |  | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String |  | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the SampleQuerySet. |
| `create_time` | String | Output only. Timestamp the SampleQuerySet was created at. |
| `display_name` | String | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `name` | String | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


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
sample_query_set_create_time = sample_query_set.create_time
sample_query_set_display_name = sample_query_set.display_name
sample_query_set_name = sample_query_set.name
```

---


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `redirect_action` | String |  | Defines a redirect-type control. |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `boost_action` | String |  | Defines a boost-type control |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `redirect_action` | String | Defines a redirect-type control. |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `boost_action` | String | Defines a boost-type control |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
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
control_associated_serving_config_ids = control.associated_serving_config_ids
control_synonyms_action = control.synonyms_action
control_redirect_action = control.redirect_action
control_conditions = control.conditions
control_display_name = control.display_name
control_filter_action = control.filter_action
control_boost_action = control.boost_action
control_use_cases = control.use_cases
control_promote_action = control.promote_action
control_name = control.name
control_solution_type = control.solution_type
```

---


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `language_info` | String |  | Language info for DataStore. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `idp_config` | String |  | Output only. Data store level identity provider config. |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `natural_language_query_understanding_config` | String |  | Optional. Configuration for Natural Language Query Understanding. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `language_info` | String | Language info for DataStore. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `idp_config` | String | Output only. Data store level identity provider config. |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |
| `natural_language_query_understanding_config` | String | Optional. Configuration for Natural Language Query Understanding. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |


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
data_store_industry_vertical = data_store.industry_vertical
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
data_store_document_processing_config = data_store.document_processing_config
data_store_display_name = data_store.display_name
data_store_kms_key_name = data_store.kms_key_name
data_store_content_config = data_store.content_config
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_solution_types = data_store.solution_types
data_store_workspace_config = data_store.workspace_config
data_store_language_info = data_store.language_info
data_store_name = data_store.name
data_store_default_schema_id = data_store.default_schema_id
data_store_cmek_config = data_store.cmek_config
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_create_time = data_store.create_time
data_store_idp_config = data_store.idp_config
data_store_billing_estimation = data_store.billing_estimation
data_store_starting_schema = data_store.starting_schema
data_store_advanced_site_search_config = data_store.advanced_site_search_config
data_store_natural_language_query_understanding_config = data_store.natural_language_query_understanding_config
data_store_acl_enabled = data_store.acl_enabled
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
```

---


### Notebook

Creates a notebook.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notebook_id` | String |  | Output only. Notebook id, which is the last segment of the notebook's resource name. |
| `sources` | Vec<String> |  | Output only. List of sources in the notebook. This is an output only field. |
| `title` | String |  | Optional. The title of the notebook. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Notebook. |
| `name` | String |  | Identifier. The identifier of the notebook. Format: `projects/{project}/locations/{location}/notebooks/{notebook_id}`. This field must be a UTF-8 encoded string. |
| `emoji` | String |  | Output only. The emoji of the notebook. |
| `metadata` | String |  | Output only. The metadata of the notebook. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notebook_id` | String | Output only. Notebook id, which is the last segment of the notebook's resource name. |
| `sources` | Vec<String> | Output only. List of sources in the notebook. This is an output only field. |
| `title` | String | Optional. The title of the notebook. |
| `cmek_config` | String | Output only. CMEK-related information for the Notebook. |
| `name` | String | Identifier. The identifier of the notebook. Format: `projects/{project}/locations/{location}/notebooks/{notebook_id}`. This field must be a UTF-8 encoded string. |
| `emoji` | String | Output only. The emoji of the notebook. |
| `metadata` | String | Output only. The metadata of the notebook. |


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
notebook_notebook_id = notebook.notebook_id
notebook_sources = notebook.sources
notebook_title = notebook.title
notebook_cmek_config = notebook.cmek_config
notebook_name = notebook.name
notebook_emoji = notebook.emoji
notebook_metadata = notebook.metadata
```

---


### File

Imports a file to an Agent. Currently only No-Code agents are supported.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_name` | String |  | Required. The name of the file. |
| `mime_type` | String |  | Optional. The content type of the file, see https://www.iana.org/assignments/media-types/media-types.xhtml. This field is required when the data source does not provide the content type. |
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


### Billing_account_license_config

This method is called from the billing account side to retract the LicenseConfig from the given project back to the billing account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `license_count` | String |  | Optional. The number of licenses to retract. Only used when full_retract is false. |
| `license_config` | String |  | Required. Full resource name of LicenseConfig. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config_id}`. |
| `full_retract` | bool |  | Optional. If set to true, retract the entire license config. Otherwise, retract the specified license count. |
| `billing_account_license_config` | String | ✅ | Required. Full resource name of BillingAccountLicenseConfig. Format: `billingAccounts/{billing_account}/billingAccountLicenseConfigs/{billing_account_license_config_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_date` | String | Optional. The planed subscription end date. |
| `subscription_name` | String | Output only. The corresponding SubV3 subscription name. |
| `subscription_term` | String | Required. The subscription term. |
| `gemini_bundle` | bool | Whether the license config is for Gemini bundle. |
| `procurement_entitlement_id` | String | The procurement entitlement id of the subscription. |
| `subscription_tier` | String | Required. The subscription tier. |
| `start_date` | String | Required. The subscription start date. |
| `auto_renew` | bool | Whether the BillingAccountLicenseConfig is auto renewed when it reaches the end date. |
| `license_config_distributions` | HashMap<String, String> | A map of LicenseConfig names to the number of licenses distributed to each. The key is the full resource name of the LicenseConfig, such as `projects/{project}/locations/{location}/licenseConfigs/{license_config}`. The value is the count of licenses allocated to it. |
| `license_count` | String | Required. The number of licenses purchased under this billing account license config. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the billing account license config. Format: `billingAccounts/{billing_account}/billingAccountLicenseConfigs/{billing_account_license_config}`. |
| `state` | String | Output only. The state of the BillingAccountLicenseConfig. |
| `subscription_display_name` | String | The subscription display name. |


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
billing_account_license_config_end_date = billing_account_license_config.end_date
billing_account_license_config_subscription_name = billing_account_license_config.subscription_name
billing_account_license_config_subscription_term = billing_account_license_config.subscription_term
billing_account_license_config_gemini_bundle = billing_account_license_config.gemini_bundle
billing_account_license_config_procurement_entitlement_id = billing_account_license_config.procurement_entitlement_id
billing_account_license_config_subscription_tier = billing_account_license_config.subscription_tier
billing_account_license_config_start_date = billing_account_license_config.start_date
billing_account_license_config_auto_renew = billing_account_license_config.auto_renew
billing_account_license_config_license_config_distributions = billing_account_license_config.license_config_distributions
billing_account_license_config_license_count = billing_account_license_config.license_count
billing_account_license_config_name = billing_account_license_config.name
billing_account_license_config_state = billing_account_license_config.state
billing_account_license_config_subscription_display_name = billing_account_license_config.subscription_display_name
```

---


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time the conversation finished. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `start_time` | String |  | Output only. The time the conversation started. |
| `state` | String |  | The state of the Conversation. |
| `messages` | Vec<String> |  | Conversation messages. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time the conversation finished. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `start_time` | String | Output only. The time the conversation started. |
| `state` | String | The state of the Conversation. |
| `messages` | Vec<String> | Conversation messages. |


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
conversation_user_pseudo_id = conversation.user_pseudo_id
conversation_name = conversation.name
conversation_start_time = conversation.start_time
conversation_state = conversation.state
conversation_messages = conversation.messages
```

---


### Data_connector

Uses the per-user refresh token minted with AcquireAndStoreRefreshToken to generate and return a new access token and its details. Takes the access token from cache if available. Rotates the stored refresh token if needed. Uses the end user identity to return the user specific access token. Does *not* return the credentials configured by the administrator. Used by action execution and UI.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the connector for which a token is queried. |


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
    name = "value"  # Required. The resource name of the connector for which a token is queried.
}

# Access data_connector outputs
data_connector_id = data_connector.id
data_connector_refresh_token_info = data_connector.refresh_token_info
```

---


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `acl_info` | String |  | Access control information for the document. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `acl_info` | String | Access control information for the document. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `schema_id` | String | The identifier of the schema located in the same data store. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |


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
document_acl_info = document.acl_info
document_json_data = document.json_data
document_schema_id = document.schema_id
document_index_time = document.index_time
document_name = document.name
document_index_status = document.index_status
document_id = document.id
document_content = document.content
document_struct_data = document.struct_data
document_derived_struct_data = document.derived_struct_data
document_parent_document_id = document.parent_document_id
```

---


### Widget_config

Gets a WidgetConfig.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_search_as_you_type` | bool |  | Whether to enable search-as-you-type behavior for the search widget |
| `content_search_spec` | String |  | The content search spec that configs the desired behavior of content search. |
| `data_store_type` | String |  | Output only. The type of the parent data store. |
| `homepage_setting` | String |  | Optional. Describes the homepage settings of the widget. |
| `facet_field` | Vec<String> |  | The configuration and appearance of facets in the end user view. |
| `industry_vertical` | String |  | Output only. The industry vertical that the WidgetConfig registers. The WidgetConfig industry vertical is based on the associated Engine. |
| `display_name` | String |  | Required. The human readable widget config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `enable_summarization` | bool |  | Turn on or off summarization for the search response. |
| `data_store_ui_configs` | Vec<String> |  | Configurable UI configurations per data store. |
| `enable_conversational_search` | bool |  | Whether to allow conversational search (LLM, multi-turn) or not (non-LLM, single-turn). |
| `allow_public_access` | bool |  | Whether allow no-auth integration with widget. If set true, public access to search or other solutions from widget is allowed without authenication token provided by customer hosted backend server. |
| `ui_branding` | String |  | Describes search widget UI branding settings, such as the widget title, logo, favicons, and colors. |
| `minimum_data_term_accepted` | bool |  | Output only. Whether the customer accepted data use terms. |
| `collection_components` | Vec<String> |  | Output only. Collection components that lists all collections and child data stores associated with the widget config, those data sources can be used for filtering in widget service APIs, users can return results that from selected data sources. |
| `enable_snippet_result_summary` | bool |  | Turn on or off summary for each snippets result. |
| `allowlisted_domains` | Vec<String> |  | Allowlisted domains that can load this widget. |
| `enable_result_score` | bool |  | Whether to show the result score. |
| `solution_type` | String |  | Required. Immutable. Specifies the solution type that this WidgetConfig can be used for. |
| `create_time` | String |  | Output only. Timestamp the WidgetConfig was created. |
| `default_search_request_order_by` | String |  | The default ordering for search results if specified. Used to set SearchRequest#order_by on applicable requests. https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1alpha/projects.locations.dataStores.servingConfigs/search#request-body |
| `enable_autocomplete` | bool |  | Whether or not to enable autocomplete. |
| `fields_ui_components_map` | HashMap<String, String> |  | The key is the UI component. Mock. Currently supported `title`, `thumbnail`, `url`, `custom1`, `custom2`, `custom3`. The value is the name of the field along with its device visibility. The 3 custom fields are optional and can be added or removed. `title`, `thumbnail`, `url` are required UI components that cannot be removed. |
| `ui_settings` | String |  | Describes general widget search settings as seen in cloud console widget configuration page. Replaces top deprecated top level properties. |
| `access_settings` | String |  | Will be used for all widget access settings seen in cloud console integration page. Replaces top deprecated top level properties. |
| `llm_enabled` | bool |  | Output only. Whether LLM is enabled in the corresponding data store. |
| `update_time` | String |  | Output only. Timestamp the WidgetConfig was updated. |
| `enable_quality_feedback` | bool |  | Turn on or off collecting the search result quality feedback from end users. |
| `name` | String |  | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `config_id` | String |  | Output only. Unique obfuscated identifier of a WidgetConfig. |
| `result_display_type` | String |  | The type of snippet to display in UCS widget. - RESULT_DISPLAY_TYPE_UNSPECIFIED for existing users. - SNIPPET for new non-enterprise search users. - EXTRACTIVE_ANSWER for new enterprise search users. |
| `gemini_bundle` | bool |  | Output only. Whether the subscription is gemini bundle or not. |
| `assistant_settings` | String |  | Optional. Output only. Describes the assistant settings of the widget. |
| `customer_provided_config` | String |  | Optional. Output only. Describes the customer related configurations, currently only used for government customers. This field cannot be modified after project onboarding. |
| `enable_safe_search` | bool |  | Whether to enable safe search. |
| `enable_web_app` | bool |  | Whether to enable standalone web app. |
| `enable_private_knowledge_graph` | bool |  | Optional. Output only. Whether to enable private knowledge graph. |
| `name` | String | ✅ | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_search_as_you_type` | bool | Whether to enable search-as-you-type behavior for the search widget |
| `content_search_spec` | String | The content search spec that configs the desired behavior of content search. |
| `data_store_type` | String | Output only. The type of the parent data store. |
| `homepage_setting` | String | Optional. Describes the homepage settings of the widget. |
| `facet_field` | Vec<String> | The configuration and appearance of facets in the end user view. |
| `industry_vertical` | String | Output only. The industry vertical that the WidgetConfig registers. The WidgetConfig industry vertical is based on the associated Engine. |
| `display_name` | String | Required. The human readable widget config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `enable_summarization` | bool | Turn on or off summarization for the search response. |
| `data_store_ui_configs` | Vec<String> | Configurable UI configurations per data store. |
| `enable_conversational_search` | bool | Whether to allow conversational search (LLM, multi-turn) or not (non-LLM, single-turn). |
| `allow_public_access` | bool | Whether allow no-auth integration with widget. If set true, public access to search or other solutions from widget is allowed without authenication token provided by customer hosted backend server. |
| `ui_branding` | String | Describes search widget UI branding settings, such as the widget title, logo, favicons, and colors. |
| `minimum_data_term_accepted` | bool | Output only. Whether the customer accepted data use terms. |
| `collection_components` | Vec<String> | Output only. Collection components that lists all collections and child data stores associated with the widget config, those data sources can be used for filtering in widget service APIs, users can return results that from selected data sources. |
| `enable_snippet_result_summary` | bool | Turn on or off summary for each snippets result. |
| `allowlisted_domains` | Vec<String> | Allowlisted domains that can load this widget. |
| `enable_result_score` | bool | Whether to show the result score. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that this WidgetConfig can be used for. |
| `create_time` | String | Output only. Timestamp the WidgetConfig was created. |
| `default_search_request_order_by` | String | The default ordering for search results if specified. Used to set SearchRequest#order_by on applicable requests. https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1alpha/projects.locations.dataStores.servingConfigs/search#request-body |
| `enable_autocomplete` | bool | Whether or not to enable autocomplete. |
| `fields_ui_components_map` | HashMap<String, String> | The key is the UI component. Mock. Currently supported `title`, `thumbnail`, `url`, `custom1`, `custom2`, `custom3`. The value is the name of the field along with its device visibility. The 3 custom fields are optional and can be added or removed. `title`, `thumbnail`, `url` are required UI components that cannot be removed. |
| `ui_settings` | String | Describes general widget search settings as seen in cloud console widget configuration page. Replaces top deprecated top level properties. |
| `access_settings` | String | Will be used for all widget access settings seen in cloud console integration page. Replaces top deprecated top level properties. |
| `llm_enabled` | bool | Output only. Whether LLM is enabled in the corresponding data store. |
| `update_time` | String | Output only. Timestamp the WidgetConfig was updated. |
| `enable_quality_feedback` | bool | Turn on or off collecting the search result quality feedback from end users. |
| `name` | String | Immutable. The full resource name of the widget config. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/widgetConfigs/{widget_config_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `config_id` | String | Output only. Unique obfuscated identifier of a WidgetConfig. |
| `result_display_type` | String | The type of snippet to display in UCS widget. - RESULT_DISPLAY_TYPE_UNSPECIFIED for existing users. - SNIPPET for new non-enterprise search users. - EXTRACTIVE_ANSWER for new enterprise search users. |
| `gemini_bundle` | bool | Output only. Whether the subscription is gemini bundle or not. |
| `assistant_settings` | String | Optional. Output only. Describes the assistant settings of the widget. |
| `customer_provided_config` | String | Optional. Output only. Describes the customer related configurations, currently only used for government customers. This field cannot be modified after project onboarding. |
| `enable_safe_search` | bool | Whether to enable safe search. |
| `enable_web_app` | bool | Whether to enable standalone web app. |
| `enable_private_knowledge_graph` | bool | Optional. Output only. Whether to enable private knowledge graph. |


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
widget_config_enable_search_as_you_type = widget_config.enable_search_as_you_type
widget_config_content_search_spec = widget_config.content_search_spec
widget_config_data_store_type = widget_config.data_store_type
widget_config_homepage_setting = widget_config.homepage_setting
widget_config_facet_field = widget_config.facet_field
widget_config_industry_vertical = widget_config.industry_vertical
widget_config_display_name = widget_config.display_name
widget_config_enable_summarization = widget_config.enable_summarization
widget_config_data_store_ui_configs = widget_config.data_store_ui_configs
widget_config_enable_conversational_search = widget_config.enable_conversational_search
widget_config_allow_public_access = widget_config.allow_public_access
widget_config_ui_branding = widget_config.ui_branding
widget_config_minimum_data_term_accepted = widget_config.minimum_data_term_accepted
widget_config_collection_components = widget_config.collection_components
widget_config_enable_snippet_result_summary = widget_config.enable_snippet_result_summary
widget_config_allowlisted_domains = widget_config.allowlisted_domains
widget_config_enable_result_score = widget_config.enable_result_score
widget_config_solution_type = widget_config.solution_type
widget_config_create_time = widget_config.create_time
widget_config_default_search_request_order_by = widget_config.default_search_request_order_by
widget_config_enable_autocomplete = widget_config.enable_autocomplete
widget_config_fields_ui_components_map = widget_config.fields_ui_components_map
widget_config_ui_settings = widget_config.ui_settings
widget_config_access_settings = widget_config.access_settings
widget_config_llm_enabled = widget_config.llm_enabled
widget_config_update_time = widget_config.update_time
widget_config_enable_quality_feedback = widget_config.enable_quality_feedback
widget_config_name = widget_config.name
widget_config_config_id = widget_config.config_id
widget_config_result_display_type = widget_config.result_display_type
widget_config_gemini_bundle = widget_config.gemini_bundle
widget_config_assistant_settings = widget_config.assistant_settings
widget_config_customer_provided_config = widget_config.customer_provided_config
widget_config_enable_safe_search = widget_config.enable_safe_search
widget_config_enable_web_app = widget_config.enable_web_app
widget_config_enable_private_knowledge_graph = widget_config.enable_private_knowledge_graph
```

---


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. The time the session finished. |
| `start_time` | String |  | Output only. The time the session started. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `state` | String |  | The state of the session. |
| `turns` | Vec<String> |  | Turns. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. The time the session finished. |
| `start_time` | String | Output only. The time the session started. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `state` | String | The state of the session. |
| `turns` | Vec<String> | Turns. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `user_pseudo_id` | String | A unique identifier for tracking users. |


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
session_end_time = session.end_time
session_start_time = session.start_time
session_is_pinned = session.is_pinned
session_display_name = session.display_name
session_labels = session.labels
session_state = session.state
session_turns = session.turns
session_name = session.name
session_user_pseudo_id = session.user_pseudo_id
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


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String |  | The query to use. |
| `ignore_record_details_in_response` | bool |  | If true, the response will contain only record ID and score. By default, it is false, the response will contain record details. |
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
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


### Media

Uploads a file for the assistant to use as a source of information within the session.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_request_info` | String |  | Media upload request metadata. |
| `blob` | String |  | Information about the file being uploaded. |
| `name` | String | ✅ | Required. The resource name of the Session. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/sessions/{session}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filename` | String | Original file name |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `length` | String | Size of the data, in bytes |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `content_type` | String | MIME type of the data |
| `token` | String | A unique fingerprint/version id for the media data |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `download_parameters` | String | Parameters for a media download. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `reference_type` | String | Describes what the field reference contains. |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |


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
media_filename = media.filename
media_algorithm = media.algorithm
media_composite_media = media.composite_media
media_content_type_info = media.content_type_info
media_path = media.path
media_is_potential_retry = media.is_potential_retry
media_diff_version_response = media.diff_version_response
media_hash = media.hash
media_cosmo_binary_reference = media.cosmo_binary_reference
media_media_id = media.media_id
media_inline = media.inline
media_timestamp = media.timestamp
media_length = media.length
media_diff_checksums_response = media.diff_checksums_response
media_hash_verified = media.hash_verified
media_md5_hash = media.md5_hash
media_content_type = media.content_type
media_token = media.token
media_diff_upload_request = media.diff_upload_request
media_diff_upload_response = media.diff_upload_response
media_blobstore2_info = media.blobstore2_info
media_download_parameters = media.download_parameters
media_sha1_hash = media.sha1_hash
media_bigstore_object_ref = media.bigstore_object_ref
media_crc32c_hash = media.crc32c_hash
media_blob_ref = media.blob_ref
media_sha256_hash = media.sha256_hash
media_reference_type = media.reference_type
media_diff_download_response = media.diff_download_response
media_object_id = media.object_id
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


### Agent

Creates an Agent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The lifecycle state of the agent. |
| `starter_prompts` | Vec<String> |  | Optional. The starter prompt suggestions to show the user on the landing page of the agent. |
| `create_time` | String |  | Output only. Timestamp when this Agent was created. |
| `display_name` | String |  | Required. Display name of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `authorization_config` | String |  | Optional. The authorizations that are required by the agent. |
| `icon` | String |  | Optional. The icon that represents the agent on the UI. |
| `name` | String |  | Identifier. Resource name of the agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}` |
| `custom_placeholder_text` | String |  | Optional. The custom placeholder text that appears in the text box before the user enters any text. |
| `description` | String |  | Required. Human-readable description of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `rejection_reason` | String |  | Output only. The reason why the agent was rejected. Only set if the state is PRIVATE, and got there via rejection. |
| `deployment_failure_reason` | String |  | Output only. The reason why the agent deployment failed. Only set if the state is DEPLOYMENT_FAILED. |
| `language_code` | String |  | Optional. The code of the language of the text in the description, display_name and starter_prompts fields. |
| `dialogflow_agent_definition` | String |  | Optional. The behavior of the agent is defined as a Dialogflow agent. |
| `adk_agent_definition` | String |  | Optional. The behavior of the agent is defined as an ADK agent. |
| `a2a_agent_definition` | String |  | Optional. The behavior of the agent is defined as an A2A agent. |
| `suspension_reason` | String |  | Output only. The reason why the agent was suspended. Only set if the state is SUSPENDED. |
| `update_time` | String |  | Output only. Timestamp when this Agent was most recently updated. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The lifecycle state of the agent. |
| `starter_prompts` | Vec<String> | Optional. The starter prompt suggestions to show the user on the landing page of the agent. |
| `create_time` | String | Output only. Timestamp when this Agent was created. |
| `display_name` | String | Required. Display name of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `authorization_config` | String | Optional. The authorizations that are required by the agent. |
| `icon` | String | Optional. The icon that represents the agent on the UI. |
| `name` | String | Identifier. Resource name of the agent. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}/agents/{agent}` |
| `custom_placeholder_text` | String | Optional. The custom placeholder text that appears in the text box before the user enters any text. |
| `description` | String | Required. Human-readable description of the agent. This might be used by an LLM to automatically select an agent to respond to a user query. |
| `rejection_reason` | String | Output only. The reason why the agent was rejected. Only set if the state is PRIVATE, and got there via rejection. |
| `deployment_failure_reason` | String | Output only. The reason why the agent deployment failed. Only set if the state is DEPLOYMENT_FAILED. |
| `language_code` | String | Optional. The code of the language of the text in the description, display_name and starter_prompts fields. |
| `dialogflow_agent_definition` | String | Optional. The behavior of the agent is defined as a Dialogflow agent. |
| `adk_agent_definition` | String | Optional. The behavior of the agent is defined as an ADK agent. |
| `a2a_agent_definition` | String | Optional. The behavior of the agent is defined as an A2A agent. |
| `suspension_reason` | String | Output only. The reason why the agent was suspended. Only set if the state is SUSPENDED. |
| `update_time` | String | Output only. Timestamp when this Agent was most recently updated. |


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
agent_state = agent.state
agent_starter_prompts = agent.starter_prompts
agent_create_time = agent.create_time
agent_display_name = agent.display_name
agent_authorization_config = agent.authorization_config
agent_icon = agent.icon
agent_name = agent.name
agent_custom_placeholder_text = agent.custom_placeholder_text
agent_description = agent.description
agent_rejection_reason = agent.rejection_reason
agent_deployment_failure_reason = agent.deployment_failure_reason
agent_language_code = agent.language_code
agent_dialogflow_agent_definition = agent.dialogflow_agent_definition
agent_adk_agent_definition = agent.adk_agent_definition
agent_a2a_agent_definition = agent.a2a_agent_definition
agent_suspension_reason = agent.suspension_reason
agent_update_time = agent.update_time
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


### User_event

Writes a single user event.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_time` | String |  | Only required for UserEventService.ImportUserEvents method. Timestamp of when the user event happened. |
| `event_type` | String |  | Required. User event type. Allowed values are: Generic values: * `search`: Search for Documents. * `view-item`: Detailed page view of a Document. * `view-item-list`: View of a panel or ordered list of Documents. * `view-home-page`: View of the home page. * `view-category-page`: View of a category page, e.g. Home > Men > Jeans Retail-related values: * `add-to-cart`: Add an item(s) to cart, e.g. in Retail online shopping * `purchase`: Purchase an item(s) Media-related values: * `media-play`: Start/resume watching a video, playing a song, etc. * `media-complete`: Finished or stopped midway through a video, song, etc. Custom conversion value: * `conversion`: Customer defined conversion event. |
| `filter` | String |  | The filter syntax consists of an expression language for constructing a predicate from one or more fields of the documents being filtered. One example is for `search` events, the associated SearchRequest may contain a filter expression in SearchRequest.filter conforming to https://google.aip.dev/160#filtering. Similarly, for `view-item-list` events that are generated from a RecommendRequest, this field may be populated directly from RecommendRequest.filter conforming to https://google.aip.dev/160#filtering. The value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `media_info` | String |  | Media-specific info. |
| `engine` | String |  | The Engine resource name, in the form of `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. Optional. Only required for Engine produced user events. For example, user events from blended search. |
| `attributes` | HashMap<String, String> |  | Extra user event features to include in the recommendation model. These attributes must NOT contain data that needs to be parsed or processed further, e.g. JSON or other encodings. If you provide custom attributes for ingested user events, also include them in the user events that you associate with prediction requests. Custom attribute formatting must be consistent between imported events and events provided with prediction requests. This lets the Discovery Engine API use those custom attributes when training models and serving predictions, which helps improve recommendation quality. This field needs to pass all below criteria, otherwise an `INVALID_ARGUMENT` error is returned: * The key must be a UTF-8 encoded string with a length limit of 5,000 characters. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. For product recommendations, an example of extra user information is `traffic_channel`, which is how a user arrives at the site. Users can arrive at the site by coming to the site directly, coming through Google search, or in other ways. |
| `panels` | Vec<String> |  | Optional. List of panels associated with this event. Used for page-level impression data. |
| `attribution_token` | String |  | Token to attribute an API response to user action(s) to trigger the event. Highly recommended for user events that are the result of RecommendationService.Recommend. This field enables accurate attribution of recommendation model performance. The value must be one of: * RecommendResponse.attribution_token for events that are the result of RecommendationService.Recommend. * SearchResponse.attribution_token for events that are the result of SearchService.Search. This token enables us to accurately attribute page view or conversion completion back to the event and the particular predict response containing this clicked/purchased product. If user clicks on product K in the recommendation results, pass RecommendResponse.attribution_token as a URL parameter to product K's page. When recording events on product K's page, log the RecommendResponse.attribution_token to this field. |
| `search_info` | String |  | SearchService.Search details related to the event. This field should be set for `search` event. |
| `session_id` | String |  | A unique identifier for tracking a visitor session with a length limit of 128 bytes. A session is an aggregation of an end user behavior in a time span. A general guideline to populate the session_id: 1. If user has no activity for 30 min, a new session_id should be assigned. 2. The session_id should be unique across users, suggest use uuid or add UserEvent.user_pseudo_id as prefix. |
| `completion_info` | String |  | CompletionService.CompleteQuery details related to the event. This field should be set for `search` event when autocomplete function is enabled and the user clicks a suggestion for search. |
| `page_info` | String |  | Page metadata such as categories and other critical information for certain event types such as `view-category-page`. |
| `conversion_type` | String |  | Optional. Conversion type. Required if UserEvent.event_type is `conversion`. This is a customer-defined conversion name in lowercase letters or numbers separated by "-", such as "watch", "good-visit" etc. Do not set the field if UserEvent.event_type is not `conversion`. This mixes the custom conversion event with predefined events like `search`, `view-item` etc. |
| `user_pseudo_id` | String |  | Required. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor log in/out of the website. Do not set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. The field should not contain PII or user-data. We recommend to use Google Analytics [Client ID](https://developers.google.com/analytics/devguides/collection/analyticsjs/field-reference#clientId) for this field. |
| `data_store` | String |  | The DataStore resource full name, of the form `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. Optional. Only required for user events whose data store can't by determined by UserEvent.engine or UserEvent.documents. If data store is set in the parent of write/import/collect user event requests, this field can be omitted. |
| `documents` | Vec<String> |  | List of Documents associated with this user event. This field is optional except for the following event types: * `view-item` * `add-to-cart` * `purchase` * `media-play` * `media-complete` In a `search` event, this field represents the documents returned to the end user on the current page (the end user may have not finished browsing the whole page yet). When a new page is returned to the end user, after pagination/filtering/ordering even for the same query, a new `search` event with different UserEvent.documents is desired. |
| `panel` | String |  | Panel metadata associated with this user event. |
| `promotion_ids` | Vec<String> |  | The promotion IDs if this is an event associated with promotions. Currently, this field is restricted to at most one ID. |
| `tag_ids` | Vec<String> |  | A list of identifiers for the independent experiment groups this user event belongs to. This is used to distinguish between user events associated with different experiment setups. |
| `transaction_info` | String |  | The transaction metadata (if any) associated with this user event. |
| `user_info` | String |  | Information about the end user. |
| `direct_user_request` | bool |  | Should set to true if the request is made directly from the end user, in which case the UserEvent.user_info.user_agent can be populated from the HTTP request. This flag should be set only if the API request is made directly from the end user such as a mobile app (and not if a gateway or a server is processing and pushing the user events). This should not be set when using the JavaScript tag in UserEventService.CollectUserEvent. |
| `parent` | String | ✅ | Required. The parent resource name. If the write user event action is applied in DataStore level, the format is: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. If the write user event action is applied in Location level, for example, the event with Document across multiple DataStore, the format is: `projects/{project}/locations/{location}`. |


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
    parent = "value"  # Required. The parent resource name. If the write user event action is applied in DataStore level, the format is: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. If the write user event action is applied in Location level, for example, the event with Document across multiple DataStore, the format is: `projects/{project}/locations/{location}`.
}

# Access user_event outputs
user_event_id = user_event.id
user_event_content_type = user_event.content_type
user_event_extensions = user_event.extensions
user_event_data = user_event.data
```

---


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_date` | String |  | Optional. The planed end date. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `alert_policy_resource_config` | String |  | Optional. The alert policy config for this license config. |
| `start_date` | String |  | Required. The start date. |
| `state` | String |  | Output only. The state of the license config. |
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `subscription_term` | String |  | Required. Subscription term. |
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_date` | String | Optional. The planed end date. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `license_count` | String | Required. Number of licenses purchased. |
| `subscription_tier` | String | Required. Subscription tier information for the license config. |
| `alert_policy_resource_config` | String | Optional. The alert policy config for this license config. |
| `start_date` | String | Required. The start date. |
| `state` | String | Output only. The state of the license config. |
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |
| `subscription_term` | String | Required. Subscription term. |
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |


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
license_config_end_date = license_config.end_date
license_config_name = license_config.name
license_config_license_count = license_config.license_count
license_config_subscription_tier = license_config.subscription_tier
license_config_alert_policy_resource_config = license_config.alert_policy_resource_config
license_config_start_date = license_config.start_date
license_config_state = license_config.state
license_config_gemini_bundle = license_config.gemini_bundle
license_config_subscription_term = license_config.subscription_term
license_config_auto_renew = license_config.auto_renew
license_config_free_trial = license_config.free_trial
```

---


### Serving_config

Answer query method (streaming). It takes one AnswerQueryRequest and returns multiple AnswerQueryResponse messages in a stream.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_user_spec` | String |  | Optional. End user specification. |
| `user_pseudo_id` | String |  | A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `related_questions_spec` | String |  | Related questions specification. |
| `answer_generation_spec` | String |  | Answer generation specification. |
| `asynchronous_mode` | bool |  | Deprecated: This field is deprecated. Streaming Answer API will be supported. Asynchronous mode control. If enabled, the response will be returned with answer/session resource name without final answer. The API users need to do the polling to get the latest status of answer/session by calling ConversationalSearchService.GetAnswer or ConversationalSearchService.GetSession method. |
| `grounding_spec` | String |  | Optional. Grounding specification. |
| `query` | String |  | Required. Current user query. |
| `safety_spec` | String |  | Model specification. |
| `search_spec` | String |  | Search specification. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `session` | String |  | The session resource name. Not required. When session field is not set, the API is in sessionless mode. We support auto session mode: users can use the wildcard symbol `-` as session ID. A new ID will be automatically generated and assigned. |
| `query_understanding_spec` | String |  | Query understanding specification. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `guided_search_spec` | String | Guided search configs. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `media_config` | String | The MediaConfig of the serving configuration. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `create_time` | String | Output only. ServingConfig created timestamp. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `embedding_config` | String | Bring your own embedding config. The config is used for search semantic retrieval. The retrieval is based on the dot product of SearchRequest.EmbeddingSpec.EmbeddingVector.vector and the document embeddings that are provided by this EmbeddingConfig. If SearchRequest.EmbeddingSpec.EmbeddingVector.vector is provided, it overrides this ServingConfig.embedding_config. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `personalization_spec` | String | The specification for personalization spec. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set, SearchRequest.personalization_spec overrides ServingConfig.personalization_spec. |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `custom_fine_tuning_spec` | String | Custom fine tuning configs. If SearchRequest.custom_fine_tuning_spec is set, it has higher priority than the configs set here. |


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
serving_config_diversity_level = serving_config.diversity_level
serving_config_guided_search_spec = serving_config.guided_search_spec
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_media_config = serving_config.media_config
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_update_time = serving_config.update_time
serving_config_generic_config = serving_config.generic_config
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
serving_config_create_time = serving_config.create_time
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_model_id = serving_config.model_id
serving_config_solution_type = serving_config.solution_type
serving_config_embedding_config = serving_config.embedding_config
serving_config_name = serving_config.name
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_display_name = serving_config.display_name
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_custom_fine_tuning_spec = serving_config.custom_fine_tuning_spec
```

---


### Authorization

Creates an Authorization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name of the authorization. Format: `projects/{project}/locations/{location}/authorizations/{authorization}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `server_side_oauth2` | String |  | Server-side OAuth2 configuration. |
| `display_name` | String |  | Required. The display name of the authorization. It must be a UTF-8 encoded string with a length limit of 128 characters. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of the authorization. Format: `projects/{project}/locations/{location}/authorizations/{authorization}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `server_side_oauth2` | String | Server-side OAuth2 configuration. |
| `display_name` | String | Required. The display name of the authorization. It must be a UTF-8 encoded string with a length limit of 128 characters. |


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
authorization_name = authorization.name
authorization_server_side_oauth2 = authorization.server_side_oauth2
authorization_display_name = authorization.display_name
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
| `blob_attachments` | Vec<String> | List of blob attachments in the answer. |
| `answer_text` | String | The textual answer. |
| `state` | String | The state of the answer generation. |
| `query_understanding_info` | String | Query understanding information. |
| `references` | Vec<String> | References. |
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |
| `create_time` | String | Output only. Answer creation timestamp. |
| `steps` | Vec<String> | Answer generation steps. |
| `related_questions` | Vec<String> | Suggested related questions. |
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |
| `citations` | Vec<String> | Citations. |


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
answer_blob_attachments = answer.blob_attachments
answer_answer_text = answer.answer_text
answer_state = answer.state
answer_query_understanding_info = answer.query_understanding_info
answer_references = answer.references
answer_safety_ratings = answer.safety_ratings
answer_name = answer.name
answer_complete_time = answer.complete_time
answer_grounding_supports = answer.grounding_supports
answer_create_time = answer.create_time
answer_steps = answer.steps
answer_related_questions = answer.related_questions
answer_grounding_score = answer.grounding_score
answer_answer_skipped_reasons = answer.answer_skipped_reasons
answer_citations = answer.citations
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


### Collection

Gets a Collection.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_connector` | String |  | Output only. The data connector, if present, manages the connection for data stores in the Collection. To set up the connector, use DataConnectorService.SetUpDataConnector method, which creates a new Collection while setting up the DataConnector singleton resource. Setting up connector on an existing Collection is not supported. This output only field contains a subset of the DataConnector fields, including `name`, `data_source`, `entities.entity_name` and `entities.data_store`. To get more details about a data connector, use the DataConnectorService.GetDataConnector method. |
| `name` | String |  | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String |  | Output only. Timestamp the Collection was created at. |
| `display_name` | String |  | Required. The Collection display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `name` | String | ✅ | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_connector` | String | Output only. The data connector, if present, manages the connection for data stores in the Collection. To set up the connector, use DataConnectorService.SetUpDataConnector method, which creates a new Collection while setting up the DataConnector singleton resource. Setting up connector on an existing Collection is not supported. This output only field contains a subset of the DataConnector fields, including `name`, `data_source`, `entities.entity_name` and `entities.data_store`. To get more details about a data connector, use the DataConnectorService.GetDataConnector method. |
| `name` | String | Immutable. The full resource name of the Collection. Format: `projects/{project}/locations/{location}/collections/{collection_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `create_time` | String | Output only. Timestamp the Collection was created at. |
| `display_name` | String | Required. The Collection display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |


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
collection_data_connector = collection.data_connector
collection_name = collection.name
collection_create_time = collection.create_time
collection_display_name = collection.display_name
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.discoveryengine_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
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
| `total_size` | i64 | The total number of items matching the request. This will always be populated in the response. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `target_sites` | Vec<String> | List of TargetSites containing the site verification status. |


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
site_search_engine_total_size = site_search_engine.total_size
site_search_engine_next_page_token = site_search_engine.next_page_token
site_search_engine_target_sites = site_search_engine.target_sites
```

---


### User_store

Creates a new User Store.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_expired_license_auto_update` | bool |  | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool |  | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `default_license_config` | String |  | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `name` | String |  | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `display_name` | String |  | The display name of the User Store. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_expired_license_auto_update` | bool | Optional. Whether to enable license auto update for users in this User Store. If true, users with expired licenses will automatically be updated to use the default license config as long as the default license config has seats left. |
| `enable_license_auto_register` | bool | Optional. Whether to enable license auto register for users in this User Store. If true, new users will automatically register under the default license config as long as the default license config has seats left. |
| `default_license_config` | String | Optional. The default subscription LicenseConfig for the UserStore, if UserStore.enable_license_auto_register is true, new users will automatically register under the default subscription. If default LicenseConfig doesn't have remaining license seats left, new users will not be assigned with license and will be blocked for Vertex AI Search features. This is used if `license_assignment_tier_rules` is not configured. |
| `name` | String | Immutable. The full resource name of the User Store, in the format of `projects/{project}/locations/{location}/userStores/{user_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `display_name` | String | The display name of the User Store. |


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
user_store_enable_expired_license_auto_update = user_store.enable_expired_license_auto_update
user_store_enable_license_auto_register = user_store.enable_license_auto_register
user_store_default_license_config = user_store.default_license_config
user_store_name = user_store.name
user_store_display_name = user_store.display_name
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

# Create user_event
user_event = provider.discoveryengine_api.User_event {
    parent = "value"  # Required. The resource name of the catalog under which the events are created. The format is `projects/{project}/locations/global/collections/{collection}/dataStores/{dataStore}`.
}

# Access user_event outputs
user_event_id = user_event.id
user_event_data = user_event.data
user_event_extensions = user_event.extensions
user_event_content_type = user_event.content_type
```

---


### Cmek_config

Gets the CmekConfig.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_version` | String |  | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String |  | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> |  | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String |  | Output only. The states of the CmekConfig. |
| `is_default` | bool |  | Output only. The default CmekConfig for the Customer. |
| `kms_key` | String |  | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `name` | String |  | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `last_rotation_timestamp_micros` | String |  | Output only. The timestamp of the last key rotation. |
| `name` | String | ✅ | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String | Output only. The states of the CmekConfig. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
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
cmek_config_kms_key_version = cmek_config.kms_key_version
cmek_config_notebooklm_state = cmek_config.notebooklm_state
cmek_config_single_region_keys = cmek_config.single_region_keys
cmek_config_state = cmek_config.state
cmek_config_is_default = cmek_config.is_default
cmek_config_kms_key = cmek_config.kms_key
cmek_config_name = cmek_config.name
cmek_config_last_rotation_timestamp_micros = cmek_config.last_rotation_timestamp_micros
```

---


### Schema

Creates a Schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> |  | The structured representation of the schema. |
| `json_schema` | String |  | The JSON representation of the schema. |
| `parent` | String | ✅ | Required. The parent data store resource name, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The full resource name of the schema, in the format of `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `struct_schema` | HashMap<String, String> | The structured representation of the schema. |
| `json_schema` | String | The JSON representation of the schema. |


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
schema_name = schema.name
schema_struct_schema = schema.struct_schema
schema_json_schema = schema.json_schema
```

---


### License_config

Creates a LicenseConfig

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the license config. |
| `start_date` | String |  | Required. The start date. |
| `subscription_tier` | String |  | Required. Subscription tier information for the license config. |
| `subscription_term` | String |  | Required. Subscription term. |
| `free_trial` | bool |  | Optional. Whether the license config is for free trial. |
| `auto_renew` | bool |  | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `gemini_bundle` | bool |  | Output only. Whether the license config is for Gemini bundle. |
| `end_date` | String |  | Optional. The planed end date. |
| `license_count` | String |  | Required. Number of licenses purchased. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the license config. |
| `start_date` | String | Required. The start date. |
| `subscription_tier` | String | Required. Subscription tier information for the license config. |
| `subscription_term` | String | Required. Subscription term. |
| `free_trial` | bool | Optional. Whether the license config is for free trial. |
| `auto_renew` | bool | Optional. Whether the license config should be auto renewed when it reaches the end date. |
| `gemini_bundle` | bool | Output only. Whether the license config is for Gemini bundle. |
| `end_date` | String | Optional. The planed end date. |
| `license_count` | String | Required. Number of licenses purchased. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the license config. Format: `projects/{project}/locations/{location}/licenseConfigs/{license_config}` |


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
license_config_state = license_config.state
license_config_start_date = license_config.start_date
license_config_subscription_tier = license_config.subscription_tier
license_config_subscription_term = license_config.subscription_term
license_config_free_trial = license_config.free_trial
license_config_auto_renew = license_config.auto_renew
license_config_gemini_bundle = license_config.gemini_bundle
license_config_end_date = license_config.end_date
license_config_license_count = license_config.license_count
license_config_name = license_config.name
```

---


### Target_site

Creates a TargetSite.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generated_uri_pattern` | String |  | Output only. This is system-generated based on the provided_uri_pattern. |
| `root_domain_uri` | String |  | Output only. Root domain of the provided_uri_pattern. |
| `site_verification_info` | String |  | Output only. Site ownership and validity verification status. |
| `indexing_status` | String |  | Output only. Indexing status. |
| `failure_reason` | String |  | Output only. Failure reason. |
| `name` | String |  | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `exact_match` | bool |  | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `type` | String |  | The type of the target site, e.g., whether the site is to be included or excluded. |
| `provided_uri_pattern` | String |  | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `update_time` | String |  | Output only. The target site's last updated time. |
| `parent` | String | ✅ | Required. Parent resource name of TargetSite, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generated_uri_pattern` | String | Output only. This is system-generated based on the provided_uri_pattern. |
| `root_domain_uri` | String | Output only. Root domain of the provided_uri_pattern. |
| `site_verification_info` | String | Output only. Site ownership and validity verification status. |
| `indexing_status` | String | Output only. Indexing status. |
| `failure_reason` | String | Output only. Failure reason. |
| `name` | String | Output only. The fully qualified resource name of the target site. `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}` The `target_site_id` is system-generated. |
| `exact_match` | bool | Immutable. If set to false, a uri_pattern is generated to include all pages whose address contains the provided_uri_pattern. If set to true, an uri_pattern is generated to try to be an exact match of the provided_uri_pattern or just the specific page if the provided_uri_pattern is a specific one. provided_uri_pattern is always normalized to generate the URI pattern to be used by the search engine. |
| `type` | String | The type of the target site, e.g., whether the site is to be included or excluded. |
| `provided_uri_pattern` | String | Required. Input only. The user provided URI pattern from which the `generated_uri_pattern` is generated. |
| `update_time` | String | Output only. The target site's last updated time. |


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
target_site_generated_uri_pattern = target_site.generated_uri_pattern
target_site_root_domain_uri = target_site.root_domain_uri
target_site_site_verification_info = target_site.site_verification_info
target_site_indexing_status = target_site.indexing_status
target_site_failure_reason = target_site.failure_reason
target_site_name = target_site.name
target_site_exact_match = target_site.exact_match
target_site_type = target_site.type
target_site_provided_uri_pattern = target_site.provided_uri_pattern
target_site_update_time = target_site.update_time
```

---


### Engine

Creates a Engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_store_ids` | Vec<String> |  | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `features` | HashMap<String, String> |  | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `display_name` | String |  | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `name` | String |  | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `solution_type` | String |  | Required. The solutions of the engine. |
| `chat_engine_metadata` | String |  | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `media_recommendation_engine_config` | String |  | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `app_type` | String |  | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `common_config` | String |  | Common config spec that specifies the metadata of the engine. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String |  | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `search_engine_config` | String |  | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_config` | String |  | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. |
| `disable_analytics` | bool |  | Optional. Whether to disable analytics for searches performed on this engine. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Engine was last updated. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_store_ids` | Vec<String> | Optional. The data stores associated with this engine. For SOLUTION_TYPE_SEARCH and SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store. If solution_type is SOLUTION_TYPE_CHAT, multiple DataStores in the same Collection can be associated here. Note that when used in CreateEngineRequest, one DataStore id must be provided as the system will use it for necessary initializations. |
| `features` | HashMap<String, String> | Optional. Feature config for the engine to opt in or opt out of features. Supported keys: * `*`: all features, if it's present, all other feature state settings are ignored. * `agent-gallery` * `no-code-agent-builder` * `prompt-gallery` * `model-selector` * `notebook-lm` * `people-search` * `people-search-org-chart` * `bi-directional-audio` * `feedback` * `session-sharing` * `personalization-memory` * `disable-agent-sharing` * `disable-image-generation` * `disable-video-generation` * `disable-onedrive-upload` * `disable-talk-to-content` * `disable-google-drive-upload` |
| `display_name` | String | Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters. |
| `name` | String | Immutable. Identifier. The fully qualified resource name of the engine. This field must be a UTF-8 encoded string with a length limit of 1024 characters. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}` engine should be 1-63 characters, and valid characters are /a-z0-9*/. Otherwise, an INVALID_ARGUMENT error is returned. |
| `solution_type` | String | Required. The solutions of the engine. |
| `chat_engine_metadata` | String | Output only. Additional information of the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `media_recommendation_engine_config` | String | Configurations for the Media Engine. Only applicable on the data stores with solution_type SOLUTION_TYPE_RECOMMENDATION and IndustryVertical.MEDIA vertical. |
| `app_type` | String | Optional. Immutable. This the application type which this engine resource represents. NOTE: this is a new concept independ of existing industry vertical or solution type. |
| `common_config` | String | Common config spec that specifies the metadata of the engine. |
| `create_time` | String | Output only. Timestamp the Recommendation Engine was created at. |
| `industry_vertical` | String | Optional. The industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: Vertical on Engine has to match vertical of the DataStore linked to the engine. |
| `search_engine_config` | String | Configurations for the Search Engine. Only applicable if solution_type is SOLUTION_TYPE_SEARCH. |
| `chat_engine_config` | String | Configurations for the Chat Engine. Only applicable if solution_type is SOLUTION_TYPE_CHAT. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. |
| `disable_analytics` | bool | Optional. Whether to disable analytics for searches performed on this engine. |
| `update_time` | String | Output only. Timestamp the Recommendation Engine was last updated. |


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
engine_data_store_ids = engine.data_store_ids
engine_features = engine.features
engine_display_name = engine.display_name
engine_name = engine.name
engine_solution_type = engine.solution_type
engine_chat_engine_metadata = engine.chat_engine_metadata
engine_media_recommendation_engine_config = engine.media_recommendation_engine_config
engine_app_type = engine.app_type
engine_common_config = engine.common_config
engine_create_time = engine.create_time
engine_industry_vertical = engine.industry_vertical
engine_search_engine_config = engine.search_engine_config
engine_chat_engine_config = engine.chat_engine_config
engine_configurable_billing_approach = engine.configurable_billing_approach
engine_disable_analytics = engine.disable_analytics
engine_update_time = engine.update_time
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.discoveryengine_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
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


### Sample_query_set

Creates a SampleQuerySet

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `description` | String |  | The description of the SampleQuerySet. |
| `create_time` | String |  | Output only. Timestamp the SampleQuerySet was created at. |
| `display_name` | String |  | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The full resource name of the SampleQuerySet, in the format of `projects/{project}/locations/{location}/sampleQuerySets/{sample_query_set}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `description` | String | The description of the SampleQuerySet. |
| `create_time` | String | Output only. Timestamp the SampleQuerySet was created at. |
| `display_name` | String | Required. The sample query set display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. |


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
sample_query_set_name = sample_query_set.name
sample_query_set_description = sample_query_set.description
sample_query_set_create_time = sample_query_set.create_time
sample_query_set_display_name = sample_query_set.display_name
```

---


### Project

Provisions the project resource. During the process, related systems will get prepared and initialized. Caller must read the [Terms for data use](https://cloud.google.com/retail/data-use-terms), and optionally specify in request to provide consent to that service terms.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `saas_params` | String |  | Optional. Parameters for Agentspace. |
| `data_use_terms_version` | String |  | Required. The version of the [Terms for data use](https://cloud.google.com/retail/data-use-terms) that caller has read and would like to give consent to. Acceptable version is `2022-11-23`, and this may change over time. |
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


### Document

Creates a Document.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_document_id` | String |  | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `schema_id` | String |  | The identifier of the schema located in the same data store. |
| `content` | String |  | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `derived_struct_data` | HashMap<String, String> |  | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `struct_data` | HashMap<String, String> |  | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String |  | Access control information for the document. |
| `json_data` | String |  | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_status` | String |  | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `name` | String |  | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `id` | String |  | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `index_time` | String |  | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_document_id` | String | The identifier of the parent document. Currently supports at most two level document hierarchy. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 63 characters. |
| `schema_id` | String | The identifier of the schema located in the same data store. |
| `content` | String | The unstructured data linked to this document. Content can only be set and must be set if this document is under a `CONTENT_REQUIRED` data store. |
| `derived_struct_data` | HashMap<String, String> | Output only. This field is OUTPUT_ONLY. It contains derived data that are not in the original input document. |
| `struct_data` | HashMap<String, String> | The structured JSON data for the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `acl_info` | String | Access control information for the document. |
| `json_data` | String | The JSON string representation of the document. It should conform to the registered Schema or an `INVALID_ARGUMENT` error is thrown. |
| `index_status` | String | Output only. The index status of the document. * If document is indexed successfully, the index_time field is populated. * Otherwise, if document is not indexed due to errors, the error_samples field is populated. * Otherwise, if document's index is in progress, the pending_message field is populated. |
| `name` | String | Immutable. The full resource name of the document. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `id` | String | Immutable. The identifier of the document. Id should conform to [RFC-1034](https://tools.ietf.org/html/rfc1034) standard with a length limit of 128 characters. |
| `index_time` | String | Output only. The last time the document was indexed. If this field is set, the document could be returned in search results. This field is OUTPUT_ONLY. If this field is not populated, it means the document has never been indexed. |


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
document_parent_document_id = document.parent_document_id
document_schema_id = document.schema_id
document_content = document.content
document_derived_struct_data = document.derived_struct_data
document_struct_data = document.struct_data
document_acl_info = document.acl_info
document_json_data = document.json_data
document_index_status = document.index_status
document_name = document.name
document_id = document.id
document_index_time = document.index_time
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


### Control

Creates a Control. By default 1000 controls are allowed for a data store. A request can be submitted to adjust this limit. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `filter_action` | String |  | Defines a filter-type control Currently not supported by Recommendation |
| `boost_action` | String |  | Defines a boost-type control |
| `conditions` | Vec<String> |  | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `use_cases` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `solution_type` | String |  | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `synonyms_action` | String |  | Treats a group of terms as synonyms of one another. |
| `promote_action` | String |  | Promote certain links based on predefined trigger queries. |
| `redirect_action` | String |  | Defines a redirect-type control. |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}` or `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Human readable name. The identifier used in UI views. Must be UTF-8 encoded string. Length limit is 128 characters. Otherwise an INVALID ARGUMENT error is thrown. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/dataStore/*/controls/*` |
| `filter_action` | String | Defines a filter-type control Currently not supported by Recommendation |
| `boost_action` | String | Defines a boost-type control |
| `conditions` | Vec<String> | Determines when the associated action will trigger. Omit to always apply the action. Currently only a single condition may be specified. Otherwise an INVALID ARGUMENT error is thrown. |
| `use_cases` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only applies to SOLUTION_TYPE_SEARCH. Currently only allow one use case per control. Must be set when solution_type is SolutionType.SOLUTION_TYPE_SEARCH. |
| `solution_type` | String | Required. Immutable. What solution the control belongs to. Must be compatible with vertical of resource. Otherwise an INVALID ARGUMENT error is thrown. |
| `synonyms_action` | String | Treats a group of terms as synonyms of one another. |
| `promote_action` | String | Promote certain links based on predefined trigger queries. |
| `redirect_action` | String | Defines a redirect-type control. |
| `associated_serving_config_ids` | Vec<String> | Output only. List of all ServingConfig IDs this control is attached to. May take up to 10 minutes to update after changes. |


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
control_display_name = control.display_name
control_name = control.name
control_filter_action = control.filter_action
control_boost_action = control.boost_action
control_conditions = control.conditions
control_use_cases = control.use_cases
control_solution_type = control.solution_type
control_synonyms_action = control.synonyms_action
control_promote_action = control.promote_action
control_redirect_action = control.redirect_action
control_associated_serving_config_ids = control.associated_serving_config_ids
```

---


### Evaluation

Creates a Evaluation. Upon creation, the evaluation will be automatically triggered and begin execution.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_spec` | String |  | Required. The specification of the evaluation. |
| `create_time` | String |  | Output only. Timestamp the Evaluation was created at. |
| `quality_metrics` | String |  | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `error` | String |  | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String |  | Output only. The state of the evaluation. |
| `name` | String |  | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `end_time` | String |  | Output only. Timestamp the Evaluation was completed at. |
| `error_samples` | Vec<String> |  | Output only. A sample of errors encountered while processing the request. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_spec` | String | Required. The specification of the evaluation. |
| `create_time` | String | Output only. Timestamp the Evaluation was created at. |
| `quality_metrics` | String | Output only. The metrics produced by the evaluation, averaged across all SampleQuerys in the SampleQuerySet. Only populated when the evaluation's state is SUCCEEDED. |
| `error` | String | Output only. The error that occurred during evaluation. Only populated when the evaluation's state is FAILED. |
| `state` | String | Output only. The state of the evaluation. |
| `name` | String | Identifier. The full resource name of the Evaluation, in the format of `projects/{project}/locations/{location}/evaluations/{evaluation}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `end_time` | String | Output only. Timestamp the Evaluation was completed at. |
| `error_samples` | Vec<String> | Output only. A sample of errors encountered while processing the request. |


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
evaluation_evaluation_spec = evaluation.evaluation_spec
evaluation_create_time = evaluation.create_time
evaluation_quality_metrics = evaluation.quality_metrics
evaluation_error = evaluation.error
evaluation_state = evaluation.state
evaluation_name = evaluation.name
evaluation_end_time = evaluation.end_time
evaluation_error_samples = evaluation.error_samples
```

---


### Completion_config

Completes the user input with advanced keyword suggestions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_pseudo_id` | String |  | Optional. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. This should be the same identifier as UserEvent.user_pseudo_id and SearchRequest.user_pseudo_id. The field must be a UTF-8 encoded string with a length limit of 128 |
| `query` | String |  | Required. The typeahead input used to fetch suggestions. Maximum length is 128 characters. The query can not be empty for most of the suggestion types. If it is empty, an `INVALID_ARGUMENT` error is returned. The exception is when the suggestion_types contains only the type `RECENT_SEARCH`, the query can be an empty string. The is called "zero prefix" feature, which returns user's recently searched queries given the empty query. |
| `suggestion_types` | Vec<String> |  | Optional. Suggestion types to return. If empty or unspecified, query suggestions are returned. Only one suggestion type is supported at the moment. |
| `user_info` | String |  | Optional. Information about the end user. This should be the same identifier information as UserEvent.user_info and SearchRequest.user_info. |
| `query_model` | String |  | Specifies the autocomplete query model, which only applies to the QUERY SuggestionType. This overrides any model specified in the Configuration > Autocomplete section of the Cloud console. Currently supported values: * `document` - Using suggestions generated from user-imported documents. * `search-history` - Using suggestions generated from the past history of SearchService.Search API calls. Do not use it when there is no traffic for Search API. * `user-event` - Using suggestions generated from user-imported search events. * `document-completable` - Using suggestions taken directly from user-imported document fields marked as completable. Default values: * `document` is the default model for regular dataStores. * `search-history` is the default model for site search dataStores. |
| `include_tail_suggestions` | bool |  | Indicates if tail suggestions should be returned if there are no suggestions that match the full query. Even if set to true, if there are suggestions that match the full query, those are returned and no tail suggestions are returned. |
| `suggestion_type_specs` | Vec<String> |  | Optional. Specification of each suggestion type. |
| `boost_spec` | String |  | Optional. Specification to boost suggestions matching the condition. |
| `experiment_ids` | Vec<String> |  | Optional. Experiment ids for this request. |
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


### Grounding_config

Performs a grounding check.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grounding_spec` | String |  | Configuration of the grounding check. |
| `answer_candidate` | String |  | Answer candidate to check. It can have a maximum length of 4096 tokens. |
| `facts` | Vec<String> |  | List of facts for the grounding check. We support up to 200 facts. |
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


### Ranking_config

Ranks a list of text records based on the given input query.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `top_n` | i64 |  | The number of results to return. If this is unset or no bigger than zero, returns all results. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `records` | Vec<String> |  | Required. A list of records to rank. |
| `model` | String |  | The identifier of the model to use. It is one of: * `semantic-ranker-512@latest`: Semantic ranking model with maximum input token size 512. It is set to `semantic-ranker-512@latest` by default if unspecified. |
| `query` | String |  | The query to use. |
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


### Conversation

Creates a Conversation. If the Conversation to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | The state of the Conversation. |
| `end_time` | String |  | Output only. The time the conversation finished. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `messages` | Vec<String> |  | Conversation messages. |
| `start_time` | String |  | Output only. The time the conversation started. |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The state of the Conversation. |
| `end_time` | String | Output only. The time the conversation finished. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/dataStore/*/conversations/*` or `projects/{project}/locations/global/collections/{collection}/engines/*/conversations/*`. |
| `messages` | Vec<String> | Conversation messages. |
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
conversation_state = conversation.state
conversation_end_time = conversation.end_time
conversation_user_pseudo_id = conversation.user_pseudo_id
conversation_name = conversation.name
conversation_messages = conversation.messages
conversation_start_time = conversation.start_time
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


### Identity_mapping_store

Creates a new Identity Mapping Store.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `cmek_config` | String |  | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |
| `parent` | String | ✅ | Required. The parent collection resource name, such as `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. The full resource name of the identity mapping store. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `cmek_config` | String | Output only. CMEK-related information for the Identity Mapping Store. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this Identity Mapping Store at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the Identity Mapping Store will be protected by the KMS key, as indicated in the cmek_config field. |


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
identity_mapping_store_cmek_config = identity_mapping_store.cmek_config
identity_mapping_store_kms_key_name = identity_mapping_store.kms_key_name
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


### Data_store

Creates a DataStore. DataStore is for storing Documents. To serve these documents for Search, or Recommendation use case, an Engine needs to be created separately.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configurable_billing_approach_update_time` | String |  | Output only. The timestamp when configurable_billing_approach was last updated. |
| `acl_enabled` | bool |  | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `identity_mapping_store` | String |  | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `industry_vertical` | String |  | Immutable. The industry vertical that the data store registers. |
| `create_time` | String |  | Output only. Timestamp the DataStore was created at. |
| `advanced_site_search_config` | String |  | Optional. Configuration for advanced site search. |
| `language_info` | String |  | Language info for DataStore. |
| `solution_types` | Vec<String> |  | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `cmek_config` | String |  | Output only. CMEK-related information for the DataStore. |
| `starting_schema` | String |  | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `workspace_config` | String |  | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `configurable_billing_approach` | String |  | Optional. Configuration for configurable billing approach. See |
| `display_name` | String |  | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `healthcare_fhir_config` | String |  | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `billing_estimation` | String |  | Output only. Data size estimation for billing. |
| `name` | String |  | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `natural_language_query_understanding_config` | String |  | Optional. Configuration for Natural Language Query Understanding. |
| `serving_config_data_store` | String |  | Optional. Stores serving config at DataStore level. |
| `document_processing_config` | String |  | Configuration for Document understanding and enrichment. |
| `kms_key_name` | String |  | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String |  | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `default_schema_id` | String |  | Output only. The id of the default Schema associated to this data store. |
| `is_infobot_faq_data_store` | bool |  | Optional. If set, this DataStore is an Infobot FAQ DataStore. |
| `parent` | String | ✅ | Required. The parent resource name, such as `projects/{project}/locations/{location}/collections/{collection}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configurable_billing_approach_update_time` | String | Output only. The timestamp when configurable_billing_approach was last updated. |
| `acl_enabled` | bool | Immutable. Whether data in the DataStore has ACL information. If set to `true`, the source data must have ACL. ACL will be ingested when data is ingested by DocumentService.ImportDocuments methods. When ACL is enabled for the DataStore, Document can't be accessed by calling DocumentService.GetDocument or DocumentService.ListDocuments. Currently ACL is only supported in `GENERIC` industry vertical with non-`PUBLIC_WEBSITE` content config. |
| `identity_mapping_store` | String | Immutable. The fully qualified resource name of the associated IdentityMappingStore. This field can only be set for acl_enabled DataStores with `THIRD_PARTY` or `GSUITE` IdP. Format: `projects/{project}/locations/{location}/identityMappingStores/{identity_mapping_store}`. |
| `industry_vertical` | String | Immutable. The industry vertical that the data store registers. |
| `create_time` | String | Output only. Timestamp the DataStore was created at. |
| `advanced_site_search_config` | String | Optional. Configuration for advanced site search. |
| `language_info` | String | Language info for DataStore. |
| `solution_types` | Vec<String> | The solutions that the data store enrolls. Available solutions for each industry_vertical: * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`. * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other solutions cannot be enrolled. |
| `cmek_config` | String | Output only. CMEK-related information for the DataStore. |
| `starting_schema` | String | The start schema to use for this DataStore when provisioning it. If unset, a default vertical specialized schema will be used. This field is only used by CreateDataStore API, and will be ignored if used in other APIs. This field will be omitted from all API responses including CreateDataStore API. To retrieve a schema of a DataStore, use SchemaService.GetSchema API instead. The provided schema will be validated against certain rules on schema. Learn more from [this doc](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema). |
| `workspace_config` | String | Config to store data store type configuration for workspace data. This must be set when DataStore.content_config is set as DataStore.ContentConfig.GOOGLE_WORKSPACE. |
| `configurable_billing_approach` | String | Optional. Configuration for configurable billing approach. See |
| `display_name` | String | Required. The data store display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `healthcare_fhir_config` | String | Optional. Configuration for `HEALTHCARE_FHIR` vertical. |
| `billing_estimation` | String | Output only. Data size estimation for billing. |
| `name` | String | Immutable. Identifier. The full resource name of the data store. Format: `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`. This field must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `natural_language_query_understanding_config` | String | Optional. Configuration for Natural Language Query Understanding. |
| `serving_config_data_store` | String | Optional. Stores serving config at DataStore level. |
| `document_processing_config` | String | Configuration for Document understanding and enrichment. |
| `kms_key_name` | String | Input only. The KMS key to be used to protect this DataStore at creation time. Must be set for requests that need to comply with CMEK Org Policy protections. If this field is set and processed successfully, the DataStore will be protected by the KMS key, as indicated in the cmek_config field. |
| `content_config` | String | Immutable. The content config of the data store. If this field is unset, the server behavior defaults to ContentConfig.NO_CONTENT. |
| `default_schema_id` | String | Output only. The id of the default Schema associated to this data store. |
| `is_infobot_faq_data_store` | bool | Optional. If set, this DataStore is an Infobot FAQ DataStore. |


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
data_store_configurable_billing_approach_update_time = data_store.configurable_billing_approach_update_time
data_store_acl_enabled = data_store.acl_enabled
data_store_identity_mapping_store = data_store.identity_mapping_store
data_store_industry_vertical = data_store.industry_vertical
data_store_create_time = data_store.create_time
data_store_advanced_site_search_config = data_store.advanced_site_search_config
data_store_language_info = data_store.language_info
data_store_solution_types = data_store.solution_types
data_store_cmek_config = data_store.cmek_config
data_store_starting_schema = data_store.starting_schema
data_store_workspace_config = data_store.workspace_config
data_store_configurable_billing_approach = data_store.configurable_billing_approach
data_store_display_name = data_store.display_name
data_store_healthcare_fhir_config = data_store.healthcare_fhir_config
data_store_billing_estimation = data_store.billing_estimation
data_store_name = data_store.name
data_store_natural_language_query_understanding_config = data_store.natural_language_query_understanding_config
data_store_serving_config_data_store = data_store.serving_config_data_store
data_store_document_processing_config = data_store.document_processing_config
data_store_kms_key_name = data_store.kms_key_name
data_store_content_config = data_store.content_config
data_store_default_schema_id = data_store.default_schema_id
data_store_is_infobot_faq_data_store = data_store.is_infobot_faq_data_store
```

---


### Session

Creates a Session. If the Session to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | Vec<String> |  | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `turns` | Vec<String> |  | Turns. |
| `state` | String |  | The state of the session. |
| `start_time` | String |  | Output only. The time the session started. |
| `end_time` | String |  | Output only. The time the session finished. |
| `is_pinned` | bool |  | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `user_pseudo_id` | String |  | A unique identifier for tracking users. |
| `display_name` | String |  | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
| `name` | String |  | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*` |
| `parent` | String | ✅ | Required. Full resource name of parent data store. Format: `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | Vec<String> | Optional. The labels for the session. Can be set as filter in ListSessionsRequest. |
| `turns` | Vec<String> | Turns. |
| `state` | String | The state of the session. |
| `start_time` | String | Output only. The time the session started. |
| `end_time` | String | Output only. The time the session finished. |
| `is_pinned` | bool | Optional. Whether the session is pinned, pinned session will be displayed on the top of the session list. |
| `user_pseudo_id` | String | A unique identifier for tracking users. |
| `display_name` | String | Optional. The display name of the session. This field is used to identify the session in the UI. By default, the display name is the first turn query text in the session. |
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
session_labels = session.labels
session_turns = session.turns
session_state = session.state
session_start_time = session.start_time
session_end_time = session.end_time
session_is_pinned = session.is_pinned
session_user_pseudo_id = session.user_pseudo_id
session_display_name = session.display_name
session_name = session.name
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
| `length` | String | Size of the data, in bytes |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `filename` | String | Original file name |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `token` | String | A unique fingerprint/version id for the media data |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `content_type` | String | MIME type of the data |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `download_parameters` | String | Parameters for a media download. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `reference_type` | String | Describes what the field reference contains. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |


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
media_length = media.length
media_blob_ref = media.blob_ref
media_is_potential_retry = media.is_potential_retry
media_path = media.path
media_sha256_hash = media.sha256_hash
media_sha1_hash = media.sha1_hash
media_filename = media.filename
media_md5_hash = media.md5_hash
media_diff_checksums_response = media.diff_checksums_response
media_token = media.token
media_composite_media = media.composite_media
media_algorithm = media.algorithm
media_content_type = media.content_type
media_object_id = media.object_id
media_content_type_info = media.content_type_info
media_timestamp = media.timestamp
media_download_parameters = media.download_parameters
media_inline = media.inline
media_diff_upload_request = media.diff_upload_request
media_cosmo_binary_reference = media.cosmo_binary_reference
media_crc32c_hash = media.crc32c_hash
media_bigstore_object_ref = media.bigstore_object_ref
media_diff_version_response = media.diff_version_response
media_hash_verified = media.hash_verified
media_media_id = media.media_id
media_diff_download_response = media.diff_download_response
media_blobstore2_info = media.blobstore2_info
media_diff_upload_response = media.diff_upload_response
media_reference_type = media.reference_type
media_hash = media.hash
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
| `answer_skipped_reasons` | Vec<String> | Additional answer-skipped reasons. This provides the reason for ignored cases. If nothing is skipped, this field is not set. |
| `steps` | Vec<String> | Answer generation steps. |
| `blob_attachments` | Vec<String> | List of blob attachments in the answer. |
| `safety_ratings` | Vec<String> | Optional. Safety ratings. |
| `answer_text` | String | The textual answer. |
| `complete_time` | String | Output only. Answer completed timestamp. |
| `citations` | Vec<String> | Citations. |
| `query_understanding_info` | String | Query understanding information. |
| `related_questions` | Vec<String> | Suggested related questions. |
| `state` | String | The state of the answer generation. |
| `references` | Vec<String> | References. |
| `create_time` | String | Output only. Answer creation timestamp. |
| `grounding_score` | f64 | A score in the range of [0, 1] describing how grounded the answer is by the reference chunks. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/global/collections/{collection}/engines/{engine}/sessions/*/answers/*` |
| `grounding_supports` | Vec<String> | Optional. Grounding supports. |


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
answer_answer_skipped_reasons = answer.answer_skipped_reasons
answer_steps = answer.steps
answer_blob_attachments = answer.blob_attachments
answer_safety_ratings = answer.safety_ratings
answer_answer_text = answer.answer_text
answer_complete_time = answer.complete_time
answer_citations = answer.citations
answer_query_understanding_info = answer.query_understanding_info
answer_related_questions = answer.related_questions
answer_state = answer.state
answer_references = answer.references
answer_create_time = answer.create_time
answer_grounding_score = answer.grounding_score
answer_name = answer.name
answer_grounding_supports = answer.grounding_supports
```

---


### Location

Sets the dedicated crawl rate for a crawl_rate_scope. If the dedicated crawl rate was not set, this will enable vertex AI's crawl bot to use the new dedicated crawl rate for crawling. If the dedicated crawl rate was set, vertex AI's crawl bot will try to update the rate to the new value. If the new value is too high, the crawl bot may crawl at a lower rate to avoid overloading the user's website.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `crawl_type` | String |  | Optional. Whether it's the crawl rate of user-triggered or auto-refresh. |
| `mode` | String |  | Optional. Whether the rate is explicitly set by users, or set by vertex AI. |
| `crawl_rate_scope` | String |  | Required. The scope of the crawl rate that the user wants to config. Currently, only domain and host name are supported. A domain name example: `example.com`. A host name example: `www.example.com`. Please do not include `/` in the domain or host name. |
| `crawl_rate` | i64 |  | Optional. The crawl QPS set by the user. It is not guaranteed that Vertex crawl bot will crawl at this QPS. If the crawl rate is too high, the real QPS may be lower than the value set by the user to avoid overloading the user's website. |
| `location` | String | ✅ | Required. The location resource where crawl rate management will be performed. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version` | String | Output only. KMS key version resource name which will be used to encrypt resources `/cryptoKeyVersions/{keyVersion}`. |
| `notebooklm_state` | String | Output only. Whether the NotebookLM Corpus is ready to be used. |
| `single_region_keys` | Vec<String> | Optional. Single-regional CMEKs that are required for some VAIS features. |
| `state` | String | Output only. The states of the CmekConfig. |
| `is_default` | bool | Output only. The default CmekConfig for the Customer. |
| `kms_key` | String | Required. KMS key resource name which will be used to encrypt resources `projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}`. |
| `name` | String | Required. The name of the CmekConfig of the form `projects/{project}/locations/{location}/cmekConfig` or `projects/{project}/locations/{location}/cmekConfigs/{cmek_config}`. |
| `last_rotation_timestamp_micros` | String | Output only. The timestamp of the last key rotation. |


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
location_kms_key_version = location.kms_key_version
location_notebooklm_state = location.notebooklm_state
location_single_region_keys = location.single_region_keys
location_state = location.state
location_is_default = location.is_default
location_kms_key = location.kms_key
location_name = location.name
location_last_rotation_timestamp_micros = location.last_rotation_timestamp_micros
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
| `query` | String |  | Optional. Current user query. Empty query is only supported if `file_ids` are provided. In this case, the answer will be generated based on those context files. |
| `generation_spec` | String |  | Optional. Specification of the generation configuration for the request. |
| `user_metadata` | String |  | Optional. Information about the user initiating the query. |
| `name` | String | ✅ | Required. The resource name of the Assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_grounding_type` | String | Optional. The type of web grounding to use. |
| `name` | String | Immutable. Resource name of the assistant. Format: `projects/{project}/locations/{location}/collections/{collection}/engines/{engine}/assistants/{assistant}` It must be a UTF-8 encoded string with a length limit of 1024 characters. |
| `customer_policy` | String | Optional. Customer policy for the assistant. |
| `enabled_tools` | HashMap<String, String> | Optional. Note: not implemented yet. Use enabled_actions instead. The enabled tools on this assistant. The keys are connector name, for example "projects/{projectId}/locations/{locationId}/collections/{collectionId}/dataconnector The values consist of admin enabled tools towards the connector instance. Admin can selectively enable multiple tools on any of the connector instances that they created in the project. For example {"jira1ConnectorName": [(toolId1, "createTicket"), (toolId2, "transferTicket")], "gmail1ConnectorName": [(toolId3, "sendEmail"),..] } |
| `generation_config` | String | Optional. Configuration for the generation of the assistant response. |


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
assistant_web_grounding_type = assistant.web_grounding_type
assistant_name = assistant.name
assistant_customer_policy = assistant.customer_policy
assistant_enabled_tools = assistant.enabled_tools
assistant_generation_config = assistant.generation_config
```

---


### Serving_config

Answer query method.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `search_spec` | String |  | Search specification. |
| `answer_generation_spec` | String |  | Answer generation specification. |
| `end_user_spec` | String |  | Optional. End user specification. |
| `related_questions_spec` | String |  | Related questions specification. |
| `session` | String |  | The session resource name. Not required. When session field is not set, the API is in sessionless mode. We support auto session mode: users can use the wildcard symbol `-` as session ID. A new ID will be automatically generated and assigned. |
| `query` | String |  | Required. Current user query. |
| `safety_spec` | String |  | Model specification. |
| `query_understanding_spec` | String |  | Query understanding specification. |
| `asynchronous_mode` | bool |  | Deprecated: This field is deprecated. Streaming Answer API will be supported. Asynchronous mode control. If enabled, the response will be returned with answer/session resource name without final answer. The API users need to do the polling to get the latest status of answer/session by calling ConversationalSearchService.GetAnswer or ConversationalSearchService.GetSession method. |
| `grounding_spec` | String |  | Optional. Grounding specification. |
| `user_labels` | HashMap<String, String> |  | The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `user_pseudo_id` | String |  | A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This field should NOT have a fixed value such as `unknown_visitor`. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an `INVALID_ARGUMENT` error is returned. |
| `serving_config` | String | ✅ | Required. The resource name of the Search serving config, such as `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`, or `projects/*/locations/global/collections/default_collection/dataStores/*/servingConfigs/default_serving_config`. This field is used to identify the serving configuration name, set of models used to make the search. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ranking_expression` | String | The ranking expression controls the customized ranking on retrieval documents. To leverage this, document embedding is required. The ranking expression setting in ServingConfig applies to all search requests served by the serving config. However, if `SearchRequest.ranking_expression` is specified, it overrides the ServingConfig ranking expression. The ranking expression is a single function or multiple functions that are joined by "+". * ranking_expression = function, { " + ", function }; Supported functions: * double * relevance_score * double * dotProduct(embedding_field_path) Function variables: * `relevance_score`: pre-defined keywords, used for measure relevance between query and document. * `embedding_field_path`: the document embedding field used with query embedding vector. * `dotProduct`: embedding function between embedding_field_path and query embedding vector. Example ranking expression: If document has an embedding field doc_embedding, the ranking expression could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`. |
| `personalization_spec` | String | The specification for personalization spec. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set, SearchRequest.personalization_spec overrides ServingConfig.personalization_spec. |
| `dissociate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. Order does not matter. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. Applied according to the order in the list. A previously replaced term can not be re-replaced. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `filter_control_ids` | Vec<String> | Filter controls to use in serving path. All triggered filter controls will be applied. Filter controls must be in the same data store as the serving config. Maximum of 20 filter controls. |
| `answer_generation_spec` | String | Optional. The specification for answer generation. |
| `model_id` | String | The id of the model to use at serving time. Currently only RecommendationModels are supported. Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `display_name` | String | Required. The human readable serving config display name. Used in Discovery UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `embedding_config` | String | Bring your own embedding config. The config is used for search semantic retrieval. The retrieval is based on the dot product of SearchRequest.EmbeddingSpec.EmbeddingVector.vector and the document embeddings that are provided by this EmbeddingConfig. If SearchRequest.EmbeddingSpec.EmbeddingVector.vector is provided, it overrides this ServingConfig.embedding_config. |
| `boost_control_ids` | Vec<String> | Boost controls to use in serving path. All triggered boost controls will be applied. Boost controls must be in the same data store as the serving config. Maximum of 20 boost controls. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. Order does not matter. Maximum number of specifications is 100. |
| `promote_control_ids` | Vec<String> | Condition promote specifications. Maximum number of specifications is 100. |
| `generic_config` | String | The GenericConfig of the serving configuration. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if SolutionType is SOLUTION_TYPE_RECOMMENDATION. |
| `redirect_control_ids` | Vec<String> | IDs of the redirect controls. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `solution_type` | String | Required. Immutable. Specifies the solution type that a serving config can be associated with. |
| `synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple synonyms conditions match, all matching synonyms controls in the list will execute. Maximum number of specifications is 100. Can only be set if SolutionType is SOLUTION_TYPE_SEARCH. |
| `update_time` | String | Output only. ServingConfig updated timestamp. |
| `name` | String | Immutable. Fully qualified name `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}` |
| `create_time` | String | Output only. ServingConfig created timestamp. |
| `media_config` | String | The MediaConfig of the serving configuration. |


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
serving_config_ranking_expression = serving_config.ranking_expression
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_dissociate_control_ids = serving_config.dissociate_control_ids
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_answer_generation_spec = serving_config.answer_generation_spec
serving_config_model_id = serving_config.model_id
serving_config_display_name = serving_config.display_name
serving_config_embedding_config = serving_config.embedding_config
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_promote_control_ids = serving_config.promote_control_ids
serving_config_generic_config = serving_config.generic_config
serving_config_diversity_level = serving_config.diversity_level
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_solution_type = serving_config.solution_type
serving_config_synonyms_control_ids = serving_config.synonyms_control_ids
serving_config_update_time = serving_config.update_time
serving_config_name = serving_config.name
serving_config_create_time = serving_config.create_time
serving_config_media_config = serving_config.media_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple sitemap resources
sitemap_0 = provider.discoveryengine_api.Sitemap {
    parent = "value-0"
}
sitemap_1 = provider.discoveryengine_api.Sitemap {
    parent = "value-1"
}
sitemap_2 = provider.discoveryengine_api.Sitemap {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sitemap = provider.discoveryengine_api.Sitemap {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Discoveryengine_api Documentation](https://cloud.google.com/discoveryengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
