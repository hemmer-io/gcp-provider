# Dataplex_api Service



**Resources**: 33

---

## Overview

The dataplex_api service provides access to 33 resource types:

- [Entrie](#entrie) [CRUD]
- [Governance_rule](#governance_rule) [CR]
- [Encryption_config](#encryption_config) [CRUD]
- [Action](#action) [R]
- [Data_asset](#data_asset) [C]
- [Glossarie](#glossarie) [CRUD]
- [Partition](#partition) [CRD]
- [Job](#job) [CR]
- [Environment](#environment) [CRUD]
- [Content](#content) [CRUD]
- [Zone](#zone) [CRUD]
- [Data_attribute_binding](#data_attribute_binding) [CRUD]
- [Entry_link](#entry_link) [CRD]
- [Aspect_type](#aspect_type) [CRUD]
- [Entry_link_type](#entry_link_type) [CR]
- [Term](#term) [CRUD]
- [Session](#session) [R]
- [Operation](#operation) [CRD]
- [Change_request](#change_request) [CR]
- [Entitie](#entitie) [CRUD]
- [Lake](#lake) [CRUD]
- [Task](#task) [CRUD]
- [Data_taxonomie](#data_taxonomie) [CRUD]
- [Entry_group](#entry_group) [CRUD]
- [Contentitem](#contentitem) [CRUD]
- [Data_scan](#data_scan) [CRUD]
- [Attribute](#attribute) [CRUD]
- [Location](#location) [CR]
- [Entry_type](#entry_type) [CRUD]
- [Metadata_job](#metadata_job) [CR]
- [Categorie](#categorie) [CRUD]
- [Data_product](#data_product) [C]
- [Asset](#asset) [CRUD]

---

## Resources


### Entrie

Creates an Entry.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fully_qualified_name` | String |  | Optional. A name for the entry that can be referenced by an external system. For more information, see Fully qualified names (https://cloud.google.com/data-catalog/docs/fully-qualified-names). The maximum size of the field is 4000 characters. |
| `aspects` | HashMap<String, String> |  | Optional. The aspects that are attached to the entry. Depending on how the aspect is attached to the entry, the format of the aspect key can be one of the following: If the aspect is attached directly to the entry: {project_id_or_number}.{location_id}.{aspect_type_id} If the aspect is attached to an entry's path: {project_id_or_number}.{location_id}.{aspect_type_id}@{path} |
| `entry_type` | String |  | Required. Immutable. The relative resource name of the entry type that was used to create this entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryTypes/{entry_type_id}. |
| `create_time` | String |  | Output only. The time when the entry was created in Dataplex Universal Catalog. |
| `parent_entry` | String |  | Optional. Immutable. The resource name of the parent entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}. |
| `update_time` | String |  | Output only. The time when the entry was last updated in Dataplex Universal Catalog. |
| `name` | String |  | Identifier. The relative resource name of the entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}. |
| `entry_source` | String |  | Optional. Information related to the source system of the data resource that is represented by the entry. |
| `parent` | String | ✅ | Required. The resource name of the parent Entry Group: projects/{project}/locations/{location}/entryGroups/{entry_group}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fully_qualified_name` | String | Optional. A name for the entry that can be referenced by an external system. For more information, see Fully qualified names (https://cloud.google.com/data-catalog/docs/fully-qualified-names). The maximum size of the field is 4000 characters. |
| `aspects` | HashMap<String, String> | Optional. The aspects that are attached to the entry. Depending on how the aspect is attached to the entry, the format of the aspect key can be one of the following: If the aspect is attached directly to the entry: {project_id_or_number}.{location_id}.{aspect_type_id} If the aspect is attached to an entry's path: {project_id_or_number}.{location_id}.{aspect_type_id}@{path} |
| `entry_type` | String | Required. Immutable. The relative resource name of the entry type that was used to create this entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryTypes/{entry_type_id}. |
| `create_time` | String | Output only. The time when the entry was created in Dataplex Universal Catalog. |
| `parent_entry` | String | Optional. Immutable. The resource name of the parent entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}. |
| `update_time` | String | Output only. The time when the entry was last updated in Dataplex Universal Catalog. |
| `name` | String | Identifier. The relative resource name of the entry, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}. |
| `entry_source` | String | Optional. Information related to the source system of the data resource that is represented by the entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.dataplex_api.Entrie {
    parent = "value"  # Required. The resource name of the parent Entry Group: projects/{project}/locations/{location}/entryGroups/{entry_group}.
}

# Access entrie outputs
entrie_id = entrie.id
entrie_fully_qualified_name = entrie.fully_qualified_name
entrie_aspects = entrie.aspects
entrie_entry_type = entrie.entry_type
entrie_create_time = entrie.create_time
entrie_parent_entry = entrie.parent_entry
entrie_update_time = entrie.update_time
entrie_name = entrie.name
entrie_entry_source = entrie.entry_source
```

---


### Governance_rule

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create governance_rule
governance_rule = provider.dataplex_api.Governance_rule {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access governance_rule outputs
governance_rule_id = governance_rule.id
governance_rule_audit_configs = governance_rule.audit_configs
governance_rule_bindings = governance_rule.bindings
governance_rule_version = governance_rule.version
governance_rule_etag = governance_rule.etag
```

---


### Encryption_config

Create an EncryptionConfig.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the Encryption configuration was created. |
| `enable_metastore_encryption` | bool |  | Optional. Represent the state of CMEK opt-in for metastore. |
| `etag` | String |  | Etag of the EncryptionConfig. This is a strong etag. |
| `name` | String |  | Identifier. The resource name of the EncryptionConfig. Format: organizations/{organization}/locations/{location}/encryptionConfigs/{encryption_config} Global location is not supported. |
| `failure_details` | String |  | Output only. Details of the failure if anything related to Cmek db fails. |
| `encryption_state` | String |  | Output only. The state of encryption of the databases. |
| `key` | String |  | Optional. If a key is chosen, it means that the customer is using CMEK. If a key is not chosen, it means that the customer is using Google managed encryption. |
| `update_time` | String |  | Output only. The time when the Encryption configuration was last updated. |
| `parent` | String | ✅ | Required. The location at which the EncryptionConfig is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the Encryption configuration was created. |
| `enable_metastore_encryption` | bool | Optional. Represent the state of CMEK opt-in for metastore. |
| `etag` | String | Etag of the EncryptionConfig. This is a strong etag. |
| `name` | String | Identifier. The resource name of the EncryptionConfig. Format: organizations/{organization}/locations/{location}/encryptionConfigs/{encryption_config} Global location is not supported. |
| `failure_details` | String | Output only. Details of the failure if anything related to Cmek db fails. |
| `encryption_state` | String | Output only. The state of encryption of the databases. |
| `key` | String | Optional. If a key is chosen, it means that the customer is using CMEK. If a key is not chosen, it means that the customer is using Google managed encryption. |
| `update_time` | String | Output only. The time when the Encryption configuration was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create encryption_config
encryption_config = provider.dataplex_api.Encryption_config {
    parent = "value"  # Required. The location at which the EncryptionConfig is to be created.
}

# Access encryption_config outputs
encryption_config_id = encryption_config.id
encryption_config_create_time = encryption_config.create_time
encryption_config_enable_metastore_encryption = encryption_config.enable_metastore_encryption
encryption_config_etag = encryption_config.etag
encryption_config_name = encryption_config.name
encryption_config_failure_details = encryption_config.failure_details
encryption_config_encryption_state = encryption_config.encryption_state
encryption_config_key = encryption_config.key
encryption_config_update_time = encryption_config.update_time
```

---


### Action

Lists action resources in a lake.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | Actions under the given parent lake/zone/asset. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access action outputs
action_id = action.id
action_actions = action.actions
action_next_page_token = action.next_page_token
```

---


### Data_asset

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_asset
data_asset = provider.dataplex_api.Data_asset {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Glossarie

Creates a new Glossary resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. Needed for resource freshness validation. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String |  | Output only. Identifier. The resource name of the Glossary. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} |
| `category_count` | i64 |  | Output only. The number of GlossaryCategories in the Glossary. |
| `create_time` | String |  | Output only. The time at which the Glossary was created. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the Glossary. |
| `term_count` | i64 |  | Output only. The number of GlossaryTerms in the Glossary. |
| `uid` | String |  | Output only. System generated unique id for the Glossary. This ID will be different if the Glossary is deleted and re-created with the same name. |
| `update_time` | String |  | Output only. The time at which the Glossary was last updated. |
| `description` | String |  | Optional. The user-mutable description of the Glossary. |
| `display_name` | String |  | Optional. User friendly display name of the Glossary. This is user-mutable. This will be same as the GlossaryId, if not specified. |
| `parent` | String | ✅ | Required. The parent resource where this Glossary will be created. Format: projects/{project_id_or_number}/locations/{location_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. Needed for resource freshness validation. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String | Output only. Identifier. The resource name of the Glossary. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} |
| `category_count` | i64 | Output only. The number of GlossaryCategories in the Glossary. |
| `create_time` | String | Output only. The time at which the Glossary was created. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the Glossary. |
| `term_count` | i64 | Output only. The number of GlossaryTerms in the Glossary. |
| `uid` | String | Output only. System generated unique id for the Glossary. This ID will be different if the Glossary is deleted and re-created with the same name. |
| `update_time` | String | Output only. The time at which the Glossary was last updated. |
| `description` | String | Optional. The user-mutable description of the Glossary. |
| `display_name` | String | Optional. User friendly display name of the Glossary. This is user-mutable. This will be same as the GlossaryId, if not specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create glossarie
glossarie = provider.dataplex_api.Glossarie {
    parent = "value"  # Required. The parent resource where this Glossary will be created. Format: projects/{project_id_or_number}/locations/{location_id} where location_id refers to a Google Cloud region.
}

# Access glossarie outputs
glossarie_id = glossarie.id
glossarie_etag = glossarie.etag
glossarie_name = glossarie.name
glossarie_category_count = glossarie.category_count
glossarie_create_time = glossarie.create_time
glossarie_labels = glossarie.labels
glossarie_term_count = glossarie.term_count
glossarie_uid = glossarie.uid
glossarie_update_time = glossarie.update_time
glossarie_description = glossarie.description
glossarie_display_name = glossarie.display_name
```

---


### Partition

Create a metadata partition.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | Required. Immutable. The location of the entity data within the partition, for example, gs://bucket/path/to/entity/key1=value1/key2=value2. Or projects//datasets//tables/ |
| `name` | String |  | Output only. Partition values used in the HTTP URL must be double encoded. For example, url_encode(url_encode(value)) can be used to encode "US:CA/CA#Sunnyvale so that the request URL ends with "/partitions/US%253ACA/CA%2523Sunnyvale". The name field in the response retains the encoded format. |
| `values` | Vec<String> |  | Required. Immutable. The set of values representing the partition, which correspond to the partition schema defined in the parent entity. |
| `etag` | String |  | Optional. The etag for this partition. |
| `parent` | String | ✅ | Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | Required. Immutable. The location of the entity data within the partition, for example, gs://bucket/path/to/entity/key1=value1/key2=value2. Or projects//datasets//tables/ |
| `name` | String | Output only. Partition values used in the HTTP URL must be double encoded. For example, url_encode(url_encode(value)) can be used to encode "US:CA/CA#Sunnyvale so that the request URL ends with "/partitions/US%253ACA/CA%2523Sunnyvale". The name field in the response retains the encoded format. |
| `values` | Vec<String> | Required. Immutable. The set of values representing the partition, which correspond to the partition schema defined in the parent entity. |
| `etag` | String | Optional. The etag for this partition. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create partition
partition = provider.dataplex_api.Partition {
    parent = "value"  # Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
}

# Access partition outputs
partition_id = partition.id
partition_location = partition.location
partition_name = partition.name
partition_values = partition.values
partition_etag = partition.etag
```

---


### Job

Cancel jobs running for the task resource.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the job: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}/job/{job_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Execution state for the DataScanJob. |
| `data_discovery_spec` | String | Output only. Settings for a data discovery scan. |
| `create_time` | String | Output only. The time when the DataScanJob was created. |
| `data_profile_result` | String | Output only. The result of a data profile scan. |
| `data_documentation_spec` | String | Output only. Settings for a data documentation scan. |
| `start_time` | String | Output only. The time when the DataScanJob was started. |
| `end_time` | String | Output only. The time when the DataScanJob ended. |
| `data_quality_spec` | String | Output only. Settings for a data quality scan. |
| `type` | String | Output only. The type of the parent DataScan. |
| `name` | String | Output only. Identifier. The relative resource name of the DataScanJob, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}/jobs/{job_id}, where project refers to a project_id or project_number and location_id refers to a Google Cloud region. |
| `data_quality_result` | String | Output only. The result of a data quality scan. |
| `data_profile_spec` | String | Output only. Settings for a data profile scan. |
| `uid` | String | Output only. System generated globally unique ID for the DataScanJob. |
| `data_discovery_result` | String | Output only. The result of a data discovery scan. |
| `data_documentation_result` | String | Output only. The result of a data documentation scan. |
| `message` | String | Output only. Additional information about the current state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.dataplex_api.Job {
    name = "value"  # Required. The resource name of the job: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}/job/{job_id}.
}

# Access job outputs
job_id = job.id
job_state = job.state
job_data_discovery_spec = job.data_discovery_spec
job_create_time = job.create_time
job_data_profile_result = job.data_profile_result
job_data_documentation_spec = job.data_documentation_spec
job_start_time = job.start_time
job_end_time = job.end_time
job_data_quality_spec = job.data_quality_spec
job_type = job.type
job_name = job.name
job_data_quality_result = job.data_quality_result
job_data_profile_spec = job.data_profile_spec
job_uid = job.uid
job_data_discovery_result = job.data_discovery_result
job_data_documentation_result = job.data_documentation_result
job_message = job.message
```

---


### Environment

Create an environment resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `infrastructure_spec` | String |  | Required. Infrastructure specification for the Environment. |
| `display_name` | String |  | Optional. User friendly display name. |
| `name` | String |  | Output only. The relative resource name of the environment, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id} |
| `session_status` | String |  | Output only. Status of sessions created for this environment. |
| `create_time` | String |  | Output only. Environment creation time. |
| `labels` | HashMap<String, String> |  | Optional. User defined labels for the environment. |
| `description` | String |  | Optional. Description of the environment. |
| `endpoints` | String |  | Output only. URI Endpoints to access sessions associated with the Environment. |
| `session_spec` | String |  | Optional. Configuration for sessions created for this environment. |
| `update_time` | String |  | Output only. The time when the environment was last updated. |
| `state` | String |  | Output only. Current state of the environment. |
| `uid` | String |  | Output only. System generated globally unique ID for the environment. This ID will be different if the environment is deleted and re-created with the same name. |
| `parent` | String | ✅ | Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `infrastructure_spec` | String | Required. Infrastructure specification for the Environment. |
| `display_name` | String | Optional. User friendly display name. |
| `name` | String | Output only. The relative resource name of the environment, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id} |
| `session_status` | String | Output only. Status of sessions created for this environment. |
| `create_time` | String | Output only. Environment creation time. |
| `labels` | HashMap<String, String> | Optional. User defined labels for the environment. |
| `description` | String | Optional. Description of the environment. |
| `endpoints` | String | Output only. URI Endpoints to access sessions associated with the Environment. |
| `session_spec` | String | Optional. Configuration for sessions created for this environment. |
| `update_time` | String | Output only. The time when the environment was last updated. |
| `state` | String | Output only. Current state of the environment. |
| `uid` | String | Output only. System generated globally unique ID for the environment. This ID will be different if the environment is deleted and re-created with the same name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.dataplex_api.Environment {
    parent = "value"  # Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}.
}

# Access environment outputs
environment_id = environment.id
environment_infrastructure_spec = environment.infrastructure_spec
environment_display_name = environment.display_name
environment_name = environment.name
environment_session_status = environment.session_status
environment_create_time = environment.create_time
environment_labels = environment.labels
environment_description = environment.description
environment_endpoints = environment.endpoints
environment_session_spec = environment.session_spec
environment_update_time = environment.update_time
environment_state = environment.state
environment_uid = environment.uid
```

---


### Content

Create a content.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id} |
| `data_text` | String |  | Required. Content data in string format. |
| `notebook` | String |  | Notebook related configurations. |
| `update_time` | String |  | Output only. The time when the content was last updated. |
| `create_time` | String |  | Output only. Content creation time. |
| `description` | String |  | Optional. Description of the content. |
| `labels` | HashMap<String, String> |  | Optional. User defined labels for the content. |
| `sql_script` | String |  | Sql Script related configurations. |
| `path` | String |  | Required. The path for the Content file, represented as directory structure. Unique within a lake. Limited to alphanumerics, hyphens, underscores, dots and slashes. |
| `uid` | String |  | Output only. System generated globally unique ID for the content. This ID will be different if the content is deleted and re-created with the same name. |
| `parent` | String | ✅ | Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id} |
| `data_text` | String | Required. Content data in string format. |
| `notebook` | String | Notebook related configurations. |
| `update_time` | String | Output only. The time when the content was last updated. |
| `create_time` | String | Output only. Content creation time. |
| `description` | String | Optional. Description of the content. |
| `labels` | HashMap<String, String> | Optional. User defined labels for the content. |
| `sql_script` | String | Sql Script related configurations. |
| `path` | String | Required. The path for the Content file, represented as directory structure. Unique within a lake. Limited to alphanumerics, hyphens, underscores, dots and slashes. |
| `uid` | String | Output only. System generated globally unique ID for the content. This ID will be different if the content is deleted and re-created with the same name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create content
content = provider.dataplex_api.Content {
    parent = "value"  # Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
}

# Access content outputs
content_id = content.id
content_name = content.name
content_data_text = content.data_text
content_notebook = content.notebook
content_update_time = content.update_time
content_create_time = content.create_time
content_description = content.description
content_labels = content.labels
content_sql_script = content.sql_script
content_path = content.path
content_uid = content.uid
```

---


### Zone

Creates a zone resource within a lake.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the zone was created. |
| `update_time` | String |  | Output only. The time when the zone was last updated. |
| `name` | String |  | Output only. The relative resource name of the zone, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}. |
| `description` | String |  | Optional. Description of the zone. |
| `labels` | HashMap<String, String> |  | Optional. User defined labels for the zone. |
| `discovery_spec` | String |  | Optional. Specification of the discovery feature applied to data in this zone. |
| `type` | String |  | Required. Immutable. The type of the zone. |
| `resource_spec` | String |  | Required. Specification of the resources that are referenced by the assets within this zone. |
| `state` | String |  | Output only. Current state of the zone. |
| `asset_status` | String |  | Output only. Aggregated status of the underlying assets of the zone. |
| `display_name` | String |  | Optional. User friendly display name. |
| `uid` | String |  | Output only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name. |
| `parent` | String | ✅ | Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the zone was created. |
| `update_time` | String | Output only. The time when the zone was last updated. |
| `name` | String | Output only. The relative resource name of the zone, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}. |
| `description` | String | Optional. Description of the zone. |
| `labels` | HashMap<String, String> | Optional. User defined labels for the zone. |
| `discovery_spec` | String | Optional. Specification of the discovery feature applied to data in this zone. |
| `type` | String | Required. Immutable. The type of the zone. |
| `resource_spec` | String | Required. Specification of the resources that are referenced by the assets within this zone. |
| `state` | String | Output only. Current state of the zone. |
| `asset_status` | String | Output only. Aggregated status of the underlying assets of the zone. |
| `display_name` | String | Optional. User friendly display name. |
| `uid` | String | Output only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create zone
zone = provider.dataplex_api.Zone {
    parent = "value"  # Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
}

# Access zone outputs
zone_id = zone.id
zone_create_time = zone.create_time
zone_update_time = zone.update_time
zone_name = zone.name
zone_description = zone.description
zone_labels = zone.labels
zone_discovery_spec = zone.discovery_spec
zone_type = zone.type
zone_resource_spec = zone.resource_spec
zone_state = zone.state
zone_asset_status = zone.asset_status
zone_display_name = zone.display_name
zone_uid = zone.uid
```

---


### Data_attribute_binding

Create a DataAttributeBinding resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the DataAttributeBinding was created. |
| `display_name` | String |  | Optional. User friendly display name. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the DataAttributeBinding. |
| `uid` | String |  | Output only. System generated globally unique ID for the DataAttributeBinding. This ID will be different if the DataAttributeBinding is deleted and re-created with the same name. |
| `attributes` | Vec<String> |  | Optional. List of attributes to be associated with the resource, provided in the form: projects/{project}/locations/{location}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id} |
| `paths` | Vec<String> |  | Optional. The list of paths for items within the associated resource (eg. columns and partitions within a table) along with attribute bindings. |
| `name` | String |  | Output only. The relative resource name of the Data Attribute Binding, of the form: projects/{project_number}/locations/{location}/dataAttributeBindings/{data_attribute_binding_id} |
| `resource` | String |  | Optional. Immutable. The resource name of the resource that is associated to attributes. Presently, only entity resource is supported in the form: projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/entities/{entity_id} Must belong in the same project and region as the attribute binding, and there can only exist one active binding for a resource. |
| `update_time` | String |  | Output only. The time when the DataAttributeBinding was last updated. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Etags must be used when calling the DeleteDataAttributeBinding and the UpdateDataAttributeBinding method. |
| `description` | String |  | Optional. Description of the DataAttributeBinding. |
| `parent` | String | ✅ | Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the DataAttributeBinding was created. |
| `display_name` | String | Optional. User friendly display name. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the DataAttributeBinding. |
| `uid` | String | Output only. System generated globally unique ID for the DataAttributeBinding. This ID will be different if the DataAttributeBinding is deleted and re-created with the same name. |
| `attributes` | Vec<String> | Optional. List of attributes to be associated with the resource, provided in the form: projects/{project}/locations/{location}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id} |
| `paths` | Vec<String> | Optional. The list of paths for items within the associated resource (eg. columns and partitions within a table) along with attribute bindings. |
| `name` | String | Output only. The relative resource name of the Data Attribute Binding, of the form: projects/{project_number}/locations/{location}/dataAttributeBindings/{data_attribute_binding_id} |
| `resource` | String | Optional. Immutable. The resource name of the resource that is associated to attributes. Presently, only entity resource is supported in the form: projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/entities/{entity_id} Must belong in the same project and region as the attribute binding, and there can only exist one active binding for a resource. |
| `update_time` | String | Output only. The time when the DataAttributeBinding was last updated. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. Etags must be used when calling the DeleteDataAttributeBinding and the UpdateDataAttributeBinding method. |
| `description` | String | Optional. Description of the DataAttributeBinding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_attribute_binding
data_attribute_binding = provider.dataplex_api.Data_attribute_binding {
    parent = "value"  # Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id}
}

# Access data_attribute_binding outputs
data_attribute_binding_id = data_attribute_binding.id
data_attribute_binding_create_time = data_attribute_binding.create_time
data_attribute_binding_display_name = data_attribute_binding.display_name
data_attribute_binding_labels = data_attribute_binding.labels
data_attribute_binding_uid = data_attribute_binding.uid
data_attribute_binding_attributes = data_attribute_binding.attributes
data_attribute_binding_paths = data_attribute_binding.paths
data_attribute_binding_name = data_attribute_binding.name
data_attribute_binding_resource = data_attribute_binding.resource
data_attribute_binding_update_time = data_attribute_binding.update_time
data_attribute_binding_etag = data_attribute_binding.etag
data_attribute_binding_description = data_attribute_binding.description
```

---


### Entry_link

Creates an Entry Link.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entry_link_type` | String |  | Required. Immutable. Relative resource name of the Entry Link Type used to create this Entry Link. For example: Entry link between synonym terms in a glossary: projects/dataplex-types/locations/global/entryLinkTypes/synonym Entry link between related terms in a glossary: projects/dataplex-types/locations/global/entryLinkTypes/related Entry link between glossary terms and data assets: projects/dataplex-types/locations/global/entryLinkTypes/definition |
| `create_time` | String |  | Output only. The time when the Entry Link was created. |
| `update_time` | String |  | Output only. The time when the Entry Link was last updated. |
| `name` | String |  | Output only. Immutable. Identifier. The relative resource name of the Entry Link, of the form: projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id} |
| `entry_references` | Vec<String> |  | Required. Specifies the Entries referenced in the Entry Link. There should be exactly two entry references. |
| `parent` | String | ✅ | Required. The resource name of the parent Entry Group: projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entry_link_type` | String | Required. Immutable. Relative resource name of the Entry Link Type used to create this Entry Link. For example: Entry link between synonym terms in a glossary: projects/dataplex-types/locations/global/entryLinkTypes/synonym Entry link between related terms in a glossary: projects/dataplex-types/locations/global/entryLinkTypes/related Entry link between glossary terms and data assets: projects/dataplex-types/locations/global/entryLinkTypes/definition |
| `create_time` | String | Output only. The time when the Entry Link was created. |
| `update_time` | String | Output only. The time when the Entry Link was last updated. |
| `name` | String | Output only. Immutable. Identifier. The relative resource name of the Entry Link, of the form: projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id} |
| `entry_references` | Vec<String> | Required. Specifies the Entries referenced in the Entry Link. There should be exactly two entry references. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_link
entry_link = provider.dataplex_api.Entry_link {
    parent = "value"  # Required. The resource name of the parent Entry Group: projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}.
}

# Access entry_link outputs
entry_link_id = entry_link.id
entry_link_entry_link_type = entry_link.entry_link_type
entry_link_create_time = entry_link.create_time
entry_link_update_time = entry_link.update_time
entry_link_name = entry_link.name
entry_link_entry_references = entry_link.entry_references
```

---


### Aspect_type

Creates an AspectType.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_classification` | String |  | Optional. Immutable. Stores data classification of the aspect. |
| `transfer_status` | String |  | Output only. Denotes the transfer status of the Aspect Type. It is unspecified for Aspect Types created from Dataplex API. |
| `etag` | String |  | The service computes this checksum. The client may send it on update and delete requests to ensure it has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the AspectType. |
| `name` | String |  | Output only. The relative resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}. |
| `description` | String |  | Optional. Description of the AspectType. |
| `authorization` | String |  | Immutable. Defines the Authorization for this type. |
| `update_time` | String |  | Output only. The time when the AspectType was last updated. |
| `uid` | String |  | Output only. System generated globally unique ID for the AspectType. If you delete and recreate the AspectType with the same name, then this ID will be different. |
| `metadata_template` | String |  | Required. MetadataTemplate of the aspect. |
| `display_name` | String |  | Optional. User friendly display name. |
| `create_time` | String |  | Output only. The time when the AspectType was created. |
| `parent` | String | ✅ | Required. The resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_classification` | String | Optional. Immutable. Stores data classification of the aspect. |
| `transfer_status` | String | Output only. Denotes the transfer status of the Aspect Type. It is unspecified for Aspect Types created from Dataplex API. |
| `etag` | String | The service computes this checksum. The client may send it on update and delete requests to ensure it has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the AspectType. |
| `name` | String | Output only. The relative resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}. |
| `description` | String | Optional. Description of the AspectType. |
| `authorization` | String | Immutable. Defines the Authorization for this type. |
| `update_time` | String | Output only. The time when the AspectType was last updated. |
| `uid` | String | Output only. System generated globally unique ID for the AspectType. If you delete and recreate the AspectType with the same name, then this ID will be different. |
| `metadata_template` | String | Required. MetadataTemplate of the aspect. |
| `display_name` | String | Optional. User friendly display name. |
| `create_time` | String | Output only. The time when the AspectType was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create aspect_type
aspect_type = provider.dataplex_api.Aspect_type {
    parent = "value"  # Required. The resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region.
}

# Access aspect_type outputs
aspect_type_id = aspect_type.id
aspect_type_data_classification = aspect_type.data_classification
aspect_type_transfer_status = aspect_type.transfer_status
aspect_type_etag = aspect_type.etag
aspect_type_labels = aspect_type.labels
aspect_type_name = aspect_type.name
aspect_type_description = aspect_type.description
aspect_type_authorization = aspect_type.authorization
aspect_type_update_time = aspect_type.update_time
aspect_type_uid = aspect_type.uid
aspect_type_metadata_template = aspect_type.metadata_template
aspect_type_display_name = aspect_type.display_name
aspect_type_create_time = aspect_type.create_time
```

---


### Entry_link_type

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_link_type
entry_link_type = provider.dataplex_api.Entry_link_type {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access entry_link_type outputs
entry_link_type_id = entry_link_type.id
entry_link_type_audit_configs = entry_link_type.audit_configs
entry_link_type_bindings = entry_link_type.bindings
entry_link_type_version = entry_link_type.version
entry_link_type_etag = entry_link_type.etag
```

---


### Term

Creates a new GlossaryTerm resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Required. The immediate parent of the GlossaryTerm in the resource-hierarchy. It can either be a Glossary or a GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} OR projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the GlossaryTerm. |
| `name` | String |  | Output only. Identifier. The resource name of the GlossaryTerm. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/terms/{term_id} |
| `uid` | String |  | Output only. System generated unique id for the GlossaryTerm. This ID will be different if the GlossaryTerm is deleted and re-created with the same name. |
| `display_name` | String |  | Optional. User friendly display name of the GlossaryTerm. This is user-mutable. This will be same as the GlossaryTermId, if not specified. |
| `update_time` | String |  | Output only. The time at which the GlossaryTerm was last updated. |
| `description` | String |  | Optional. The user-mutable description of the GlossaryTerm. |
| `create_time` | String |  | Output only. The time at which the GlossaryTerm was created. |
| `parent` | String | ✅ | Required. The parent resource where the GlossaryTerm will be created. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent` | String | Required. The immediate parent of the GlossaryTerm in the resource-hierarchy. It can either be a Glossary or a GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} OR projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the GlossaryTerm. |
| `name` | String | Output only. Identifier. The resource name of the GlossaryTerm. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/terms/{term_id} |
| `uid` | String | Output only. System generated unique id for the GlossaryTerm. This ID will be different if the GlossaryTerm is deleted and re-created with the same name. |
| `display_name` | String | Optional. User friendly display name of the GlossaryTerm. This is user-mutable. This will be same as the GlossaryTermId, if not specified. |
| `update_time` | String | Output only. The time at which the GlossaryTerm was last updated. |
| `description` | String | Optional. The user-mutable description of the GlossaryTerm. |
| `create_time` | String | Output only. The time at which the GlossaryTerm was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create term
term = provider.dataplex_api.Term {
    parent = "value"  # Required. The parent resource where the GlossaryTerm will be created. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} where location_id refers to a Google Cloud region.
}

# Access term outputs
term_id = term.id
term_parent = term.parent
term_labels = term.labels
term_name = term.name
term_uid = term.uid
term_display_name = term.display_name
term_update_time = term.update_time
term_description = term.description
term_create_time = term.create_time
```

---


### Session

Lists session resources in an environment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `sessions` | Vec<String> | Sessions under a given environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access session outputs
session_id = session.id
session_next_page_token = session.next_page_token
session_sessions = session.sessions
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.dataplex_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Change_request

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy. |
| `version` | i64 | Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change_request
change_request = provider.dataplex_api.Change_request {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access change_request outputs
change_request_id = change_request.id
change_request_audit_configs = change_request.audit_configs
change_request_bindings = change_request.bindings
change_request_version = change_request.version
change_request_etag = change_request.etag
```

---


### Entitie

Create a metadata entity.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. System generated unique ID for the Entity. This ID will be different if the Entity is deleted and re-created with the same name. |
| `data_path_pattern` | String |  | Optional. The set of items within the data path constituting the data in the entity, represented as a glob path. Example: gs://bucket/path/to/data/**/*.csv. |
| `asset` | String |  | Required. Immutable. The ID of the asset associated with the storage location containing the entity data. The entity must be with in the same zone with the asset. |
| `create_time` | String |  | Output only. The time when the entity was created. |
| `access` | String |  | Output only. Identifies the access mechanism to the entity. Not user settable. |
| `name` | String |  | Output only. The resource name of the entity, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{id}. |
| `data_path` | String |  | Required. Immutable. The storage path of the entity data. For Cloud Storage data, this is the fully-qualified path to the entity, such as gs://bucket/path/to/data. For BigQuery data, this is the name of the table resource, such as projects/project_id/datasets/dataset_id/tables/table_id. |
| `type` | String |  | Required. Immutable. The type of entity. |
| `id` | String |  | Required. A user-provided entity ID. It is mutable, and will be used as the published table name. Specifying a new ID in an update entity request will override the existing value. The ID must contain only letters (a-z, A-Z), numbers (0-9), and underscores, and consist of 256 or fewer characters. |
| `description` | String |  | Optional. User friendly longer description text. Must be shorter than or equal to 1024 characters. |
| `etag` | String |  | Optional. The etag associated with the entity, which can be retrieved with a GetEntity request. Required for update and delete requests. |
| `catalog_entry` | String |  | Output only. The name of the associated Data Catalog entry. |
| `display_name` | String |  | Optional. Display name must be shorter than or equal to 256 characters. |
| `format` | String |  | Required. Identifies the storage format of the entity data. It does not apply to entities with data stored in BigQuery. |
| `schema` | String |  | Required. The description of the data structure and layout. The schema is not included in list responses. It is only included in SCHEMA and FULL entity views of a GetEntity response. |
| `system` | String |  | Required. Immutable. Identifies the storage system of the entity data. |
| `compatibility` | String |  | Output only. Metadata stores that the entity is compatible with. |
| `update_time` | String |  | Output only. The time when the entity was last updated. |
| `parent` | String | ✅ | Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. System generated unique ID for the Entity. This ID will be different if the Entity is deleted and re-created with the same name. |
| `data_path_pattern` | String | Optional. The set of items within the data path constituting the data in the entity, represented as a glob path. Example: gs://bucket/path/to/data/**/*.csv. |
| `asset` | String | Required. Immutable. The ID of the asset associated with the storage location containing the entity data. The entity must be with in the same zone with the asset. |
| `create_time` | String | Output only. The time when the entity was created. |
| `access` | String | Output only. Identifies the access mechanism to the entity. Not user settable. |
| `name` | String | Output only. The resource name of the entity, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{id}. |
| `data_path` | String | Required. Immutable. The storage path of the entity data. For Cloud Storage data, this is the fully-qualified path to the entity, such as gs://bucket/path/to/data. For BigQuery data, this is the name of the table resource, such as projects/project_id/datasets/dataset_id/tables/table_id. |
| `type` | String | Required. Immutable. The type of entity. |
| `id` | String | Required. A user-provided entity ID. It is mutable, and will be used as the published table name. Specifying a new ID in an update entity request will override the existing value. The ID must contain only letters (a-z, A-Z), numbers (0-9), and underscores, and consist of 256 or fewer characters. |
| `description` | String | Optional. User friendly longer description text. Must be shorter than or equal to 1024 characters. |
| `etag` | String | Optional. The etag associated with the entity, which can be retrieved with a GetEntity request. Required for update and delete requests. |
| `catalog_entry` | String | Output only. The name of the associated Data Catalog entry. |
| `display_name` | String | Optional. Display name must be shorter than or equal to 256 characters. |
| `format` | String | Required. Identifies the storage format of the entity data. It does not apply to entities with data stored in BigQuery. |
| `schema` | String | Required. The description of the data structure and layout. The schema is not included in list responses. It is only included in SCHEMA and FULL entity views of a GetEntity response. |
| `system` | String | Required. Immutable. Identifies the storage system of the entity data. |
| `compatibility` | String | Output only. Metadata stores that the entity is compatible with. |
| `update_time` | String | Output only. The time when the entity was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitie
entitie = provider.dataplex_api.Entitie {
    parent = "value"  # Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
}

# Access entitie outputs
entitie_id = entitie.id
entitie_uid = entitie.uid
entitie_data_path_pattern = entitie.data_path_pattern
entitie_asset = entitie.asset
entitie_create_time = entitie.create_time
entitie_access = entitie.access
entitie_name = entitie.name
entitie_data_path = entitie.data_path
entitie_type = entitie.type
entitie_id = entitie.id
entitie_description = entitie.description
entitie_etag = entitie.etag
entitie_catalog_entry = entitie.catalog_entry
entitie_display_name = entitie.display_name
entitie_format = entitie.format
entitie_schema = entitie.schema
entitie_system = entitie.system
entitie_compatibility = entitie.compatibility
entitie_update_time = entitie.update_time
```

---


### Lake

Creates a lake resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. System generated globally unique ID for the lake. This ID will be different if the lake is deleted and re-created with the same name. |
| `display_name` | String |  | Optional. User friendly display name. |
| `state` | String |  | Output only. Current state of the lake. |
| `update_time` | String |  | Output only. The time when the lake was last updated. |
| `asset_status` | String |  | Output only. Aggregated status of the underlying assets of the lake. |
| `metastore` | String |  | Optional. Settings to manage lake and Dataproc Metastore service instance association. |
| `metastore_status` | String |  | Output only. Metastore status of the lake. |
| `description` | String |  | Optional. Description of the lake. |
| `name` | String |  | Output only. The relative resource name of the lake, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the lake. |
| `create_time` | String |  | Output only. The time when the lake was created. |
| `service_account` | String |  | Output only. Service account associated with this lake. This service account must be authorized to access or operate on resources managed by the lake. |
| `parent` | String | ✅ | Required. The resource name of the lake location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. System generated globally unique ID for the lake. This ID will be different if the lake is deleted and re-created with the same name. |
| `display_name` | String | Optional. User friendly display name. |
| `state` | String | Output only. Current state of the lake. |
| `update_time` | String | Output only. The time when the lake was last updated. |
| `asset_status` | String | Output only. Aggregated status of the underlying assets of the lake. |
| `metastore` | String | Optional. Settings to manage lake and Dataproc Metastore service instance association. |
| `metastore_status` | String | Output only. Metastore status of the lake. |
| `description` | String | Optional. Description of the lake. |
| `name` | String | Output only. The relative resource name of the lake, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the lake. |
| `create_time` | String | Output only. The time when the lake was created. |
| `service_account` | String | Output only. Service account associated with this lake. This service account must be authorized to access or operate on resources managed by the lake. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lake
lake = provider.dataplex_api.Lake {
    parent = "value"  # Required. The resource name of the lake location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region.
}

# Access lake outputs
lake_id = lake.id
lake_uid = lake.uid
lake_display_name = lake.display_name
lake_state = lake.state
lake_update_time = lake.update_time
lake_asset_status = lake.asset_status
lake_metastore = lake.metastore
lake_metastore_status = lake.metastore_status
lake_description = lake.description
lake_name = lake.name
lake_labels = lake.labels
lake_create_time = lake.create_time
lake_service_account = lake.service_account
```

---


### Task

Creates a task resource within a lake.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `execution_spec` | String |  | Required. Spec related to how a task is executed. |
| `display_name` | String |  | Optional. User friendly display name. |
| `trigger_spec` | String |  | Required. Spec related to how often and when a task should be triggered. |
| `notebook` | String |  | Config related to running scheduled Notebooks. |
| `name` | String |  | Output only. The relative resource name of the task, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/ tasks/{task_id}. |
| `create_time` | String |  | Output only. The time when the task was created. |
| `uid` | String |  | Output only. System generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name. |
| `update_time` | String |  | Output only. The time when the task was last updated. |
| `description` | String |  | Optional. Description of the task. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the task. |
| `state` | String |  | Output only. Current state of the task. |
| `spark` | String |  | Config related to running custom Spark tasks. |
| `execution_status` | String |  | Output only. Status of the latest task executions. |
| `parent` | String | ✅ | Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_spec` | String | Required. Spec related to how a task is executed. |
| `display_name` | String | Optional. User friendly display name. |
| `trigger_spec` | String | Required. Spec related to how often and when a task should be triggered. |
| `notebook` | String | Config related to running scheduled Notebooks. |
| `name` | String | Output only. The relative resource name of the task, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/ tasks/{task_id}. |
| `create_time` | String | Output only. The time when the task was created. |
| `uid` | String | Output only. System generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name. |
| `update_time` | String | Output only. The time when the task was last updated. |
| `description` | String | Optional. Description of the task. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the task. |
| `state` | String | Output only. Current state of the task. |
| `spark` | String | Config related to running custom Spark tasks. |
| `execution_status` | String | Output only. Status of the latest task executions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create task
task = provider.dataplex_api.Task {
    parent = "value"  # Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
}

# Access task outputs
task_id = task.id
task_execution_spec = task.execution_spec
task_display_name = task.display_name
task_trigger_spec = task.trigger_spec
task_notebook = task.notebook
task_name = task.name
task_create_time = task.create_time
task_uid = task.uid
task_update_time = task.update_time
task_description = task.description
task_labels = task.labels
task_state = task.state
task_spark = task.spark
task_execution_status = task.execution_status
```

---


### Data_taxonomie

Create a DataTaxonomy resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribute_count` | i64 |  | Output only. The number of attributes in the DataTaxonomy. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `class_count` | i64 |  | Output only. The number of classes in the DataTaxonomy. |
| `name` | String |  | Output only. The relative resource name of the DataTaxonomy, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}. |
| `uid` | String |  | Output only. System generated globally unique ID for the dataTaxonomy. This ID will be different if the DataTaxonomy is deleted and re-created with the same name. |
| `update_time` | String |  | Output only. The time when the DataTaxonomy was last updated. |
| `create_time` | String |  | Output only. The time when the DataTaxonomy was created. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the DataTaxonomy. |
| `display_name` | String |  | Optional. User friendly display name. |
| `description` | String |  | Optional. Description of the DataTaxonomy. |
| `parent` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribute_count` | i64 | Output only. The number of attributes in the DataTaxonomy. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `class_count` | i64 | Output only. The number of classes in the DataTaxonomy. |
| `name` | String | Output only. The relative resource name of the DataTaxonomy, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}. |
| `uid` | String | Output only. System generated globally unique ID for the dataTaxonomy. This ID will be different if the DataTaxonomy is deleted and re-created with the same name. |
| `update_time` | String | Output only. The time when the DataTaxonomy was last updated. |
| `create_time` | String | Output only. The time when the DataTaxonomy was created. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the DataTaxonomy. |
| `display_name` | String | Optional. User friendly display name. |
| `description` | String | Optional. Description of the DataTaxonomy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_taxonomie
data_taxonomie = provider.dataplex_api.Data_taxonomie {
    parent = "value"  # Required field
}

# Access data_taxonomie outputs
data_taxonomie_id = data_taxonomie.id
data_taxonomie_attribute_count = data_taxonomie.attribute_count
data_taxonomie_etag = data_taxonomie.etag
data_taxonomie_class_count = data_taxonomie.class_count
data_taxonomie_name = data_taxonomie.name
data_taxonomie_uid = data_taxonomie.uid
data_taxonomie_update_time = data_taxonomie.update_time
data_taxonomie_create_time = data_taxonomie.create_time
data_taxonomie_labels = data_taxonomie.labels
data_taxonomie_display_name = data_taxonomie.display_name
data_taxonomie_description = data_taxonomie.description
```

---


### Entry_group

Creates an EntryGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time when the EntryGroup was created. |
| `uid` | String |  | Output only. System generated globally unique ID for the EntryGroup. If you delete and recreate the EntryGroup with the same name, this ID will be different. |
| `update_time` | String |  | Output only. The time when the EntryGroup was last updated. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the EntryGroup. |
| `description` | String |  | Optional. Description of the EntryGroup. |
| `etag` | String |  | This checksum is computed by the service, and might be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String |  | Output only. The relative resource name of the EntryGroup, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}. |
| `display_name` | String |  | Optional. User friendly display name. |
| `transfer_status` | String |  | Output only. Denotes the transfer status of the Entry Group. It is unspecified for Entry Group created from Dataplex API. |
| `parent` | String | ✅ | Required. The resource name of the entryGroup, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time when the EntryGroup was created. |
| `uid` | String | Output only. System generated globally unique ID for the EntryGroup. If you delete and recreate the EntryGroup with the same name, this ID will be different. |
| `update_time` | String | Output only. The time when the EntryGroup was last updated. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the EntryGroup. |
| `description` | String | Optional. Description of the EntryGroup. |
| `etag` | String | This checksum is computed by the service, and might be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `name` | String | Output only. The relative resource name of the EntryGroup, in the format projects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}. |
| `display_name` | String | Optional. User friendly display name. |
| `transfer_status` | String | Output only. Denotes the transfer status of the Entry Group. It is unspecified for Entry Group created from Dataplex API. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_group
entry_group = provider.dataplex_api.Entry_group {
    parent = "value"  # Required. The resource name of the entryGroup, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region.
}

# Access entry_group outputs
entry_group_id = entry_group.id
entry_group_create_time = entry_group.create_time
entry_group_uid = entry_group.uid
entry_group_update_time = entry_group.update_time
entry_group_labels = entry_group.labels
entry_group_description = entry_group.description
entry_group_etag = entry_group.etag
entry_group_name = entry_group.name
entry_group_display_name = entry_group.display_name
entry_group_transfer_status = entry_group.transfer_status
```

---


### Contentitem

Create a content.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id} |
| `data_text` | String |  | Required. Content data in string format. |
| `notebook` | String |  | Notebook related configurations. |
| `update_time` | String |  | Output only. The time when the content was last updated. |
| `create_time` | String |  | Output only. Content creation time. |
| `description` | String |  | Optional. Description of the content. |
| `labels` | HashMap<String, String> |  | Optional. User defined labels for the content. |
| `sql_script` | String |  | Sql Script related configurations. |
| `path` | String |  | Required. The path for the Content file, represented as directory structure. Unique within a lake. Limited to alphanumerics, hyphens, underscores, dots and slashes. |
| `uid` | String |  | Output only. System generated globally unique ID for the content. This ID will be different if the content is deleted and re-created with the same name. |
| `parent` | String | ✅ | Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id} |
| `data_text` | String | Required. Content data in string format. |
| `notebook` | String | Notebook related configurations. |
| `update_time` | String | Output only. The time when the content was last updated. |
| `create_time` | String | Output only. Content creation time. |
| `description` | String | Optional. Description of the content. |
| `labels` | HashMap<String, String> | Optional. User defined labels for the content. |
| `sql_script` | String | Sql Script related configurations. |
| `path` | String | Required. The path for the Content file, represented as directory structure. Unique within a lake. Limited to alphanumerics, hyphens, underscores, dots and slashes. |
| `uid` | String | Output only. System generated globally unique ID for the content. This ID will be different if the content is deleted and re-created with the same name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create contentitem
contentitem = provider.dataplex_api.Contentitem {
    parent = "value"  # Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
}

# Access contentitem outputs
contentitem_id = contentitem.id
contentitem_name = contentitem.name
contentitem_data_text = contentitem.data_text
contentitem_notebook = contentitem.notebook
contentitem_update_time = contentitem.update_time
contentitem_create_time = contentitem.create_time
contentitem_description = contentitem.description
contentitem_labels = contentitem.labels
contentitem_sql_script = contentitem.sql_script
contentitem_path = contentitem.path
contentitem_uid = contentitem.uid
```

---


### Data_scan

Creates a DataScan resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_documentation_result` | String |  | Output only. The result of a data documentation scan. |
| `execution_spec` | String |  | Optional. DataScan execution settings.If not specified, the fields in it will use their default values. |
| `data_profile_spec` | String |  | Settings for a data profile scan. |
| `data` | String |  | Required. The data source for DataScan. |
| `state` | String |  | Output only. Current state of the DataScan. |
| `data_profile_result` | String |  | Output only. The result of a data profile scan. |
| `uid` | String |  | Output only. System generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name. |
| `display_name` | String |  | Optional. User friendly display name. Must be between 1-256 characters. |
| `description` | String |  | Optional. Description of the scan. Must be between 1-1024 characters. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the scan. |
| `data_discovery_result` | String |  | Output only. The result of a data discovery scan. |
| `create_time` | String |  | Output only. The time when the scan was created. |
| `data_documentation_spec` | String |  | Settings for a data documentation scan. |
| `name` | String |  | Output only. Identifier. The relative resource name of the scan, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}, where project refers to a project_id or project_number and location_id refers to a Google Cloud region. |
| `data_quality_result` | String |  | Output only. The result of a data quality scan. |
| `data_discovery_spec` | String |  | Settings for a data discovery scan. |
| `execution_status` | String |  | Output only. Status of the data scan execution. |
| `type` | String |  | Output only. The type of DataScan. |
| `update_time` | String |  | Output only. The time when the scan was last updated. |
| `data_quality_spec` | String |  | Settings for a data quality scan. |
| `parent` | String | ✅ | Required. The resource name of the parent location: projects/{project}/locations/{location_id} where project refers to a project_id or project_number and location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_documentation_result` | String | Output only. The result of a data documentation scan. |
| `execution_spec` | String | Optional. DataScan execution settings.If not specified, the fields in it will use their default values. |
| `data_profile_spec` | String | Settings for a data profile scan. |
| `data` | String | Required. The data source for DataScan. |
| `state` | String | Output only. Current state of the DataScan. |
| `data_profile_result` | String | Output only. The result of a data profile scan. |
| `uid` | String | Output only. System generated globally unique ID for the scan. This ID will be different if the scan is deleted and re-created with the same name. |
| `display_name` | String | Optional. User friendly display name. Must be between 1-256 characters. |
| `description` | String | Optional. Description of the scan. Must be between 1-1024 characters. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the scan. |
| `data_discovery_result` | String | Output only. The result of a data discovery scan. |
| `create_time` | String | Output only. The time when the scan was created. |
| `data_documentation_spec` | String | Settings for a data documentation scan. |
| `name` | String | Output only. Identifier. The relative resource name of the scan, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}, where project refers to a project_id or project_number and location_id refers to a Google Cloud region. |
| `data_quality_result` | String | Output only. The result of a data quality scan. |
| `data_discovery_spec` | String | Settings for a data discovery scan. |
| `execution_status` | String | Output only. Status of the data scan execution. |
| `type` | String | Output only. The type of DataScan. |
| `update_time` | String | Output only. The time when the scan was last updated. |
| `data_quality_spec` | String | Settings for a data quality scan. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_scan
data_scan = provider.dataplex_api.Data_scan {
    parent = "value"  # Required. The resource name of the parent location: projects/{project}/locations/{location_id} where project refers to a project_id or project_number and location_id refers to a Google Cloud region.
}

# Access data_scan outputs
data_scan_id = data_scan.id
data_scan_data_documentation_result = data_scan.data_documentation_result
data_scan_execution_spec = data_scan.execution_spec
data_scan_data_profile_spec = data_scan.data_profile_spec
data_scan_data = data_scan.data
data_scan_state = data_scan.state
data_scan_data_profile_result = data_scan.data_profile_result
data_scan_uid = data_scan.uid
data_scan_display_name = data_scan.display_name
data_scan_description = data_scan.description
data_scan_labels = data_scan.labels
data_scan_data_discovery_result = data_scan.data_discovery_result
data_scan_create_time = data_scan.create_time
data_scan_data_documentation_spec = data_scan.data_documentation_spec
data_scan_name = data_scan.name
data_scan_data_quality_result = data_scan.data_quality_result
data_scan_data_discovery_spec = data_scan.data_discovery_spec
data_scan_execution_status = data_scan.execution_status
data_scan_type = data_scan.type
data_scan_update_time = data_scan.update_time
data_scan_data_quality_spec = data_scan.data_quality_spec
```

---


### Attribute

Create a DataAttribute resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the DataAttribute. |
| `display_name` | String |  | Optional. User friendly display name. |
| `description` | String |  | Optional. Description of the DataAttribute. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String |  | Output only. The time when the DataAttribute was created. |
| `name` | String |  | Output only. The relative resource name of the dataAttribute, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}. |
| `attribute_count` | i64 |  | Output only. The number of child attributes present for this attribute. |
| `parent_id` | String |  | Optional. The ID of the parent DataAttribute resource, should belong to the same data taxonomy. Circular dependency in parent chain is not valid. Maximum depth of the hierarchy allowed is 4. a -> b -> c -> d -> e, depth = 4 |
| `resource_access_spec` | String |  | Optional. Specified when applied to a resource (eg: Cloud Storage bucket, BigQuery dataset, BigQuery table). |
| `uid` | String |  | Output only. System generated globally unique ID for the DataAttribute. This ID will be different if the DataAttribute is deleted and re-created with the same name. |
| `data_access_spec` | String |  | Optional. Specified when applied to data stored on the resource (eg: rows, columns in BigQuery Tables). |
| `update_time` | String |  | Output only. The time when the DataAttribute was last updated. |
| `parent` | String | ✅ | Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. User-defined labels for the DataAttribute. |
| `display_name` | String | Optional. User friendly display name. |
| `description` | String | Optional. Description of the DataAttribute. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String | Output only. The time when the DataAttribute was created. |
| `name` | String | Output only. The relative resource name of the dataAttribute, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}. |
| `attribute_count` | i64 | Output only. The number of child attributes present for this attribute. |
| `parent_id` | String | Optional. The ID of the parent DataAttribute resource, should belong to the same data taxonomy. Circular dependency in parent chain is not valid. Maximum depth of the hierarchy allowed is 4. a -> b -> c -> d -> e, depth = 4 |
| `resource_access_spec` | String | Optional. Specified when applied to a resource (eg: Cloud Storage bucket, BigQuery dataset, BigQuery table). |
| `uid` | String | Output only. System generated globally unique ID for the DataAttribute. This ID will be different if the DataAttribute is deleted and re-created with the same name. |
| `data_access_spec` | String | Optional. Specified when applied to data stored on the resource (eg: rows, columns in BigQuery Tables). |
| `update_time` | String | Output only. The time when the DataAttribute was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attribute
attribute = provider.dataplex_api.Attribute {
    parent = "value"  # Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}
}

# Access attribute outputs
attribute_id = attribute.id
attribute_labels = attribute.labels
attribute_display_name = attribute.display_name
attribute_description = attribute.description
attribute_etag = attribute.etag
attribute_create_time = attribute.create_time
attribute_name = attribute.name
attribute_attribute_count = attribute.attribute_count
attribute_parent_id = attribute.parent_id
attribute_resource_access_spec = attribute.resource_access_spec
attribute_uid = attribute.uid
attribute_data_access_spec = attribute.data_access_spec
attribute_update_time = attribute.update_time
```

---


### Location

Searches for Entries matching the given query and scope.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The project to which the request should be attributed in the following form: projects/{project}/locations/global. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
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
location = provider.dataplex_api.Location {
    name = "value"  # Required. The project to which the request should be attributed in the following form: projects/{project}/locations/global.
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
```

---


### Entry_type

Creates an EntryType.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. This checksum is computed by the service, and might be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `update_time` | String |  | Output only. The time when the EntryType was last updated. |
| `platform` | String |  | Optional. The platform that Entries of this type belongs to. |
| `system` | String |  | Optional. The system that Entries of this type belongs to. Examples include CloudSQL, MariaDB etc |
| `uid` | String |  | Output only. System generated globally unique ID for the EntryType. This ID will be different if the EntryType is deleted and re-created with the same name. |
| `required_aspects` | Vec<String> |  | AspectInfo for the entry type. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the EntryType. |
| `type_aliases` | Vec<String> |  | Optional. Indicates the classes this Entry Type belongs to, for example, TABLE, DATABASE, MODEL. |
| `name` | String |  | Output only. The relative resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}. |
| `description` | String |  | Optional. Description of the EntryType. |
| `authorization` | String |  | Immutable. Authorization defined for this type. |
| `create_time` | String |  | Output only. The time when the EntryType was created. |
| `display_name` | String |  | Optional. User friendly display name. |
| `parent` | String | ✅ | Required. The resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. This checksum is computed by the service, and might be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `update_time` | String | Output only. The time when the EntryType was last updated. |
| `platform` | String | Optional. The platform that Entries of this type belongs to. |
| `system` | String | Optional. The system that Entries of this type belongs to. Examples include CloudSQL, MariaDB etc |
| `uid` | String | Output only. System generated globally unique ID for the EntryType. This ID will be different if the EntryType is deleted and re-created with the same name. |
| `required_aspects` | Vec<String> | AspectInfo for the entry type. |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the EntryType. |
| `type_aliases` | Vec<String> | Optional. Indicates the classes this Entry Type belongs to, for example, TABLE, DATABASE, MODEL. |
| `name` | String | Output only. The relative resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}. |
| `description` | String | Optional. Description of the EntryType. |
| `authorization` | String | Immutable. Authorization defined for this type. |
| `create_time` | String | Output only. The time when the EntryType was created. |
| `display_name` | String | Optional. User friendly display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_type
entry_type = provider.dataplex_api.Entry_type {
    parent = "value"  # Required. The resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a Google Cloud region.
}

# Access entry_type outputs
entry_type_id = entry_type.id
entry_type_etag = entry_type.etag
entry_type_update_time = entry_type.update_time
entry_type_platform = entry_type.platform
entry_type_system = entry_type.system
entry_type_uid = entry_type.uid
entry_type_required_aspects = entry_type.required_aspects
entry_type_labels = entry_type.labels
entry_type_type_aliases = entry_type.type_aliases
entry_type_name = entry_type.name
entry_type_description = entry_type.description
entry_type_authorization = entry_type.authorization
entry_type_create_time = entry_type.create_time
entry_type_display_name = entry_type.display_name
```

---


### Metadata_job

Creates a metadata job. For example, use a metadata job to import metadata from a third-party system into Dataplex Universal Catalog.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `export_spec` | String |  | Export job specification. |
| `export_result` | String |  | Output only. Export job result. |
| `name` | String |  | Output only. Identifier. The name of the resource that the configuration is applied to, in the format projects/{project_number}/locations/{location_id}/metadataJobs/{metadata_job_id}. |
| `create_time` | String |  | Output only. The time when the metadata job was created. |
| `import_result` | String |  | Output only. Import job result. |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels. |
| `type` | String |  | Required. Metadata job type. |
| `uid` | String |  | Output only. A system-generated, globally unique ID for the metadata job. If the metadata job is deleted and then re-created with the same name, this ID is different. |
| `update_time` | String |  | Output only. The time when the metadata job was updated. |
| `import_spec` | String |  | Import job specification. |
| `status` | String |  | Output only. Metadata job status. |
| `parent` | String | ✅ | Required. The resource name of the parent location, in the format projects/{project_id_or_number}/locations/{location_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_spec` | String | Export job specification. |
| `export_result` | String | Output only. Export job result. |
| `name` | String | Output only. Identifier. The name of the resource that the configuration is applied to, in the format projects/{project_number}/locations/{location_id}/metadataJobs/{metadata_job_id}. |
| `create_time` | String | Output only. The time when the metadata job was created. |
| `import_result` | String | Output only. Import job result. |
| `labels` | HashMap<String, String> | Optional. User-defined labels. |
| `type` | String | Required. Metadata job type. |
| `uid` | String | Output only. A system-generated, globally unique ID for the metadata job. If the metadata job is deleted and then re-created with the same name, this ID is different. |
| `update_time` | String | Output only. The time when the metadata job was updated. |
| `import_spec` | String | Import job specification. |
| `status` | String | Output only. Metadata job status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_job
metadata_job = provider.dataplex_api.Metadata_job {
    parent = "value"  # Required. The resource name of the parent location, in the format projects/{project_id_or_number}/locations/{location_id}
}

# Access metadata_job outputs
metadata_job_id = metadata_job.id
metadata_job_export_spec = metadata_job.export_spec
metadata_job_export_result = metadata_job.export_result
metadata_job_name = metadata_job.name
metadata_job_create_time = metadata_job.create_time
metadata_job_import_result = metadata_job.import_result
metadata_job_labels = metadata_job.labels
metadata_job_type = metadata_job.type
metadata_job_uid = metadata_job.uid
metadata_job_update_time = metadata_job.update_time
metadata_job_import_spec = metadata_job.import_spec
metadata_job_status = metadata_job.status
```

---


### Categorie

Creates a new GlossaryCategory resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which the GlossaryCategory was created. |
| `update_time` | String |  | Output only. The time at which the GlossaryCategory was last updated. |
| `parent` | String |  | Required. The immediate parent of the GlossaryCategory in the resource-hierarchy. It can either be a Glossary or a GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} OR projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `labels` | HashMap<String, String> |  | Optional. User-defined labels for the GlossaryCategory. |
| `name` | String |  | Output only. Identifier. The resource name of the GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `uid` | String |  | Output only. System generated unique id for the GlossaryCategory. This ID will be different if the GlossaryCategory is deleted and re-created with the same name. |
| `display_name` | String |  | Optional. User friendly display name of the GlossaryCategory. This is user-mutable. This will be same as the GlossaryCategoryId, if not specified. |
| `description` | String |  | Optional. The user-mutable description of the GlossaryCategory. |
| `parent` | String | ✅ | Required. The parent resource where this GlossaryCategory will be created. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} where locationId refers to a Google Cloud region. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the GlossaryCategory was created. |
| `update_time` | String | Output only. The time at which the GlossaryCategory was last updated. |
| `parent` | String | Required. The immediate parent of the GlossaryCategory in the resource-hierarchy. It can either be a Glossary or a GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} OR projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `labels` | HashMap<String, String> | Optional. User-defined labels for the GlossaryCategory. |
| `name` | String | Output only. Identifier. The resource name of the GlossaryCategory. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id}/categories/{category_id} |
| `uid` | String | Output only. System generated unique id for the GlossaryCategory. This ID will be different if the GlossaryCategory is deleted and re-created with the same name. |
| `display_name` | String | Optional. User friendly display name of the GlossaryCategory. This is user-mutable. This will be same as the GlossaryCategoryId, if not specified. |
| `description` | String | Optional. The user-mutable description of the GlossaryCategory. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create categorie
categorie = provider.dataplex_api.Categorie {
    parent = "value"  # Required. The parent resource where this GlossaryCategory will be created. Format: projects/{project_id_or_number}/locations/{location_id}/glossaries/{glossary_id} where locationId refers to a Google Cloud region.
}

# Access categorie outputs
categorie_id = categorie.id
categorie_create_time = categorie.create_time
categorie_update_time = categorie.update_time
categorie_parent = categorie.parent
categorie_labels = categorie.labels
categorie_name = categorie.name
categorie_uid = categorie.uid
categorie_display_name = categorie.display_name
categorie_description = categorie.description
```

---


### Data_product

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag" |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_product
data_product = provider.dataplex_api.Data_product {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Asset

Creates an asset resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `discovery_status` | String |  | Output only. Status of the discovery feature applied to data referenced by this asset. |
| `resource_spec` | String |  | Required. Specification of the resource that is referenced by this asset. |
| `resource_status` | String |  | Output only. Status of the resource referenced by this asset. |
| `discovery_spec` | String |  | Optional. Specification of the discovery feature applied to data referenced by this asset. When this spec is left unset, the asset will use the spec set on the parent zone. |
| `create_time` | String |  | Output only. The time when the asset was created. |
| `security_status` | String |  | Output only. Status of the security policy applied to resource referenced by this asset. |
| `state` | String |  | Output only. Current state of the asset. |
| `labels` | HashMap<String, String> |  | Optional. User defined labels for the asset. |
| `update_time` | String |  | Output only. The time when the asset was last updated. |
| `description` | String |  | Optional. Description of the asset. |
| `display_name` | String |  | Optional. User friendly display name. |
| `uid` | String |  | Output only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name. |
| `name` | String |  | Output only. The relative resource name of the asset, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}. |
| `parent` | String | ✅ | Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `discovery_status` | String | Output only. Status of the discovery feature applied to data referenced by this asset. |
| `resource_spec` | String | Required. Specification of the resource that is referenced by this asset. |
| `resource_status` | String | Output only. Status of the resource referenced by this asset. |
| `discovery_spec` | String | Optional. Specification of the discovery feature applied to data referenced by this asset. When this spec is left unset, the asset will use the spec set on the parent zone. |
| `create_time` | String | Output only. The time when the asset was created. |
| `security_status` | String | Output only. Status of the security policy applied to resource referenced by this asset. |
| `state` | String | Output only. Current state of the asset. |
| `labels` | HashMap<String, String> | Optional. User defined labels for the asset. |
| `update_time` | String | Output only. The time when the asset was last updated. |
| `description` | String | Optional. Description of the asset. |
| `display_name` | String | Optional. User friendly display name. |
| `uid` | String | Output only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name. |
| `name` | String | Output only. The relative resource name of the asset, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create asset
asset = provider.dataplex_api.Asset {
    parent = "value"  # Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
}

# Access asset outputs
asset_id = asset.id
asset_discovery_status = asset.discovery_status
asset_resource_spec = asset.resource_spec
asset_resource_status = asset.resource_status
asset_discovery_spec = asset.discovery_spec
asset_create_time = asset.create_time
asset_security_status = asset.security_status
asset_state = asset.state
asset_labels = asset.labels
asset_update_time = asset.update_time
asset_description = asset.description
asset_display_name = asset.display_name
asset_uid = asset.uid
asset_name = asset.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple entrie resources
entrie_0 = provider.dataplex_api.Entrie {
    parent = "value-0"
}
entrie_1 = provider.dataplex_api.Entrie {
    parent = "value-1"
}
entrie_2 = provider.dataplex_api.Entrie {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entrie = provider.dataplex_api.Entrie {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Dataplex_api Documentation](https://cloud.google.com/dataplex_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
