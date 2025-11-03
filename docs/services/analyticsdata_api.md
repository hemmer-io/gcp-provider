# Analyticsdata_api Service



**Resources**: 4

---

## Overview

The analyticsdata_api service provides access to 4 resource types:

- [Propertie](#propertie) [CR]
- [Analyticsdata](#analyticsdata) [C]
- [Audience_export](#audience_export) [CR]
- [Propertie](#propertie) [CR]

---

## Resources


### Propertie

The Google Analytics Realtime API returns a customized report of realtime event data for your property. These reports show events and usage from the last 30 minutes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric_aggregations` | Vec<String> |  | Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)". |
| `metric_filter` | String |  | The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter. |
| `order_bys` | Vec<String> |  | Specifies how rows are ordered in the response. |
| `limit` | String |  | The number of rows to return. If the `limit` parameter is unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. |
| `dimension_filter` | String |  | The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter. |
| `return_property_quota` | bool |  | Toggles whether to return the current state of this Analytics Property's Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota). |
| `metrics` | Vec<String> |  | The metrics requested and displayed. |
| `dimensions` | Vec<String> |  | The dimensions requested and displayed. |
| `property` | String | ✅ | A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | Vec<String> | The metric descriptions. |
| `name` | String | Resource name of this metadata. |
| `dimensions` | Vec<String> | The dimension descriptions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create propertie
propertie = provider.analyticsdata_api.Propertie {
    property = "value"  # A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234
}

# Access propertie outputs
propertie_id = propertie.id
propertie_metrics = propertie.metrics
propertie_name = propertie.name
propertie_dimensions = propertie.dimensions
```

---


### Analyticsdata

Returns multiple pivot reports in a batch. All reports must be for the same Entity.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Individual requests. Each request has a separate pivot report response. Each batch request is allowed up to 5 requests. |
| `entity` | String |  | A property whose events are tracked. This entity must be specified for the batch. The entity within RunPivotReportRequest may either be unspecified or consistent with this entity. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create analyticsdata
analyticsdata = provider.analyticsdata_api.Analyticsdata {
}

```

---


### Audience_export

Creates an audience export for later retrieval. This method quickly returns the audience export's resource name and initiates a long running asynchronous request to form an audience export. To export the users in an audience export, first create the audience export through this method and then send the audience resource name to the `QueryAudienceExport` method. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. An audience export is a snapshot of the users currently in the audience at the time of audience export creation. Creating audience exports for one audience on different days will return different results as users enter and exit the audience. Audiences in Google Analytics 4 allow you to segment your users in the ways that are important to your business. To learn more, see https://support.google.com/analytics/answer/9267572. Audience exports contain the users in each audience. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error_message` | String |  | Output only. Error message is populated when an audience export fails during creation. A common reason for such a failure is quota exhaustion. |
| `audience_display_name` | String |  | Output only. The descriptive display name for this audience. For example, "Purchasers". |
| `creation_quota_tokens_charged` | i64 |  | Output only. The total quota tokens charged during creation of the AudienceExport. Because this token count is based on activity from the `CREATING` state, this tokens charged will be fixed once an AudienceExport enters the `ACTIVE` or `FAILED` states. |
| `dimensions` | Vec<String> |  | Required. The dimensions requested and displayed in the query response. |
| `audience` | String |  | Required. The audience resource name. This resource name identifies the audience being listed and is shared between the Analytics Data & Admin APIs. Format: `properties/{property}/audiences/{audience}` |
| `row_count` | i64 |  | Output only. The total number of rows in the AudienceExport result. |
| `begin_creating_time` | String |  | Output only. The time when CreateAudienceExport was called and the AudienceExport began the `CREATING` state. |
| `percentage_completed` | f64 |  | Output only. The percentage completed for this audience export ranging between 0 to 100. |
| `name` | String |  | Output only. Identifier. The audience export resource name assigned during creation. This resource name identifies this `AudienceExport`. Format: `properties/{property}/audienceExports/{audience_export}` |
| `state` | String |  | Output only. The current state for this AudienceExport. |
| `parent` | String | ✅ | Required. The parent resource where this audience export will be created. Format: `properties/{property}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_message` | String | Output only. Error message is populated when an audience export fails during creation. A common reason for such a failure is quota exhaustion. |
| `audience_display_name` | String | Output only. The descriptive display name for this audience. For example, "Purchasers". |
| `creation_quota_tokens_charged` | i64 | Output only. The total quota tokens charged during creation of the AudienceExport. Because this token count is based on activity from the `CREATING` state, this tokens charged will be fixed once an AudienceExport enters the `ACTIVE` or `FAILED` states. |
| `dimensions` | Vec<String> | Required. The dimensions requested and displayed in the query response. |
| `audience` | String | Required. The audience resource name. This resource name identifies the audience being listed and is shared between the Analytics Data & Admin APIs. Format: `properties/{property}/audiences/{audience}` |
| `row_count` | i64 | Output only. The total number of rows in the AudienceExport result. |
| `begin_creating_time` | String | Output only. The time when CreateAudienceExport was called and the AudienceExport began the `CREATING` state. |
| `percentage_completed` | f64 | Output only. The percentage completed for this audience export ranging between 0 to 100. |
| `name` | String | Output only. Identifier. The audience export resource name assigned during creation. This resource name identifies this `AudienceExport`. Format: `properties/{property}/audienceExports/{audience_export}` |
| `state` | String | Output only. The current state for this AudienceExport. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create audience_export
audience_export = provider.analyticsdata_api.Audience_export {
    parent = "value"  # Required. The parent resource where this audience export will be created. Format: `properties/{property}`
}

# Access audience_export outputs
audience_export_id = audience_export.id
audience_export_error_message = audience_export.error_message
audience_export_audience_display_name = audience_export.audience_display_name
audience_export_creation_quota_tokens_charged = audience_export.creation_quota_tokens_charged
audience_export_dimensions = audience_export.dimensions
audience_export_audience = audience_export.audience
audience_export_row_count = audience_export.row_count
audience_export_begin_creating_time = audience_export.begin_creating_time
audience_export_percentage_completed = audience_export.percentage_completed
audience_export_name = audience_export.name
audience_export_state = audience_export.state
```

---


### Propertie

Returns multiple reports in a batch. All reports must be for the same Google Analytics property.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Individual requests. Each request has a separate report response. Each batch request is allowed up to 5 requests. |
| `property` | String | ✅ | A Google Analytics property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | Vec<String> | The metric descriptions. |
| `comparisons` | Vec<String> | The comparison descriptions. |
| `name` | String | Resource name of this metadata. |
| `dimensions` | Vec<String> | The dimension descriptions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create propertie
propertie = provider.analyticsdata_api.Propertie {
    property = "value"  # A Google Analytics property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234
}

# Access propertie outputs
propertie_id = propertie.id
propertie_metrics = propertie.metrics
propertie_comparisons = propertie.comparisons
propertie_name = propertie.name
propertie_dimensions = propertie.dimensions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple propertie resources
propertie_0 = provider.analyticsdata_api.Propertie {
    property = "value-0"
}
propertie_1 = provider.analyticsdata_api.Propertie {
    property = "value-1"
}
propertie_2 = provider.analyticsdata_api.Propertie {
    property = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    propertie = provider.analyticsdata_api.Propertie {
        property = "production-value"
    }
```

---

## Related Documentation

- [GCP Analyticsdata_api Documentation](https://cloud.google.com/analyticsdata_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
