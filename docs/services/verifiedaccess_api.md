# Verifiedaccess_api Service



**Resources**: 2

---

## Overview

The verifiedaccess_api service provides access to 2 resource types:

- [Challenge](#challenge) [C]
- [Challenge](#challenge) [C]

---

## Resources


### Challenge

CreateChallenge API

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create challenge
challenge = provider.verifiedaccess_api.Challenge {
}

```

---


### Challenge

Generates a new challenge.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create challenge
challenge = provider.verifiedaccess_api.Challenge {
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

# Create multiple challenge resources
challenge_0 = provider.verifiedaccess_api.Challenge {
}
challenge_1 = provider.verifiedaccess_api.Challenge {
}
challenge_2 = provider.verifiedaccess_api.Challenge {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    challenge = provider.verifiedaccess_api.Challenge {
    }
```

---

## Related Documentation

- [GCP Verifiedaccess_api Documentation](https://cloud.google.com/verifiedaccess_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
