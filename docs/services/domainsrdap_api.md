# Domainsrdap_api Service



**Resources**: 6

---

## Overview

The domainsrdap_api service provides access to 6 resource types:

- [Autnum](#autnum) [R]
- [Domain](#domain) [R]
- [Entity](#entity) [R]
- [Nameserver](#nameserver) [R]
- [Ip](#ip) [R]
- [Domainsrdap](#domainsrdap) [R]

---

## Resources


### Autnum

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `title` | String | Error title. |
| `description` | Vec<String> | Error description. |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `rdap_conformance` | Vec<String> | RDAP conformance level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autnum outputs
autnum_id = autnum.id
autnum_error_code = autnum.error_code
autnum_json_response = autnum.json_response
autnum_title = autnum.title
autnum_description = autnum.description
autnum_lang = autnum.lang
autnum_notices = autnum.notices
autnum_rdap_conformance = autnum.rdap_conformance
```

---


### Domain

Look up RDAP information for a domain by name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access domain outputs
domain_id = domain.id
domain_data = domain.data
domain_content_type = domain.content_type
domain_extensions = domain.extensions
```

---


### Entity

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `title` | String | Error title. |
| `description` | Vec<String> | Error description. |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `rdap_conformance` | Vec<String> | RDAP conformance level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entity outputs
entity_id = entity.id
entity_error_code = entity.error_code
entity_json_response = entity.json_response
entity_title = entity.title
entity_description = entity.description
entity_lang = entity.lang
entity_notices = entity.notices
entity_rdap_conformance = entity.rdap_conformance
```

---


### Nameserver

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `title` | String | Error title. |
| `description` | Vec<String> | Error description. |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `rdap_conformance` | Vec<String> | RDAP conformance level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access nameserver outputs
nameserver_id = nameserver.id
nameserver_error_code = nameserver.error_code
nameserver_json_response = nameserver.json_response
nameserver_title = nameserver.title
nameserver_description = nameserver.description
nameserver_lang = nameserver.lang
nameserver_notices = nameserver.notices
nameserver_rdap_conformance = nameserver.rdap_conformance
```

---


### Ip

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `title` | String | Error title. |
| `description` | Vec<String> | Error description. |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `rdap_conformance` | Vec<String> | RDAP conformance level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access ip outputs
ip_id = ip.id
ip_error_code = ip.error_code
ip_json_response = ip.json_response
ip_title = ip.title
ip_description = ip.description
ip_lang = ip.lang
ip_notices = ip.notices
ip_rdap_conformance = ip.rdap_conformance
```

---


### Domainsrdap

Get help information for the RDAP API, including links to documentation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access domainsrdap outputs
domainsrdap_id = domainsrdap.id
domainsrdap_data = domainsrdap.data
domainsrdap_content_type = domainsrdap.content_type
domainsrdap_extensions = domainsrdap.extensions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple autnum resources
autnum_0 = provider.domainsrdap_api.Autnum {
}
autnum_1 = provider.domainsrdap_api.Autnum {
}
autnum_2 = provider.domainsrdap_api.Autnum {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    autnum = provider.domainsrdap_api.Autnum {
    }
```

---

## Related Documentation

- [GCP Domainsrdap_api Documentation](https://cloud.google.com/domainsrdap_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
