# Language_api Service



**Resources**: 4

---

## Overview

The language_api service provides access to 4 resource types:

- [Document](#document) [C]
- [Document](#document) [C]
- [Document](#document) [C]
- [Document](#document) [C]

---

## Resources


### Document

A convenience method that provides all the features that analyzeSentiment, analyzeEntities, and analyzeSyntax provide in one call.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document` | String |  | Required. Input document. |
| `encoding_type` | String |  | The encoding type used by the API to calculate offsets. |
| `features` | String |  | Required. The enabled features. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.language_api.Document {
}

```

---


### Document

Finds named entities (currently proper names and common nouns) in the text along with entity types, salience, mentions for each entity, and other properties.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encoding_type` | String |  | The encoding type used by the API to calculate offsets. |
| `document` | String |  | Required. Input document. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.language_api.Document {
}

```

---


### Document

Moderates a document for harmful and sensitive categories.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_version` | String |  | Optional. The model version to use for ModerateText. |
| `document` | String |  | Required. Input document. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.language_api.Document {
}

```

---


### Document

Analyzes the sentiment of the provided text.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document` | String |  | Input document. |
| `encoding_type` | String |  | The encoding type used by the API to calculate sentence offsets for the sentence sentiment. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.language_api.Document {
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple document resources
document_0 = provider.language_api.Document {
}
document_1 = provider.language_api.Document {
}
document_2 = provider.language_api.Document {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    document = provider.language_api.Document {
    }
```

---

## Related Documentation

- [GCP Language_api Documentation](https://cloud.google.com/language_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
