# Oauth2_api Service



**Resources**: 6

---

## Overview

The oauth2_api service provides access to 6 resource types:

- [Oauth2](#oauth2) [C]
- [Userinfo](#userinfo) [R]
- [Me](#me) [R]
- [Userinfo](#userinfo) [R]
- [Me](#me) [R]
- [Oauth2](#oauth2) [C]

---

## Resources


### Oauth2

Get token info

**Operations**: ✅ Create

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

# Create oauth2
oauth2 = provider.oauth2_api.Oauth2 {
}

```

---


### Userinfo

Get user info

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `picture` | String | URL of the user's picture image. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `email` | String | The user's email address. |
| `family_name` | String | The user's last name. |
| `given_name` | String | The user's first name. |
| `id` | String | The obfuscated ID of the user. |
| `name` | String | The user's full name. |
| `gender` | String | The user's gender. |
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access userinfo outputs
userinfo_id = userinfo.id
userinfo_picture = userinfo.picture
userinfo_verified_email = userinfo.verified_email
userinfo_email = userinfo.email
userinfo_family_name = userinfo.family_name
userinfo_given_name = userinfo.given_name
userinfo_id = userinfo.id
userinfo_name = userinfo.name
userinfo_gender = userinfo.gender
userinfo_hd = userinfo.hd
userinfo_link = userinfo.link
userinfo_locale = userinfo.locale
```

---


### Me

Get user info

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `picture` | String | URL of the user's picture image. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `email` | String | The user's email address. |
| `family_name` | String | The user's last name. |
| `given_name` | String | The user's first name. |
| `id` | String | The obfuscated ID of the user. |
| `name` | String | The user's full name. |
| `gender` | String | The user's gender. |
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access me outputs
me_id = me.id
me_picture = me.picture
me_verified_email = me.verified_email
me_email = me.email
me_family_name = me.family_name
me_given_name = me.given_name
me_id = me.id
me_name = me.name
me_gender = me.gender
me_hd = me.hd
me_link = me.link
me_locale = me.locale
```

---


### Userinfo



**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `gender` | String | The user's gender. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |
| `name` | String | The user's full name. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `id` | String | The obfuscated ID of the user. |
| `given_name` | String | The user's first name. |
| `email` | String | The user's email address. |
| `picture` | String | URL of the user's picture image. |
| `family_name` | String | The user's last name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access userinfo outputs
userinfo_id = userinfo.id
userinfo_hd = userinfo.hd
userinfo_gender = userinfo.gender
userinfo_link = userinfo.link
userinfo_locale = userinfo.locale
userinfo_name = userinfo.name
userinfo_verified_email = userinfo.verified_email
userinfo_id = userinfo.id
userinfo_given_name = userinfo.given_name
userinfo_email = userinfo.email
userinfo_picture = userinfo.picture
userinfo_family_name = userinfo.family_name
```

---


### Me



**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `gender` | String | The user's gender. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |
| `name` | String | The user's full name. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `id` | String | The obfuscated ID of the user. |
| `given_name` | String | The user's first name. |
| `email` | String | The user's email address. |
| `picture` | String | URL of the user's picture image. |
| `family_name` | String | The user's last name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access me outputs
me_id = me.id
me_hd = me.hd
me_gender = me.gender
me_link = me.link
me_locale = me.locale
me_name = me.name
me_verified_email = me.verified_email
me_id = me.id
me_given_name = me.given_name
me_email = me.email
me_picture = me.picture
me_family_name = me.family_name
```

---


### Oauth2



**Operations**: ✅ Create

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

# Create oauth2
oauth2 = provider.oauth2_api.Oauth2 {
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

# Create multiple oauth2 resources
oauth2_0 = provider.oauth2_api.Oauth2 {
}
oauth2_1 = provider.oauth2_api.Oauth2 {
}
oauth2_2 = provider.oauth2_api.Oauth2 {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    oauth2 = provider.oauth2_api.Oauth2 {
    }
```

---

## Related Documentation

- [GCP Oauth2_api Documentation](https://cloud.google.com/oauth2_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
