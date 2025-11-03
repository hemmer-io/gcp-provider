# Oauth2_api Service



**Resources**: 6

---

## Overview

The oauth2_api service provides access to 6 resource types:

- [Me](#me) [R]
- [Oauth2](#oauth2) [C]
- [Userinfo](#userinfo) [R]
- [Userinfo](#userinfo) [R]
- [Me](#me) [R]
- [Oauth2](#oauth2) [C]

---

## Resources


### Me

Get user info

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `email` | String | The user's email address. |
| `link` | String | URL of the profile page. |
| `given_name` | String | The user's first name. |
| `picture` | String | URL of the user's picture image. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `family_name` | String | The user's last name. |
| `gender` | String | The user's gender. |
| `locale` | String | The user's preferred locale. |
| `name` | String | The user's full name. |
| `id` | String | The obfuscated ID of the user. |


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
me_email = me.email
me_link = me.link
me_given_name = me.given_name
me_picture = me.picture
me_verified_email = me.verified_email
me_family_name = me.family_name
me_gender = me.gender
me_locale = me.locale
me_name = me.name
me_id = me.id
```

---


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
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |
| `email` | String | The user's email address. |
| `link` | String | URL of the profile page. |
| `given_name` | String | The user's first name. |
| `picture` | String | URL of the user's picture image. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `family_name` | String | The user's last name. |
| `gender` | String | The user's gender. |
| `locale` | String | The user's preferred locale. |
| `name` | String | The user's full name. |
| `id` | String | The obfuscated ID of the user. |


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
userinfo_email = userinfo.email
userinfo_link = userinfo.link
userinfo_given_name = userinfo.given_name
userinfo_picture = userinfo.picture
userinfo_verified_email = userinfo.verified_email
userinfo_family_name = userinfo.family_name
userinfo_gender = userinfo.gender
userinfo_locale = userinfo.locale
userinfo_name = userinfo.name
userinfo_id = userinfo.id
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
| `gender` | String | The user's gender. |
| `family_name` | String | The user's last name. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `given_name` | String | The user's first name. |
| `name` | String | The user's full name. |
| `picture` | String | URL of the user's picture image. |
| `email` | String | The user's email address. |
| `id` | String | The obfuscated ID of the user. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |


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
userinfo_gender = userinfo.gender
userinfo_family_name = userinfo.family_name
userinfo_verified_email = userinfo.verified_email
userinfo_given_name = userinfo.given_name
userinfo_name = userinfo.name
userinfo_picture = userinfo.picture
userinfo_email = userinfo.email
userinfo_id = userinfo.id
userinfo_link = userinfo.link
userinfo_locale = userinfo.locale
userinfo_hd = userinfo.hd
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
| `gender` | String | The user's gender. |
| `family_name` | String | The user's last name. |
| `verified_email` | bool | Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address. |
| `given_name` | String | The user's first name. |
| `name` | String | The user's full name. |
| `picture` | String | URL of the user's picture image. |
| `email` | String | The user's email address. |
| `id` | String | The obfuscated ID of the user. |
| `link` | String | URL of the profile page. |
| `locale` | String | The user's preferred locale. |
| `hd` | String | The hosted domain e.g. example.com if the user is Google apps user. |


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
me_gender = me.gender
me_family_name = me.family_name
me_verified_email = me.verified_email
me_given_name = me.given_name
me_name = me.name
me_picture = me.picture
me_email = me.email
me_id = me.id
me_link = me.link
me_locale = me.locale
me_hd = me.hd
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

# Create multiple me resources
me_0 = provider.oauth2_api.Me {
}
me_1 = provider.oauth2_api.Me {
}
me_2 = provider.oauth2_api.Me {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    me = provider.oauth2_api.Me {
    }
```

---

## Related Documentation

- [GCP Oauth2_api Documentation](https://cloud.google.com/oauth2_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
