# Adsensehost_api Service



**Resources**: 7

---

## Overview

The adsensehost_api service provides access to 7 resource types:

- [Customchannel](#customchannel) [CRUD]
- [Urlchannel](#urlchannel) [CRD]
- [Account](#account) [R]
- [Adclient](#adclient) [R]
- [Adunit](#adunit) [CRUD]
- [Associationsession](#associationsession) [R]
- [Report](#report) [R]

---

## Resources


### Customchannel

Add a new custom channel to the host AdSense account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String |  | Name of this custom channel. |
| `code` | String |  | Code of this custom channel, not necessarily unique across ad clients. |
| `kind` | String |  | Kind of resource this is, in this case adsensehost#customChannel. |
| `ad_client_id` | String | ✅ | Ad client to which the new custom channel will be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `name` | String | Name of this custom channel. |
| `code` | String | Code of this custom channel, not necessarily unique across ad clients. |
| `kind` | String | Kind of resource this is, in this case adsensehost#customChannel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customchannel
customchannel = provider.adsensehost_api.Customchannel {
    ad_client_id = "value"  # Ad client to which the new custom channel will be added.
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_id = customchannel.id
customchannel_name = customchannel.name
customchannel_code = customchannel.code
customchannel_kind = customchannel.kind
```

---


### Urlchannel

Add a new URL channel to the host AdSense account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url_pattern` | String |  | URL Pattern of this URL channel. Does not include "http://" or "https://". Example: www.example.com/home |
| `id` | String |  | Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `kind` | String |  | Kind of resource this is, in this case adsensehost#urlChannel. |
| `ad_client_id` | String | ✅ | Ad client to which the new URL channel will be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of list this is, in this case adsensehost#urlChannels. |
| `items` | Vec<String> | The URL channels returned in this list response. |
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

# Create urlchannel
urlchannel = provider.adsensehost_api.Urlchannel {
    ad_client_id = "value"  # Ad client to which the new URL channel will be added.
}

# Access urlchannel outputs
urlchannel_id = urlchannel.id
urlchannel_kind = urlchannel.kind
urlchannel_items = urlchannel.items
urlchannel_etag = urlchannel.etag
urlchannel_next_page_token = urlchannel.next_page_token
```

---


### Account

Get information about the selected associated AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Approval status of this account. One of: PENDING, APPROVED, DISABLED. |
| `kind` | String | Kind of resource this is, in this case adsensehost#account. |
| `name` | String | Name of this account. |
| `id` | String | Unique identifier of this account. |


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
account_status = account.status
account_kind = account.kind
account_name = account.name
account_id = account.id
```

---


### Adclient

Get information about one of the ad clients in the Host AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_code` | String | This ad client's product code, which corresponds to the PRODUCT_CODE report dimension. |
| `id` | String | Unique identifier of this ad client. |
| `supports_reporting` | bool | Whether this ad client supports being reported on. |
| `kind` | String | Kind of resource this is, in this case adsensehost#adClient. |
| `arc_opt_in` | bool | Whether this ad client is opted in to ARC. |


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
adclient_product_code = adclient.product_code
adclient_id = adclient.id
adclient_supports_reporting = adclient.supports_reporting
adclient_kind = adclient.kind
adclient_arc_opt_in = adclient.arc_opt_in
```

---


### Adunit

Insert the supplied ad unit into the specified publisher AdSense account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `mobile_content_ads_settings` | String |  | Settings specific to WAP mobile content ads (AFMC - deprecated). |
| `name` | String |  | Name of this ad unit. |
| `kind` | String |  | Kind of resource this is, in this case adsensehost#adUnit. |
| `content_ads_settings` | String |  | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `custom_style` | String |  | Custom style information specific to this ad unit. |
| `id` | String |  | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `code` | String |  | Identity code of this ad unit, not necessarily unique across ad clients. |
| `ad_client_id` | String | ✅ | Ad client into which to insert the ad unit. |
| `account_id` | String | ✅ | Account which will contain the ad unit. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `mobile_content_ads_settings` | String | Settings specific to WAP mobile content ads (AFMC - deprecated). |
| `name` | String | Name of this ad unit. |
| `kind` | String | Kind of resource this is, in this case adsensehost#adUnit. |
| `content_ads_settings` | String | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `custom_style` | String | Custom style information specific to this ad unit. |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create adunit
adunit = provider.adsensehost_api.Adunit {
    ad_client_id = "value"  # Ad client into which to insert the ad unit.
    account_id = "value"  # Account which will contain the ad unit.
}

# Access adunit outputs
adunit_id = adunit.id
adunit_status = adunit.status
adunit_mobile_content_ads_settings = adunit.mobile_content_ads_settings
adunit_name = adunit.name
adunit_kind = adunit.kind
adunit_content_ads_settings = adunit.content_ads_settings
adunit_custom_style = adunit.custom_style
adunit_id = adunit.id
adunit_code = adunit.code
```

---


### Associationsession

Verify an association session after the association callback returns from AdSense signup.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_locale` | String | The preferred locale of the user themselves when going through the AdSense association flow. |
| `website_locale` | String | The locale of the user's hosted website. |
| `status` | String | Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR. |
| `website_url` | String | The URL of the user's hosted website. |
| `account_id` | String | Hosted account id of the associated publisher after association. Present if status is ACCEPTED. |
| `id` | String | Unique identifier of this association session. |
| `redirect_url` | String | Redirect URL of this association session. Used to redirect users into the AdSense association flow. |
| `product_codes` | Vec<String> | The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated) |
| `kind` | String | Kind of resource this is, in this case adsensehost#associationSession. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access associationsession outputs
associationsession_id = associationsession.id
associationsession_user_locale = associationsession.user_locale
associationsession_website_locale = associationsession.website_locale
associationsession_status = associationsession.status
associationsession_website_url = associationsession.website_url
associationsession_account_id = associationsession.account_id
associationsession_id = associationsession.id
associationsession_redirect_url = associationsession.redirect_url
associationsession_product_codes = associationsession.product_codes
associationsession_kind = associationsession.kind
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
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `kind` | String | Kind this is, in this case adsensehost#report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
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
report_warnings = report.warnings
report_averages = report.averages
report_headers = report.headers
report_kind = report.kind
report_rows = report.rows
report_totals = report.totals
report_total_matched_rows = report.total_matched_rows
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple customchannel resources
customchannel_0 = provider.adsensehost_api.Customchannel {
    ad_client_id = "value-0"
}
customchannel_1 = provider.adsensehost_api.Customchannel {
    ad_client_id = "value-1"
}
customchannel_2 = provider.adsensehost_api.Customchannel {
    ad_client_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    customchannel = provider.adsensehost_api.Customchannel {
        ad_client_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Adsensehost_api Documentation](https://cloud.google.com/adsensehost_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
