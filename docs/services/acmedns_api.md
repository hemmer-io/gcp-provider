# Acmedns_api Service



**Resources**: 1

---

## Overview

The acmedns_api service provides access to 1 resource type:

- [Acme_challenge_set](#acme_challenge_set) [CR]

---

## Resources


### Acme_challenge_set

Rotate the ACME challenges for a given domain name. By default, removes any challenges that are older than 30 days. Domain names must be provided in Punycode.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `records_to_remove` | Vec<String> |  | ACME TXT record challenges to remove. |
| `access_token` | String |  | Required. ACME DNS access token. This is a base64 token secret that is procured from the Google Domains website. It authorizes ACME TXT record updates for a domain. |
| `keep_expired_records` | bool |  | Keep records older than 30 days that were used for previous requests. |
| `records_to_add` | Vec<String> |  | ACME TXT record challenges to add. Supports multiple challenges on the same FQDN. |
| `root_domain` | String | ✅ | Required. SLD + TLD domain name to update records for. For example, this would be "google.com" for any FQDN under "google.com". That includes challenges for "subdomain.google.com". This MAY be Unicode or Punycode. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `record` | Vec<String> | The ACME challenges on the requested domain represented as individual TXT records. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create acme_challenge_set
acme_challenge_set = provider.acmedns_api.Acme_challenge_set {
    root_domain = "value"  # Required. SLD + TLD domain name to update records for. For example, this would be "google.com" for any FQDN under "google.com". That includes challenges for "subdomain.google.com". This MAY be Unicode or Punycode.
}

# Access acme_challenge_set outputs
acme_challenge_set_id = acme_challenge_set.id
acme_challenge_set_record = acme_challenge_set.record
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple acme_challenge_set resources
acme_challenge_set_0 = provider.acmedns_api.Acme_challenge_set {
    root_domain = "value-0"
}
acme_challenge_set_1 = provider.acmedns_api.Acme_challenge_set {
    root_domain = "value-1"
}
acme_challenge_set_2 = provider.acmedns_api.Acme_challenge_set {
    root_domain = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    acme_challenge_set = provider.acmedns_api.Acme_challenge_set {
        root_domain = "production-value"
    }
```

---

## Related Documentation

- [GCP Acmedns_api Documentation](https://cloud.google.com/acmedns_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
