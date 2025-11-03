# Pagespeedonline_api Service



**Resources**: 4

---

## Overview

The pagespeedonline_api service provides access to 4 resource types:

- [Pagespeedapi](#pagespeedapi) [R]
- [Pagespeedapi](#pagespeedapi) [R]
- [Pagespeedapi](#pagespeedapi) [R]
- [Pagespeedapi](#pagespeedapi) [R]

---

## Resources


### Pagespeedapi

Runs PageSpeed analysis on the page at the specified URL, and returns a PageSpeed score, a list of suggestions to make that page faster, and other information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Canonicalized and final URL for the document, after following page redirects (if any). |
| `invalid_rules` | Vec<String> | List of rules that were specified in the request, but which the server did not know how to instantiate. |
| `page_stats` | String | Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc. |
| `response_code` | i64 | Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error. |
| `score` | i64 | The PageSpeed Score (0-100), which indicates how much faster a page could be. A high score indicates little room for improvement, while a lower score indicates more room for improvement. |
| `screenshot` | String | Base64-encoded screenshot of the page that was analyzed. |
| `formatted_results` | String | Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server. |
| `captcha_result` | String | The captcha verify result |
| `title` | String | Title of the page, as displayed in the browser's title bar. |
| `version` | String | The version of PageSpeed used to generate these results. |
| `kind` | String | Kind of result. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pagespeedapi outputs
pagespeedapi_id = pagespeedapi.id
pagespeedapi_id = pagespeedapi.id
pagespeedapi_invalid_rules = pagespeedapi.invalid_rules
pagespeedapi_page_stats = pagespeedapi.page_stats
pagespeedapi_response_code = pagespeedapi.response_code
pagespeedapi_score = pagespeedapi.score
pagespeedapi_screenshot = pagespeedapi.screenshot
pagespeedapi_formatted_results = pagespeedapi.formatted_results
pagespeedapi_captcha_result = pagespeedapi.captcha_result
pagespeedapi_title = pagespeedapi.title
pagespeedapi_version = pagespeedapi.version
pagespeedapi_kind = pagespeedapi.kind
```

---


### Pagespeedapi

Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `page_stats` | String | Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc. |
| `rule_groups` | HashMap<String, String> | A map with one entry for each rule group in these results. |
| `formatted_results` | String | Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server. |
| `kind` | String | Kind of result. |
| `screenshot` | String | Base64-encoded screenshot of the page that was analyzed. |
| `title` | String | Title of the page, as displayed in the browser's title bar. |
| `response_code` | i64 | Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error. |
| `invalid_rules` | Vec<String> | List of rules that were specified in the request, but which the server did not know how to instantiate. |
| `version` | String | The version of PageSpeed used to generate these results. |
| `captcha_result` | String | The captcha verify result |
| `id` | String | Canonicalized and final URL for the document, after following page redirects (if any). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pagespeedapi outputs
pagespeedapi_id = pagespeedapi.id
pagespeedapi_page_stats = pagespeedapi.page_stats
pagespeedapi_rule_groups = pagespeedapi.rule_groups
pagespeedapi_formatted_results = pagespeedapi.formatted_results
pagespeedapi_kind = pagespeedapi.kind
pagespeedapi_screenshot = pagespeedapi.screenshot
pagespeedapi_title = pagespeedapi.title
pagespeedapi_response_code = pagespeedapi.response_code
pagespeedapi_invalid_rules = pagespeedapi.invalid_rules
pagespeedapi_version = pagespeedapi.version
pagespeedapi_captcha_result = pagespeedapi.captcha_result
pagespeedapi_id = pagespeedapi.id
```

---


### Pagespeedapi

Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `captcha_result` | String | The captcha verify result |
| `id` | String | Canonicalized and final URL for the document, after following page redirects (if any). |
| `origin_loading_experience` | String | Metrics of the aggregated page loading experience of the origin |
| `version` | String | The version of PageSpeed used to generate these results. |
| `kind` | String | Kind of result. |
| `analysis_utc_timestamp` | String | The UTC timestamp of this analysis. |
| `lighthouse_result` | String | Lighthouse response for the audit url as an object. |
| `loading_experience` | String | Metrics of end users' page loading experience. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pagespeedapi outputs
pagespeedapi_id = pagespeedapi.id
pagespeedapi_captcha_result = pagespeedapi.captcha_result
pagespeedapi_id = pagespeedapi.id
pagespeedapi_origin_loading_experience = pagespeedapi.origin_loading_experience
pagespeedapi_version = pagespeedapi.version
pagespeedapi_kind = pagespeedapi.kind
pagespeedapi_analysis_utc_timestamp = pagespeedapi.analysis_utc_timestamp
pagespeedapi_lighthouse_result = pagespeedapi.lighthouse_result
pagespeedapi_loading_experience = pagespeedapi.loading_experience
```

---


### Pagespeedapi

Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `screenshot` | String | Base64-encoded screenshot of the page that was analyzed. |
| `page_stats` | String | Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc. |
| `kind` | String | Kind of result. |
| `invalid_rules` | Vec<String> | List of rules that were specified in the request, but which the server did not know how to instantiate. |
| `snapshots` | Vec<String> | Additional base64-encoded screenshots of the page, in various partial render states. |
| `title` | String | Title of the page, as displayed in the browser's title bar. |
| `version` | String | The version of PageSpeed used to generate these results. |
| `loading_experience` | String | Metrics of end users' page loading experience. |
| `id` | String | Canonicalized and final URL for the document, after following page redirects (if any). |
| `formatted_results` | String | Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server. |
| `rule_groups` | HashMap<String, String> | A map with one entry for each rule group in these results. |
| `captcha_result` | String | The captcha verify result |
| `response_code` | i64 | Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pagespeedapi outputs
pagespeedapi_id = pagespeedapi.id
pagespeedapi_screenshot = pagespeedapi.screenshot
pagespeedapi_page_stats = pagespeedapi.page_stats
pagespeedapi_kind = pagespeedapi.kind
pagespeedapi_invalid_rules = pagespeedapi.invalid_rules
pagespeedapi_snapshots = pagespeedapi.snapshots
pagespeedapi_title = pagespeedapi.title
pagespeedapi_version = pagespeedapi.version
pagespeedapi_loading_experience = pagespeedapi.loading_experience
pagespeedapi_id = pagespeedapi.id
pagespeedapi_formatted_results = pagespeedapi.formatted_results
pagespeedapi_rule_groups = pagespeedapi.rule_groups
pagespeedapi_captcha_result = pagespeedapi.captcha_result
pagespeedapi_response_code = pagespeedapi.response_code
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple pagespeedapi resources
pagespeedapi_0 = provider.pagespeedonline_api.Pagespeedapi {
}
pagespeedapi_1 = provider.pagespeedonline_api.Pagespeedapi {
}
pagespeedapi_2 = provider.pagespeedonline_api.Pagespeedapi {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pagespeedapi = provider.pagespeedonline_api.Pagespeedapi {
    }
```

---

## Related Documentation

- [GCP Pagespeedonline_api Documentation](https://cloud.google.com/pagespeedonline_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
