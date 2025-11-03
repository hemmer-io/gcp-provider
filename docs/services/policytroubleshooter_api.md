# Policytroubleshooter_api Service



**Resources**: 2

---

## Overview

The policytroubleshooter_api service provides access to 2 resource types:

- [Iam](#iam) [C]
- [Iam](#iam) [C]

---

## Resources


### Iam

Checks whether a principal has a specific permission for a specific resource, and explains why the principal does or does not have that permission.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_tuple` | String |  | The information to use for checking whether a principal has a permission for a resource. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create iam
iam = provider.policytroubleshooter_api.Iam {
}

```

---


### Iam

Checks whether a member has a specific permission for a specific resource, and explains why the member does or does not have that permission.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_tuple` | String |  | The information to use for checking whether a member has a permission for a resource. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create iam
iam = provider.policytroubleshooter_api.Iam {
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

# Create multiple iam resources
iam_0 = provider.policytroubleshooter_api.Iam {
}
iam_1 = provider.policytroubleshooter_api.Iam {
}
iam_2 = provider.policytroubleshooter_api.Iam {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    iam = provider.policytroubleshooter_api.Iam {
    }
```

---

## Related Documentation

- [GCP Policytroubleshooter_api Documentation](https://cloud.google.com/policytroubleshooter_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
