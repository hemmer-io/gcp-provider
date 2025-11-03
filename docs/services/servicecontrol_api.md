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

Checks whether an operation on a service should be allowed to proceed based on the configuration of the service and related policies. It must be called before the operation is executed. If feasible, the client should cache the check results and reuse them for 60 seconds. In case of any server errors, the client should rely on the cached results for much longer time to avoid outage. WARNING: There is general 60s delay for the configuration and policy propagation, therefore callers MUST NOT depend on the `Check` method having the latest policy information. NOTE: the CheckRequest has the size limit (wire-format byte size) of 1MB. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see [Cloud IAM](https://cloud.google.com/iam).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_project_settings` | bool |  | Requests the project settings to be returned as part of the check response. |
| `operation` | String |  | The operation to be checked. |
| `skip_activation_check` | bool |  | Indicates if service activation check should be skipped for this request. Default behavior is to perform the check and apply relevant quota. WARNING: Setting this flag to "true" will disable quota enforcement. |
| `service_config_id` | String |  | Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used. |
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


### Service

This method provides telemetry reporting for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It reports a list of operations that have occurred on a service. It must be called after the operations have been executed. For more information, see [Telemetry Reporting](https://cloud.google.com/service-infrastructure/docs/telemetry-reporting). NOTE: The telemetry reporting has a hard limit of 100 operations and 1MB per Report call. This method requires the `servicemanagement.services.report` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operations` | Vec<String> |  | Describes the list of operations to be reported. Each operation is represented as an AttributeContext, and contains all attributes around an API access. |
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
