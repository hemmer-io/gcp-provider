# Proximitybeacon_api Service



**Resources**: 6

---

## Overview

The proximitybeacon_api service provides access to 6 resource types:

- [Proximitybeacon](#proximitybeacon) [R]
- [Beaconinfo](#beaconinfo) [C]
- [Attachment](#attachment) [CRD]
- [Namespace](#namespace) [RU]
- [Beacon](#beacon) [CRUD]
- [Diagnostic](#diagnostic) [R]

---

## Resources


### Proximitybeacon

Gets the Proximity Beacon API's current public key and associated
parameters used to initiate the Diffie-Hellman key exchange required to
register a beacon that broadcasts the Eddystone-EID format. This key
changes periodically; clients may cache it and re-use the same public key
to provision and register multiple beacons. However, clients should be
prepared to refresh this key when they encounter an error registering an
Eddystone-EID beacon.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_ecdh_public_key` | String | The beacon service's public key for use by a beacon to derive its
Identity Key using Elliptic Curve Diffie-Hellman key exchange. |
| `max_rotation_period_exponent` | i64 | Indicates the maximum rotation period supported by the service.
See
EddystoneEidRegistration.rotation_period_exponent |
| `min_rotation_period_exponent` | i64 | Indicates the minimum rotation period supported by the service.
See
EddystoneEidRegistration.rotation_period_exponent |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access proximitybeacon outputs
proximitybeacon_id = proximitybeacon.id
proximitybeacon_service_ecdh_public_key = proximitybeacon.service_ecdh_public_key
proximitybeacon_max_rotation_period_exponent = proximitybeacon.max_rotation_period_exponent
proximitybeacon_min_rotation_period_exponent = proximitybeacon.min_rotation_period_exponent
```

---


### Beaconinfo

Given one or more beacon observations, returns any beacon information
and attachments accessible to your application. Authorize by using the
[API
key](https://developers.google.com/beacons/proximity/get-started#request_a_browser_api_key)
for the application.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `observations` | Vec<String> |  | The beacons that the client has encountered.
At least one must be given. |
| `namespaced_types` | Vec<String> |  | Specifies what kind of attachments to include in the response.
When given, the response will include only attachments of the given types.
When empty, no attachments will be returned. Must be in the format
<var>namespace/type</var>. Accepts `*` to specify all types in
all namespaces owned by the client.
Optional. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create beaconinfo
beaconinfo = provider.proximitybeacon_api.Beaconinfo {
}

```

---


### Attachment

Associates the given data with the specified beacon. Attachment data must
contain two parts:
<ul>
<li>A namespaced type.</li>
<li>The actual attachment data itself.</li>
</ul>
The namespaced type consists of two parts, the namespace and the type.
The namespace must be one of the values returned by the `namespaces`
endpoint, while the type can be a string of any characters except for the
forward slash (`/`) up to 100 characters in length.

Attachment data can be up to 1024 bytes long.

Authenticate using an [OAuth access
token](https://developers.google.com/identity/protocols/OAuth2) from a
signed-in user with **Is owner** or **Can edit** permissions in the Google
Developers Console project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment_name` | String |  | Resource name of this attachment. Attachment names have the format:
<code>beacons/<var>beacon_id</var>/attachments/<var>attachment_id</var></code>.
Leave this empty on creation. |
| `data` | String |  | An opaque data container for client-provided data. Must be
[base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
requests, and will be so encoded (with padding) in responses.
Required. |
| `max_distance_meters` | f64 |  | The distance away from the beacon at which this attachment should be
delivered to a mobile app.

Setting this to a value greater than zero indicates that the app should
behave as if the beacon is "seen" when the mobile device is less than this
distance away from the beacon.

Different attachments on the same beacon can have different max distances.

Note that even though this value is expressed with fractional meter
precision, real-world behavior is likley to be much less precise than one
meter, due to the nature of current Bluetooth radio technology.

Optional. When not set or zero, the attachment should be delivered at the
beacon's outer limit of detection.

Negative values are invalid and return an error. |
| `namespaced_type` | String |  | Specifies what kind of attachment this is. Tells a client how to
interpret the `data` field. Format is <var>namespace/type</var>. Namespace
provides type separation between clients. Type describes the type of
`data`, for use by the client when parsing the `data` field.
Required. |
| `creation_time_ms` | String |  | The UTC time when this attachment was created, in milliseconds since the
UNIX epoch. |
| `beacon_name` | String | ✅ | Beacon on which the attachment should be created. A beacon name has the
format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
by the beacon and N is a code for the beacon's type. Possible values are
`3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
for AltBeacon. For Eddystone-EID beacons, you may use either the
current EID or the beacon's "stable" UID.
Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attachments` | Vec<String> | The attachments that corresponded to the request params. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attachment
attachment = provider.proximitybeacon_api.Attachment {
    beacon_name = "value"  # Beacon on which the attachment should be created. A beacon name has the
format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
by the beacon and N is a code for the beacon's type. Possible values are
`3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
for AltBeacon. For Eddystone-EID beacons, you may use either the
current EID or the beacon's "stable" UID.
Required.
}

# Access attachment outputs
attachment_id = attachment.id
attachment_attachments = attachment.attachments
```

---


### Namespace

Lists all attachment namespaces owned by your Google Developers Console
project. Attachment data associated with a beacon must include a
namespaced type, and the namespace must be owned by your project.

Authenticate using an [OAuth access
token](https://developers.google.com/identity/protocols/OAuth2) from a
signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
the Google Developers Console project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace_name` | String |  | Resource name of this namespace. Namespaces names have the format:
<code>namespaces/<var>namespace</var></code>. |
| `serving_visibility` | String |  | Specifies what clients may receive attachments under this namespace
via `beaconinfo.getforobserved`. |
| `namespace_name` | String | ✅ | Resource name of this namespace. Namespaces names have the format:
<code>namespaces/<var>namespace</var></code>. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `namespaces` | Vec<String> | The attachments that corresponded to the request params. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access namespace outputs
namespace_id = namespace.id
namespace_namespaces = namespace.namespaces
```

---


### Beacon

Decommissions the specified beacon in the service. This beacon will no
longer be returned from `beaconinfo.getforobserved`. This operation is
permanent -- you will not be able to re-register a beacon with this ID
again.

Authenticate using an [OAuth access
token](https://developers.google.com/identity/protocols/OAuth2) from a
signed-in user with **Is owner** or **Can edit** permissions in the Google
Developers Console project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `beacon_name` | String | ✅ | Beacon that should be decommissioned. A beacon name has the format
"beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
the beacon and N is a code for the beacon's type. Possible values are
`3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
for AltBeacon. For Eddystone-EID beacons, you may use either the
current EID of the beacon's "stable" UID.
Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `place_id` | String | The [Google Places API](/places/place-id) Place ID of the place where
the beacon is deployed. This is given when the beacon is registered or
updated, not automatically detected in any way.
Optional. |
| `indoor_level` | String | The indoor level information for this beacon, if known. As returned by the
Google Maps API.
Optional. |
| `provisioning_key` | String | Some beacons may require a user to provide an authorization key before
changing any of its configuration (e.g. broadcast frames, transmit power).
This field provides a place to store and control access to that key.
This field is populated in responses to `GET /v1beta1/beacons/3!beaconId`
from users with write access to the given beacon. That is to say: If the
user is authorized to write the beacon's confidential data in the service,
the service considers them authorized to configure the beacon. Note
that this key grants nothing on the service, only on the beacon itself. |
| `status` | String | Current status of the beacon.
Required. |
| `properties` | HashMap<String, String> | Properties of the beacon device, for example battery type or firmware
version.
Optional. |
| `ephemeral_id_registration` | String | Write-only registration parameters for beacons using Eddystone-EID
(remotely resolved ephemeral ID) format. This information will not be
populated in API responses. When submitting this data, the `advertised_id`
field must contain an ID of type Eddystone-UID. Any other ID type will
result in an error. |
| `expected_stability` | String | Expected location stability. This is set when the beacon is registered or
updated, not automatically detected in any way.
Optional. |
| `advertised_id` | String | The identifier of a beacon as advertised by it. This field must be
populated when registering. It may be empty when updating a beacon
record because it is ignored in updates.

When registering a beacon that broadcasts Eddystone-EID, this field
should contain a "stable" Eddystone-UID that identifies the beacon and
links it to its attachments. The stable Eddystone-UID is only used for
administering the beacon. |
| `description` | String | Free text used to identify and describe the beacon. Maximum length 140
characters.
Optional. |
| `lat_lng` | String | The location of the beacon, expressed as a latitude and longitude pair.
This location is given when the beacon is registered or updated. It does
not necessarily indicate the actual current location of the beacon.
Optional. |
| `beacon_name` | String | Resource name of this beacon. A beacon name has the format
"beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
the beacon and N is a code for the beacon's type. Possible values are
`3` for Eddystone, `1` for iBeacon, or `5` for AltBeacon.

This field must be left empty when registering. After reading a beacon,
clients can use the name for future operations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create beacon
beacon = provider.proximitybeacon_api.Beacon {
    beacon_name = "value"  # Beacon that should be decommissioned. A beacon name has the format
"beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
the beacon and N is a code for the beacon's type. Possible values are
`3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
for AltBeacon. For Eddystone-EID beacons, you may use either the
current EID of the beacon's "stable" UID.
Required.
}

# Access beacon outputs
beacon_id = beacon.id
beacon_place_id = beacon.place_id
beacon_indoor_level = beacon.indoor_level
beacon_provisioning_key = beacon.provisioning_key
beacon_status = beacon.status
beacon_properties = beacon.properties
beacon_ephemeral_id_registration = beacon.ephemeral_id_registration
beacon_expected_stability = beacon.expected_stability
beacon_advertised_id = beacon.advertised_id
beacon_description = beacon.description
beacon_lat_lng = beacon.lat_lng
beacon_beacon_name = beacon.beacon_name
```

---


### Diagnostic

List the diagnostics for a single beacon. You can also list diagnostics for
all the beacons owned by your Google Developers Console project by using
the beacon name `beacons/-`.

Authenticate using an [OAuth access
token](https://developers.google.com/identity/protocols/OAuth2) from a
signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
the Google Developers Console project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diagnostics` | Vec<String> | The diagnostics matching the given request. |
| `next_page_token` | String | Token that can be used for pagination. Returned only if the
request matches more beacons than can be returned in this response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access diagnostic outputs
diagnostic_id = diagnostic.id
diagnostic_diagnostics = diagnostic.diagnostics
diagnostic_next_page_token = diagnostic.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple proximitybeacon resources
proximitybeacon_0 = provider.proximitybeacon_api.Proximitybeacon {
}
proximitybeacon_1 = provider.proximitybeacon_api.Proximitybeacon {
}
proximitybeacon_2 = provider.proximitybeacon_api.Proximitybeacon {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    proximitybeacon = provider.proximitybeacon_api.Proximitybeacon {
    }
```

---

## Related Documentation

- [GCP Proximitybeacon_api Documentation](https://cloud.google.com/proximitybeacon_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
