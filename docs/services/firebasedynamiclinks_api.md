# Firebasedynamiclinks_api Service



**Resources**: 3

---

## Overview

The firebasedynamiclinks_api service provides access to 3 resource types:

- [Firebasedynamiclink](#firebasedynamiclink) [CR]
- [Managed_short_link](#managed_short_link) [C]
- [Short_link](#short_link) [C]

---

## Resources


### Firebasedynamiclink

Get iOS strong/weak-match info for post-install attribution.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retrieval_method` | String |  | App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link. |
| `visual_style` | String |  | Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie. |
| `bundle_id` | String |  | APP bundle ID. |
| `device` | String |  | Device information. |
| `ios_version` | String |  | iOS version, ie: 9.3.5. Consider adding "build". |
| `app_installation_time` | String |  | App installation epoch time (https://en.wikipedia.org/wiki/Unix_time). This is a client signal for a more accurate weak match. |
| `sdk_version` | String |  | Google SDK version. Version takes the form "$major.$minor.$patch" |
| `unique_match_link_to_check` | String |  | Possible unique matched link that server need to check before performing device heuristics match. If passed link is short server need to expand the link. If link is long server need to vslidate the link. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `link_event_stats` | Vec<String> | Dynamic Link event stats. |
| `warnings` | Vec<String> | Optional warnings associated this API request. |


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
firebasedynamiclink_link_event_stats = firebasedynamiclink.link_event_stats
firebasedynamiclink_warnings = firebasedynamiclink.warnings
```

---


### Managed_short_link

Creates a managed short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. This differs from CreateShortDynamicLink in the following ways: - The request will also contain a name for the link (non unique name for the front end). - The response must be authenticated with an auth token (generated with the admin service account). - The link will appear in the FDL list of links in the console front end. The Dynamic Link domain in the request must be owned by requester's Firebase project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suffix` | String |  | Short Dynamic Link suffix. Optional. |
| `long_dynamic_link` | String |  | Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `dynamic_link_info` | String |  | Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `name` | String |  | Link name to associate with the link. It's used for marketer to identify manually-created links in the Firebase console (https://console.firebase.google.com/). Links must be named to be tracked. |
| `sdk_version` | String |  | Google SDK version. Version takes the form "$major.$minor.$patch" |



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


### Short_link

Creates a short Dynamic Link given either a valid long Dynamic Link or details such as Dynamic Link domain, Android and iOS app information. The created short Dynamic Link will not expire. Repeated calls with the same long Dynamic Link or Dynamic Link information will produce the same short Dynamic Link. The Dynamic Link domain in the request must be owned by requester's Firebase project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dynamic_link_info` | String |  | Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
| `long_dynamic_link` | String |  | Full long Dynamic Link URL with desired query parameters specified. For example, "https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener). |
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple firebasedynamiclink resources
firebasedynamiclink_0 = provider.firebasedynamiclinks_api.Firebasedynamiclink {
}
firebasedynamiclink_1 = provider.firebasedynamiclinks_api.Firebasedynamiclink {
}
firebasedynamiclink_2 = provider.firebasedynamiclinks_api.Firebasedynamiclink {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    firebasedynamiclink = provider.firebasedynamiclinks_api.Firebasedynamiclink {
    }
```

---

## Related Documentation

- [GCP Firebasedynamiclinks_api Documentation](https://cloud.google.com/firebasedynamiclinks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
