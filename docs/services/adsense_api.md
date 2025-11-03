# Adsense_api Service



**Resources**: 34

---

## Overview

The adsense_api service provides access to 34 resource types:

- [Account](#account) [R]
- [Adclient](#adclient) [R]
- [Adunit](#adunit) [CRU]
- [Customchannel](#customchannel) [CRUD]
- [Alert](#alert) [R]
- [Payment](#payment) [R]
- [Site](#site) [R]
- [Policy_issue](#policy_issue) [R]
- [Saved](#saved) [R]
- [Urlchannel](#urlchannel) [R]
- [Report](#report) [R]
- [Metric](#metric) [R]
- [Dimension](#dimension) [R]
- [Account](#account) [R]
- [Savedadstyle](#savedadstyle) [R]
- [Adunit](#adunit) [R]
- [Urlchannel](#urlchannel) [R]
- [Report](#report) [R]
- [Adclient](#adclient) [R]
- [Customchannel](#customchannel) [R]
- [Alert](#alert) [RD]
- [Saved](#saved) [R]
- [Payment](#payment) [R]
- [Customchannel](#customchannel) [R]
- [Account](#account) [R]
- [Savedadstyle](#savedadstyle) [R]
- [Report](#report) [R]
- [Saved](#saved) [R]
- [Adclient](#adclient) [R]
- [Dimension](#dimension) [R]
- [Metric](#metric) [R]
- [Urlchannel](#urlchannel) [R]
- [Adunit](#adunit) [R]
- [Alert](#alert) [R]

---

## Resources


### Account

Gets information about the selected AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the account. |
| `pending_tasks` | Vec<String> | Output only. Outstanding tasks that need to be completed as part of the sign-up process for a new account. e.g. "billing-profile-creation", "phone-pin-verification". |
| `display_name` | String | Output only. Display name of this account. |
| `premium` | bool | Output only. Whether this account is premium. Premium accounts have access to additional spam-related metrics. |
| `name` | String | Output only. Resource name of the account. Format: accounts/pub-[0-9]+ |
| `time_zone` | String | The account time zone, as used by reporting. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725). |
| `create_time` | String | Output only. Creation time of the account. |


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
account_state = account.state
account_pending_tasks = account.pending_tasks
account_display_name = account.display_name
account_premium = account.premium
account_name = account.name
account_time_zone = account.time_zone
account_create_time = account.create_time
```

---


### Adclient

Gets the ad client from the given resource name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the ad client. Format: accounts/{account}/adclients/{adclient} |
| `product_code` | String | Output only. Reporting product code of the ad client. For example, "AFC" for AdSense for Content. Corresponds to the `PRODUCT_CODE` dimension, and present only if the ad client supports reporting. |
| `state` | String | Output only. State of the ad client. |
| `reporting_dimension_id` | String | Output only. Unique ID of the ad client as used in the `AD_CLIENT_ID` reporting dimension. Present only if the ad client supports reporting. |


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
adclient_name = adclient.name
adclient_product_code = adclient.product_code
adclient_state = adclient.state
adclient_reporting_dimension_id = adclient.reporting_dimension_id
```

---


### Adunit

Creates an ad unit. This method can be called only by a restricted set of projects, which are usually owned by [AdSense for Platforms](https://developers.google.com/adsense/platforms/) publishers. Contact your account manager if you need to use this method. Note that ad units can only be created for ad clients with an "AFC" product code. For more info see the [AdClient resource](/adsense/management/reference/rest/v2/accounts.adclients). For now, this method can only be used to create `DISPLAY` ad units. See: https://support.google.com/adsense/answer/9183566

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the ad unit. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit} |
| `state` | String |  | Required. State of the ad unit. |
| `display_name` | String |  | Required. Display name of the ad unit, as provided when the ad unit was created. |
| `content_ads_settings` | String |  | Required. Settings specific to content ads (AFC). |
| `reporting_dimension_id` | String |  | Output only. Unique ID of the ad unit as used in the `AD_UNIT_ID` reporting dimension. |
| `parent` | String | ✅ | Required. Ad client to create an ad unit under. Format: accounts/{account}/adclients/{adclient} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the ad unit. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit} |
| `state` | String | Required. State of the ad unit. |
| `display_name` | String | Required. Display name of the ad unit, as provided when the ad unit was created. |
| `content_ads_settings` | String | Required. Settings specific to content ads (AFC). |
| `reporting_dimension_id` | String | Output only. Unique ID of the ad unit as used in the `AD_UNIT_ID` reporting dimension. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create adunit
adunit = provider.adsense_api.Adunit {
    parent = "value"  # Required. Ad client to create an ad unit under. Format: accounts/{account}/adclients/{adclient}
}

# Access adunit outputs
adunit_id = adunit.id
adunit_name = adunit.name
adunit_state = adunit.state
adunit_display_name = adunit.display_name
adunit_content_ads_settings = adunit.content_ads_settings
adunit_reporting_dimension_id = adunit.reporting_dimension_id
```

---


### Customchannel

Creates a custom channel. This method can be called only by a restricted set of projects, which are usually owned by [AdSense for Platforms](https://developers.google.com/adsense/platforms/) publishers. Contact your account manager if you need to use this method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `active` | bool |  | Whether the custom channel is active and collecting data. See https://support.google.com/adsense/answer/10077192. |
| `reporting_dimension_id` | String |  | Output only. Unique ID of the custom channel as used in the `CUSTOM_CHANNEL_ID` reporting dimension. |
| `name` | String |  | Output only. Resource name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel} |
| `display_name` | String |  | Required. Display name of the custom channel. |
| `parent` | String | ✅ | Required. The ad client to create a custom channel under. Format: accounts/{account}/adclients/{adclient} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active` | bool | Whether the custom channel is active and collecting data. See https://support.google.com/adsense/answer/10077192. |
| `reporting_dimension_id` | String | Output only. Unique ID of the custom channel as used in the `CUSTOM_CHANNEL_ID` reporting dimension. |
| `name` | String | Output only. Resource name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel} |
| `display_name` | String | Required. Display name of the custom channel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customchannel
customchannel = provider.adsense_api.Customchannel {
    parent = "value"  # Required. The ad client to create a custom channel under. Format: accounts/{account}/adclients/{adclient}
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_active = customchannel.active
customchannel_reporting_dimension_id = customchannel.reporting_dimension_id
customchannel_name = customchannel.name
customchannel_display_name = customchannel.display_name
```

---


### Alert

Lists all the alerts available in an account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alerts` | Vec<String> | The alerts returned in this list response. |


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
alert_alerts = alert.alerts
```

---


### Payment

Lists all the payments available for an account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `payments` | Vec<String> | The payments returned in this list response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access payment outputs
payment_id = payment.id
payment_payments = payment.payments
```

---


### Site

Gets information about the selected site.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of a site. |
| `auto_ads_enabled` | bool | Whether auto ads is turned on for the site. |
| `reporting_dimension_id` | String | Output only. Unique ID of the site as used in the `OWNED_SITE_ID` reporting dimension. |
| `name` | String | Output only. Resource name of a site. Format: accounts/{account}/sites/{site} |
| `domain` | String | Domain (or subdomain) of the site, e.g. "example.com" or "www.example.com". This is used in the `OWNED_SITE_DOMAIN_NAME` reporting dimension. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access site outputs
site_id = site.id
site_state = site.state
site_auto_ads_enabled = site.auto_ads_enabled
site_reporting_dimension_id = site.reporting_dimension_id
site_name = site.name
site_domain = site.domain
```

---


### Policy_issue

Gets information about the selected policy issue.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warning_escalation_date` | String | Optional. The date (in the America/Los_Angeles timezone) when the entity will have ad serving demand restricted or ad serving disabled. This is present only for issues with a `WARNED` enforcement action. See https://support.google.com/adsense/answer/11066888. |
| `name` | String | Required. Resource name of the entity with policy issues. Format: accounts/{account}/policyIssues/{policy_issue} |
| `uri` | String | Optional. URI of the page having policy violations (for example "foo.com/bar" or "www.foo.com/bar"). This will be present if the `entity_type` is `PAGE` and will be absent for other entity types. |
| `last_detected_date` | String | Required. The date (in the America/Los_Angeles timezone) when policy violations were last detected on the entity. |
| `action` | String | Required. The most severe action taken on the entity over the past seven days. |
| `ad_clients` | Vec<String> | Optional. List of ad clients associated with the policy issue (either as the primary ad client or an associated host/secondary ad client). In the latter case, this will be an ad client that is not owned by the current account. |
| `policy_topics` | Vec<String> | Required. Unordered list. The policy topics that this entity was found to violate over the past seven days. |
| `site` | String | Required. Hostname/domain of the entity (for example "foo.com" or "www.foo.com"). This _should_ be a bare domain/host name without any protocol. This will be present for all policy issues. |
| `ad_request_count` | String | Required. Total number of ad requests affected by the policy violations over the past seven days. |
| `site_section` | String | Optional. Prefix of the site-section having policy issues (For example "foo.com/bar-section"). This will be present if the `entity_type` is `SITE_SECTION` and will be absent for other entity types. |
| `entity_type` | String | Required. Type of the entity indicating if the entity is a site, site-section, or page. |
| `first_detected_date` | String | Required. The date (in the America/Los_Angeles timezone) when policy violations were first detected on the entity. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access policy_issue outputs
policy_issue_id = policy_issue.id
policy_issue_warning_escalation_date = policy_issue.warning_escalation_date
policy_issue_name = policy_issue.name
policy_issue_uri = policy_issue.uri
policy_issue_last_detected_date = policy_issue.last_detected_date
policy_issue_action = policy_issue.action
policy_issue_ad_clients = policy_issue.ad_clients
policy_issue_policy_topics = policy_issue.policy_topics
policy_issue_site = policy_issue.site
policy_issue_ad_request_count = policy_issue.ad_request_count
policy_issue_site_section = policy_issue.site_section
policy_issue_entity_type = policy_issue.entity_type
policy_issue_first_detected_date = policy_issue.first_detected_date
```

---


### Saved

Generates a saved report.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_date` | String | Required. Start date of the range (inclusive). |
| `end_date` | String | Required. End date of the range (inclusive). |
| `headers` | Vec<String> | The header information; one for each dimension in the request, followed by one for each metric in the request. |
| `averages` | String | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `rows` | Vec<String> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. |
| `total_matched_rows` | String | The total number of rows matched by the report request. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. These warnings are always returned in English. |
| `totals` | String | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |


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
saved_start_date = saved.start_date
saved_end_date = saved.end_date
saved_headers = saved.headers
saved_averages = saved.averages
saved_rows = saved.rows
saved_total_matched_rows = saved.total_matched_rows
saved_warnings = saved.warnings
saved_totals = saved.totals
```

---


### Urlchannel

Gets information about the selected url channel.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uri_pattern` | String | URI pattern of the channel. Does not include "http://" or "https://". Example: www.example.com/home |
| `name` | String | Output only. Resource name of the URL channel. Format: accounts/{account}/adclients/{adclient}/urlchannels/{urlchannel} |
| `reporting_dimension_id` | String | Output only. Unique ID of the custom channel as used in the `URL_CHANNEL_ID` reporting dimension. |


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
urlchannel_uri_pattern = urlchannel.uri_pattern
urlchannel_name = urlchannel.name
urlchannel_reporting_dimension_id = urlchannel.reporting_dimension_id
```

---


### Report

Generates a csv formatted ad hoc report.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |


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
report_extensions = report.extensions
report_content_type = report.content_type
report_data = report.data
```

---


### Metric

List the metadata for the metrics available to this AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#metadata. |
| `items` | Vec<String> |  |


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
metric_kind = metric.kind
metric_items = metric.items
```

---


### Dimension

List the metadata for the dimensions available to this AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#metadata. |
| `items` | Vec<String> |  |


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
dimension_kind = dimension.kind
dimension_items = dimension.items
```

---


### Account

Get information about the selected AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timezone` | String | AdSense timezone of this account. |
| `kind` | String | Kind of resource this is, in this case adsense#account. |
| `premium` | bool | Whether this account is premium. |
| `name` | String | Name of this account. |
| `id` | String | Unique identifier of this account. |
| `sub_accounts` | Vec<String> | Sub accounts of the this account. |
| `creation_time` | String |  |


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
account_timezone = account.timezone
account_kind = account.kind
account_premium = account.premium
account_name = account.name
account_id = account.id
account_sub_accounts = account.sub_accounts
account_creation_time = account.creation_time
```

---


### Savedadstyle

Get a specific saved ad style from the user's account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_style` | String | The AdStyle itself. |
| `kind` | String | Kind of resource this is, in this case adsense#savedAdStyle. |
| `id` | String | Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String | The user selected name of this SavedAdStyle. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access savedadstyle outputs
savedadstyle_id = savedadstyle.id
savedadstyle_ad_style = savedadstyle.ad_style
savedadstyle_kind = savedadstyle.kind
savedadstyle_id = savedadstyle.id
savedadstyle_name = savedadstyle.name
```

---


### Adunit

Gets the specified ad unit in the specified ad client for the specified account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_style` | String | Custom style information specific to this ad unit. |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `saved_style_id` | String | ID of the saved ad style which holds this ad unit's style information. |
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `kind` | String | Kind of resource this is, in this case adsense#adUnit. |
| `mobile_content_ads_settings` | String | Settings specific to WAP mobile content ads (AFMC) - deprecated. |
| `name` | String | Name of this ad unit. |
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |
| `content_ads_settings` | String | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `feed_ads_settings` | String | Settings specific to feed ads (AFF) - deprecated. |


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
adunit_custom_style = adunit.custom_style
adunit_id = adunit.id
adunit_saved_style_id = adunit.saved_style_id
adunit_status = adunit.status
adunit_kind = adunit.kind
adunit_mobile_content_ads_settings = adunit.mobile_content_ads_settings
adunit_name = adunit.name
adunit_code = adunit.code
adunit_content_ads_settings = adunit.content_ads_settings
adunit_feed_ads_settings = adunit.feed_ads_settings
```

---


### Urlchannel

List all URL channels in the specified ad client for the specified account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The URL channels returned in this list response. |
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `kind` | String | Kind of list this is, in this case adsense#urlChannels. |
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
urlchannel_items = urlchannel.items
urlchannel_next_page_token = urlchannel.next_page_token
urlchannel_kind = urlchannel.kind
urlchannel_etag = urlchannel.etag
```

---


### Report

Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_date` | String | The requested start date in yyyy-mm-dd format. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adsense#report. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `end_date` | String | The requested end date in yyyy-mm-dd format. |


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
report_start_date = report.start_date
report_averages = report.averages
report_totals = report.totals
report_warnings = report.warnings
report_headers = report.headers
report_kind = report.kind
report_total_matched_rows = report.total_matched_rows
report_rows = report.rows
report_end_date = report.end_date
```

---


### Adclient

List all ad clients in the specified account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of this response for caching purposes. |
| `next_page_token` | String | Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `items` | Vec<String> | The ad clients returned in this list response. |
| `kind` | String | Kind of list this is, in this case adsense#adClients. |


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
adclient_next_page_token = adclient.next_page_token
adclient_items = adclient.items
adclient_kind = adclient.kind
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
| `kind` | String | Kind of resource this is, in this case adsense#customChannel. |
| `name` | String | Name of this custom channel. |
| `targeting_info` | String | The targeting information of this custom channel, if activated. |


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
customchannel_kind = customchannel.kind
customchannel_name = customchannel.name
customchannel_targeting_info = customchannel.targeting_info
```

---


### Alert

List the alerts for the specified AdSense account.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The alerts returned in this list response. |
| `kind` | String | Kind of list this is, in this case adsense#alerts. |


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


### Saved

Generate an AdSense report based on the saved report ID sent in the query parameters.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_date` | String | The requested start date in yyyy-mm-dd format. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adsense#report. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `end_date` | String | The requested end date in yyyy-mm-dd format. |


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
saved_start_date = saved.start_date
saved_averages = saved.averages
saved_totals = saved.totals
saved_warnings = saved.warnings
saved_headers = saved.headers
saved_kind = saved.kind
saved_total_matched_rows = saved.total_matched_rows
saved_rows = saved.rows
saved_end_date = saved.end_date
```

---


### Payment

List the payments for the specified AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#payments. |
| `items` | Vec<String> | The list of Payments for the account. One or both of a) the account's most recent payment; and b) the account's upcoming payment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access payment outputs
payment_id = payment.id
payment_kind = payment.kind
payment_items = payment.items
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
| `kind` | String | Kind of resource this is, in this case adsense#customChannel. |
| `name` | String | Name of this custom channel. |
| `targeting_info` | String | The targeting information of this custom channel, if activated. |


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
customchannel_kind = customchannel.kind
customchannel_name = customchannel.name
customchannel_targeting_info = customchannel.targeting_info
```

---


### Account

Get information about the selected AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier of this account. |
| `name` | String | Name of this account. |
| `premium` | bool | Whether this account is premium. |
| `kind` | String | Kind of resource this is, in this case adsense#account. |
| `sub_accounts` | Vec<String> | Sub accounts of the this account. |


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
account_name = account.name
account_premium = account.premium
account_kind = account.kind
account_sub_accounts = account.sub_accounts
```

---


### Savedadstyle

Get a specific saved ad style from the user's account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String | The user selected name of this SavedAdStyle. |
| `ad_style` | String | The AdStyle itself. |
| `kind` | String | Kind of resource this is, in this case adsense#savedAdStyle. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access savedadstyle outputs
savedadstyle_id = savedadstyle.id
savedadstyle_id = savedadstyle.id
savedadstyle_name = savedadstyle.name
savedadstyle_ad_style = savedadstyle.ad_style
savedadstyle_kind = savedadstyle.kind
```

---


### Report

Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adsense#report. |


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
report_total_matched_rows = report.total_matched_rows
report_totals = report.totals
report_warnings = report.warnings
report_averages = report.averages
report_rows = report.rows
report_headers = report.headers
report_kind = report.kind
```

---


### Saved

Generate an AdSense report based on the saved report ID sent in the query parameters.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adsense#report. |


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
saved_total_matched_rows = saved.total_matched_rows
saved_totals = saved.totals
saved_warnings = saved.warnings
saved_averages = saved.averages
saved_rows = saved.rows
saved_headers = saved.headers
saved_kind = saved.kind
```

---


### Adclient

List all ad clients in the specified account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `etag` | String | ETag of this response for caching purposes. |
| `items` | Vec<String> | The ad clients returned in this list response. |
| `kind` | String | Kind of list this is, in this case adsense#adClients. |


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
adclient_next_page_token = adclient.next_page_token
adclient_etag = adclient.etag
adclient_items = adclient.items
adclient_kind = adclient.kind
```

---


### Dimension

List the metadata for the dimensions available to this AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#metadata. |
| `items` | Vec<String> |  |


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
dimension_kind = dimension.kind
dimension_items = dimension.items
```

---


### Metric

List the metadata for the metrics available to this AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#metadata. |
| `items` | Vec<String> |  |


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
metric_kind = metric.kind
metric_items = metric.items
```

---


### Urlchannel

List all URL channels in the specified ad client for the specified account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `etag` | String | ETag of this response for caching purposes. |
| `kind` | String | Kind of list this is, in this case adsense#urlChannels. |
| `items` | Vec<String> | The URL channels returned in this list response. |


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
urlchannel_etag = urlchannel.etag
urlchannel_kind = urlchannel.kind
urlchannel_items = urlchannel.items
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
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |
| `feed_ads_settings` | String | Settings specific to feed ads (AFF) - deprecated. |
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `saved_style_id` | String | ID of the saved ad style which holds this ad unit's style information. |
| `mobile_content_ads_settings` | String | Settings specific to WAP mobile content ads (AFMC) - deprecated. |
| `name` | String | Name of this ad unit. |
| `content_ads_settings` | String | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `kind` | String | Kind of resource this is, in this case adsense#adUnit. |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `custom_style` | String | Custom style information specific to this ad unit. |


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
adunit_code = adunit.code
adunit_feed_ads_settings = adunit.feed_ads_settings
adunit_status = adunit.status
adunit_saved_style_id = adunit.saved_style_id
adunit_mobile_content_ads_settings = adunit.mobile_content_ads_settings
adunit_name = adunit.name
adunit_content_ads_settings = adunit.content_ads_settings
adunit_kind = adunit.kind
adunit_id = adunit.id
adunit_custom_style = adunit.custom_style
```

---


### Alert

List the alerts for the specified AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsense#alerts. |
| `items` | Vec<String> | The alerts returned in this list response. |


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
alert_kind = alert.kind
alert_items = alert.items
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple account resources
account_0 = provider.adsense_api.Account {
}
account_1 = provider.adsense_api.Account {
}
account_2 = provider.adsense_api.Account {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account = provider.adsense_api.Account {
    }
```

---

## Related Documentation

- [GCP Adsense_api Documentation](https://cloud.google.com/adsense_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
