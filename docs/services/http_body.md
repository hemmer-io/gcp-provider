# Http_body Service



**Resources**: 1

---

## Overview

The http_body service provides access to 1 resource type:

- [Fhir](#fhir) [CR]

---

## Resources


### Fhir

Creates a FHIR resource.


**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response
for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `parent` | String | ✅ | The name of the FHIR store this resource belongs to. |
| `type` | String | ✅ | The type of the resource to create. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response
for streaming APIs. |
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

# Create fhir
fhir = provider.http_body.Fhir {
    parent = "value"  # The name of the FHIR store this resource belongs to.
    type = "value"  # The type of the resource to create.
}

# Access fhir outputs
fhir_id = fhir.id
fhir_extensions = fhir.extensions
fhir_content_type = fhir.content_type
fhir_data = fhir.data
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple fhir resources
fhir_0 = provider.http_body.Fhir {
    parent = "value-0"
    type = "value-0"
}
fhir_1 = provider.http_body.Fhir {
    parent = "value-1"
    type = "value-1"
}
fhir_2 = provider.http_body.Fhir {
    parent = "value-2"
    type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    fhir = provider.http_body.Fhir {
        parent = "production-value"
        type = "production-value"
    }
```

---

## Related Documentation

- [GCP Http_body Documentation](https://cloud.google.com/http_body/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
