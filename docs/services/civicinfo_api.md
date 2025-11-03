# Civicinfo_api Service



**Resources**: 2

---

## Overview

The civicinfo_api service provides access to 2 resource types:

- [Election](#election) [R]
- [Division](#division) [R]

---

## Resources


### Election

List of available elections to query.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `elections` | Vec<String> | A list of available elections |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "civicinfo#electionsQueryResponse". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access election outputs
election_id = election.id
election_elections = election.elections
election_kind = election.kind
```

---


### Division

Lookup OCDIDs and names for divisions related to an address.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `normalized_input` | String | The normalized version of the requested address. |
| `divisions` | HashMap<String, String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access division outputs
division_id = division.id
division_normalized_input = division.normalized_input
division_divisions = division.divisions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple election resources
election_0 = provider.civicinfo_api.Election {
}
election_1 = provider.civicinfo_api.Election {
}
election_2 = provider.civicinfo_api.Election {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    election = provider.civicinfo_api.Election {
    }
```

---

## Related Documentation

- [GCP Civicinfo_api Documentation](https://cloud.google.com/civicinfo_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
