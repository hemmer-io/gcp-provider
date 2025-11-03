# Domainsrdap_api Service



**Resources**: 6

---

## Overview

The domainsrdap_api service provides access to 6 resource types:

- [Domainsrdap](#domainsrdap) [R]
- [Ip](#ip) [R]
- [Autnum](#autnum) [R]
- [Nameserver](#nameserver) [R]
- [Domain](#domain) [R]
- [Entity](#entity) [R]

---

## Resources


### Domainsrdap

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rdap_conformance` | Vec<String> | RDAP conformance level. |
| `title` | String | Error title. |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `description` | Vec<String> | Error description. |


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
domainsrdap_rdap_conformance = domainsrdap.rdap_conformance
domainsrdap_title = domainsrdap.title
domainsrdap_json_response = domainsrdap.json_response
domainsrdap_error_code = domainsrdap.error_code
domainsrdap_lang = domainsrdap.lang
domainsrdap_notices = domainsrdap.notices
domainsrdap_description = domainsrdap.description
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
| `rdap_conformance` | Vec<String> | RDAP conformance level. |
| `title` | String | Error title. |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `description` | Vec<String> | Error description. |


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
ip_rdap_conformance = ip.rdap_conformance
ip_title = ip.title
ip_json_response = ip.json_response
ip_error_code = ip.error_code
ip_lang = ip.lang
ip_notices = ip.notices
ip_description = ip.description
```

---


### Autnum

The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rdap_conformance` | Vec<String> | RDAP conformance level. |
| `title` | String | Error title. |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `description` | Vec<String> | Error description. |


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
autnum_rdap_conformance = autnum.rdap_conformance
autnum_title = autnum.title
autnum_json_response = autnum.json_response
autnum_error_code = autnum.error_code
autnum_lang = autnum.lang
autnum_notices = autnum.notices
autnum_description = autnum.description
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
| `rdap_conformance` | Vec<String> | RDAP conformance level. |
| `title` | String | Error title. |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `description` | Vec<String> | Error description. |


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
nameserver_rdap_conformance = nameserver.rdap_conformance
nameserver_title = nameserver.title
nameserver_json_response = nameserver.json_response
nameserver_error_code = nameserver.error_code
nameserver_lang = nameserver.lang
nameserver_notices = nameserver.notices
nameserver_description = nameserver.description
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

# Access domain outputs
domain_id = domain.id
domain_extensions = domain.extensions
domain_content_type = domain.content_type
domain_data = domain.data
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
| `rdap_conformance` | Vec<String> | RDAP conformance level. |
| `title` | String | Error title. |
| `json_response` | String | HTTP response with content type set to "application/json+rdap". |
| `error_code` | i64 | Error HTTP code. Example: "501". |
| `lang` | String | Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6). |
| `notices` | Vec<String> | Notices applying to this response. |
| `description` | Vec<String> | Error description. |


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
entity_rdap_conformance = entity.rdap_conformance
entity_title = entity.title
entity_json_response = entity.json_response
entity_error_code = entity.error_code
entity_lang = entity.lang
entity_notices = entity.notices
entity_description = entity.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple domainsrdap resources
domainsrdap_0 = provider.domainsrdap_api.Domainsrdap {
}
domainsrdap_1 = provider.domainsrdap_api.Domainsrdap {
}
domainsrdap_2 = provider.domainsrdap_api.Domainsrdap {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domainsrdap = provider.domainsrdap_api.Domainsrdap {
    }
```

---

## Related Documentation

- [GCP Domainsrdap_api Documentation](https://cloud.google.com/domainsrdap_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
