# Iamcredentials_api Service



**Resources**: 1

---

## Overview

The iamcredentials_api service provides access to 1 resource type:

- [Service_account](#service_account) [C]

---

## Resources


### Service_account

Generates an OpenID Connect ID token for a service account.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_number_included` | bool |  | Include the organization number of the service account in the token. If set to `true`, the token will contain a `google.organization_number` claim. The value of the claim will be `null` if the service account isn't associated with an organization. |
| `delegates` | Vec<String> |  | The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid. |
| `include_email` | bool |  | Include the service account email in the token. If set to `true`, the token will contain `email` and `email_verified` claims. |
| `audience` | String |  | Required. The audience for the token, such as the API or account that this token grants access to. |
| `name` | String | ✅ | Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_account
service_account = provider.iamcredentials_api.Service_account {
    name = "value"  # Required. The resource name of the service account for which the credentials are requested, in the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid.
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

# Create multiple service_account resources
service_account_0 = provider.iamcredentials_api.Service_account {
    name = "value-0"
}
service_account_1 = provider.iamcredentials_api.Service_account {
    name = "value-1"
}
service_account_2 = provider.iamcredentials_api.Service_account {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_account = provider.iamcredentials_api.Service_account {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Iamcredentials_api Documentation](https://cloud.google.com/iamcredentials_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
