# Bigquery_api Service



**Resources**: 8

---

## Overview

The bigquery_api service provides access to 8 resource types:

- [Routine](#routine) [CRUD]
- [Project](#project) [R]
- [Tabledata](#tabledata) [CR]
- [Dataset](#dataset) [CRUD]
- [Table](#table) [CRUD]
- [Job](#job) [CRD]
- [Model](#model) [RUD]
- [Row_access_policie](#row_access_policie) [CRUD]

---

## Resources


### Routine

Creates a new routine in the dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_mode` | String |  | Optional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. |
| `spark_options` | String |  | Optional. Spark specific options. |
| `remote_function_options` | String |  | Optional. Remote function specific options. |
| `routine_reference` | String |  | Required. Reference describing the ID of this routine. |
| `arguments` | Vec<String> |  | Optional. |
| `return_type` | String |  | Optional if language = "SQL"; required otherwise. Cannot be set if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: "FLOAT64"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64. |
| `routine_type` | String |  | Required. The type of routine. |
| `strict_mode` | bool |  | Optional. Use this option to catch many common errors. Error checking is not exhaustive, and successfully creating a procedure doesn't guarantee that the procedure will successfully execute at runtime. If `strictMode` is set to `TRUE`, the procedure body is further checked for errors such as non-existent tables or columns. The `CREATE PROCEDURE` statement fails if the body fails any of these checks. If `strictMode` is set to `FALSE`, the procedure body is checked only for syntax. For procedures that invoke themselves recursively, specify `strictMode=FALSE` to avoid non-existent procedure errors during validation. Default value is `TRUE`. |
| `determinism_level` | String |  | Optional. The determinism level of the JavaScript UDF, if defined. |
| `etag` | String |  | Output only. A hash of this resource. |
| `definition_body` | String |  | Required. The body of the routine. For functions, this is the expression in the AS clause. If `language = "SQL"`, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, "\n", y))` The definition_body is `concat(x, "\n", y)` (\n is not replaced with linebreak). If `language="JAVASCRIPT"`, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return "\n";\n'` The definition_body is `return "\n";\n` Note that both \n are replaced with linebreaks. If `definition_body` references another routine, then that routine must be fully qualified with its project ID. |
| `return_table_type` | String |  | Optional. Can be set only if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return table type is inferred from definition_body at query time in each query that references this routine. If present, then the columns in the evaluated table result will be cast to match the column types specified in return table type, at query time. |
| `python_options` | String |  | Optional. Options for the Python UDF. [Preview](https://cloud.google.com/products/#product-launch-stages) |
| `last_modified_time` | String |  | Output only. The time when this routine was last modified, in milliseconds since the epoch. |
| `description` | String |  | Optional. The description of the routine, if defined. |
| `data_governance_type` | String |  | Optional. If set to `DATA_MASKING`, the function is validated and made available as a masking function. For more information, see [Create custom masking routines](https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask). |
| `creation_time` | String |  | Output only. The time when this routine was created, in milliseconds since the epoch. |
| `external_runtime_options` | String |  | Optional. Options for the runtime of the external system executing the routine. This field is only applicable for Python UDFs. [Preview](https://cloud.google.com/products/#product-launch-stages) |
| `language` | String |  | Optional. Defaults to "SQL" if remote_function_options field is absent, not set otherwise. |
| `imported_libraries` | Vec<String> |  | Optional. If language = "JAVASCRIPT", this field stores the path of the imported JAVASCRIPT libraries. |
| `dataset_id` | String | ✅ | Required. Dataset ID of the new routine |
| `project_id` | String | ✅ | Required. Project ID of the new routine |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_mode` | String | Optional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. |
| `spark_options` | String | Optional. Spark specific options. |
| `remote_function_options` | String | Optional. Remote function specific options. |
| `routine_reference` | String | Required. Reference describing the ID of this routine. |
| `arguments` | Vec<String> | Optional. |
| `return_type` | String | Optional if language = "SQL"; required otherwise. Cannot be set if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: "FLOAT64"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64. |
| `routine_type` | String | Required. The type of routine. |
| `strict_mode` | bool | Optional. Use this option to catch many common errors. Error checking is not exhaustive, and successfully creating a procedure doesn't guarantee that the procedure will successfully execute at runtime. If `strictMode` is set to `TRUE`, the procedure body is further checked for errors such as non-existent tables or columns. The `CREATE PROCEDURE` statement fails if the body fails any of these checks. If `strictMode` is set to `FALSE`, the procedure body is checked only for syntax. For procedures that invoke themselves recursively, specify `strictMode=FALSE` to avoid non-existent procedure errors during validation. Default value is `TRUE`. |
| `determinism_level` | String | Optional. The determinism level of the JavaScript UDF, if defined. |
| `etag` | String | Output only. A hash of this resource. |
| `definition_body` | String | Required. The body of the routine. For functions, this is the expression in the AS clause. If `language = "SQL"`, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, "\n", y))` The definition_body is `concat(x, "\n", y)` (\n is not replaced with linebreak). If `language="JAVASCRIPT"`, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return "\n";\n'` The definition_body is `return "\n";\n` Note that both \n are replaced with linebreaks. If `definition_body` references another routine, then that routine must be fully qualified with its project ID. |
| `return_table_type` | String | Optional. Can be set only if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return table type is inferred from definition_body at query time in each query that references this routine. If present, then the columns in the evaluated table result will be cast to match the column types specified in return table type, at query time. |
| `python_options` | String | Optional. Options for the Python UDF. [Preview](https://cloud.google.com/products/#product-launch-stages) |
| `last_modified_time` | String | Output only. The time when this routine was last modified, in milliseconds since the epoch. |
| `description` | String | Optional. The description of the routine, if defined. |
| `data_governance_type` | String | Optional. If set to `DATA_MASKING`, the function is validated and made available as a masking function. For more information, see [Create custom masking routines](https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask). |
| `creation_time` | String | Output only. The time when this routine was created, in milliseconds since the epoch. |
| `external_runtime_options` | String | Optional. Options for the runtime of the external system executing the routine. This field is only applicable for Python UDFs. [Preview](https://cloud.google.com/products/#product-launch-stages) |
| `language` | String | Optional. Defaults to "SQL" if remote_function_options field is absent, not set otherwise. |
| `imported_libraries` | Vec<String> | Optional. If language = "JAVASCRIPT", this field stores the path of the imported JAVASCRIPT libraries. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create routine
routine = provider.bigquery_api.Routine {
    dataset_id = "value"  # Required. Dataset ID of the new routine
    project_id = "value"  # Required. Project ID of the new routine
}

# Access routine outputs
routine_id = routine.id
routine_security_mode = routine.security_mode
routine_spark_options = routine.spark_options
routine_remote_function_options = routine.remote_function_options
routine_routine_reference = routine.routine_reference
routine_arguments = routine.arguments
routine_return_type = routine.return_type
routine_routine_type = routine.routine_type
routine_strict_mode = routine.strict_mode
routine_determinism_level = routine.determinism_level
routine_etag = routine.etag
routine_definition_body = routine.definition_body
routine_return_table_type = routine.return_table_type
routine_python_options = routine.python_options
routine_last_modified_time = routine.last_modified_time
routine_description = routine.description
routine_data_governance_type = routine.data_governance_type
routine_creation_time = routine.creation_time
routine_external_runtime_options = routine.external_runtime_options
routine_language = routine.language
routine_imported_libraries = routine.imported_libraries
```

---


### Project

RPC to list projects to which the user has been granted any project role. Users of this method are encouraged to consider the [Resource Manager](https://cloud.google.com/resource-manager/docs/) API, which provides the underlying data for this method and has more capabilities.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The resource type of the response. |
| `total_items` | i64 | The total number of projects in the page. A wrapper is used here because the field should still be in the response when the value is 0. |
| `projects` | Vec<String> | Projects to which the user has at least READ access. |
| `etag` | String | A hash of the page of results. |
| `next_page_token` | String | Use this token to request the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_kind = project.kind
project_total_items = project.total_items
project_projects = project.projects
project_etag = project.etag
project_next_page_token = project.next_page_token
```

---


### Tabledata

Streams data into BigQuery one record at a time without needing to run a load job.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_suffix` | String |  | Optional. If specified, treats the destination table as a base template, and inserts the rows into an instance table named "{destination}{templateSuffix}". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables. |
| `ignore_unknown_values` | bool |  | Optional. Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors. |
| `kind` | String |  | Optional. The resource type of the response. The value is not checked at the backend. Historically, it has been set to "bigquery#tableDataInsertAllRequest" but you are not required to set it. |
| `skip_invalid_rows` | bool |  | Optional. Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist. |
| `trace_id` | String |  | Optional. Unique request trace id. Used for debugging purposes only. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. |
| `rows` | Vec<String> |  |  |
| `project_id` | String | ✅ | Required. Project ID of the destination. |
| `table_id` | String | ✅ | Required. Table ID of the destination. |
| `dataset_id` | String | ✅ | Required. Dataset ID of the destination. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | A hash of this page of results. |
| `page_token` | String | A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing. |
| `kind` | String | The resource type of the response. |
| `rows` | Vec<String> | Rows of results. |
| `total_rows` | String | Total rows of the entire table. In order to show default value 0 we have to present it as string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tabledata
tabledata = provider.bigquery_api.Tabledata {
    project_id = "value"  # Required. Project ID of the destination.
    table_id = "value"  # Required. Table ID of the destination.
    dataset_id = "value"  # Required. Dataset ID of the destination.
}

# Access tabledata outputs
tabledata_id = tabledata.id
tabledata_etag = tabledata.etag
tabledata_page_token = tabledata.page_token
tabledata_kind = tabledata.kind
tabledata_rows = tabledata.rows
tabledata_total_rows = tabledata.total_rows
```

---


### Dataset

Creates a new empty dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `linked_dataset_source` | String |  | Optional. The source dataset reference when the dataset is of type LINKED. For all other dataset types it is not set. This field cannot be updated once it is set. Any attempt to update this field using Update and Patch API Operations will be ignored. |
| `labels` | HashMap<String, String> |  | The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See [Creating and Updating Dataset Labels](https://cloud.google.com/bigquery/docs/creating-managing-labels#creating_and_updating_dataset_labels) for more information. |
| `external_catalog_dataset_options` | String |  | Optional. Options defining open source compatible datasets living in the BigQuery catalog. Contains metadata of open source database, schema or namespace represented by the current dataset. |
| `restrictions` | String |  | Optional. Output only. Restriction config for all tables and dataset. If set, restrict certain accesses on the dataset and all its tables based on the config. See [Data egress](https://cloud.google.com/bigquery/docs/analytics-hub-introduction#data_egress) for more details. |
| `id` | String |  | Output only. The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field. |
| `default_partition_expiration_ms` | String |  | This default partition expiration, expressed in milliseconds. When new time-partitioned tables are created in a dataset where this property is set, the table will inherit this value, propagated as the `TimePartitioning.expirationMs` property on the new table. If you set `TimePartitioning.expirationMs` explicitly when creating a table, the `defaultPartitionExpirationMs` of the containing dataset is ignored. When creating a partitioned table, if `defaultPartitionExpirationMs` is set, the `defaultTableExpirationMs` value is ignored and the table will not be inherit a table expiration deadline. |
| `friendly_name` | String |  | Optional. A descriptive name for the dataset. |
| `max_time_travel_hours` | String |  | Optional. Defines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days). The default value is 168 hours if this is not set. |
| `last_modified_time` | String |  | Output only. The date when this dataset was last modified, in milliseconds since the epoch. |
| `dataset_reference` | String |  | Required. A reference that identifies the dataset. |
| `access` | Vec<String> |  | Optional. An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER; If you patch a dataset, then this field is overwritten by the patched dataset's access field. To add entities, you must supply the entire existing access array in addition to any new entities that you want to add. |
| `description` | String |  | Optional. A user-friendly description of the dataset. |
| `default_collation` | String |  | Optional. Defines the default collation specification of future tables created in the dataset. If a table is created in this dataset without table-level default collation, then the table inherits the dataset default collation, which is applied to the string fields that do not have explicit collation specified. A change to this field affects only tables created afterwards, and does not alter the existing tables. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior. |
| `linked_dataset_metadata` | String |  | Output only. Metadata about the LinkedDataset. Filled out when the dataset type is LINKED. |
| `kind` | String |  | Output only. The resource type. |
| `etag` | String |  | Output only. A hash of the resource. |
| `storage_billing_model` | String |  | Optional. Updates storage_billing_model for the dataset. |
| `location` | String |  | The geographic location where the dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations. |
| `default_encryption_configuration` | String |  | The default encryption key for all tables in the dataset. After this property is set, the encryption key of all newly-created tables in the dataset is set to this value unless the table creation request or query explicitly overrides the key. |
| `default_table_expiration_ms` | String |  | Optional. The default lifetime of all tables in the dataset, in milliseconds. The minimum lifetime value is 3600000 milliseconds (one hour). To clear an existing default expiration with a PATCH request, set to 0. Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property. |
| `external_dataset_reference` | String |  | Optional. Reference to a read-only external dataset defined in data catalogs outside of BigQuery. Filled out when the dataset type is EXTERNAL. |
| `creation_time` | String |  | Output only. The time when this dataset was created, in milliseconds since the epoch. |
| `resource_tags` | HashMap<String, String> |  | Optional. The [tags](https://cloud.google.com/bigquery/docs/tags) attached to this dataset. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example "123456789012/environment" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example "Production". See [Tag definitions](https://cloud.google.com/iam/docs/tags-access-control#definitions) for more details. |
| `default_rounding_mode` | String |  | Optional. Defines the default rounding mode specification of new tables created within this dataset. During table creation, if this field is specified, the table within this dataset will inherit the default rounding mode of the dataset. Setting the default rounding mode on a table overrides this option. Existing tables in the dataset are unaffected. If columns are defined during that table creation, they will immediately inherit the table's default rounding mode, unless otherwise specified. |
| `is_case_insensitive` | bool |  | Optional. TRUE if the dataset and its table names are case-insensitive, otherwise FALSE. By default, this is FALSE, which means the dataset and its table names are case-sensitive. This field does not affect routine references. |
| `tags` | Vec<String> |  | Output only. Tags for the dataset. To provide tags as inputs, use the `resourceTags` field. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `type` | String |  | Output only. Same as `type` in `ListFormatDataset`. The type of the dataset, one of: * DEFAULT - only accessible by owner and authorized accounts, * PUBLIC - accessible by everyone, * LINKED - linked dataset, * EXTERNAL - dataset with definition in external metadata catalog. |
| `self_link` | String |  | Output only. A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource. |
| `project_id` | String | ✅ | Required. Project ID of the new dataset |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `linked_dataset_source` | String | Optional. The source dataset reference when the dataset is of type LINKED. For all other dataset types it is not set. This field cannot be updated once it is set. Any attempt to update this field using Update and Patch API Operations will be ignored. |
| `labels` | HashMap<String, String> | The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See [Creating and Updating Dataset Labels](https://cloud.google.com/bigquery/docs/creating-managing-labels#creating_and_updating_dataset_labels) for more information. |
| `external_catalog_dataset_options` | String | Optional. Options defining open source compatible datasets living in the BigQuery catalog. Contains metadata of open source database, schema or namespace represented by the current dataset. |
| `restrictions` | String | Optional. Output only. Restriction config for all tables and dataset. If set, restrict certain accesses on the dataset and all its tables based on the config. See [Data egress](https://cloud.google.com/bigquery/docs/analytics-hub-introduction#data_egress) for more details. |
| `id` | String | Output only. The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field. |
| `default_partition_expiration_ms` | String | This default partition expiration, expressed in milliseconds. When new time-partitioned tables are created in a dataset where this property is set, the table will inherit this value, propagated as the `TimePartitioning.expirationMs` property on the new table. If you set `TimePartitioning.expirationMs` explicitly when creating a table, the `defaultPartitionExpirationMs` of the containing dataset is ignored. When creating a partitioned table, if `defaultPartitionExpirationMs` is set, the `defaultTableExpirationMs` value is ignored and the table will not be inherit a table expiration deadline. |
| `friendly_name` | String | Optional. A descriptive name for the dataset. |
| `max_time_travel_hours` | String | Optional. Defines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days). The default value is 168 hours if this is not set. |
| `last_modified_time` | String | Output only. The date when this dataset was last modified, in milliseconds since the epoch. |
| `dataset_reference` | String | Required. A reference that identifies the dataset. |
| `access` | Vec<String> | Optional. An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER; If you patch a dataset, then this field is overwritten by the patched dataset's access field. To add entities, you must supply the entire existing access array in addition to any new entities that you want to add. |
| `description` | String | Optional. A user-friendly description of the dataset. |
| `default_collation` | String | Optional. Defines the default collation specification of future tables created in the dataset. If a table is created in this dataset without table-level default collation, then the table inherits the dataset default collation, which is applied to the string fields that do not have explicit collation specified. A change to this field affects only tables created afterwards, and does not alter the existing tables. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior. |
| `linked_dataset_metadata` | String | Output only. Metadata about the LinkedDataset. Filled out when the dataset type is LINKED. |
| `kind` | String | Output only. The resource type. |
| `etag` | String | Output only. A hash of the resource. |
| `storage_billing_model` | String | Optional. Updates storage_billing_model for the dataset. |
| `location` | String | The geographic location where the dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations. |
| `default_encryption_configuration` | String | The default encryption key for all tables in the dataset. After this property is set, the encryption key of all newly-created tables in the dataset is set to this value unless the table creation request or query explicitly overrides the key. |
| `default_table_expiration_ms` | String | Optional. The default lifetime of all tables in the dataset, in milliseconds. The minimum lifetime value is 3600000 milliseconds (one hour). To clear an existing default expiration with a PATCH request, set to 0. Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property. |
| `external_dataset_reference` | String | Optional. Reference to a read-only external dataset defined in data catalogs outside of BigQuery. Filled out when the dataset type is EXTERNAL. |
| `creation_time` | String | Output only. The time when this dataset was created, in milliseconds since the epoch. |
| `resource_tags` | HashMap<String, String> | Optional. The [tags](https://cloud.google.com/bigquery/docs/tags) attached to this dataset. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example "123456789012/environment" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example "Production". See [Tag definitions](https://cloud.google.com/iam/docs/tags-access-control#definitions) for more details. |
| `default_rounding_mode` | String | Optional. Defines the default rounding mode specification of new tables created within this dataset. During table creation, if this field is specified, the table within this dataset will inherit the default rounding mode of the dataset. Setting the default rounding mode on a table overrides this option. Existing tables in the dataset are unaffected. If columns are defined during that table creation, they will immediately inherit the table's default rounding mode, unless otherwise specified. |
| `is_case_insensitive` | bool | Optional. TRUE if the dataset and its table names are case-insensitive, otherwise FALSE. By default, this is FALSE, which means the dataset and its table names are case-sensitive. This field does not affect routine references. |
| `tags` | Vec<String> | Output only. Tags for the dataset. To provide tags as inputs, use the `resourceTags` field. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `type` | String | Output only. Same as `type` in `ListFormatDataset`. The type of the dataset, one of: * DEFAULT - only accessible by owner and authorized accounts, * PUBLIC - accessible by everyone, * LINKED - linked dataset, * EXTERNAL - dataset with definition in external metadata catalog. |
| `self_link` | String | Output only. A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.bigquery_api.Dataset {
    project_id = "value"  # Required. Project ID of the new dataset
}

# Access dataset outputs
dataset_id = dataset.id
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_linked_dataset_source = dataset.linked_dataset_source
dataset_labels = dataset.labels
dataset_external_catalog_dataset_options = dataset.external_catalog_dataset_options
dataset_restrictions = dataset.restrictions
dataset_id = dataset.id
dataset_default_partition_expiration_ms = dataset.default_partition_expiration_ms
dataset_friendly_name = dataset.friendly_name
dataset_max_time_travel_hours = dataset.max_time_travel_hours
dataset_last_modified_time = dataset.last_modified_time
dataset_dataset_reference = dataset.dataset_reference
dataset_access = dataset.access
dataset_description = dataset.description
dataset_default_collation = dataset.default_collation
dataset_linked_dataset_metadata = dataset.linked_dataset_metadata
dataset_kind = dataset.kind
dataset_etag = dataset.etag
dataset_storage_billing_model = dataset.storage_billing_model
dataset_location = dataset.location
dataset_default_encryption_configuration = dataset.default_encryption_configuration
dataset_default_table_expiration_ms = dataset.default_table_expiration_ms
dataset_external_dataset_reference = dataset.external_dataset_reference
dataset_creation_time = dataset.creation_time
dataset_resource_tags = dataset.resource_tags
dataset_default_rounding_mode = dataset.default_rounding_mode
dataset_is_case_insensitive = dataset.is_case_insensitive
dataset_tags = dataset.tags
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_type = dataset.type
dataset_self_link = dataset.self_link
```

---


### Table

Creates a new, empty table in the dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Output only. An opaque ID uniquely identifying the table. |
| `num_partitions` | String |  | Output only. The number of partitions present in the table or materialized view. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `view` | String |  | Optional. The view definition. |
| `labels` | HashMap<String, String> |  | The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key. |
| `num_active_logical_bytes` | String |  | Output only. Number of logical bytes that are less than 90 days old. |
| `clone_definition` | String |  | Output only. Contains information about the clone. This value is set via the clone operation. |
| `num_long_term_logical_bytes` | String |  | Output only. Number of logical bytes that are more than 90 days old. |
| `num_total_logical_bytes` | String |  | Output only. Total number of logical bytes in the table or materialized view. |
| `encryption_configuration` | String |  | Custom encryption configuration (e.g., Cloud KMS keys). |
| `type` | String |  | Output only. Describes the table type. The following values are supported: * `TABLE`: A normal BigQuery table. * `VIEW`: A virtual table defined by a SQL query. * `EXTERNAL`: A table that references data stored in an external storage system, such as Google Cloud Storage. * `MATERIALIZED_VIEW`: A precomputed view defined by a SQL query. * `SNAPSHOT`: An immutable BigQuery table that preserves the contents of a base table at a particular time. See additional information on [table snapshots](https://cloud.google.com/bigquery/docs/table-snapshots-intro). The default value is `TABLE`. |
| `friendly_name` | String |  | Optional. A descriptive name for this table. |
| `creation_time` | String |  | Output only. The time when this table was created, in milliseconds since the epoch. |
| `num_long_term_physical_bytes` | String |  | Output only. Number of physical bytes more than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `range_partitioning` | String |  | If specified, configures range partitioning for this table. |
| `require_partition_filter` | bool |  | Optional. If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. |
| `partition_definition` | String |  | Optional. The partition information for all table formats, including managed partitioned tables, hive partitioned tables, iceberg partitioned, and metastore partitioned tables. This field is only populated for metastore partitioned tables. For other table formats, this is an output only field. |
| `snapshot_definition` | String |  | Output only. Contains information about the snapshot. This value is set via snapshot creation. |
| `default_rounding_mode` | String |  | Optional. Defines the default rounding mode specification of new decimal fields (NUMERIC OR BIGNUMERIC) in the table. During table creation or update, if a decimal field is added to this table without an explicit rounding mode specified, then the field inherits the table default rounding mode. Changing this field doesn't affect existing fields. |
| `clustering` | String |  | Clustering specification for the table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered. |
| `replicas` | Vec<String> |  | Optional. Output only. Table references of all replicas currently active on the table. |
| `materialized_view` | String |  | Optional. The materialized view definition. |
| `table_constraints` | String |  | Optional. Tables Primary Key and Foreign Key information |
| `last_modified_time` | String |  | Output only. The time when this table was last modified, in milliseconds since the epoch. |
| `max_staleness` | String |  | Optional. The maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of sql IntervalValue type. |
| `external_data_configuration` | String |  | Optional. Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table. |
| `description` | String |  | Optional. A user-friendly description of this table. |
| `num_rows` | String |  | Output only. The number of rows of data in this table, excluding any data in the streaming buffer. |
| `num_time_travel_physical_bytes` | String |  | Output only. Number of physical bytes used by time travel storage (deleted or changed data). This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `streaming_buffer` | String |  | Output only. Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer. |
| `table_reference` | String |  | Required. Reference describing the ID of this table. |
| `time_partitioning` | String |  | If specified, configures time-based partitioning for this table. |
| `model` | String |  | Deprecated. |
| `num_current_physical_bytes` | String |  | Output only. Number of physical bytes used by current live data storage. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `schema` | String |  | Optional. Describes the schema of this table. |
| `num_total_physical_bytes` | String |  | Output only. The physical size of this table in bytes. This also includes storage used for time travel. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `self_link` | String |  | Output only. A URL that can be used to access this resource again. |
| `table_replication_info` | String |  | Optional. Table replication info for table created `AS REPLICA` DDL like: `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv` |
| `expiration_time` | String |  | Optional. The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables. |
| `biglake_configuration` | String |  | Optional. Specifies the configuration of a BigQuery table for Apache Iceberg. |
| `materialized_view_status` | String |  | Output only. The materialized view status. |
| `num_active_physical_bytes` | String |  | Output only. Number of physical bytes less than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `etag` | String |  | Output only. A hash of this resource. |
| `managed_table_type` | String |  | Optional. If set, overrides the default managed table type configured in the dataset. |
| `kind` | String |  | The type of resource ID. |
| `location` | String |  | Output only. The geographic location where the table resides. This value is inherited from the dataset. |
| `num_long_term_bytes` | String |  | Output only. The number of logical bytes in the table that are considered "long-term storage". |
| `num_bytes` | String |  | Output only. The size of this table in logical bytes, excluding any data in the streaming buffer. |
| `num_physical_bytes` | String |  | Output only. The physical size of this table in bytes. This includes storage used for time travel. |
| `default_collation` | String |  | Optional. Defines the default collation specification of new STRING fields in the table. During table creation or update, if a STRING field is added to this table without explicit collation specified, then the table inherits the table default collation. A change to this field affects only fields added afterwards, and does not alter the existing fields. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior. |
| `external_catalog_table_options` | String |  | Optional. Options defining open source compatible table. |
| `resource_tags` | HashMap<String, String> |  | [Optional] The tags associated with this table. Tag keys are globally unique. See additional information on [tags](https://cloud.google.com/iam/docs/tags-access-control#definitions). An object containing a list of "key": value pairs. The key is the namespaced friendly name of the tag key, e.g. "12345/environment" where 12345 is parent id. The value is the friendly short name of the tag value, e.g. "production". |
| `restrictions` | String |  | Optional. Output only. Restriction config for table. If set, restrict certain accesses on the table based on the config. See [Data egress](https://cloud.google.com/bigquery/docs/analytics-hub-introduction#data_egress) for more details. |
| `project_id` | String | ✅ | Required. Project ID of the new table |
| `dataset_id` | String | ✅ | Required. Dataset ID of the new table |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Output only. An opaque ID uniquely identifying the table. |
| `num_partitions` | String | Output only. The number of partitions present in the table or materialized view. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `view` | String | Optional. The view definition. |
| `labels` | HashMap<String, String> | The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key. |
| `num_active_logical_bytes` | String | Output only. Number of logical bytes that are less than 90 days old. |
| `clone_definition` | String | Output only. Contains information about the clone. This value is set via the clone operation. |
| `num_long_term_logical_bytes` | String | Output only. Number of logical bytes that are more than 90 days old. |
| `num_total_logical_bytes` | String | Output only. Total number of logical bytes in the table or materialized view. |
| `encryption_configuration` | String | Custom encryption configuration (e.g., Cloud KMS keys). |
| `type` | String | Output only. Describes the table type. The following values are supported: * `TABLE`: A normal BigQuery table. * `VIEW`: A virtual table defined by a SQL query. * `EXTERNAL`: A table that references data stored in an external storage system, such as Google Cloud Storage. * `MATERIALIZED_VIEW`: A precomputed view defined by a SQL query. * `SNAPSHOT`: An immutable BigQuery table that preserves the contents of a base table at a particular time. See additional information on [table snapshots](https://cloud.google.com/bigquery/docs/table-snapshots-intro). The default value is `TABLE`. |
| `friendly_name` | String | Optional. A descriptive name for this table. |
| `creation_time` | String | Output only. The time when this table was created, in milliseconds since the epoch. |
| `num_long_term_physical_bytes` | String | Output only. Number of physical bytes more than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `range_partitioning` | String | If specified, configures range partitioning for this table. |
| `require_partition_filter` | bool | Optional. If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. |
| `partition_definition` | String | Optional. The partition information for all table formats, including managed partitioned tables, hive partitioned tables, iceberg partitioned, and metastore partitioned tables. This field is only populated for metastore partitioned tables. For other table formats, this is an output only field. |
| `snapshot_definition` | String | Output only. Contains information about the snapshot. This value is set via snapshot creation. |
| `default_rounding_mode` | String | Optional. Defines the default rounding mode specification of new decimal fields (NUMERIC OR BIGNUMERIC) in the table. During table creation or update, if a decimal field is added to this table without an explicit rounding mode specified, then the field inherits the table default rounding mode. Changing this field doesn't affect existing fields. |
| `clustering` | String | Clustering specification for the table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered. |
| `replicas` | Vec<String> | Optional. Output only. Table references of all replicas currently active on the table. |
| `materialized_view` | String | Optional. The materialized view definition. |
| `table_constraints` | String | Optional. Tables Primary Key and Foreign Key information |
| `last_modified_time` | String | Output only. The time when this table was last modified, in milliseconds since the epoch. |
| `max_staleness` | String | Optional. The maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of sql IntervalValue type. |
| `external_data_configuration` | String | Optional. Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table. |
| `description` | String | Optional. A user-friendly description of this table. |
| `num_rows` | String | Output only. The number of rows of data in this table, excluding any data in the streaming buffer. |
| `num_time_travel_physical_bytes` | String | Output only. Number of physical bytes used by time travel storage (deleted or changed data). This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `streaming_buffer` | String | Output only. Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer. |
| `table_reference` | String | Required. Reference describing the ID of this table. |
| `time_partitioning` | String | If specified, configures time-based partitioning for this table. |
| `model` | String | Deprecated. |
| `num_current_physical_bytes` | String | Output only. Number of physical bytes used by current live data storage. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `schema` | String | Optional. Describes the schema of this table. |
| `num_total_physical_bytes` | String | Output only. The physical size of this table in bytes. This also includes storage used for time travel. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `self_link` | String | Output only. A URL that can be used to access this resource again. |
| `table_replication_info` | String | Optional. Table replication info for table created `AS REPLICA` DDL like: `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv` |
| `expiration_time` | String | Optional. The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables. |
| `biglake_configuration` | String | Optional. Specifies the configuration of a BigQuery table for Apache Iceberg. |
| `materialized_view_status` | String | Output only. The materialized view status. |
| `num_active_physical_bytes` | String | Output only. Number of physical bytes less than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes. |
| `etag` | String | Output only. A hash of this resource. |
| `managed_table_type` | String | Optional. If set, overrides the default managed table type configured in the dataset. |
| `kind` | String | The type of resource ID. |
| `location` | String | Output only. The geographic location where the table resides. This value is inherited from the dataset. |
| `num_long_term_bytes` | String | Output only. The number of logical bytes in the table that are considered "long-term storage". |
| `num_bytes` | String | Output only. The size of this table in logical bytes, excluding any data in the streaming buffer. |
| `num_physical_bytes` | String | Output only. The physical size of this table in bytes. This includes storage used for time travel. |
| `default_collation` | String | Optional. Defines the default collation specification of new STRING fields in the table. During table creation or update, if a STRING field is added to this table without explicit collation specified, then the table inherits the table default collation. A change to this field affects only fields added afterwards, and does not alter the existing fields. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior. |
| `external_catalog_table_options` | String | Optional. Options defining open source compatible table. |
| `resource_tags` | HashMap<String, String> | [Optional] The tags associated with this table. Tag keys are globally unique. See additional information on [tags](https://cloud.google.com/iam/docs/tags-access-control#definitions). An object containing a list of "key": value pairs. The key is the namespaced friendly name of the tag key, e.g. "12345/environment" where 12345 is parent id. The value is the friendly short name of the tag value, e.g. "production". |
| `restrictions` | String | Optional. Output only. Restriction config for table. If set, restrict certain accesses on the table based on the config. See [Data egress](https://cloud.google.com/bigquery/docs/analytics-hub-introduction#data_egress) for more details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.bigquery_api.Table {
    project_id = "value"  # Required. Project ID of the new table
    dataset_id = "value"  # Required. Dataset ID of the new table
}

# Access table outputs
table_id = table.id
table_id = table.id
table_num_partitions = table.num_partitions
table_view = table.view
table_labels = table.labels
table_num_active_logical_bytes = table.num_active_logical_bytes
table_clone_definition = table.clone_definition
table_num_long_term_logical_bytes = table.num_long_term_logical_bytes
table_num_total_logical_bytes = table.num_total_logical_bytes
table_encryption_configuration = table.encryption_configuration
table_type = table.type
table_friendly_name = table.friendly_name
table_creation_time = table.creation_time
table_num_long_term_physical_bytes = table.num_long_term_physical_bytes
table_range_partitioning = table.range_partitioning
table_require_partition_filter = table.require_partition_filter
table_partition_definition = table.partition_definition
table_snapshot_definition = table.snapshot_definition
table_default_rounding_mode = table.default_rounding_mode
table_clustering = table.clustering
table_replicas = table.replicas
table_materialized_view = table.materialized_view
table_table_constraints = table.table_constraints
table_last_modified_time = table.last_modified_time
table_max_staleness = table.max_staleness
table_external_data_configuration = table.external_data_configuration
table_description = table.description
table_num_rows = table.num_rows
table_num_time_travel_physical_bytes = table.num_time_travel_physical_bytes
table_streaming_buffer = table.streaming_buffer
table_table_reference = table.table_reference
table_time_partitioning = table.time_partitioning
table_model = table.model
table_num_current_physical_bytes = table.num_current_physical_bytes
table_schema = table.schema
table_num_total_physical_bytes = table.num_total_physical_bytes
table_self_link = table.self_link
table_table_replication_info = table.table_replication_info
table_expiration_time = table.expiration_time
table_biglake_configuration = table.biglake_configuration
table_materialized_view_status = table.materialized_view_status
table_num_active_physical_bytes = table.num_active_physical_bytes
table_etag = table.etag
table_managed_table_type = table.managed_table_type
table_kind = table.kind
table_location = table.location
table_num_long_term_bytes = table.num_long_term_bytes
table_num_bytes = table.num_bytes
table_num_physical_bytes = table.num_physical_bytes
table_default_collation = table.default_collation
table_external_catalog_table_options = table.external_catalog_table_options
table_resource_tags = table.resource_tags
table_restrictions = table.restrictions
```

---


### Job

Starts a new asynchronous job. This API has two different kinds of endpoint URIs, as this method supports a variety of use cases. * The *Metadata* URI is used for most interactions, as it accepts the job configuration directly. * The *Upload* URI is ONLY for the case when you're sending both a load job configuration and a data stream together. In this case, the Upload URI accepts the job configuration and the data as two distinct multipart MIME parts.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration` | String |  | Required. Describes the job configuration. |
| `kind` | String |  | Output only. The type of the resource. |
| `id` | String |  | Output only. Opaque ID field of the job. |
| `job_creation_reason` | String |  | Output only. The reason why a Job was created. |
| `etag` | String |  | Output only. A hash of this resource. |
| `statistics` | String |  | Output only. Information about the job, including starting time and ending time of the job. |
| `status` | String |  | Output only. The status of this job. Examine this value when polling an asynchronous job to see if the job is complete. |
| `job_reference` | String |  | Optional. Reference describing the unique-per-user name of the job. |
| `user_email` | String |  | Output only. Email address of the user who ran the job. |
| `principal_subject` | String |  | Output only. [Full-projection-only] String representation of identity of requesting party. Populated for both first- and third-party identities. Only present for APIs that support third-party identities. |
| `self_link` | String |  | Output only. A URL that can be used to access the resource again. |
| `project_id` | String | ✅ | Project ID of project that will be billed for the job. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | Required. Describes the job configuration. |
| `kind` | String | Output only. The type of the resource. |
| `id` | String | Output only. Opaque ID field of the job. |
| `job_creation_reason` | String | Output only. The reason why a Job was created. |
| `etag` | String | Output only. A hash of this resource. |
| `statistics` | String | Output only. Information about the job, including starting time and ending time of the job. |
| `status` | String | Output only. The status of this job. Examine this value when polling an asynchronous job to see if the job is complete. |
| `job_reference` | String | Optional. Reference describing the unique-per-user name of the job. |
| `user_email` | String | Output only. Email address of the user who ran the job. |
| `principal_subject` | String | Output only. [Full-projection-only] String representation of identity of requesting party. Populated for both first- and third-party identities. Only present for APIs that support third-party identities. |
| `self_link` | String | Output only. A URL that can be used to access the resource again. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.bigquery_api.Job {
    project_id = "value"  # Project ID of project that will be billed for the job.
}

# Access job outputs
job_id = job.id
job_configuration = job.configuration
job_kind = job.kind
job_id = job.id
job_job_creation_reason = job.job_creation_reason
job_etag = job.etag
job_statistics = job.statistics
job_status = job.status
job_job_reference = job.job_reference
job_user_email = job.user_email
job_principal_subject = job.principal_subject
job_self_link = job.self_link
```

---


### Model

Gets the specified model resource by model ID.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_modified_time` | String |  | Output only. The time when this model was last modified, in millisecs since the epoch. |
| `hparam_trials` | Vec<String> |  | Output only. Trials of a [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) model sorted by trial_id. |
| `training_runs` | Vec<String> |  | Information for all training runs in increasing order of start_time. |
| `label_columns` | Vec<String> |  | Output only. Label columns that were used to train this model. The output of the model will have a "predicted_" prefix to these columns. |
| `optimal_trial_ids` | Vec<String> |  | Output only. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it only contains the best trial. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it contains all Pareto optimal trials sorted by trial_id. |
| `etag` | String |  | Output only. A hash of this resource. |
| `default_trial_id` | String |  | Output only. The default trial_id to use in TVFs when the trial_id is not passed in. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the best trial ID. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the smallest trial ID among all Pareto optimal trials. |
| `remote_model_info` | String |  | Output only. Remote model info |
| `description` | String |  | Optional. A user-friendly description of this model. |
| `friendly_name` | String |  | Optional. A descriptive name for this model. |
| `model_reference` | String |  | Required. Unique identifier for this model. |
| `feature_columns` | Vec<String> |  | Output only. Input feature columns for the model inference. If the model is trained with TRANSFORM clause, these are the input of the TRANSFORM clause. |
| `transform_columns` | Vec<String> |  | Output only. This field will be populated if a TRANSFORM clause was used to train a model. TRANSFORM clause (if used) takes feature_columns as input and outputs transform_columns. transform_columns then are used to train the model. |
| `best_trial_id` | String |  | The best trial_id across all training runs. |
| `labels` | HashMap<String, String> |  | The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key. |
| `location` | String |  | Output only. The geographic location where the model resides. This value is inherited from the dataset. |
| `hparam_search_spaces` | String |  | Output only. All hyperparameter search spaces in this model. |
| `creation_time` | String |  | Output only. The time when this model was created, in millisecs since the epoch. |
| `expiration_time` | String |  | Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models. |
| `encryption_configuration` | String |  | Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model. |
| `model_type` | String |  | Output only. Type of the model resource. |
| `dataset_id` | String | ✅ | Required. Dataset ID of the model to patch. |
| `project_id` | String | ✅ | Required. Project ID of the model to patch. |
| `model_id` | String | ✅ | Required. Model ID of the model to patch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | Output only. The time when this model was last modified, in millisecs since the epoch. |
| `hparam_trials` | Vec<String> | Output only. Trials of a [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) model sorted by trial_id. |
| `training_runs` | Vec<String> | Information for all training runs in increasing order of start_time. |
| `label_columns` | Vec<String> | Output only. Label columns that were used to train this model. The output of the model will have a "predicted_" prefix to these columns. |
| `optimal_trial_ids` | Vec<String> | Output only. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it only contains the best trial. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it contains all Pareto optimal trials sorted by trial_id. |
| `etag` | String | Output only. A hash of this resource. |
| `default_trial_id` | String | Output only. The default trial_id to use in TVFs when the trial_id is not passed in. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the best trial ID. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the smallest trial ID among all Pareto optimal trials. |
| `remote_model_info` | String | Output only. Remote model info |
| `description` | String | Optional. A user-friendly description of this model. |
| `friendly_name` | String | Optional. A descriptive name for this model. |
| `model_reference` | String | Required. Unique identifier for this model. |
| `feature_columns` | Vec<String> | Output only. Input feature columns for the model inference. If the model is trained with TRANSFORM clause, these are the input of the TRANSFORM clause. |
| `transform_columns` | Vec<String> | Output only. This field will be populated if a TRANSFORM clause was used to train a model. TRANSFORM clause (if used) takes feature_columns as input and outputs transform_columns. transform_columns then are used to train the model. |
| `best_trial_id` | String | The best trial_id across all training runs. |
| `labels` | HashMap<String, String> | The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key. |
| `location` | String | Output only. The geographic location where the model resides. This value is inherited from the dataset. |
| `hparam_search_spaces` | String | Output only. All hyperparameter search spaces in this model. |
| `creation_time` | String | Output only. The time when this model was created, in millisecs since the epoch. |
| `expiration_time` | String | Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models. |
| `encryption_configuration` | String | Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model. |
| `model_type` | String | Output only. Type of the model resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access model outputs
model_id = model.id
model_last_modified_time = model.last_modified_time
model_hparam_trials = model.hparam_trials
model_training_runs = model.training_runs
model_label_columns = model.label_columns
model_optimal_trial_ids = model.optimal_trial_ids
model_etag = model.etag
model_default_trial_id = model.default_trial_id
model_remote_model_info = model.remote_model_info
model_description = model.description
model_friendly_name = model.friendly_name
model_model_reference = model.model_reference
model_feature_columns = model.feature_columns
model_transform_columns = model.transform_columns
model_best_trial_id = model.best_trial_id
model_labels = model.labels
model_location = model.location
model_hparam_search_spaces = model.hparam_search_spaces
model_creation_time = model.creation_time
model_expiration_time = model.expiration_time
model_encryption_configuration = model.encryption_configuration
model_model_type = model.model_type
```

---


### Row_access_policie

Creates a row access policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_time` | String |  | Output only. The time when this row access policy was created, in milliseconds since the epoch. |
| `filter_predicate` | String |  | Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region="EU" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0 |
| `grantees` | Vec<String> |  | Optional. Input only. The optional list of iam_member users or groups that specifies the initial members that the row-level access policy should be created with. grantees types: - "user:alice@example.com": An email address that represents a specific Google account. - "serviceAccount:my-other-app@appspot.gserviceaccount.com": An email address that represents a service account. - "group:admins@example.com": An email address that represents a Google group. - "domain:example.com":The Google Workspace domain (primary) that represents all the users of that domain. - "allAuthenticatedUsers": A special identifier that represents all service accounts and all users on the internet who have authenticated with a Google Account. This identifier includes accounts that aren't connected to a Google Workspace or Cloud Identity domain, such as personal Gmail accounts. Users who aren't authenticated, such as anonymous visitors, aren't included. - "allUsers":A special identifier that represents anyone who is on the internet, including authenticated and unauthenticated users. Because BigQuery requires authentication before a user can access the service, allUsers includes only authenticated users. |
| `etag` | String |  | Output only. A hash of this resource. |
| `last_modified_time` | String |  | Output only. The time when this row access policy was last modified, in milliseconds since the epoch. |
| `row_access_policy_reference` | String |  | Required. Reference describing the ID of this row access policy. |
| `dataset_id` | String | ✅ | Required. Dataset ID of the table to get the row access policy. |
| `project_id` | String | ✅ | Required. Project ID of the table to get the row access policy. |
| `table_id` | String | ✅ | Required. Table ID of the table to get the row access policy. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | Output only. The time when this row access policy was created, in milliseconds since the epoch. |
| `filter_predicate` | String | Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region="EU" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0 |
| `grantees` | Vec<String> | Optional. Input only. The optional list of iam_member users or groups that specifies the initial members that the row-level access policy should be created with. grantees types: - "user:alice@example.com": An email address that represents a specific Google account. - "serviceAccount:my-other-app@appspot.gserviceaccount.com": An email address that represents a service account. - "group:admins@example.com": An email address that represents a Google group. - "domain:example.com":The Google Workspace domain (primary) that represents all the users of that domain. - "allAuthenticatedUsers": A special identifier that represents all service accounts and all users on the internet who have authenticated with a Google Account. This identifier includes accounts that aren't connected to a Google Workspace or Cloud Identity domain, such as personal Gmail accounts. Users who aren't authenticated, such as anonymous visitors, aren't included. - "allUsers":A special identifier that represents anyone who is on the internet, including authenticated and unauthenticated users. Because BigQuery requires authentication before a user can access the service, allUsers includes only authenticated users. |
| `etag` | String | Output only. A hash of this resource. |
| `last_modified_time` | String | Output only. The time when this row access policy was last modified, in milliseconds since the epoch. |
| `row_access_policy_reference` | String | Required. Reference describing the ID of this row access policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create row_access_policie
row_access_policie = provider.bigquery_api.Row_access_policie {
    dataset_id = "value"  # Required. Dataset ID of the table to get the row access policy.
    project_id = "value"  # Required. Project ID of the table to get the row access policy.
    table_id = "value"  # Required. Table ID of the table to get the row access policy.
}

# Access row_access_policie outputs
row_access_policie_id = row_access_policie.id
row_access_policie_creation_time = row_access_policie.creation_time
row_access_policie_filter_predicate = row_access_policie.filter_predicate
row_access_policie_grantees = row_access_policie.grantees
row_access_policie_etag = row_access_policie.etag
row_access_policie_last_modified_time = row_access_policie.last_modified_time
row_access_policie_row_access_policy_reference = row_access_policie.row_access_policy_reference
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple routine resources
routine_0 = provider.bigquery_api.Routine {
    dataset_id = "value-0"
    project_id = "value-0"
}
routine_1 = provider.bigquery_api.Routine {
    dataset_id = "value-1"
    project_id = "value-1"
}
routine_2 = provider.bigquery_api.Routine {
    dataset_id = "value-2"
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    routine = provider.bigquery_api.Routine {
        dataset_id = "production-value"
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigquery_api Documentation](https://cloud.google.com/bigquery_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
