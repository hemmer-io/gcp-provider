# Serviceuser_api Service



**Resources**: 1

---

## Overview

The serviceuser_api service provides access to 1 resource type:

- [Service](#service) [CR]

---

## Resources


### Service

Enable a service so it can be used with a project.
See [Cloud Auth Guide](https://cloud.google.com/docs/authentication) for
more information.

Operation<response: google.protobuf.Empty>

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Name of the consumer and the service to enable for that consumer.

A valid path would be:
- projects/my-project/services/servicemanagement.googleapis.com |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token that can be passed to `ListAvailableServices` to resume a paginated
query. |
| `services` | Vec<String> | Services available publicly or available to the authenticated caller. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.serviceuser_api.Service {
    name = "value"  # Name of the consumer and the service to enable for that consumer.

A valid path would be:
- projects/my-project/services/servicemanagement.googleapis.com
}

# Access service outputs
service_id = service.id
service_next_page_token = service.next_page_token
service_services = service.services
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple service resources
service_0 = provider.serviceuser_api.Service {
    name = "value-0"
}
service_1 = provider.serviceuser_api.Service {
    name = "value-1"
}
service_2 = provider.serviceuser_api.Service {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service = provider.serviceuser_api.Service {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Serviceuser_api Documentation](https://cloud.google.com/serviceuser_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
