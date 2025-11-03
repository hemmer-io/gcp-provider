# Spectrum_api Service



**Resources**: 1

---

## Overview

The spectrum_api service provides access to 1 resource type:

- [Paw](#paw) [C]

---

## Resources


### Paw

Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | String |  | The PAWS version. Must be exactly 1.0.

Required field. |
| `request_type` | String |  | The request type parameter is an optional parameter that can be used to modify an available spectrum request, but its use depends on applicable regulatory rules. It may be used, for example, to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the deviceDesc parameter for the device on whose behalf the request is made is required. |
| `antenna` | String |  | Depending on device type and regulatory domain, the characteristics of the antenna may be required. |
| `capabilities` | String |  | The master device may include its device capabilities to limit the available-spectrum response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities. |
| `location` | String |  | The geolocation of the master device (a device with geolocation capability that makes an available spectrum request) is required whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability). The location must be the location of the radiation center of the master device's antenna. To support mobile devices, a regulatory domain may allow the anticipated position of the master device to be given instead. If the location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error code if it does not support query by region. |
| `master_device_desc` | String |  | When an available spectrum request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device, which is provided in a separate parameter). |
| `owner` | String |  | Depending on device type and regulatory domain, device owner information may be included in an available spectrum request. This allows the device to register and get spectrum-availability information in a single request. |
| `device_desc` | String |  | When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for that device is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain. |
| `type` | String |  | The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).

Required field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create paw
paw = provider.spectrum_api.Paw {
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

# Create multiple paw resources
paw_0 = provider.spectrum_api.Paw {
}
paw_1 = provider.spectrum_api.Paw {
}
paw_2 = provider.spectrum_api.Paw {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    paw = provider.spectrum_api.Paw {
    }
```

---

## Related Documentation

- [GCP Spectrum_api Documentation](https://cloud.google.com/spectrum_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
