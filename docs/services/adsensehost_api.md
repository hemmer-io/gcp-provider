# Adsensehost_api Service



**Resources**: 7

---

## Overview

The adsensehost_api service provides access to 7 resource types:

- [Adunit](#adunit) [CRUD]
- [Urlchannel](#urlchannel) [CRD]
- [Report](#report) [R]
- [Account](#account) [R]
- [Customchannel](#customchannel) [CRUD]
- [Adclient](#adclient) [R]
- [Associationsession](#associationsession) [R]

---

## Resources


### Adunit

Insert the supplied ad unit into the specified publisher AdSense account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_ads_settings` | String |  | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `name` | String |  | Name of this ad unit. |
| `status` | String |  | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `kind` | String |  | Kind of resource this is, in this case adsensehost#adUnit. |
| `code` | String |  | Identity code of this ad unit, not necessarily unique across ad clients. |
| `custom_style` | String |  | Custom style information specific to this ad unit. |
| `mobile_content_ads_settings` | String |  | Settings specific to WAP mobile content ads (AFMC - deprecated). |
| `id` | String |  | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `ad_client_id` | String | ✅ | Ad client into which to insert the ad unit. |
| `account_id` | String | ✅ | Account which will contain the ad unit. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_ads_settings` | String | Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated). |
| `name` | String | Name of this ad unit. |
| `status` | String | Status of this ad unit. Possible values are:
NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.

ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.

INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days. |
| `kind` | String | Kind of resource this is, in this case adsensehost#adUnit. |
| `code` | String | Identity code of this ad unit, not necessarily unique across ad clients. |
| `custom_style` | String | Custom style information specific to this ad unit. |
| `mobile_content_ads_settings` | String | Settings specific to WAP mobile content ads (AFMC - deprecated). |
| `id` | String | Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |


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
adunit_content_ads_settings = adunit.content_ads_settings
adunit_name = adunit.name
adunit_status = adunit.status
adunit_kind = adunit.kind
adunit_code = adunit.code
adunit_custom_style = adunit.custom_style
adunit_mobile_content_ads_settings = adunit.mobile_content_ads_settings
adunit_id = adunit.id
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
| `etag` | String | ETag of this response for caching purposes. |
| `next_page_token` | String | Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's "pageToken" value to this. |
| `kind` | String | Kind of list this is, in this case adsensehost#urlChannels. |
| `items` | Vec<String> | The URL channels returned in this list response. |


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
urlchannel_etag = urlchannel.etag
urlchannel_next_page_token = urlchannel.next_page_token
urlchannel_kind = urlchannel.kind
urlchannel_items = urlchannel.items
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
| `averages` | Vec<String> | The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |
| `headers` | Vec<String> | The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request. |
| `total_matched_rows` | String | The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit. |
| `warnings` | Vec<String> | Any warnings associated with generation of the report. |
| `rows` | Vec<Vec<String>> | The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers. |
| `kind` | String | Kind this is, in this case adsensehost#report. |
| `totals` | Vec<String> | The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty. |


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
report_total_matched_rows = report.total_matched_rows
report_warnings = report.warnings
report_rows = report.rows
report_kind = report.kind
report_totals = report.totals
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
| `id` | String | Unique identifier of this account. |
| `name` | String | Name of this account. |
| `kind` | String | Kind of resource this is, in this case adsensehost#account. |


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
account_id = account.id
account_name = account.name
account_kind = account.kind
```

---


### Customchannel

Add a new custom channel to the host AdSense account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Kind of resource this is, in this case adsensehost#customChannel. |
| `id` | String |  | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
| `code` | String |  | Code of this custom channel, not necessarily unique across ad clients. |
| `name` | String |  | Name of this custom channel. |
| `ad_client_id` | String | ✅ | Ad client to which the new custom channel will be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of resource this is, in this case adsensehost#customChannel. |
| `id` | String | Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format. |
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

# Create customchannel
customchannel = provider.adsensehost_api.Customchannel {
    ad_client_id = "value"  # Ad client to which the new custom channel will be added.
}

# Access customchannel outputs
customchannel_id = customchannel.id
customchannel_kind = customchannel.kind
customchannel_id = customchannel.id
customchannel_code = customchannel.code
customchannel_name = customchannel.name
```

---


### Adclient

Get information about one of the ad clients in the specified publisher's AdSense account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of resource this is, in this case adsensehost#adClient. |
| `id` | String | Unique identifier of this ad client. |
| `arc_opt_in` | bool | Whether this ad client is opted in to ARC. |
| `product_code` | String | This ad client's product code, which corresponds to the PRODUCT_CODE report dimension. |
| `supports_reporting` | bool | Whether this ad client supports being reported on. |


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
adclient_kind = adclient.kind
adclient_id = adclient.id
adclient_arc_opt_in = adclient.arc_opt_in
adclient_product_code = adclient.product_code
adclient_supports_reporting = adclient.supports_reporting
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
| `website_locale` | String | The locale of the user's hosted website. |
| `kind` | String | Kind of resource this is, in this case adsensehost#associationSession. |
| `website_url` | String | The URL of the user's hosted website. |
| `account_id` | String | Hosted account id of the associated publisher after association. Present if status is ACCEPTED. |
| `redirect_url` | String | Redirect URL of this association session. Used to redirect users into the AdSense association flow. |
| `product_codes` | Vec<String> | The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated) |
| `user_locale` | String | The preferred locale of the user themselves when going through the AdSense association flow. |
| `id` | String | Unique identifier of this association session. |
| `status` | String | Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR. |


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
associationsession_website_locale = associationsession.website_locale
associationsession_kind = associationsession.kind
associationsession_website_url = associationsession.website_url
associationsession_account_id = associationsession.account_id
associationsession_redirect_url = associationsession.redirect_url
associationsession_product_codes = associationsession.product_codes
associationsession_user_locale = associationsession.user_locale
associationsession_id = associationsession.id
associationsession_status = associationsession.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple adunit resources
adunit_0 = provider.adsensehost_api.Adunit {
    ad_client_id = "value-0"
    account_id = "value-0"
}
adunit_1 = provider.adsensehost_api.Adunit {
    ad_client_id = "value-1"
    account_id = "value-1"
}
adunit_2 = provider.adsensehost_api.Adunit {
    ad_client_id = "value-2"
    account_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    adunit = provider.adsensehost_api.Adunit {
        ad_client_id = "production-value"
        account_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Adsensehost_api Documentation](https://cloud.google.com/adsensehost_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
