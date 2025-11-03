# Doubleclicksearch_api Service



**Resources**: 3

---

## Overview

The doubleclicksearch_api service provides access to 3 resource types:

- [Saved_column](#saved_column) [R]
- [Conversion](#conversion) [CRU]
- [Report](#report) [CR]

---

## Resources


### Saved_column

Retrieve the list of saved columns for a specified advertiser.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The saved columns being requested. |
| `kind` | String | Identifies this as a SavedColumnList resource. Value: the fixed string doubleclicksearch#savedColumnList. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved_column outputs
saved_column_id = saved_column.id
saved_column_items = saved_column.items
saved_column_kind = saved_column.kind
```

---


### Conversion

Inserts a batch of new conversions into DoubleClick Search.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies this as a ConversionList resource. Value: the fixed string doubleclicksearch#conversionList. |
| `conversion` | Vec<String> |  | The conversions being requested. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies this as a ConversionList resource. Value: the fixed string doubleclicksearch#conversionList. |
| `conversion` | Vec<String> | The conversions being requested. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion
conversion = provider.doubleclicksearch_api.Conversion {
}

# Access conversion outputs
conversion_id = conversion.id
conversion_kind = conversion.kind
conversion_conversion = conversion.conversion
```

---


### Report

Inserts a report request into the reporting system.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_row` | i64 |  | Synchronous report only. Zero-based index of the first row to return. Acceptable values are `0` to `50000`, inclusive. Defaults to `0`. |
| `statistics_currency` | String |  | Specifies the currency in which monetary will be returned. Possible values are: `usd`, `agency` (valid if the report is scoped to agency or lower), `advertiser` (valid if the report is scoped to * advertiser or lower), or `account` (valid if the report is scoped to engine account or lower). |
| `time_range` | String |  | If metrics are requested in a report, this argument will be used to restrict the metrics to a specific time range. |
| `filters` | Vec<String> |  | A list of filters to be applied to the report.\ The maximum number of filters per request is 300. |
| `order_by` | Vec<String> |  | Synchronous report only. A list of columns and directions defining sorting to be performed on the report rows.\ The maximum number of orderings per request is 300. |
| `verify_single_time_zone` | bool |  | If `true`, the report would only be created if all the requested stat data are sourced from a single timezone. Defaults to `false`. |
| `download_format` | String |  | Format that the report should be returned in. Currently `csv` or `tsv` is supported. |
| `include_removed_entities` | bool |  | Determines if removed entities should be included in the report. Defaults to `false`. |
| `max_rows_per_file` | i64 |  | Asynchronous report only. The maximum number of rows per report file. A large report is split into many files based on this field. Acceptable values are `1000000` to `100000000`, inclusive. |
| `columns` | Vec<String> |  | The columns to include in the report. This includes both DoubleClick Search columns and saved columns. For DoubleClick Search columns, only the `columnName` parameter is required. For saved columns only the `savedColumnName` parameter is required. Both `columnName` and `savedColumnName` cannot be set in the same stanza.\ The maximum number of columns per request is 300. |
| `report_scope` | String |  | The reportScope is a set of IDs that are used to determine which subset of entities will be returned in the report. The full lineage of IDs from the lowest scoped level desired up through agency is required. |
| `report_type` | String |  | Determines the type of rows that are returned in the report. For example, if you specify `reportType: keyword`, each row in the report will contain data about a keyword. See the [Types of Reports](/search-ads/v2/report-types/) reference for the columns that are available for each type. |
| `include_deleted_entities` | bool |  | Determines if removed entities should be included in the report. Defaults to `false`. Deprecated, please use `includeRemovedEntities` instead. |
| `row_count` | i64 |  | Synchronous report only. The maximum number of rows to return; additional rows are dropped. Acceptable values are `0` to `10000`, inclusive. Defaults to `10000`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies this as a Report resource. Value: the fixed string `doubleclicksearch#report`. |
| `statistics_time_zone` | String | If all statistics of the report are sourced from the same time zone, this would be it. Otherwise the field is unset. |
| `statistics_currency_code` | String | The currency code of all monetary values produced in the report, including values that are set by users (e.g., keyword bid settings) and metrics (e.g., cost and revenue). The currency code of a report is determined by the `statisticsCurrency` field of the report request. |
| `files` | Vec<String> | Asynchronous report only. Contains a list of generated report files once the report has successfully completed. |
| `id` | String | Asynchronous report only. Id of the report. |
| `is_report_ready` | bool | Asynchronous report only. True if and only if the report has completed successfully and the report files are ready to be downloaded. |
| `row_count` | i64 | The number of report rows generated by the report, not including headers. |
| `rows` | Vec<HashMap<String, String>> | Synchronous report only. Generated report rows. |
| `request` | String | The request that created the report. Optional fields not specified in the original request are filled with default values. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.doubleclicksearch_api.Report {
}

# Access report outputs
report_id = report.id
report_kind = report.kind
report_statistics_time_zone = report.statistics_time_zone
report_statistics_currency_code = report.statistics_currency_code
report_files = report.files
report_id = report.id
report_is_report_ready = report.is_report_ready
report_row_count = report.row_count
report_rows = report.rows
report_request = report.request
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple saved_column resources
saved_column_0 = provider.doubleclicksearch_api.Saved_column {
}
saved_column_1 = provider.doubleclicksearch_api.Saved_column {
}
saved_column_2 = provider.doubleclicksearch_api.Saved_column {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    saved_column = provider.doubleclicksearch_api.Saved_column {
    }
```

---

## Related Documentation

- [GCP Doubleclicksearch_api Documentation](https://cloud.google.com/doubleclicksearch_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
