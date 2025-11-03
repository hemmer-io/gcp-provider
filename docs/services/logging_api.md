# Logging_api Service



**Resources**: 23

---

## Overview

The logging_api service provides access to 23 resource types:

- [Entrie](#entrie) [C]
- [Metric](#metric) [CRUD]
- [Monitored_resource_descriptor](#monitored_resource_descriptor) [R]
- [Sink](#sink) [CRUD]
- [Billing_account](#billing_account) [R]
- [Sink](#sink) [CRUD]
- [Saved_querie](#saved_querie) [CRUD]
- [Link](#link) [CRD]
- [View](#view) [CRUD]
- [Logging](#logging) [RU]
- [Location](#location) [R]
- [Project](#project) [R]
- [Exclusion](#exclusion) [CRUD]
- [Organization](#organization) [RU]
- [Recent_querie](#recent_querie) [R]
- [Monitored_resource_descriptor](#monitored_resource_descriptor) [R]
- [Operation](#operation) [CR]
- [Entrie](#entrie) [C]
- [Bucket](#bucket) [CRUD]
- [Log_scope](#log_scope) [CRUD]
- [Folder](#folder) [RU]
- [Log](#log) [RD]
- [Metric](#metric) [CRUD]

---

## Resources


### Entrie

Lists log entries. Use this method to retrieve log entries that originated from a project/folder/organization/billing account. For ways to export log entries, see Exporting Logs.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_names` | Vec<String> |  | Required. Names of one or more parent resources from which to retrieve log entries:
"projects/[PROJECT_ID]"
"organizations/[ORGANIZATION_ID]"
"billingAccounts/[BILLING_ACCOUNT_ID]"
"folders/[FOLDER_ID]"
Projects listed in the project_ids field are added to this list. |
| `page_token` | String |  | Optional. If present, then retrieve the next batch of results from the preceding call to this method. page_token must be the value of next_page_token from the previous response. The values of other method parameters should be identical to those in the previous call. |
| `order_by` | String |  | Optional. How the results should be sorted. Presently, the only permitted values are "timestamp asc" (default) and "timestamp desc". The first option returns entries in order of increasing values of LogEntry.timestamp (oldest first), and the second option returns entries in order of decreasing timestamps (newest first). Entries with equal timestamps are returned in order of their insert_id values. |
| `filter` | String |  | Optional. A filter that chooses which log entries to return. See Advanced Logs Filters. Only log entries that match the filter are returned. An empty filter matches all log entries in the resources listed in resource_names. Referencing a parent resource that is not listed in resource_names will cause the filter to return no results. The maximum length of the filter is 20000 characters. |
| `page_size` | i64 |  | Optional. The maximum number of results to return from this request. Non-positive values are ignored. The presence of next_page_token in the response indicates that more results might be available. |
| `project_ids` | Vec<String> |  | Deprecated. Use resource_names instead. One or more project identifiers or project numbers from which to retrieve log entries. Example: "my-project-1A". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.logging_api.Entrie {
}

```

---


### Metric

Creates a logs-based metric.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket_options` | String |  | Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values. |
| `description` | String |  | Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters. |
| `update_time` | String |  | Output only. The last update timestamp of the metric.This field may not be present for older metrics. |
| `label_extractors` | HashMap<String, String> |  | Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If the either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project. |
| `metric_descriptor` | String |  | Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description. |
| `value_extractor` | String |  | Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The argument are:  1. field: The name of the log entry field from which the value is to be  extracted.  2. regex: A regular expression using the Google RE2 syntax  (https://github.com/google/re2/wiki/Syntax) with a single capture  group to extract data from the specified log entry field. The value  of the field is converted to a string before applying the regex.  It is an error to specify a regex that does not include exactly one  capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*") |
| `create_time` | String |  | Output only. The creation timestamp of the metric.This field may not be present for older metrics. |
| `name` | String |  | Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.The metric identifier in this field must not be URL-encoded (https://en.wikipedia.org/wiki/Percent-encoding). However, when the metric identifier appears as the [METRIC_ID] part of a metric_name API parameter, then the metric identifier must be URL-encoded. Example: "projects/my-project/metrics/nginx%2Frequests". |
| `filter` | String |  | Required. An advanced logs filter which is used to match log entries. Example:
"resource.type=gae_app AND severity>=ERROR"
The maximum length of the filter is 20000 characters. |
| `version` | String |  | Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed. |
| `parent` | String | ✅ | The resource name of the project in which to create the metric:
"projects/[PROJECT_ID]"
The new metric must be provided in the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket_options` | String | Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values. |
| `description` | String | Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters. |
| `update_time` | String | Output only. The last update timestamp of the metric.This field may not be present for older metrics. |
| `label_extractors` | HashMap<String, String> | Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If the either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project. |
| `metric_descriptor` | String | Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description. |
| `value_extractor` | String | Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The argument are:  1. field: The name of the log entry field from which the value is to be  extracted.  2. regex: A regular expression using the Google RE2 syntax  (https://github.com/google/re2/wiki/Syntax) with a single capture  group to extract data from the specified log entry field. The value  of the field is converted to a string before applying the regex.  It is an error to specify a regex that does not include exactly one  capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*") |
| `create_time` | String | Output only. The creation timestamp of the metric.This field may not be present for older metrics. |
| `name` | String | Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.The metric identifier in this field must not be URL-encoded (https://en.wikipedia.org/wiki/Percent-encoding). However, when the metric identifier appears as the [METRIC_ID] part of a metric_name API parameter, then the metric identifier must be URL-encoded. Example: "projects/my-project/metrics/nginx%2Frequests". |
| `filter` | String | Required. An advanced logs filter which is used to match log entries. Example:
"resource.type=gae_app AND severity>=ERROR"
The maximum length of the filter is 20000 characters. |
| `version` | String | Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metric
metric = provider.logging_api.Metric {
    parent = "value"  # The resource name of the project in which to create the metric:
"projects/[PROJECT_ID]"
The new metric must be provided in the request.
}

# Access metric outputs
metric_id = metric.id
metric_bucket_options = metric.bucket_options
metric_description = metric.description
metric_update_time = metric.update_time
metric_label_extractors = metric.label_extractors
metric_metric_descriptor = metric.metric_descriptor
metric_value_extractor = metric.value_extractor
metric_create_time = metric.create_time
metric_name = metric.name
metric_filter = metric.filter
metric_version = metric.version
```

---


### Monitored_resource_descriptor

Lists the descriptors for monitored resource types used by Logging.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken. |
| `resource_descriptors` | Vec<String> | A list of resource descriptors. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access monitored_resource_descriptor outputs
monitored_resource_descriptor_id = monitored_resource_descriptor.id
monitored_resource_descriptor_next_page_token = monitored_resource_descriptor.next_page_token
monitored_resource_descriptor_resource_descriptors = monitored_resource_descriptor.resource_descriptors
```

---


### Sink

Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `include_children` | bool |  | Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then logs from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression. For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent. To only export entries from certain child projects, filter on the project part of the log name:
logName:("projects/test-project1/" OR "projects/test-project2/") AND
resource.type=gce_instance
 |
| `update_time` | String |  | Output only. The last update timestamp of the sink.This field may not be present for older sinks. |
| `create_time` | String |  | Output only. The creation timestamp of the sink.This field may not be present for older sinks. |
| `filter` | String |  | Optional. An advanced logs filter. The only exported log entries are those that are in the resource owning the sink and that match the filter. For example:
logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR
 |
| `writer_identity` | String |  | Output only. An IAM identity&mdash;a service account or group&mdash;under which Logging writes the exported log entries to the sink's destination. This field is set by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource. Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity. |
| `name` | String |  | Required. The client-assigned sink identifier, unique within the project. Example: "my-syslog-errors-to-pubsub". Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods. |
| `destination` | String |  | Required. The export destination:
"storage.googleapis.com/[GCS_BUCKET]"
"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]"
"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]"
The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks. |
| `output_version_format` | String |  | Deprecated. The log entry format to use for this sink's exported log entries. The v2 format is used by default and cannot be changed. |
| `parent` | String | ✅ | Required. The resource in which to create the sink:
"projects/[PROJECT_ID]"
"organizations/[ORGANIZATION_ID]"
"billingAccounts/[BILLING_ACCOUNT_ID]"
"folders/[FOLDER_ID]"
Examples: "projects/my-logging-project", "organizations/123456789". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `include_children` | bool | Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then logs from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression. For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent. To only export entries from certain child projects, filter on the project part of the log name:
logName:("projects/test-project1/" OR "projects/test-project2/") AND
resource.type=gce_instance
 |
| `update_time` | String | Output only. The last update timestamp of the sink.This field may not be present for older sinks. |
| `create_time` | String | Output only. The creation timestamp of the sink.This field may not be present for older sinks. |
| `filter` | String | Optional. An advanced logs filter. The only exported log entries are those that are in the resource owning the sink and that match the filter. For example:
logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR
 |
| `writer_identity` | String | Output only. An IAM identity&mdash;a service account or group&mdash;under which Logging writes the exported log entries to the sink's destination. This field is set by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource. Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity. |
| `name` | String | Required. The client-assigned sink identifier, unique within the project. Example: "my-syslog-errors-to-pubsub". Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods. |
| `destination` | String | Required. The export destination:
"storage.googleapis.com/[GCS_BUCKET]"
"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]"
"pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]"
The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks. |
| `output_version_format` | String | Deprecated. The log entry format to use for this sink's exported log entries. The v2 format is used by default and cannot be changed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sink
sink = provider.logging_api.Sink {
    parent = "value"  # Required. The resource in which to create the sink:
"projects/[PROJECT_ID]"
"organizations/[ORGANIZATION_ID]"
"billingAccounts/[BILLING_ACCOUNT_ID]"
"folders/[FOLDER_ID]"
Examples: "projects/my-logging-project", "organizations/123456789".
}

# Access sink outputs
sink_id = sink.id
sink_include_children = sink.include_children
sink_update_time = sink.update_time
sink_create_time = sink.create_time
sink_filter = sink.filter
sink_writer_identity = sink.writer_identity
sink_name = sink.name
sink_destination = sink.destination
sink_output_version_format = sink.output_version_format
```

---


### Billing_account

Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations, and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version_name` | String | Output only. The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.If this field is populated, the kms_key is tied to a specific CryptoKeyVersion. |
| `kms_key_name` | String | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the needed cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | Output only. The resource name of the CMEK settings. |
| `service_account_id` | String | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access billing_account outputs
billing_account_id = billing_account.id
billing_account_kms_key_version_name = billing_account.kms_key_version_name
billing_account_kms_key_name = billing_account.kms_key_name
billing_account_name = billing_account.name
billing_account_service_account_id = billing_account.service_account_id
```

---


### Sink

Creates a sink that exports specified log entries to a destination. The export begins upon ingress, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disabled` | bool |  | Optional. If set to true, then this sink is disabled and it does not export any log entries. |
| `create_time` | String |  | Output only. The creation timestamp of the sink.This field may not be present for older sinks. |
| `filter` | String |  | Optional. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries). The only exported log entries are those that are in the resource owning the sink and that match the filter.For example:logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR |
| `description` | String |  | Optional. A description of this sink.The maximum length of the description is 8000 characters. |
| `output_version_format` | String |  | Deprecated. This field is unused. |
| `exclusions` | Vec<String> |  | Optional. Log entries that match any of these exclusion filters will not be exported.If a log entry is matched by both filter and one of exclusions it will not be exported. |
| `destination` | String |  | Required. The export destination: "storage.googleapis.com/[GCS_BUCKET]" "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]" "pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]" "logging.googleapis.com/projects/[PROJECT_ID]" "logging.googleapis.com/projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks (https://cloud.google.com/logging/docs/api/tasks/exporting-logs). |
| `intercept_children` | bool |  | Optional. This field applies only to sinks owned by organizations and folders.When the value of 'intercept_children' is true, the following restrictions apply: The sink must have the include_children flag set to true. The sink destination must be a Cloud project.Also, the following behaviors apply: Any logs matched by the sink won't be included by non-_Required sinks owned by child resources. The sink appears in the results of a ListSinks call from a child resource if the value of the filter field in its request is either 'in_scope("ALL")' or 'in_scope("ANCESTOR")'. |
| `include_children` | bool |  | Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then log entries from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression.For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent.To only export entries from certain child projects, filter on the project part of the log name:logName:("projects/test-project1/" OR "projects/test-project2/") AND resource.type=gce_instance |
| `name` | String |  | Optional. The client-assigned sink identifier, unique within the project.For example: "my-syslog-errors-to-pubsub".Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, periods.First character has to be alphanumeric. |
| `resource_name` | String |  | Output only. The resource name of the sink. "projects/[PROJECT_ID]/sinks/[SINK_NAME] "organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME] "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME] "folders/[FOLDER_ID]/sinks/[SINK_NAME] For example: projects/my_project/sinks/SINK_NAME |
| `update_time` | String |  | Output only. The last update timestamp of the sink.This field may not be present for older sinks. |
| `writer_identity` | String |  | Output only. An IAM identity—a service account or group—under which Cloud Logging writes the exported log entries to the sink's destination. This field is either set by specifying custom_writer_identity or set automatically by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource (https://cloud.google.com/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity.Sinks that have a destination that is a log bucket in the same project as the sink cannot have a writer_identity and no additional permissions are required. |
| `bigquery_options` | String |  | Optional. Options that affect sinks exporting data to BigQuery. |
| `parent` | String | ✅ | Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disabled` | bool | Optional. If set to true, then this sink is disabled and it does not export any log entries. |
| `create_time` | String | Output only. The creation timestamp of the sink.This field may not be present for older sinks. |
| `filter` | String | Optional. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries). The only exported log entries are those that are in the resource owning the sink and that match the filter.For example:logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR |
| `description` | String | Optional. A description of this sink.The maximum length of the description is 8000 characters. |
| `output_version_format` | String | Deprecated. This field is unused. |
| `exclusions` | Vec<String> | Optional. Log entries that match any of these exclusion filters will not be exported.If a log entry is matched by both filter and one of exclusions it will not be exported. |
| `destination` | String | Required. The export destination: "storage.googleapis.com/[GCS_BUCKET]" "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]" "pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]" "logging.googleapis.com/projects/[PROJECT_ID]" "logging.googleapis.com/projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks (https://cloud.google.com/logging/docs/api/tasks/exporting-logs). |
| `intercept_children` | bool | Optional. This field applies only to sinks owned by organizations and folders.When the value of 'intercept_children' is true, the following restrictions apply: The sink must have the include_children flag set to true. The sink destination must be a Cloud project.Also, the following behaviors apply: Any logs matched by the sink won't be included by non-_Required sinks owned by child resources. The sink appears in the results of a ListSinks call from a child resource if the value of the filter field in its request is either 'in_scope("ALL")' or 'in_scope("ANCESTOR")'. |
| `include_children` | bool | Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then log entries from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression.For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent.To only export entries from certain child projects, filter on the project part of the log name:logName:("projects/test-project1/" OR "projects/test-project2/") AND resource.type=gce_instance |
| `name` | String | Optional. The client-assigned sink identifier, unique within the project.For example: "my-syslog-errors-to-pubsub".Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, periods.First character has to be alphanumeric. |
| `resource_name` | String | Output only. The resource name of the sink. "projects/[PROJECT_ID]/sinks/[SINK_NAME] "organizations/[ORGANIZATION_ID]/sinks/[SINK_NAME] "billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_NAME] "folders/[FOLDER_ID]/sinks/[SINK_NAME] For example: projects/my_project/sinks/SINK_NAME |
| `update_time` | String | Output only. The last update timestamp of the sink.This field may not be present for older sinks. |
| `writer_identity` | String | Output only. An IAM identity—a service account or group—under which Cloud Logging writes the exported log entries to the sink's destination. This field is either set by specifying custom_writer_identity or set automatically by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource (https://cloud.google.com/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity.Sinks that have a destination that is a log bucket in the same project as the sink cannot have a writer_identity and no additional permissions are required. |
| `bigquery_options` | String | Optional. Options that affect sinks exporting data to BigQuery. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sink
sink = provider.logging_api.Sink {
    parent = "value"  # Required. The resource in which to create the sink: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-project" "organizations/123456789"
}

# Access sink outputs
sink_id = sink.id
sink_disabled = sink.disabled
sink_create_time = sink.create_time
sink_filter = sink.filter
sink_description = sink.description
sink_output_version_format = sink.output_version_format
sink_exclusions = sink.exclusions
sink_destination = sink.destination
sink_intercept_children = sink.intercept_children
sink_include_children = sink.include_children
sink_name = sink.name
sink_resource_name = sink.resource_name
sink_update_time = sink.update_time
sink_writer_identity = sink.writer_identity
sink_bigquery_options = sink.bigquery_options
```

---


### Saved_querie

Creates a new SavedQuery for the user making the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the saved query was created. |
| `name` | String |  | Output only. Resource name of the saved query.In the format: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/savedQueries/[QUERY_ID]" For a list of supported locations, see Supported Regions (https://cloud.google.com/logging/docs/region-support#bucket-regions)After the saved query is created, the location cannot be changed.If the user doesn't provide a QUERY_ID, the system will generate an alphanumeric ID. |
| `display_name` | String |  | Required. The user specified title for the SavedQuery. |
| `logging_query` | String |  | Logging query that can be executed in Logs Explorer or via Logging API. |
| `description` | String |  | Optional. A human readable description of the saved query. |
| `ops_analytics_query` | String |  | Analytics query that can be executed in Log Analytics. |
| `update_time` | String |  | Output only. The timestamp when the saved query was last updated. |
| `visibility` | String |  | Required. The visibility status of this query, which determines its ownership. |
| `parent` | String | ✅ | Required. The parent resource in which to create the saved query: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" For example: "projects/my-project/locations/global" "organizations/123456789/locations/us-central1"  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the saved query was created. |
| `name` | String | Output only. Resource name of the saved query.In the format: "projects/[PROJECT_ID]/locations/[LOCATION_ID]/savedQueries/[QUERY_ID]" For a list of supported locations, see Supported Regions (https://cloud.google.com/logging/docs/region-support#bucket-regions)After the saved query is created, the location cannot be changed.If the user doesn't provide a QUERY_ID, the system will generate an alphanumeric ID. |
| `display_name` | String | Required. The user specified title for the SavedQuery. |
| `logging_query` | String | Logging query that can be executed in Logs Explorer or via Logging API. |
| `description` | String | Optional. A human readable description of the saved query. |
| `ops_analytics_query` | String | Analytics query that can be executed in Log Analytics. |
| `update_time` | String | Output only. The timestamp when the saved query was last updated. |
| `visibility` | String | Required. The visibility status of this query, which determines its ownership. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create saved_querie
saved_querie = provider.logging_api.Saved_querie {
    parent = "value"  # Required. The parent resource in which to create the saved query: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" For example: "projects/my-project/locations/global" "organizations/123456789/locations/us-central1" 
}

# Access saved_querie outputs
saved_querie_id = saved_querie.id
saved_querie_create_time = saved_querie.create_time
saved_querie_name = saved_querie.name
saved_querie_display_name = saved_querie.display_name
saved_querie_logging_query = saved_querie.logging_query
saved_querie_description = saved_querie.description
saved_querie_ops_analytics_query = saved_querie.ops_analytics_query
saved_querie_update_time = saved_querie.update_time
saved_querie_visibility = saved_querie.visibility
```

---


### Link

Asynchronously creates a linked dataset in BigQuery which makes it possible to use BigQuery to read the logs stored in the log bucket. A log bucket may currently only contain one link.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation timestamp of the link. |
| `name` | String |  | Output only. The resource name of the link. The name can have up to 100 characters. A valid link id (at the end of the link name) must only have alphanumeric characters and underscores within it. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" For example:`projects/my-project/locations/global/buckets/my-bucket/links/my_link |
| `bigquery_dataset` | String |  | Optional. The information of a BigQuery Dataset. When a link is created, a BigQuery dataset is created along with it, in the same project as the LogBucket it's linked to. This dataset will also have BigQuery Views corresponding to the LogViews in the bucket. |
| `lifecycle_state` | String |  | Output only. The resource lifecycle state. |
| `description` | String |  | Optional. Describes this link.The maximum length of the description is 8000 characters. |
| `parent` | String | ✅ | Required. The full resource name of the bucket to create a link for. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of the link. |
| `name` | String | Output only. The resource name of the link. The name can have up to 100 characters. A valid link id (at the end of the link name) must only have alphanumeric characters and underscores within it. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/links/[LINK_ID]" For example:`projects/my-project/locations/global/buckets/my-bucket/links/my_link |
| `bigquery_dataset` | String | Optional. The information of a BigQuery Dataset. When a link is created, a BigQuery dataset is created along with it, in the same project as the LogBucket it's linked to. This dataset will also have BigQuery Views corresponding to the LogViews in the bucket. |
| `lifecycle_state` | String | Output only. The resource lifecycle state. |
| `description` | String | Optional. Describes this link.The maximum length of the description is 8000 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create link
link = provider.logging_api.Link {
    parent = "value"  # Required. The full resource name of the bucket to create a link for. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" 
}

# Access link outputs
link_id = link.id
link_create_time = link.create_time
link_name = link.name
link_bigquery_dataset = link.bigquery_dataset
link_lifecycle_state = link.lifecycle_state
link_description = link.description
```

---


### View

Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation timestamp of the view. |
| `filter` | String |  | Optional. Filter that restricts which log entries in a bucket are visible in this view.Filters must be logical conjunctions that use the AND operator, and they can use any of the following qualifiers: SOURCE(), which specifies a project, folder, organization, or billing account of origin. resource.type, which specifies the resource type. LOG_ID(), which identifies the log.They can also use the negations of these qualifiers with the NOT operator.For example:SOURCE("projects/myproject") AND resource.type = "gce_instance" AND NOT LOG_ID("stdout") |
| `description` | String |  | Optional. Describes this view. |
| `name` | String |  | Output only. The resource name of the view.For example:projects/my-project/locations/global/buckets/my-bucket/views/my-view |
| `update_time` | String |  | Output only. The last update timestamp of the view. |
| `parent` | String | ✅ | Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of the view. |
| `filter` | String | Optional. Filter that restricts which log entries in a bucket are visible in this view.Filters must be logical conjunctions that use the AND operator, and they can use any of the following qualifiers: SOURCE(), which specifies a project, folder, organization, or billing account of origin. resource.type, which specifies the resource type. LOG_ID(), which identifies the log.They can also use the negations of these qualifiers with the NOT operator.For example:SOURCE("projects/myproject") AND resource.type = "gce_instance" AND NOT LOG_ID("stdout") |
| `description` | String | Optional. Describes this view. |
| `name` | String | Output only. The resource name of the view.For example:projects/my-project/locations/global/buckets/my-bucket/views/my-view |
| `update_time` | String | Output only. The last update timestamp of the view. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create view
view = provider.logging_api.View {
    parent = "value"  # Required. The bucket in which to create the view `"projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]"` For example:"projects/my-project/locations/global/buckets/my-bucket"
}

# Access view outputs
view_id = view.id
view_create_time = view.create_time
view_filter = view.filter
view_description = view.description
view_name = view.name
view_update_time = view.update_time
```

---


### Logging

Gets the settings for the given resource.Note: Settings can be retrieved for Google Cloud projects, folders, organizations, and billing accounts.See View default resource settings for Logging (https://cloud.google.com/logging/docs/default-settings#view-org-settings) for more information.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_version_name` | String |  | Output only. The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.If this field is populated, the kms_key is tied to a specific CryptoKeyVersion. |
| `kms_key_name` | String |  | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the needed cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String |  | Output only. The resource name of the CMEK settings. |
| `service_account_id` | String |  | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | ✅ | Required. The resource name for the CMEK settings to update. "projects/[PROJECT_ID]/cmekSettings" "organizations/[ORGANIZATION_ID]/cmekSettings" "billingAccounts/[BILLING_ACCOUNT_ID]/cmekSettings" "folders/[FOLDER_ID]/cmekSettings" For example:"organizations/12345/cmekSettings"Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the settings. |
| `logging_service_account_id` | String | Output only. The service account for the given resource container, such as project or folder. Log sinks use this service account as their writer_identity if no custom service account is provided in the request when calling the create sink method. |
| `kms_key_name` | String | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK, set this field to a valid kms_key_name for which the associated service account has the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `storage_location` | String | Optional. The storage location for the _Default and _Required log buckets of newly created projects and folders, unless the storage location is explicitly provided.Example value: europe-west1.Note: this setting does not affect the location of resources where a location is explicitly provided when created, such as custom log buckets. |
| `default_sink_config` | String | Optional. Overrides the built-in configuration for _Default sink. |
| `disable_default_sink` | bool | Optional. If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed. |
| `kms_service_account_id` | String | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK, you must first assign the role roles/cloudkms.cryptoKeyEncrypterDecrypter to the service account that will be used to access your Cloud KMS key. Use GetSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access logging outputs
logging_id = logging.id
logging_name = logging.name
logging_logging_service_account_id = logging.logging_service_account_id
logging_kms_key_name = logging.kms_key_name
logging_storage_location = logging.storage_location
logging_default_sink_config = logging.default_sink_config
logging_disable_default_sink = logging.disable_default_sink
logging_kms_service_account_id = logging.kms_service_account_id
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
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
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
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
```

---


### Project

Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations, and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version_name` | String | Output only. The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.If this field is populated, the kms_key is tied to a specific CryptoKeyVersion. |
| `kms_key_name` | String | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the needed cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | Output only. The resource name of the CMEK settings. |
| `service_account_id` | String | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |


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
project_kms_key_version_name = project.kms_key_version_name
project_kms_key_name = project.kms_key_name
project_name = project.name
project_service_account_id = project.service_account_id
```

---


### Exclusion

Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disabled` | bool |  | Optional. If set to True, then this exclusion is disabled and it does not exclude any log entries. You can update an exclusion to change the value of this field. |
| `create_time` | String |  | Output only. The creation timestamp of the exclusion.This field may not be present for older exclusions. |
| `name` | String |  | Optional. A client-assigned identifier, such as "load-balancer-exclusion". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric. |
| `update_time` | String |  | Output only. The last update timestamp of the exclusion.This field may not be present for older exclusions. |
| `filter` | String |  | Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries) that matches the log entries to be excluded. By using the sample function (https://cloud.google.com/logging/docs/view/advanced-queries#sample), you can exclude less than 100% of the matching log entries.For example, the following query matches 99% of low-severity log entries from Google Cloud Storage buckets:resource.type=gcs_bucket severity<ERROR sample(insertId, 0.99) |
| `description` | String |  | Optional. A description of this exclusion. |
| `parent` | String | ✅ | Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disabled` | bool | Optional. If set to True, then this exclusion is disabled and it does not exclude any log entries. You can update an exclusion to change the value of this field. |
| `create_time` | String | Output only. The creation timestamp of the exclusion.This field may not be present for older exclusions. |
| `name` | String | Optional. A client-assigned identifier, such as "load-balancer-exclusion". Identifiers are limited to 100 characters and can include only letters, digits, underscores, hyphens, and periods. First character has to be alphanumeric. |
| `update_time` | String | Output only. The last update timestamp of the exclusion.This field may not be present for older exclusions. |
| `filter` | String | Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-queries) that matches the log entries to be excluded. By using the sample function (https://cloud.google.com/logging/docs/view/advanced-queries#sample), you can exclude less than 100% of the matching log entries.For example, the following query matches 99% of low-severity log entries from Google Cloud Storage buckets:resource.type=gcs_bucket severity<ERROR sample(insertId, 0.99) |
| `description` | String | Optional. A description of this exclusion. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create exclusion
exclusion = provider.logging_api.Exclusion {
    parent = "value"  # Required. The parent resource in which to create the exclusion: "projects/[PROJECT_ID]" "organizations/[ORGANIZATION_ID]" "billingAccounts/[BILLING_ACCOUNT_ID]" "folders/[FOLDER_ID]" For examples:"projects/my-logging-project" "organizations/123456789"
}

# Access exclusion outputs
exclusion_id = exclusion.id
exclusion_disabled = exclusion.disabled
exclusion_create_time = exclusion.create_time
exclusion_name = exclusion.name
exclusion_update_time = exclusion.update_time
exclusion_filter = exclusion.filter
exclusion_description = exclusion.description
```

---


### Organization

Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations, and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the settings. |
| `logging_service_account_id` | String |  | Output only. The service account for the given resource container, such as project or folder. Log sinks use this service account as their writer_identity if no custom service account is provided in the request when calling the create sink method. |
| `kms_key_name` | String |  | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK, set this field to a valid kms_key_name for which the associated service account has the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `storage_location` | String |  | Optional. The storage location for the _Default and _Required log buckets of newly created projects and folders, unless the storage location is explicitly provided.Example value: europe-west1.Note: this setting does not affect the location of resources where a location is explicitly provided when created, such as custom log buckets. |
| `default_sink_config` | String |  | Optional. Overrides the built-in configuration for _Default sink. |
| `disable_default_sink` | bool |  | Optional. If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed. |
| `kms_service_account_id` | String |  | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK, you must first assign the role roles/cloudkms.cryptoKeyEncrypterDecrypter to the service account that will be used to access your Cloud KMS key. Use GetSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | ✅ | Required. The resource name for the settings to update. "organizations/[ORGANIZATION_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version_name` | String | Output only. The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.If this field is populated, the kms_key is tied to a specific CryptoKeyVersion. |
| `kms_key_name` | String | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the needed cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | Output only. The resource name of the CMEK settings. |
| `service_account_id` | String | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_kms_key_version_name = organization.kms_key_version_name
organization_kms_key_name = organization.kms_key_name
organization_name = organization.name
organization_service_account_id = organization.service_account_id
```

---


### Recent_querie

Lists the RecentQueries that were created by the user making the request.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unreachable` | Vec<String> | The unreachable resources. Each resource can be either 1) a saved query if a specific query is unreachable or 2) a location if a specific location is unreachable. "projects/[PROJECT_ID]/locations/[LOCATION_ID]/recentQueries/[QUERY_ID]" "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global/recentQueries/12345678" "projects/my-project/locations/global"If there are unreachable resources, the response will first return pages that contain recent queries, and then return pages that contain the unreachable resources. |
| `recent_queries` | Vec<String> | A list of recent queries. |
| `next_page_token` | String | If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recent_querie outputs
recent_querie_id = recent_querie.id
recent_querie_unreachable = recent_querie.unreachable
recent_querie_recent_queries = recent_querie.recent_queries
recent_querie_next_page_token = recent_querie.next_page_token
```

---


### Monitored_resource_descriptor

Lists the descriptors for monitored resource types used by Logging.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken. |
| `resource_descriptors` | Vec<String> | A list of resource descriptors. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access monitored_resource_descriptor outputs
monitored_resource_descriptor_id = monitored_resource_descriptor.id
monitored_resource_descriptor_next_page_token = monitored_resource_descriptor.next_page_token
monitored_resource_descriptor_resource_descriptors = monitored_resource_descriptor.resource_descriptors
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.logging_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
```

---


### Entrie

Streaming read of log entries as they are received. Until the stream is terminated, it will continue reading logs.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `buffer_window` | String |  | Optional. The amount of time to buffer log entries at the server before being returned to prevent out of order results due to late arriving log entries. Valid values are between 0-60000 milliseconds. Defaults to 2000 milliseconds. |
| `filter` | String |  | Optional. Only log entries that match the filter are returned. An empty filter matches all log entries in the resources listed in resource_names. Referencing a parent resource that is not listed in resource_names will cause the filter to return no results. The maximum length of a filter is 20,000 characters. |
| `resource_names` | Vec<String> |  | Required. Name of a parent resource from which to retrieve log entries: projects/[PROJECT_ID] organizations/[ORGANIZATION_ID] billingAccounts/[BILLING_ACCOUNT_ID] folders/[FOLDER_ID]May alternatively be one or more views: projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] billingAccounts/[BILLING_ACCOUNT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] folders/[FOLDER_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID] |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.logging_api.Entrie {
}

```

---


### Bucket

Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation timestamp of the bucket. This is not set for any of the default buckets. |
| `cmek_settings` | String |  | Optional. The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by updating the log bucket. Changing the KMS key is allowed. |
| `index_configs` | Vec<String> |  | Optional. A list of indexed fields and related configuration data. |
| `name` | String |  | Output only. The resource name of the bucket.For example:projects/my-project/locations/global/buckets/my-bucketFor a list of supported locations, see Supported Regions (https://cloud.google.com/logging/docs/region-support)For the location of global it is unspecified where log entries are actually stored.After a bucket has been created, the location cannot be changed. |
| `restricted_fields` | Vec<String> |  | Optional. Log entry field paths that are denied access in this bucket.The following fields and their children are eligible: textPayload, jsonPayload, protoPayload, httpRequest, labels, sourceLocation.Restricting a repeated field will restrict all values. Adding a parent will block all child fields. (e.g. foo.bar will block foo.bar.baz) |
| `lifecycle_state` | String |  | Output only. The bucket lifecycle state. |
| `retention_days` | i64 |  | Optional. Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. |
| `update_time` | String |  | Output only. The last update timestamp of the bucket. |
| `analytics_enabled` | bool |  | Optional. Whether log analytics is enabled for this bucket.Once enabled, log analytics features cannot be disabled. |
| `description` | String |  | Optional. Describes this bucket. |
| `locked` | bool |  | Optional. Whether the bucket is locked.The retention period on a locked bucket cannot be changed. Locked buckets may only be deleted if they are empty. |
| `parent` | String | ✅ | Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of the bucket. This is not set for any of the default buckets. |
| `cmek_settings` | String | Optional. The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by updating the log bucket. Changing the KMS key is allowed. |
| `index_configs` | Vec<String> | Optional. A list of indexed fields and related configuration data. |
| `name` | String | Output only. The resource name of the bucket.For example:projects/my-project/locations/global/buckets/my-bucketFor a list of supported locations, see Supported Regions (https://cloud.google.com/logging/docs/region-support)For the location of global it is unspecified where log entries are actually stored.After a bucket has been created, the location cannot be changed. |
| `restricted_fields` | Vec<String> | Optional. Log entry field paths that are denied access in this bucket.The following fields and their children are eligible: textPayload, jsonPayload, protoPayload, httpRequest, labels, sourceLocation.Restricting a repeated field will restrict all values. Adding a parent will block all child fields. (e.g. foo.bar will block foo.bar.baz) |
| `lifecycle_state` | String | Output only. The bucket lifecycle state. |
| `retention_days` | i64 | Optional. Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. |
| `update_time` | String | Output only. The last update timestamp of the bucket. |
| `analytics_enabled` | bool | Optional. Whether log analytics is enabled for this bucket.Once enabled, log analytics features cannot be disabled. |
| `description` | String | Optional. Describes this bucket. |
| `locked` | bool | Optional. Whether the bucket is locked.The retention period on a locked bucket cannot be changed. Locked buckets may only be deleted if they are empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket
bucket = provider.logging_api.Bucket {
    parent = "value"  # Required. The resource in which to create the log bucket: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
}

# Access bucket outputs
bucket_id = bucket.id
bucket_create_time = bucket.create_time
bucket_cmek_settings = bucket.cmek_settings
bucket_index_configs = bucket.index_configs
bucket_name = bucket.name
bucket_restricted_fields = bucket.restricted_fields
bucket_lifecycle_state = bucket.lifecycle_state
bucket_retention_days = bucket.retention_days
bucket_update_time = bucket.update_time
bucket_analytics_enabled = bucket.analytics_enabled
bucket_description = bucket.description
bucket_locked = bucket.locked
```

---


### Log_scope

Creates a log scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation timestamp of the log scope. |
| `name` | String |  | Output only. The resource name of the log scope.Log scopes are only available in the global location. For example:projects/my-project/locations/global/logScopes/my-log-scope |
| `resource_names` | Vec<String> |  | Required. Names of one or more parent resources (organizations and folders are not supported.): projects/[PROJECT_ID]May alternatively be one or more views: projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]A log scope can include a maximum of 5 projects and a maximum of 100 resources in total. |
| `update_time` | String |  | Output only. The last update timestamp of the log scope. |
| `description` | String |  | Optional. Describes this log scope.The maximum length of the description is 8000 characters. |
| `parent` | String | ✅ | Required. The parent resource in which to create the log scope: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of the log scope. |
| `name` | String | Output only. The resource name of the log scope.Log scopes are only available in the global location. For example:projects/my-project/locations/global/logScopes/my-log-scope |
| `resource_names` | Vec<String> | Required. Names of one or more parent resources (organizations and folders are not supported.): projects/[PROJECT_ID]May alternatively be one or more views: projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]A log scope can include a maximum of 5 projects and a maximum of 100 resources in total. |
| `update_time` | String | Output only. The last update timestamp of the log scope. |
| `description` | String | Optional. Describes this log scope.The maximum length of the description is 8000 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create log_scope
log_scope = provider.logging_api.Log_scope {
    parent = "value"  # Required. The parent resource in which to create the log scope: "projects/[PROJECT_ID]/locations/[LOCATION_ID]" "organizations/[ORGANIZATION_ID]/locations/[LOCATION_ID]" "folders/[FOLDER_ID]/locations/[LOCATION_ID]" For example:"projects/my-project/locations/global"
}

# Access log_scope outputs
log_scope_id = log_scope.id
log_scope_create_time = log_scope.create_time
log_scope_name = log_scope.name
log_scope_resource_names = log_scope.resource_names
log_scope_update_time = log_scope.update_time
log_scope_description = log_scope.description
```

---


### Folder

Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations, and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the settings. |
| `logging_service_account_id` | String |  | Output only. The service account for the given resource container, such as project or folder. Log sinks use this service account as their writer_identity if no custom service account is provided in the request when calling the create sink method. |
| `kms_key_name` | String |  | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK, set this field to a valid kms_key_name for which the associated service account has the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `storage_location` | String |  | Optional. The storage location for the _Default and _Required log buckets of newly created projects and folders, unless the storage location is explicitly provided.Example value: europe-west1.Note: this setting does not affect the location of resources where a location is explicitly provided when created, such as custom log buckets. |
| `default_sink_config` | String |  | Optional. Overrides the built-in configuration for _Default sink. |
| `disable_default_sink` | bool |  | Optional. If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed. |
| `kms_service_account_id` | String |  | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK, you must first assign the role roles/cloudkms.cryptoKeyEncrypterDecrypter to the service account that will be used to access your Cloud KMS key. Use GetSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | ✅ | Required. The resource name for the settings to update. "organizations/[ORGANIZATION_ID]/settings" "folders/[FOLDER_ID]/settings" For example:"organizations/12345/settings" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_version_name` | String | Output only. The CryptoKeyVersion resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]/cryptoKeyVersions/[VERSION]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"This is a read-only field used to convey the specific configured CryptoKeyVersion of kms_key that has been configured. It will be populated in cases where the CMEK settings are bound to a single key version.If this field is populated, the kms_key is tied to a specific CryptoKeyVersion. |
| `kms_key_name` | String | Optional. The resource name for the configured Cloud KMS key.KMS key name format: "projects/[PROJECT_ID]/locations/[LOCATION]/keyRings/[KEYRING]/cryptoKeys/[KEY]" For example:"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"To enable CMEK for the Log Router, set this field to a valid kms_key_name for which the associated service account has the needed cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.The Cloud KMS key used by the Log Router can be updated by changing the kms_key_name to a new valid key name or disabled by setting the key name to an empty string. Encryption operations that are in progress will be completed with the key that was in use when they started. Decryption operations will be completed using the key that was used at the time of encryption unless access to that key has been revoked.To disable CMEK for the Log Router, set this field to an empty string.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |
| `name` | String | Output only. The resource name of the CMEK settings. |
| `service_account_id` | String | Output only. The service account that will be used by the Log Router to access your Cloud KMS key.Before enabling CMEK for Log Router, you must first assign the cloudkms.cryptoKeyEncrypterDecrypter role to the service account that the Log Router will use to access your Cloud KMS key. Use GetCmekSettings to obtain the service account ID.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access folder outputs
folder_id = folder.id
folder_kms_key_version_name = folder.kms_key_version_name
folder_kms_key_name = folder.kms_key_name
folder_name = folder.name
folder_service_account_id = folder.service_account_id
```

---


### Log

Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_names` | Vec<String> | A list of log names. For example, "projects/my-project/logs/syslog" or "organizations/123/logs/cloudresourcemanager.googleapis.com%2Factivity". |
| `next_page_token` | String | If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access log outputs
log_id = log.id
log_log_names = log.log_names
log_next_page_token = log.next_page_token
```

---


### Metric

Creates a logs-based metric.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket_options` | String |  | Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values. |
| `metric_descriptor` | String |  | Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description. |
| `bucket_name` | String |  | Optional. The resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects are supported. The bucket has to be in the same project as the metric.For example:projects/my-project/locations/global/buckets/my-bucketIf empty, then the Log Metric is considered a non-Bucket Log Metric. |
| `name` | String |  | Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.This field is the [METRIC_ID] part of a metric resource name in the format "projects/PROJECT_ID/metrics/METRIC_ID". Example: If the resource name of a metric is "projects/my-project/metrics/nginx%2Frequests", this field's value is "nginx/requests". |
| `resource_name` | String |  | Output only. The resource name of the metric: "projects/[PROJECT_ID]/metrics/[METRIC_ID]"  |
| `filter` | String |  | Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced_filters) which is used to match log entries. Example: "resource.type=gae_app AND severity>=ERROR" The maximum length of the filter is 20000 characters. |
| `update_time` | String |  | Output only. The last update timestamp of the metric.This field may not be present for older metrics. |
| `version` | String |  | Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed. |
| `value_extractor` | String |  | Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The arguments are: field: The name of the log entry field from which the value is to be extracted. regex: A regular expression using the Google RE2 syntax (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified log entry field. The value of the field is converted to a string before applying the regex. It is an error to specify a regex that does not include exactly one capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*") |
| `label_extractors` | HashMap<String, String> |  | Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project. |
| `description` | String |  | Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters. |
| `create_time` | String |  | Output only. The creation timestamp of the metric.This field may not be present for older metrics. |
| `disabled` | bool |  | Optional. If set to True, then this metric is disabled and it does not generate any points. |
| `parent` | String | ✅ | Required. The resource name of the project in which to create the metric: "projects/[PROJECT_ID]" The new metric must be provided in the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket_options` | String | Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values. |
| `metric_descriptor` | String | Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description. |
| `bucket_name` | String | Optional. The resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects are supported. The bucket has to be in the same project as the metric.For example:projects/my-project/locations/global/buckets/my-bucketIf empty, then the Log Metric is considered a non-Bucket Log Metric. |
| `name` | String | Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.This field is the [METRIC_ID] part of a metric resource name in the format "projects/PROJECT_ID/metrics/METRIC_ID". Example: If the resource name of a metric is "projects/my-project/metrics/nginx%2Frequests", this field's value is "nginx/requests". |
| `resource_name` | String | Output only. The resource name of the metric: "projects/[PROJECT_ID]/metrics/[METRIC_ID]"  |
| `filter` | String | Required. An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced_filters) which is used to match log entries. Example: "resource.type=gae_app AND severity>=ERROR" The maximum length of the filter is 20000 characters. |
| `update_time` | String | Output only. The last update timestamp of the metric.This field may not be present for older metrics. |
| `version` | String | Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed. |
| `value_extractor` | String | Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The arguments are: field: The name of the log entry field from which the value is to be extracted. regex: A regular expression using the Google RE2 syntax (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified log entry field. The value of the field is converted to a string before applying the regex. It is an error to specify a regex that does not include exactly one capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*") |
| `label_extractors` | HashMap<String, String> | Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project. |
| `description` | String | Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters. |
| `create_time` | String | Output only. The creation timestamp of the metric.This field may not be present for older metrics. |
| `disabled` | bool | Optional. If set to True, then this metric is disabled and it does not generate any points. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metric
metric = provider.logging_api.Metric {
    parent = "value"  # Required. The resource name of the project in which to create the metric: "projects/[PROJECT_ID]" The new metric must be provided in the request.
}

# Access metric outputs
metric_id = metric.id
metric_bucket_options = metric.bucket_options
metric_metric_descriptor = metric.metric_descriptor
metric_bucket_name = metric.bucket_name
metric_name = metric.name
metric_resource_name = metric.resource_name
metric_filter = metric.filter
metric_update_time = metric.update_time
metric_version = metric.version
metric_value_extractor = metric.value_extractor
metric_label_extractors = metric.label_extractors
metric_description = metric.description
metric_create_time = metric.create_time
metric_disabled = metric.disabled
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
entrie_0 = provider.logging_api.Entrie {
}
entrie_1 = provider.logging_api.Entrie {
}
entrie_2 = provider.logging_api.Entrie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entrie = provider.logging_api.Entrie {
    }
```

---

## Related Documentation

- [GCP Logging_api Documentation](https://cloud.google.com/logging_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
