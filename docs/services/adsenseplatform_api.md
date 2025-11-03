# Adsenseplatform_api Service



**Resources**: 8

---

## Overview

The adsenseplatform_api service provides access to 8 resource types:

- [Site](#site) [CRD]
- [Event](#event) [C]
- [Account](#account) [CR]
- [Site](#site) [CRUD]
- [Platform](#platform) [R]
- [Group](#group) [RU]
- [Event](#event) [C]
- [Account](#account) [CR]

---

## Resources


### Site

Creates a site for a specified account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of a site. Format: platforms/{platform}/accounts/{account}/sites/{site} |
| `state` | String |  | Output only. State of a site. |
| `domain` | String |  | Domain/sub-domain of the site. Must be a valid domain complying with [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt) and formatted as punycode [RFC 3492](https://www.ietf.org/rfc/rfc3492.txt) in case the domain contains unicode characters. |
| `parent` | String | ✅ | Required. Account to create site. Format: platforms/{platform}/accounts/{account_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of a site. Format: platforms/{platform}/accounts/{account}/sites/{site} |
| `state` | String | Output only. State of a site. |
| `domain` | String | Domain/sub-domain of the site. Must be a valid domain complying with [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt) and formatted as punycode [RFC 3492](https://www.ietf.org/rfc/rfc3492.txt) in case the domain contains unicode characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site
site = provider.adsenseplatform_api.Site {
    parent = "value"  # Required. Account to create site. Format: platforms/{platform}/accounts/{account_id}
}

# Access site outputs
site_id = site.id
site_name = site.name
site_state = site.state
site_domain = site.domain
```

---


### Event

Creates an account event.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_info` | String |  | Required. Information associated with the event. |
| `event_type` | String |  | Required. Event type. |
| `event_time` | String |  | Required. Event timestamp. |
| `parent` | String | ✅ | Required. Account to log events about. Format: platforms/{platform}/accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.adsenseplatform_api.Event {
    parent = "value"  # Required. Account to log events about. Format: platforms/{platform}/accounts/{account}
}

```

---


### Account

Creates a sub-account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the account. Format: platforms/pub-[0-9]+/accounts/pub-[0-9]+ |
| `time_zone` | String |  | Required. The IANA TZ timezone code of this account. For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. This field is used for reporting. It is recommended to set it to the same value for all child accounts. |
| `display_name` | String |  | Display name of this account. |
| `creation_request_id` | String |  | Required. An opaque token that uniquely identifies the account among all the platform's accounts. This string may contain at most 64 non-whitespace ASCII characters, but otherwise has no predefined structure. However, it is expected to be a platform-specific identifier for the user creating the account, so that only a single account can be created for any given user. This field must not contain any information that is recognizable as personally identifiable information. e.g. it should not be an email address or login name. Once an account has been created, a second attempt to create an account using the same creation_request_id will result in an ALREADY_EXISTS error. |
| `region_code` | String |  | Required. Input only. CLDR region code of the country/region of the address. Set this to country code of the child account if known, otherwise to your own country code. |
| `create_time` | String |  | Output only. Creation time of the account. |
| `state` | String |  | Output only. Approval state of the account. |
| `parent` | String | ✅ | Required. Platform to create an account for. Format: platforms/{platform} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the account. Format: platforms/pub-[0-9]+/accounts/pub-[0-9]+ |
| `time_zone` | String | Required. The IANA TZ timezone code of this account. For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. This field is used for reporting. It is recommended to set it to the same value for all child accounts. |
| `display_name` | String | Display name of this account. |
| `creation_request_id` | String | Required. An opaque token that uniquely identifies the account among all the platform's accounts. This string may contain at most 64 non-whitespace ASCII characters, but otherwise has no predefined structure. However, it is expected to be a platform-specific identifier for the user creating the account, so that only a single account can be created for any given user. This field must not contain any information that is recognizable as personally identifiable information. e.g. it should not be an email address or login name. Once an account has been created, a second attempt to create an account using the same creation_request_id will result in an ALREADY_EXISTS error. |
| `region_code` | String | Required. Input only. CLDR region code of the country/region of the address. Set this to country code of the child account if known, otherwise to your own country code. |
| `create_time` | String | Output only. Creation time of the account. |
| `state` | String | Output only. Approval state of the account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.adsenseplatform_api.Account {
    parent = "value"  # Required. Platform to create an account for. Format: platforms/{platform}
}

# Access account outputs
account_id = account.id
account_name = account.name
account_time_zone = account.time_zone
account_display_name = account.display_name
account_creation_request_id = account.creation_request_id
account_region_code = account.region_code
account_create_time = account.create_time
account_state = account.state
```

---


### Site

Creates a site for a specified account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of a site. Format: platforms/{platform}/accounts/{account}/sites/{site} |
| `state` | String |  | Output only. State of a site. |
| `domain` | String |  | Domain/sub-domain of the site. Must be a valid domain complying with [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt) and formatted as punycode [RFC 3492](https://www.ietf.org/rfc/rfc3492.txt) in case the domain contains unicode characters. |
| `parent` | String | ✅ | Required. Account to create site. Format: platforms/{platform}/accounts/{account_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain` | String | Output only. Domain URL of the Platform Child Site. Part of the PlatformChildSite name. |
| `name` | String | Identifier. Format: accounts/{account}/platforms/{platform}/childAccounts/{child_account}/sites/{platform_child_site} |
| `platform_group` | String | Resource name of the Platform Group of the Platform Child Site. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site
site = provider.adsenseplatform_api.Site {
    parent = "value"  # Required. Account to create site. Format: platforms/{platform}/accounts/{account_id}
}

# Access site outputs
site_id = site.id
site_domain = site.domain
site_name = site.name
site_platform_group = site.platform_group
```

---


### Platform

Gets a platform.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Output only. Description of the platform. |
| `default_platform_group` | String | Default platform group for the platform. |
| `name` | String | Identifier. Resource name of a platform. Format: accounts/{account}/platforms/{platform} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access platform outputs
platform_id = platform.id
platform_description = platform.description
platform_default_platform_group = platform.default_platform_group
platform_name = platform.name
```

---


### Group

Gets a Platform Group for a specified Platform and group.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Format: accounts/{account}/platforms/{platform}/groups/{platform_group} |
| `description` | String |  | Required. Description of the PlatformGroup. |
| `revshare_millipercent` | String |  | Output only. The revenue share of the PlatformGroup, in millipercent (e.g. 15000 = 15%). |
| `name` | String | ✅ | Identifier. Format: accounts/{account}/platforms/{platform}/groups/{platform_group} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Format: accounts/{account}/platforms/{platform}/groups/{platform_group} |
| `description` | String | Required. Description of the PlatformGroup. |
| `revshare_millipercent` | String | Output only. The revenue share of the PlatformGroup, in millipercent (e.g. 15000 = 15%). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access group outputs
group_id = group.id
group_name = group.name
group_description = group.description
group_revshare_millipercent = group.revshare_millipercent
```

---


### Event

Creates an account event.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_type` | String |  | Required. Event type. |
| `event_time` | String |  | Required. Event timestamp. |
| `event_info` | String |  | Required. Information associated with the event. |
| `parent` | String | ✅ | Required. Account to log events about. Format: platforms/{platform}/accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.adsenseplatform_api.Event {
    parent = "value"  # Required. Account to log events about. Format: platforms/{platform}/accounts/{account}
}

```

---


### Account

Creates a sub-account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. Approval state of the account. |
| `time_zone` | String |  | Required. The IANA TZ timezone code of this account. For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. This field is used for reporting. It is recommended to set it to the same value for all child accounts. |
| `create_time` | String |  | Output only. Creation time of the account. |
| `name` | String |  | Output only. Resource name of the account. Format: platforms/pub-[0-9]+/accounts/pub-[0-9]+ |
| `creation_request_id` | String |  | Required. An opaque token that uniquely identifies the account among all the platform's accounts. This string may contain at most 64 non-whitespace ASCII characters, but otherwise has no predefined structure. However, it is expected to be a platform-specific identifier for the user creating the account, so that only a single account can be created for any given user. This field must not contain any information that is recognizable as personally identifiable information. e.g. it should not be an email address or login name. Once an account has been created, a second attempt to create an account using the same creation_request_id will result in an ALREADY_EXISTS error. |
| `display_name` | String |  | Display name of this account. |
| `region_code` | String |  | Required. Input only. CLDR region code of the country/region of the address. Set this to country code of the child account if known, otherwise to your own country code. |
| `parent` | String | ✅ | Required. Platform to create an account for. Format: platforms/{platform} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Approval state of the account. |
| `time_zone` | String | Required. The IANA TZ timezone code of this account. For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. This field is used for reporting. It is recommended to set it to the same value for all child accounts. |
| `create_time` | String | Output only. Creation time of the account. |
| `name` | String | Output only. Resource name of the account. Format: platforms/pub-[0-9]+/accounts/pub-[0-9]+ |
| `creation_request_id` | String | Required. An opaque token that uniquely identifies the account among all the platform's accounts. This string may contain at most 64 non-whitespace ASCII characters, but otherwise has no predefined structure. However, it is expected to be a platform-specific identifier for the user creating the account, so that only a single account can be created for any given user. This field must not contain any information that is recognizable as personally identifiable information. e.g. it should not be an email address or login name. Once an account has been created, a second attempt to create an account using the same creation_request_id will result in an ALREADY_EXISTS error. |
| `display_name` | String | Display name of this account. |
| `region_code` | String | Required. Input only. CLDR region code of the country/region of the address. Set this to country code of the child account if known, otherwise to your own country code. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.adsenseplatform_api.Account {
    parent = "value"  # Required. Platform to create an account for. Format: platforms/{platform}
}

# Access account outputs
account_id = account.id
account_state = account.state
account_time_zone = account.time_zone
account_create_time = account.create_time
account_name = account.name
account_creation_request_id = account.creation_request_id
account_display_name = account.display_name
account_region_code = account.region_code
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple site resources
site_0 = provider.adsenseplatform_api.Site {
    parent = "value-0"
}
site_1 = provider.adsenseplatform_api.Site {
    parent = "value-1"
}
site_2 = provider.adsenseplatform_api.Site {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    site = provider.adsenseplatform_api.Site {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Adsenseplatform_api Documentation](https://cloud.google.com/adsenseplatform_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
