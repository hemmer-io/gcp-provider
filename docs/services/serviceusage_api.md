# Serviceusage_api Service



**Resources**: 8

---

## Overview

The serviceusage_api service provides access to 8 resource types:

- [Operation](#operation) [CRD]
- [Service](#service) [CR]
- [Admin_override](#admin_override) [CRUD]
- [Service](#service) [CR]
- [Consumer_quota_metric](#consumer_quota_metric) [CR]
- [Operation](#operation) [R]
- [Consumer_override](#consumer_override) [CRUD]
- [Limit](#limit) [R]

---

## Resources


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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.serviceusage_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Service

Enable a service so that it can be used with a project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Name of the consumer and service to enable the service on. The `EnableService` and `DisableService` methods currently only support projects. Enabling a service requires that the service is public or is shared with the user enabling the service. An example name would be: `projects/123/services/serviceusage.googleapis.com` where `123` is the project number. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the consumer and service. A valid name would be: - projects/123/services/serviceusage.googleapis.com |
| `state` | String | Whether or not the service has been enabled for use by the consumer. |
| `config` | String | The service configuration of the available service. Some fields may be filtered out of the configuration in responses to the `ListServices` method. These fields are present only in responses to the `GetService` method. |
| `parent` | String | The resource name of the consumer. A valid name would be: - projects/123 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.serviceusage_api.Service {
    name = "value"  # Name of the consumer and service to enable the service on. The `EnableService` and `DisableService` methods currently only support projects. Enabling a service requires that the service is public or is shared with the user enabling the service. An example name would be: `projects/123/services/serviceusage.googleapis.com` where `123` is the project number.
}

# Access service outputs
service_id = service.id
service_name = service.name
service_state = service.state
service_config = service.config
service_parent = service.parent
```

---


### Admin_override

Creates an admin override. An admin override is applied by an administrator of a parent folder or parent organization of the consumer receiving the override. An admin override is intended to limit the amount of quota the consumer can use out of the total quota pool allocated to all children of the folder or organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unit` | String |  | The limit unit of the limit to which this override applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `override_value` | String |  | The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota). |
| `admin_override_ancestor` | String |  | The resource name of the ancestor that requested the override. For example: `organizations/12345` or `folders/67890`. Used by admin overrides only. |
| `metric` | String |  | The name of the metric to which this override applies. An example name would be: `compute.googleapis.com/cpus` |
| `name` | String |  | The resource name of the override. This name is generated by the server when the override is created. Example names would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d` `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `dimensions` | HashMap<String, String> |  | If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit. For example, an override on a limit with the unit `1/{project}/{region}` could contain an entry with the key `region` and the value `us-east-1`; the override is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in `{brackets}` in the unit (besides `{project}` or `{user}`) is a defined key. * `project` is not a valid key; the project is already specified in the parent resource name. * `user` is not a valid key; the API does not support quota overrides that apply only to a specific user. * If `region` appears as a key, its value must be a valid Cloud region. * If `zone` appears as a key, its value must be a valid Cloud zone. * If any valid key other than `region` or `zone` appears in the map, then all valid keys other than `region` or `zone` must also appear in the map. |
| `parent` | String | ✅ | The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token identifying which result to start with; returned by a previous list call. |
| `overrides` | Vec<String> | Admin overrides on this limit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create admin_override
admin_override = provider.serviceusage_api.Admin_override {
    parent = "value"  # The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
}

# Access admin_override outputs
admin_override_id = admin_override.id
admin_override_next_page_token = admin_override.next_page_token
admin_override_overrides = admin_override.overrides
```

---


### Service

Generates service identity for service.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Name of the consumer and service to generate an identity for. The `GenerateServiceIdentity` methods currently support projects, folders, organizations. Example parents would be: `projects/123/services/example.googleapis.com` `folders/123/services/example.googleapis.com` `organizations/123/services/example.googleapis.com` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the consumer and service. A valid name would be: - `projects/123/services/serviceusage.googleapis.com` |
| `config` | String | The service configuration of the available service. Some fields may be filtered out of the configuration in responses to the `ListServices` method. These fields are present only in responses to the `GetService` method. |
| `parent` | String | The resource name of the consumer. A valid name would be: - `projects/123` |
| `state` | String | Whether or not the service has been enabled for use by the consumer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.serviceusage_api.Service {
    parent = "value"  # Name of the consumer and service to generate an identity for. The `GenerateServiceIdentity` methods currently support projects, folders, organizations. Example parents would be: `projects/123/services/example.googleapis.com` `folders/123/services/example.googleapis.com` `organizations/123/services/example.googleapis.com`
}

# Access service outputs
service_id = service.id
service_name = service.name
service_config = service.config
service_parent = service.parent
service_state = service.state
```

---


### Consumer_quota_metric

Creates or updates multiple admin overrides atomically, all on the same consumer, but on many different metrics or limits. The name field in the quota override message should not be set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force` | bool |  | Whether to force the creation of the quota overrides. Setting the force parameter to 'true' ignores all quota safety checks that would fail the request. QuotaSafetyCheck lists all such validations. If force is set to true, it is recommended to include a case id in "X-Goog-Request-Reason" header when sending the request. |
| `force_only` | Vec<String> |  | The list of quota safety checks to ignore before the override mutation. Unlike 'force' field that ignores all the quota safety checks, the 'force_only' field ignores only the specified checks; other checks are still enforced. The 'force' and 'force_only' fields cannot both be set. If force_only is specified, it is recommended to include a case id in "X-Goog-Request-Reason" header when sending the request. |
| `inline_source` | String |  | The import data is specified in the request message itself |
| `parent` | String | ✅ | The resource name of the consumer. An example name would be: `projects/123/services/compute.googleapis.com` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric` | String | The name of the metric. An example name would be: `compute.googleapis.com/cpus` |
| `consumer_quota_limits` | Vec<String> | The consumer quota for each quota limit defined on the metric. |
| `unit` | String | The units in which the metric value is reported. |
| `descendant_consumer_quota_limits` | Vec<String> | The quota limits targeting the descendant containers of the consumer in request. If the consumer in request is of type `organizations` or `folders`, the field will list per-project limits in the metric; if the consumer in request is of type `project`, the field will be empty. The `quota_buckets` field of each descendant consumer quota limit will not be populated. |
| `name` | String | The resource name of the quota settings on this metric for this consumer. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `display_name` | String | The display name of the metric. An example name would be: `CPUs` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consumer_quota_metric
consumer_quota_metric = provider.serviceusage_api.Consumer_quota_metric {
    parent = "value"  # The resource name of the consumer. An example name would be: `projects/123/services/compute.googleapis.com`
}

# Access consumer_quota_metric outputs
consumer_quota_metric_id = consumer_quota_metric.id
consumer_quota_metric_metric = consumer_quota_metric.metric
consumer_quota_metric_consumer_quota_limits = consumer_quota_metric.consumer_quota_limits
consumer_quota_metric_unit = consumer_quota_metric.unit
consumer_quota_metric_descendant_consumer_quota_limits = consumer_quota_metric.descendant_consumer_quota_limits
consumer_quota_metric_name = consumer_quota_metric.name
consumer_quota_metric_display_name = consumer_quota_metric.display_name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
```

---


### Consumer_override

Creates a consumer override. A consumer override is applied to the consumer on its own authority to limit its own quota usage. Consumer overrides cannot be used to grant more quota than would be allowed by admin overrides, producer overrides, or the default limit of the service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unit` | String |  | The limit unit of the limit to which this override applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `override_value` | String |  | The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota). |
| `admin_override_ancestor` | String |  | The resource name of the ancestor that requested the override. For example: `organizations/12345` or `folders/67890`. Used by admin overrides only. |
| `metric` | String |  | The name of the metric to which this override applies. An example name would be: `compute.googleapis.com/cpus` |
| `name` | String |  | The resource name of the override. This name is generated by the server when the override is created. Example names would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d` `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `dimensions` | HashMap<String, String> |  | If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit. For example, an override on a limit with the unit `1/{project}/{region}` could contain an entry with the key `region` and the value `us-east-1`; the override is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in `{brackets}` in the unit (besides `{project}` or `{user}`) is a defined key. * `project` is not a valid key; the project is already specified in the parent resource name. * `user` is not a valid key; the API does not support quota overrides that apply only to a specific user. * If `region` appears as a key, its value must be a valid Cloud region. * If `zone` appears as a key, its value must be a valid Cloud zone. * If any valid key other than `region` or `zone` appears in the map, then all valid keys other than `region` or `zone` must also appear in the map. |
| `parent` | String | ✅ | The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token identifying which result to start with; returned by a previous list call. |
| `overrides` | Vec<String> | Consumer overrides on this limit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consumer_override
consumer_override = provider.serviceusage_api.Consumer_override {
    parent = "value"  # The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
}

# Access consumer_override outputs
consumer_override_id = consumer_override.id
consumer_override_next_page_token = consumer_override.next_page_token
consumer_override_overrides = consumer_override.overrides
```

---


### Limit

Retrieves a summary of quota information for a specific quota limit.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unit` | String | The limit unit. An example unit would be `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `allows_admin_overrides` | bool | Whether admin overrides are allowed on this limit |
| `metric` | String | The name of the parent metric of this limit. An example name would be: `compute.googleapis.com/cpus` |
| `quota_buckets` | Vec<String> | Summary of the enforced quota buckets, organized by quota dimension, ordered from least specific to most specific (for example, the global default bucket, with no quota dimensions, will always appear first). |
| `name` | String | The resource name of the quota limit. An example name would be: `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `supported_locations` | Vec<String> | List of all supported locations. This field is present only if the limit has a {region} or {zone} dimension. |
| `is_precise` | bool | Whether this limit is precise or imprecise. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access limit outputs
limit_id = limit.id
limit_unit = limit.unit
limit_allows_admin_overrides = limit.allows_admin_overrides
limit_metric = limit.metric
limit_quota_buckets = limit.quota_buckets
limit_name = limit.name
limit_supported_locations = limit.supported_locations
limit_is_precise = limit.is_precise
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
operation_0 = provider.serviceusage_api.Operation {
    name = "value-0"
}
operation_1 = provider.serviceusage_api.Operation {
    name = "value-1"
}
operation_2 = provider.serviceusage_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.serviceusage_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Serviceusage_api Documentation](https://cloud.google.com/serviceusage_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
