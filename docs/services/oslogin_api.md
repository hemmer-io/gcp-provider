# Oslogin_api Service



**Resources**: 14

---

## Overview

The oslogin_api service provides access to 14 resource types:

- [User](#user) [CR]
- [Ssh_public_key](#ssh_public_key) [CRUD]
- [Location](#location) [C]
- [Project](#project) [D]
- [User](#user) [CR]
- [Ssh_public_key](#ssh_public_key) [CRUD]
- [Zone](#zone) [C]
- [Project](#project) [CD]
- [Location](#location) [C]
- [User](#user) [CR]
- [Location](#location) [C]
- [Ssh_public_key](#ssh_public_key) [CRUD]
- [Zone](#zone) [C]
- [Project](#project) [CD]

---

## Resources


### User

Adds an SSH public key and returns the profile information. Default POSIX account information is set when no username and UID exist as part of the login profile.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `name` | String |  | Output only. The canonical resource name. |
| `parent` | String | ✅ | Required. The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ssh_public_keys` | HashMap<String, String> | A map from SSH public key fingerprint to the associated key object. |
| `name` | String | Required. A unique user ID. |
| `posix_accounts` | Vec<String> | The list of POSIX accounts associated with the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.oslogin_api.User {
    parent = "value"  # Required. The unique ID for the user in format `users/{user}`.
}

# Access user outputs
user_id = user.id
user_ssh_public_keys = user.ssh_public_keys
user_name = user.name
user_posix_accounts = user.posix_accounts
```

---


### Ssh_public_key

Create an SSH public key

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `name` | String |  | Output only. The canonical resource name. |
| `parent` | String | ✅ | Required. The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key` | String | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String | An expiration time in microseconds since epoch. |
| `fingerprint` | String | Output only. The SHA-256 fingerprint of the SSH public key. |
| `name` | String | Output only. The canonical resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssh_public_key
ssh_public_key = provider.oslogin_api.Ssh_public_key {
    parent = "value"  # Required. The unique ID for the user in format `users/{user}`.
}

# Access ssh_public_key outputs
ssh_public_key_id = ssh_public_key.id
ssh_public_key_key = ssh_public_key.key
ssh_public_key_expiration_time_usec = ssh_public_key.expiration_time_usec
ssh_public_key_fingerprint = ssh_public_key.fingerprint
ssh_public_key_name = ssh_public_key.name
```

---


### Location

Signs an SSH public key for a user to authenticate to a virtual machine on Google Compute Engine.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | Optional. The service account for the instance. If the instance in question does not have a service account, this field should be left empty. If the wrong service account is provided, this operation will return a signed certificate that will not be accepted by the VM. |
| `ssh_public_key` | String |  | Required. The SSH public key to sign. |
| `compute_instance` | String |  | The Compute instance to sign the SSH public key for. Expected format: projects/{project}/zones/{zone}/instances/{numeric_instance_id} |
| `app_engine_instance` | String |  | The App Engine instance to sign the SSH public key for. Expected format: apps/{app}/services/{service}/versions/{version}/instances/{instance} |
| `parent` | String | ✅ | Required. The parent for the signing request. Format: projects/{project}/locations/{location} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.oslogin_api.Location {
    parent = "value"  # Required. The parent for the signing request. Format: projects/{project}/locations/{location}
}

```

---


### Project



**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### User

Adds an SSH public key and returns the profile information. Default POSIX account information is set when no username and UID exist as part of the login profile.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `name` | String |  | Output only. The canonical resource name. |
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `parent` | String | ✅ | The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_keys` | Vec<String> | The registered security key credentials for a user. |
| `name` | String | Required. A unique user ID. |
| `posix_accounts` | Vec<String> | The list of POSIX accounts associated with the user. |
| `ssh_public_keys` | HashMap<String, String> | A map from SSH public key fingerprint to the associated key object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.oslogin_api.User {
    parent = "value"  # The unique ID for the user in format `users/{user}`.
}

# Access user outputs
user_id = user.id
user_security_keys = user.security_keys
user_name = user.name
user_posix_accounts = user.posix_accounts
user_ssh_public_keys = user.ssh_public_keys
```

---


### Ssh_public_key

Create an SSH public key

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `name` | String |  | Output only. The canonical resource name. |
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `parent` | String | ✅ | Required. The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expiration_time_usec` | String | An expiration time in microseconds since epoch. |
| `name` | String | Output only. The canonical resource name. |
| `key` | String | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `fingerprint` | String | Output only. The SHA-256 fingerprint of the SSH public key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssh_public_key
ssh_public_key = provider.oslogin_api.Ssh_public_key {
    parent = "value"  # Required. The unique ID for the user in format `users/{user}`.
}

# Access ssh_public_key outputs
ssh_public_key_id = ssh_public_key.id
ssh_public_key_expiration_time_usec = ssh_public_key.expiration_time_usec
ssh_public_key_name = ssh_public_key.name
ssh_public_key_key = ssh_public_key.key
ssh_public_key_fingerprint = ssh_public_key.fingerprint
```

---


### Zone

Signs an SSH public key for a user to authenticate to a virtual machine on Google Compute Engine.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_public_key` | String |  | Required. The SSH public key to sign. |
| `parent` | String | ✅ | Required. The parent project and region for the signing request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create zone
zone = provider.oslogin_api.Zone {
    parent = "value"  # Required. The parent project and region for the signing request.
}

```

---


### Project

Create a POSIX account if it doesn't exist.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `regions` | Vec<String> |  | Optional. The regions to wait for a POSIX account to be written to before returning a response. If unspecified, defaults to all regions. Regions are listed at https://cloud.google.com/about/locations#region. |
| `name` | String | ✅ | Required. The unique ID for the user in format `users/{user}/projects/{project}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.oslogin_api.Project {
    name = "value"  # Required. The unique ID for the user in format `users/{user}/projects/{project}`.
}

```

---


### Location

Signs an SSH public key for a user to authenticate to a virtual machine on Google Compute Engine.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_public_key` | String |  | Required. The SSH public key to sign. |
| `compute_instance` | String |  | The Compute instance to sign the SSH public key for. Expected format: projects/{project}/zones/{zone}/instances/{numeric_instance_id} |
| `service_account` | String |  | Optional. The service account for the instance. If the instance in question does not have a service account, this field should be left empty. If the wrong service account is provided, this operation will return a signed certificate that will not be accepted by the VM. |
| `app_engine_instance` | String |  | The App Engine instance to sign the SSH public key for. Expected format: apps/{app}/services/{service}/versions/{version}/instances/{instance} |
| `parent` | String | ✅ | Required. The parent for the signing request. Format: projects/{project}/locations/{location} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.oslogin_api.Location {
    parent = "value"  # Required. The parent for the signing request. Format: projects/{project}/locations/{location}
}

```

---


### User

Adds an SSH public key and returns the profile information. Default POSIX account information is set when no username and UID exist as part of the login profile.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `name` | String |  | Output only. The canonical resource name. |
| `parent` | String | ✅ | The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `posix_accounts` | Vec<String> | The list of POSIX accounts associated with the user. |
| `name` | String | Required. A unique user ID. |
| `security_keys` | Vec<String> | The registered security key credentials for a user. |
| `ssh_public_keys` | HashMap<String, String> | A map from SSH public key fingerprint to the associated key object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.oslogin_api.User {
    parent = "value"  # The unique ID for the user in format `users/{user}`.
}

# Access user outputs
user_id = user.id
user_posix_accounts = user.posix_accounts
user_name = user.name
user_security_keys = user.security_keys
user_ssh_public_keys = user.ssh_public_keys
```

---


### Location

Signs an SSH public key for a user to authenticate to an instance.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_public_key` | String |  | Required. The SSH public key to sign. |
| `parent` | String | ✅ | Required. The parent project and region for the signing request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.oslogin_api.Location {
    parent = "value"  # Required. The parent project and region for the signing request.
}

```

---


### Ssh_public_key

Create an SSH public key

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | Output only. The SHA-256 fingerprint of the SSH public key. |
| `key` | String |  | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String |  | An expiration time in microseconds since epoch. |
| `name` | String |  | Output only. The canonical resource name. |
| `parent` | String | ✅ | Required. The unique ID for the user in format `users/{user}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | Output only. The SHA-256 fingerprint of the SSH public key. |
| `key` | String | Required. Public key text in SSH format, defined by [RFC4253](https://www.ietf.org/rfc/rfc4253.txt) section 6.6. |
| `expiration_time_usec` | String | An expiration time in microseconds since epoch. |
| `name` | String | Output only. The canonical resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ssh_public_key
ssh_public_key = provider.oslogin_api.Ssh_public_key {
    parent = "value"  # Required. The unique ID for the user in format `users/{user}`.
}

# Access ssh_public_key outputs
ssh_public_key_id = ssh_public_key.id
ssh_public_key_fingerprint = ssh_public_key.fingerprint
ssh_public_key_key = ssh_public_key.key
ssh_public_key_expiration_time_usec = ssh_public_key.expiration_time_usec
ssh_public_key_name = ssh_public_key.name
```

---


### Zone

Signs an SSH public key for a user to authenticate to an instance.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_public_key` | String |  | Required. The SSH public key to sign. |
| `parent` | String | ✅ | Required. The parent project and region for the signing request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create zone
zone = provider.oslogin_api.Zone {
    parent = "value"  # Required. The parent project and region for the signing request.
}

```

---


### Project

Create a POSIX account if it doesn't exist.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `regions` | Vec<String> |  | Optional. The regions to wait for a POSIX account to be written to before returning a response. If unspecified, defaults to all regions. Regions are listed at https://cloud.google.com/about/locations#region. |
| `name` | String | ✅ | Required. The unique ID for the user in format `users/{user}/projects/{project}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.oslogin_api.Project {
    name = "value"  # Required. The unique ID for the user in format `users/{user}/projects/{project}`.
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

# Create multiple user resources
user_0 = provider.oslogin_api.User {
    parent = "value-0"
}
user_1 = provider.oslogin_api.User {
    parent = "value-1"
}
user_2 = provider.oslogin_api.User {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user = provider.oslogin_api.User {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Oslogin_api Documentation](https://cloud.google.com/oslogin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
