# Firebasedynamiclinks_api Service



**Resources**: 3

---

## Overview

The firebasedynamiclinks_api service provides access to 3 resource types:

- [Short_link](#short_link) [C]
- [Firebasedynamiclink](#firebasedynamiclink) [CR]
- [Managed_short_link](#managed_short_link) [C]

---

## Resources


### Short_link

Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `long_dynamic_link` | String |  | Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `dynamic_link_info` | String |  | Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `suffix` | String |  | Short Dynamic Link suffix. Optional. |
| `sdk_version` | String |  | Google SDK version. Version takes the form "$major.$minor.$patch" |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create short_link
short_link = provider.firebasedynamiclinks_api.Short_link {
}

```

---


### Firebasedynamiclink

Get iOS reopen attribution for app universal link open deeplinking.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bundle_id` | String |  | APP bundle ID. |
| `requested_link` | String |  | FDL link to be verified from an app universal link open. The FDL link can be one of: 1) short FDL. e.g. .page.link/, or 2) long FDL. e.g. .page.link/?{query params}, or 3) Invite FDL. e.g. .page.link/i/ |
| `sdk_version` | String |  | Google SDK version. Version takes the form "$major.$minor.$patch" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warnings` | Vec<String> | Optional warnings associated this API request. |
| `link_event_stats` | Vec<String> | Dynamic Link event stats. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firebasedynamiclink
firebasedynamiclink = provider.firebasedynamiclinks_api.Firebasedynamiclink {
}

# Access firebasedynamiclink outputs
firebasedynamiclink_id = firebasedynamiclink.id
firebasedynamiclink_warnings = firebasedynamiclink.warnings
firebasedynamiclink_link_event_stats = firebasedynamiclink.link_event_stats
```

---


### Managed_short_link

Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sdk_version` | String |  | Google SDK version. Version takes the form "$major.$minor.$patch" |
| `suffix` | String |  | Short Dynamic Link suffix. Optional. |
| `name` | String |  | Link name to associate with the link. It's used for marketer to identify manually-created links in the Firebase console (https://console.firebase.google.com/). Links must be named to be tracked. |
| `dynamic_link_info` | String |  | Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `long_dynamic_link` | String |  | Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_short_link
managed_short_link = provider.firebasedynamiclinks_api.Managed_short_link {
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

# Create multiple short_link resources
short_link_0 = provider.firebasedynamiclinks_api.Short_link {
}
short_link_1 = provider.firebasedynamiclinks_api.Short_link {
}
short_link_2 = provider.firebasedynamiclinks_api.Short_link {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    short_link = provider.firebasedynamiclinks_api.Short_link {
    }
```

---

## Related Documentation

- [GCP Firebasedynamiclinks_api Documentation](https://cloud.google.com/firebasedynamiclinks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
