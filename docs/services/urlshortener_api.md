# Urlshortener_api Service



**Resources**: 1

---

## Overview

The urlshortener_api service provides access to 1 resource type:

- [Url](#url) [CR]

---

## Resources


### Url

Creates a new short URL.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | The fixed string "urlshortener#url". |
| `status` | String |  | Status of the target URL. Possible values: "OK", "MALWARE", "PHISHING", or "REMOVED". A URL might be marked "REMOVED" if it was flagged as spam, for example. |
| `long_url` | String |  | Long URL, e.g. "http://www.google.com/". Might not be present if the status is "REMOVED". |
| `created` | String |  | Time the short URL was created; ISO 8601 representation using the yyyy-MM-dd'T'HH:mm:ss.SSSZZ format, e.g. "2010-10-14T19:01:24.944+00:00". |
| `id` | String |  | Short URL, e.g. "http://goo.gl/l6MS". |
| `analytics` | String |  | A summary of the click analytics for the short and long URL. Might not be present if not requested or currently unavailable. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The fixed string "urlshortener#url". |
| `status` | String | Status of the target URL. Possible values: "OK", "MALWARE", "PHISHING", or "REMOVED". A URL might be marked "REMOVED" if it was flagged as spam, for example. |
| `long_url` | String | Long URL, e.g. "http://www.google.com/". Might not be present if the status is "REMOVED". |
| `created` | String | Time the short URL was created; ISO 8601 representation using the yyyy-MM-dd'T'HH:mm:ss.SSSZZ format, e.g. "2010-10-14T19:01:24.944+00:00". |
| `id` | String | Short URL, e.g. "http://goo.gl/l6MS". |
| `analytics` | String | A summary of the click analytics for the short and long URL. Might not be present if not requested or currently unavailable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create url
url = provider.urlshortener_api.Url {
}

# Access url outputs
url_id = url.id
url_kind = url.kind
url_status = url.status
url_long_url = url.long_url
url_created = url.created
url_id = url.id
url_analytics = url.analytics
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple url resources
url_0 = provider.urlshortener_api.Url {
}
url_1 = provider.urlshortener_api.Url {
}
url_2 = provider.urlshortener_api.Url {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    url = provider.urlshortener_api.Url {
    }
```

---

## Related Documentation

- [GCP Urlshortener_api Documentation](https://cloud.google.com/urlshortener_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
