# Serviceconsumermanagement_api Service



**Resources**: 8

---

## Overview

The serviceconsumermanagement_api service provides access to 8 resource types:

- [Tenancy_unit](#tenancy_unit) [CRD]
- [Operation](#operation) [CRD]
- [Service](#service) [R]
- [Operation](#operation) [R]
- [Limit](#limit) [R]
- [Producer_quota_policie](#producer_quota_policie) [CRUD]
- [Consumer_quota_metric](#consumer_quota_metric) [CR]
- [Producer_override](#producer_override) [CRUD]

---

## Resources


### Tenancy_unit

Creates a tenancy unit with no tenant resources. If tenancy unit already exists, it will be returned, however, in this case, returned TenancyUnit does not have tenant_resources field set and ListTenancyUnits has to be used to get a complete TenancyUnit with all fields populated.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tenancy_unit_id` | String |  | Optional. Optional service producer-provided identifier of the tenancy unit. Must be no longer than 40 characters and preferably URI friendly. If it isn't provided, a UID for the tenancy unit is automatically generated. The identifier must be unique across a managed service. If the tenancy unit already exists for the managed service and service consumer pair, calling `CreateTenancyUnit` returns the existing tenancy unit if the provided identifier is identical or empty, otherwise the call fails. |
| `parent` | String | ✅ | Required. services/{service}/{collection id}/{resource id} {collection id} is the cloud resource collection type representing the service consumer, for example 'projects', or 'organizations'. {resource id} is the consumer numeric id, such as project number: '123456'. {service} the name of a managed service, such as 'service.googleapis.com'. Enables service binding using the new tenancy unit. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token for large results. |
| `tenancy_units` | Vec<String> | Tenancy units matching the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tenancy_unit
tenancy_unit = provider.serviceconsumermanagement_api.Tenancy_unit {
    parent = "value"  # Required. services/{service}/{collection id}/{resource id} {collection id} is the cloud resource collection type representing the service consumer, for example 'projects', or 'organizations'. {resource id} is the consumer numeric id, such as project number: '123456'. {service} the name of a managed service, such as 'service.googleapis.com'. Enables service binding using the new tenancy unit.
}

# Access tenancy_unit outputs
tenancy_unit_id = tenancy_unit.id
tenancy_unit_next_page_token = tenancy_unit.next_page_token
tenancy_unit_tenancy_units = tenancy_unit.tenancy_units
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.serviceconsumermanagement_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Service

Search tenancy units for a managed service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tenancy_units` | Vec<String> | Tenancy Units matching the request. |
| `next_page_token` | String | Pagination token for large results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_tenancy_units = service.tenancy_units
service_next_page_token = service.next_page_token
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
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
| `unit` | String | The limit unit. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `metric` | String | The name of the parent metric of this limit. An example name would be: `compute.googleapis.com/cpus` |
| `name` | String | The resource name of the quota limit. An example name would be: `services/compute.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `is_precise` | bool | Whether this limit is precise or imprecise. |
| `supported_locations` | Vec<String> | List of all supported locations. This field is present only if the limit has a {region} or {zone} dimension. |
| `quota_buckets` | Vec<String> | Summary of the enforced quota buckets, organized by quota dimension, ordered from least specific to most specific (for example, the global default bucket, with no quota dimensions, will always appear first). |


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
limit_metric = limit.metric
limit_name = limit.name
limit_is_precise = limit.is_precise
limit_supported_locations = limit.supported_locations
limit_quota_buckets = limit.quota_buckets
```

---


### Producer_quota_policie

Creates a producer quota policy. A producer quota policy is applied by the owner or administrator of a service at an org or folder node to set the default quota limit for all consumers under the node where the policy is created. To create multiple policies at once, use ImportProducerQuotaPolicies instead. If a policy with the specified dimensions already exists, this call will fail. To overwrite an existing policy if one is already present ("upsert" semantics), use ImportProducerQuotaPolicies instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name of the producer policy. An example name would be: `services/compute.googleapis.com/organizations/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/producerQuotaPolicies/4a3f2c1d` |
| `metric` | String |  | The name of the metric to which this policy applies. An example name would be: `compute.googleapis.com/cpus` |
| `container` | String |  | The cloud resource container at which the quota policy is created. The format is {container_type}/{container_number} |
| `policy_value` | String |  | The quota policy value. Can be any nonnegative integer, or -1 (unlimited quota). |
| `unit` | String |  | The limit unit of the limit to which this policy applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `dimensions` | HashMap<String, String> |  |  If this map is nonempty, then this policy applies only to specific values for dimensions defined in the limit unit. For example, a policy on a limit with the unit 1/{project}/{region} could contain an entry with the key "region" and the value "us-east-1"; the policy is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in {brackets} in the unit (besides {project} or {user}) is a defined key. * "project" is not a valid key; the project is already specified in the parent resource name. * "user" is not a valid key; the API does not support quota polcies that apply only to a specific user. * If "region" appears as a key, its value must be a valid Cloud region. * If "zone" appears as a key, its value must be a valid Cloud zone. * If any valid key other than "region" or "zone" appears in the map, then all valid keys other than "region" or "zone" must also appear in the map. |
| `parent` | String | ✅ | Required. The resource name of the parent quota limit. An example name would be: `services/compute.googleapis.com/organizations/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `producer_quota_policies` | Vec<String> | Producer policies on this limit. |
| `next_page_token` | String | Token identifying which result to start with; returned by a previous list call. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create producer_quota_policie
producer_quota_policie = provider.serviceconsumermanagement_api.Producer_quota_policie {
    parent = "value"  # Required. The resource name of the parent quota limit. An example name would be: `services/compute.googleapis.com/organizations/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
}

# Access producer_quota_policie outputs
producer_quota_policie_id = producer_quota_policie.id
producer_quota_policie_producer_quota_policies = producer_quota_policie.producer_quota_policies
producer_quota_policie_next_page_token = producer_quota_policie.next_page_token
```

---


### Consumer_quota_metric

Create or update multiple producer quota policies atomically, all on the same ancestor, but on many different metrics or limits. The name field in the quota policy message should not be set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force_skip_quota_usage_check` | bool |  | If set to true, skip the quota usage check. This field is only used when the effective limit can be decreased. If the force field is not set, this field will be ignored. |
| `inline_source` | String |  | The import data is specified in the request message itself |
| `validate_only` | bool |  | If set to true, validate the request, but do not actually update. |
| `force` | bool |  | Whether quota policy can result in a decrease of effective limit. Don't allow any decreases if force is not specified. If force is specified, then don't allow any decreases below 120% of the 7d quota usage, or for cases where usage cannot be examined (custom dimensions/ per user/per resource), only allow a 10% decrease. |
| `force_justification` | String |  | If force or force_skip_quota_usage_check option is set to true, force_justification is suggested to be set to log the reason in audit logs. |
| `parent` | String | ✅ | The resource name of the consumer. An example name would be: `services/compute.googleapis.com/organizations/123` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumer_quota_limits` | Vec<String> | The consumer quota for each quota limit defined on the metric. |
| `descendant_consumer_quota_limits` | Vec<String> | The quota limits targeting the descendant containers of the consumer in request. If the consumer in request is of type `organizations` or `folders`, the field will list per-project limits in the metric; if the consumer in request is of type `project`, the field will be empty. The `quota_buckets` field of each descendant consumer quota limit will not be populated. |
| `metric` | String | The name of the metric. An example name would be: `compute.googleapis.com/cpus` |
| `display_name` | String | The display name of the metric. An example name would be: "CPUs" |
| `name` | String | The resource name of the quota settings on this metric for this consumer. An example name would be: `services/serviceconsumermanagement.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus` The resource name is intended to be opaque and should not be parsed for its component strings, since its representation could change in the future. |
| `unit` | String | The units in which the metric value is reported. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consumer_quota_metric
consumer_quota_metric = provider.serviceconsumermanagement_api.Consumer_quota_metric {
    parent = "value"  # The resource name of the consumer. An example name would be: `services/compute.googleapis.com/organizations/123`
}

# Access consumer_quota_metric outputs
consumer_quota_metric_id = consumer_quota_metric.id
consumer_quota_metric_consumer_quota_limits = consumer_quota_metric.consumer_quota_limits
consumer_quota_metric_descendant_consumer_quota_limits = consumer_quota_metric.descendant_consumer_quota_limits
consumer_quota_metric_metric = consumer_quota_metric.metric
consumer_quota_metric_display_name = consumer_quota_metric.display_name
consumer_quota_metric_name = consumer_quota_metric.name
consumer_quota_metric_unit = consumer_quota_metric.unit
```

---


### Producer_override

Creates a producer override. A producer override is applied by the owner or administrator of a service to increase or decrease the amount of quota a consumer of the service is allowed to use. To create multiple overrides at once, use ImportProducerOverrides instead. If an override with the specified dimensions already exists, this call will fail. To overwrite an existing override if one is already present ("upsert" semantics), use ImportProducerOverrides instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric` | String |  | The name of the metric to which this override applies. An example name would be: `compute.googleapis.com/cpus` |
| `admin_override_ancestor` | String |  | The resource name of the ancestor that requested the override. For example: "organizations/12345" or "folders/67890". Used by admin overrides only. |
| `dimensions` | HashMap<String, String> |  |  If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit. For example, an override on a limit with the unit 1/{project}/{region} could contain an entry with the key "region" and the value "us-east-1"; the override is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in {brackets} in the unit (besides {project} or {user}) is a defined key. * "project" is not a valid key; the project is already specified in the parent resource name. * "user" is not a valid key; the API does not support quota overrides that apply only to a specific user. * If "region" appears as a key, its value must be a valid Cloud region. * If "zone" appears as a key, its value must be a valid Cloud zone. * If any valid key other than "region" or "zone" appears in the map, then all valid keys other than "region" or "zone" must also appear in the map. |
| `name` | String |  | The resource name of the producer override. An example name would be: `services/compute.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/producerOverrides/4a3f2c1d` |
| `override_value` | String |  | The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota). |
| `unit` | String |  | The limit unit of the limit to which this override applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string. |
| `parent` | String | ✅ | The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `services/compute.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `overrides` | Vec<String> | Producer overrides on this limit. |
| `next_page_token` | String | Token identifying which result to start with; returned by a previous list call. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create producer_override
producer_override = provider.serviceconsumermanagement_api.Producer_override {
    parent = "value"  # The resource name of the parent quota limit, returned by a ListConsumerQuotaMetrics or GetConsumerQuotaMetric call. An example name would be: `services/compute.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
}

# Access producer_override outputs
producer_override_id = producer_override.id
producer_override_overrides = producer_override.overrides
producer_override_next_page_token = producer_override.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple tenancy_unit resources
tenancy_unit_0 = provider.serviceconsumermanagement_api.Tenancy_unit {
    parent = "value-0"
}
tenancy_unit_1 = provider.serviceconsumermanagement_api.Tenancy_unit {
    parent = "value-1"
}
tenancy_unit_2 = provider.serviceconsumermanagement_api.Tenancy_unit {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tenancy_unit = provider.serviceconsumermanagement_api.Tenancy_unit {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Serviceconsumermanagement_api Documentation](https://cloud.google.com/serviceconsumermanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
