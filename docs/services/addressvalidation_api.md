# Addressvalidation_api Service



**Resources**: 1

---

## Overview

The addressvalidation_api service provides access to 1 resource type:

- [Addressvalidation](#addressvalidation) [C]

---

## Resources


### Addressvalidation

Feedback about the outcome of the sequence of validation attempts. This should be the last call made after a sequence of validation calls for the same address, and should be called once the transaction is concluded. This should only be sent once for the sequence of `ValidateAddress` requests needed to validate an address fully.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conclusion` | String |  | Required. The outcome of the sequence of validation attempts. If this field is set to `VALIDATION_CONCLUSION_UNSPECIFIED`, an `INVALID_ARGUMENT` error will be returned. |
| `response_id` | String |  | Required. The ID of the response that this feedback is for. This should be the response_id from the first response in a series of address validation attempts. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create addressvalidation
addressvalidation = provider.addressvalidation_api.Addressvalidation {
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

# Create multiple addressvalidation resources
addressvalidation_0 = provider.addressvalidation_api.Addressvalidation {
}
addressvalidation_1 = provider.addressvalidation_api.Addressvalidation {
}
addressvalidation_2 = provider.addressvalidation_api.Addressvalidation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    addressvalidation = provider.addressvalidation_api.Addressvalidation {
    }
```

---

## Related Documentation

- [GCP Addressvalidation_api Documentation](https://cloud.google.com/addressvalidation_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
