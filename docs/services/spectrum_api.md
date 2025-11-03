# Spectrum_api Service



**Resources**: 1

---

## Overview

The spectrum_api service provides access to 1 resource type:

- [Paw](#paw) [C]

---

## Resources


### Paw

The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_type` | String |  | The request type parameter is an optional parameter that can be used to modify an available spectrum batch request, but its use depends on applicable regulatory rules. For example, It may be used to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the device descriptor parameter for the device on whose behalf the batch request is made is required. |
| `type` | String |  | The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).

Required field. |
| `capabilities` | String |  | The master device may include its device capabilities to limit the available-spectrum batch response to the spectrum that is compatible with its capabilities. The database should not return spectrum that is incompatible with the specified capabilities. |
| `master_device_desc` | String |  | When an available spectrum batch request is made by the master device (a device with geolocation capability) on behalf of a slave device (a device without geolocation capability), the rules of the applicable regulatory domain may require the master device to provide its own device descriptor information (in addition to device descriptor information for the slave device in a separate parameter). |
| `locations` | Vec<String> |  | A geolocation list is required. This allows a device to specify its current location plus additional anticipated locations when allowed by the regulatory domain. At least one location must be included. Geolocation must be given as the location of the radiation center of the device's antenna. If a location specifies a region, rather than a point, the database may return an UNIMPLEMENTED error if it does not support query by region.

There is no upper limit on the number of locations included in a available spectrum batch request, but the database may restrict the number of locations it supports by returning a response with fewer locations than specified in the batch request. Note that geolocations must be those of the master device (a device with geolocation capability that makes an available spectrum batch request), whether the master device is making the request on its own behalf or on behalf of a slave device (one without geolocation capability). |
| `antenna` | String |  | Depending on device type and regulatory domain, antenna characteristics may be required. |
| `device_desc` | String |  | When the available spectrum request is made on behalf of a specific device (a master or slave device), device descriptor information for the device on whose behalf the request is made is required (in such cases, the requestType parameter must be empty). When a requestType value is specified, device descriptor information may be optional or required according to the rules of the applicable regulatory domain. |
| `owner` | String |  | Depending on device type and regulatory domain, device owner information may be included in an available spectrum batch request. This allows the device to register and get spectrum-availability information in a single request. |
| `version` | String |  | The PAWS version. Must be exactly 1.0.

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
