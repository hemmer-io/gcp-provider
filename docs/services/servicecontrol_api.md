# Servicecontrol_api Service



**Resources**: 2

---

## Overview

The servicecontrol_api service provides access to 2 resource types:

- [Service](#service) [C]
- [Service](#service) [C]

---

## Resources


### Service

Attempts to allocate quota for the specified consumer. It should be called before the operation is executed. This method requires the `servicemanagement.services.quota` permission on the specified service. For more information, see [Cloud IAM](https://cloud.google.com/iam). **NOTE:** The client **must** fail-open on server errors `INTERNAL`, `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system reliability, the server may inject these errors to prohibit any hard dependency on the quota functionality.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_config_id` | String |  | Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used. |
| `allocate_operation` | String |  | Operation that describes the quota allocation. |
| `service_name` | String | ✅ | Name of the service as specified in the service configuration. For example, `"pubsub.googleapis.com"`. See google.api.Service for the definition of a service name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicecontrol_api.Service {
    service_name = "value"  # Name of the service as specified in the service configuration. For example, `"pubsub.googleapis.com"`. See google.api.Service for the definition of a service name.
}

```

---


### Service

This method provides admission control for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It checks whether an operation should be allowed based on the service configuration and relevant policies. It must be called before the operation is executed. For more information, see [Admission Control](https://cloud.google.com/service-infrastructure/docs/admission-control). NOTE: The admission control has an expected policy propagation delay of 60s. The caller **must** not depend on the most recent policy changes. NOTE: The admission control has a hard limit of 1 referenced resources per call. If an operation refers to more than 1 resources, the caller must call the Check method multiple times. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> |  | Describes the resources and the policies applied to each resource. |
| `attributes` | String |  | Describes attributes about the operation being executed by the service. |
| `flags` | String |  | Optional. Contains a comma-separated list of flags. |
| `service_config_id` | String |  | Specifies the version of the service configuration that should be used to process the request. Must not be empty. Set this field to 'latest' to specify using the latest configuration. |
| `service_name` | String | ✅ | The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicecontrol_api.Service {
    service_name = "value"  # The service name as specified in its service configuration. For example, `"pubsub.googleapis.com"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name.
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

# Create multiple service resources
service_0 = provider.servicecontrol_api.Service {
    service_name = "value-0"
}
service_1 = provider.servicecontrol_api.Service {
    service_name = "value-1"
}
service_2 = provider.servicecontrol_api.Service {
    service_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service = provider.servicecontrol_api.Service {
        service_name = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicecontrol_api Documentation](https://cloud.google.com/servicecontrol_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
