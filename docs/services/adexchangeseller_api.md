# Adexchangeseller_api Service



**Resources**: 27

---

## Overview

The adexchangeseller_api service provides access to 27 resource types:

- [Saved](#saved) [R]
- [Adclient](#adclient) [R]
- [Urlchannel](#urlchannel) [R]
- [Adunit](#adunit) [R]
- [Report](#report) [R]
- [Customchannel](#customchannel) [R]
- [Saved](#saved) [R]
- [Metric](#metric) [R]
- [Account](#account) [R]
- [Urlchannel](#urlchannel) [R]
- [Report](#report) [R]
- [Adclient](#adclient) [R]
- [Preferreddeal](#preferreddeal) [R]
- [Dimension](#dimension) [R]
- [Customchannel](#customchannel) [R]
- [Alert](#alert) [R]
- [Alert](#alert) [R]
- [Dimension](#dimension) [R]
- [Urlchannel](#urlchannel) [R]
- [Preferreddeal](#preferreddeal) [R]
- [Account](#account) [R]
- [Adclient](#adclient) [R]
- [Saved](#saved) [R]
- [Metric](#metric) [R]
- [Report](#report) [R]
- [Adunit](#adunit) [R]
- [Customchannel](#customchannel) [R]

---

## Resources


### Saved

Generate an Ad Exchange report based on the saved report ID sent in the query parameters.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved outputs
saved_id = saved.id
saved_totals = saved.totals
saved_warnings = saved.warnings
saved_rows = saved.rows
saved_headers = saved.headers
saved_averages = saved.averages
saved_kind = saved.kind
saved_total_matched_rows = saved.total_matched_rows
```

---


### Adclient

List all ad clients in this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The ad clients returned in this list response. |
| `etag` | String | ETag of this response for caching purposes. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#adClients. |
| `next_page_token` | String | Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adclient outputs
adclient_id = adclient.id
adclient_items = adclient.items
adclient_etag = adclient.etag
adclient_kind = adclient.kind
adclient_next_page_token = adclient.next_page_token
```

---


### Urlchannel

List all URL channels in the specified ad client for this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of this response for caching purposes. |
| `items` | Vec<String> | The URL channels returned in this list response. |
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#urlChannels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access urlchannel outputs
urlchannel_id = urlchannel.id
urlchannel_etag = urlchannel.etag
urlchannel_items = urlchannel.items
urlchannel_next_page_token = urlchannel.next_page_token
urlchannel_kind = urlchannel.kind
```

---


### Adunit

Gets the specified ad unit in the specified ad client.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of resource this is, in this case adexchangeseller#adUnit. |
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String | Name of this ad unit. |
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adunit outputs
adunit_id = adunit.id
adunit_kind = adunit.kind
adunit_code = adunit.code
adunit_id = adunit.id
adunit_name = adunit.name
adunit_status = adunit.status
```

---


### Report

Generate an Ad Exchange report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_totals = report.totals
report_warnings = report.warnings
report_rows = report.rows
report_headers = report.headers
report_averages = report.averages
report_kind = report.kind
report_total_matched_rows = report.total_matched_rows
```

---


### Customchannel

Get the specified custom channel from the specified ad client.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `code` | String | Code of this custom channel, not necessarily unique across ad clients. |
| `id` | String | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `targeting_info` | String | The targeting information of this custom channel, if activated. |
| `name` | String | Name of this custom channel. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#customChannel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_code = customchannel.code
customchannel_id = customchannel.id
customchannel_targeting_info = customchannel.targeting_info
customchannel_name = customchannel.name
customchannel_kind = customchannel.kind
```

---


### Saved

Generate an Ad Exchange report based on the saved report ID sent in the query parameters.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved outputs
saved_id = saved.id
saved_rows = saved.rows
saved_total_matched_rows = saved.total_matched_rows
saved_totals = saved.totals
saved_warnings = saved.warnings
saved_headers = saved.headers
saved_averages = saved.averages
saved_kind = saved.kind
```

---


### Metric

List the metadata for the metrics available to this AdExchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> |  |
| `kind` | String | Kind of list this is, in this case adexchangeseller#metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access metric outputs
metric_id = metric.id
metric_items = metric.items
metric_kind = metric.kind
```

---


### Account

Get information about the selected Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier of this account. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#account. |
| `name` | String | Name of this account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_id = account.id
account_kind = account.kind
account_name = account.name
```

---


### Urlchannel

List all URL channels in the specified ad client for this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The URL channels returned in this list response. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#urlChannels. |
| `etag` | String | ETag of this response for caching purposes. |
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access urlchannel outputs
urlchannel_id = urlchannel.id
urlchannel_items = urlchannel.items
urlchannel_kind = urlchannel.kind
urlchannel_etag = urlchannel.etag
urlchannel_next_page_token = urlchannel.next_page_token
```

---


### Report

Generate an Ad Exchange report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_rows = report.rows
report_total_matched_rows = report.total_matched_rows
report_totals = report.totals
report_warnings = report.warnings
report_headers = report.headers
report_averages = report.averages
report_kind = report.kind
```

---


### Adclient

List all ad clients in this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of this response for caching purposes. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#adClients. |
| `next_page_token` | String | Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `items` | Vec<String> | The ad clients returned in this list response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adclient outputs
adclient_id = adclient.id
adclient_etag = adclient.etag
adclient_kind = adclient.kind
adclient_next_page_token = adclient.next_page_token
adclient_items = adclient.items
```

---


### Preferreddeal

Get information about the selected Ad Exchange Preferred Deal.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `currency_code` | String | The currency code that applies to the fixed_cpm value. If not set then assumed to be USD. |
| `advertiser_name` | String | The name of the advertiser this deal is for. |
| `end_time` | String | Time when this deal stops being active in seconds since the epoch (GMT). If not set then this deal is valid until manually disabled by the publisher. |
| `fixed_cpm` | String | The fixed price for this preferred deal. In cpm micros of currency according to currencyCode. If set, then this preferred deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price). |
| `buyer_network_name` | String | The name of the buyer network this deal is for. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#preferredDeal. |
| `start_time` | String | Time when this deal becomes active in seconds since the epoch (GMT). If not set then this deal is active immediately upon creation. |
| `id` | String | Unique identifier of this preferred deal. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access preferreddeal outputs
preferreddeal_id = preferreddeal.id
preferreddeal_currency_code = preferreddeal.currency_code
preferreddeal_advertiser_name = preferreddeal.advertiser_name
preferreddeal_end_time = preferreddeal.end_time
preferreddeal_fixed_cpm = preferreddeal.fixed_cpm
preferreddeal_buyer_network_name = preferreddeal.buyer_network_name
preferreddeal_kind = preferreddeal.kind
preferreddeal_start_time = preferreddeal.start_time
preferreddeal_id = preferreddeal.id
```

---


### Dimension

List the metadata for the dimensions available to this AdExchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> |  |
| `kind` | String | Kind of list this is, in this case adexchangeseller#metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dimension outputs
dimension_id = dimension.id
dimension_items = dimension.items
dimension_kind = dimension.kind
```

---


### Customchannel

Get the specified custom channel from the specified ad client.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `targeting_info` | String | The targeting information of this custom channel, if activated. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#customChannel. |
| `id` | String | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String | Name of this custom channel. |
| `code` | String | Code of this custom channel, not necessarily unique across ad clients. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_targeting_info = customchannel.targeting_info
customchannel_kind = customchannel.kind
customchannel_id = customchannel.id
customchannel_name = customchannel.name
customchannel_code = customchannel.code
```

---


### Alert

List the alerts for this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The alerts returned in this list response. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#alerts. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access alert outputs
alert_id = alert.id
alert_items = alert.items
alert_kind = alert.kind
```

---


### Alert

List the alerts for this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The alerts returned in this list response. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#alerts. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access alert outputs
alert_id = alert.id
alert_items = alert.items
alert_kind = alert.kind
```

---


### Dimension

List the metadata for the dimensions available to this AdExchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> |  |
| `kind` | String | Kind of list this is, in this case adexchangeseller#metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dimension outputs
dimension_id = dimension.id
dimension_items = dimension.items
dimension_kind = dimension.kind
```

---


### Urlchannel

List all URL channels in the specified ad client for this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `items` | Vec<String> | The URL channels returned in this list response. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#urlChannels. |
| `etag` | String | ETag of this response for caching purposes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access urlchannel outputs
urlchannel_id = urlchannel.id
urlchannel_next_page_token = urlchannel.next_page_token
urlchannel_items = urlchannel.items
urlchannel_kind = urlchannel.kind
urlchannel_etag = urlchannel.etag
```

---


### Preferreddeal

Get information about the selected Ad Exchange Preferred Deal.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Time when this deal becomes active in seconds since the epoch (GMT). If not set then this deal is active immediately upon creation. |
| `end_time` | String | Time when this deal stops being active in seconds since the epoch (GMT). If not set then this deal is valid until manually disabled by the publisher. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#preferredDeal. |
| `currency_code` | String | The currency code that applies to the fixed_cpm value. If not set then assumed to be USD. |
| `id` | String | Unique identifier of this preferred deal. |
| `buyer_network_name` | String | The name of the buyer network this deal is for. |
| `fixed_cpm` | String | The fixed price for this preferred deal. In cpm micros of currency according to currencyCode. If set, then this preferred deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price). |
| `advertiser_name` | String | The name of the advertiser this deal is for. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access preferreddeal outputs
preferreddeal_id = preferreddeal.id
preferreddeal_start_time = preferreddeal.start_time
preferreddeal_end_time = preferreddeal.end_time
preferreddeal_kind = preferreddeal.kind
preferreddeal_currency_code = preferreddeal.currency_code
preferreddeal_id = preferreddeal.id
preferreddeal_buyer_network_name = preferreddeal.buyer_network_name
preferreddeal_fixed_cpm = preferreddeal.fixed_cpm
preferreddeal_advertiser_name = preferreddeal.advertiser_name
```

---


### Account

Get information about the selected Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of this account. |
| `id` | String | Unique identifier of this account. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_name = account.name
account_id = account.id
account_kind = account.kind
```

---


### Adclient

List all ad clients in this Ad Exchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The ad clients returned in this list response. |
| `next_page_token` | String | Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `etag` | String | ETag of this response for caching purposes. |
| `kind` | String | Kind of list this is, in this case adexchangeseller#adClients. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adclient outputs
adclient_id = adclient.id
adclient_items = adclient.items
adclient_next_page_token = adclient.next_page_token
adclient_etag = adclient.etag
adclient_kind = adclient.kind
```

---


### Saved

Generate an Ad Exchange report based on the saved report ID sent in the query parameters.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved outputs
saved_id = saved.id
saved_averages = saved.averages
saved_headers = saved.headers
saved_kind = saved.kind
saved_rows = saved.rows
saved_totals = saved.totals
saved_total_matched_rows = saved.total_matched_rows
saved_warnings = saved.warnings
```

---


### Metric

List the metadata for the metrics available to this AdExchange account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> |  |
| `kind` | String | Kind of list this is, in this case adexchangeseller#metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access metric outputs
metric_id = metric.id
metric_items = metric.items
metric_kind = metric.kind
```

---


### Report

Generate an Ad Exchange report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adexchangeseller#report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_averages = report.averages
report_headers = report.headers
report_kind = report.kind
report_rows = report.rows
report_totals = report.totals
report_total_matched_rows = report.total_matched_rows
report_warnings = report.warnings
```

---


### Adunit

Gets the specified ad unit in the specified ad client.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of this ad unit. |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#adUnit. |
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adunit outputs
adunit_id = adunit.id
adunit_name = adunit.name
adunit_id = adunit.id
adunit_kind = adunit.kind
adunit_status = adunit.status
adunit_code = adunit.code
```

---


### Customchannel

Get the specified custom channel from the specified ad client.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `targeting_info` | String | The targeting information of this custom channel, if activated. |
| `kind` | String | Kind of resource this is, in this case adexchangeseller#customChannel. |
| `code` | String | Code of this custom channel, not necessarily unique across ad clients. |
| `name` | String | Name of this custom channel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_id = customchannel.id
customchannel_targeting_info = customchannel.targeting_info
customchannel_kind = customchannel.kind
customchannel_code = customchannel.code
customchannel_name = customchannel.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple saved resources
saved_0 = provider.adexchangeseller_api.Saved {
}
saved_1 = provider.adexchangeseller_api.Saved {
}
saved_2 = provider.adexchangeseller_api.Saved {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    saved = provider.adexchangeseller_api.Saved {
    }
```

---

## Related Documentation

- [GCP Adexchangeseller_api Documentation](https://cloud.google.com/adexchangeseller_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
