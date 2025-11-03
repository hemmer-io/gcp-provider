# Digitalassetlinks_api Service



**Resources**: 2

---

## Overview

The digitalassetlinks_api service provides access to 2 resource types:

- [Statement](#statement) [R]
- [Assetlink](#assetlink) [CR]

---

## Resources


### Statement

Retrieves a list of all statements from a given source that match the specified target and statement string. The API guarantees that all statements with secure source assets, such as HTTPS websites or Android apps, have been made in a secure way by the owner of those assets, as described in the [Digital Asset Links technical design specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md). Specifically, you should consider that for insecure websites (that is, where the URL starts with `http://` instead of `https://`), this guarantee cannot be made. The `List` command is most useful in cases where the API client wants to know all the ways in which two assets are related, or enumerate all the relationships from a particular source asset. Example: a feature that helps users navigate to related items. When a mobile app is running on a device, the feature would make it easy to navigate to the corresponding web site or Google+ profile.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statements` | Vec<String> | A list of all the matching statements that have been found. |
| `max_age` | String | From serving time, how much longer the response should be considered valid barring further updates. REQUIRED |
| `debug_string` | String | Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below. |
| `error_code` | Vec<String> | Error codes that describe the result of the List operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access statement outputs
statement_id = statement.id
statement_statements = statement.statements
statement_max_age = statement.max_age
statement_debug_string = statement.debug_string
statement_error_code = statement.error_code
```

---


### Assetlink

Send a bundle of statement checks in a single RPC to minimize latency and service load. Statements need not be all for the same source and/or target. We recommend using this method when you need to check more than one statement in a short period of time.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `statements` | Vec<String> |  | List of statements to check. For each statement, you can omit a field if the corresponding default_* field below was supplied. Minimum 1 statement; maximum 1,000 statements. Any additional statements will be ignored. |
| `default_relation` | String |  | If specified, will be used in any given template statement that doesn’t specify a relation. |
| `default_target` | String |  | If specified, will be used in any given template statement that doesn’t specify a target. |
| `return_relation_extensions` | bool |  | Same configuration as in CheckRequest; all statement checks will use the same configuration. |
| `default_source` | String |  | If specified, will be used in any given template statement that doesn’t specify a source. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linked` | bool | Set to true if the assets specified in the request are linked by the relation specified in the request. |
| `debug_string` | String | Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below. |
| `relation_extensions` | Vec<HashMap<String, String>> | Statements may specify relation level extensions/payloads to express more details when declaring permissions to grant from the source asset to the target asset. When requested, the API will return relation_extensions specified in any and all statements linking the requested source and target assets by the relation specified in the request. |
| `error_code` | Vec<String> | Error codes that describe the result of the Check operation. NOTE: Error codes may be populated even when `linked` is true. The error codes do not necessarily imply that the request failed, but rather, specify any errors encountered in the statements file(s) which may or may not impact whether the server determines the requested source and target to be linked. |
| `max_age` | String | From serving time, how much longer the response should be considered valid barring further updates. REQUIRED |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assetlink
assetlink = provider.digitalassetlinks_api.Assetlink {
}

# Access assetlink outputs
assetlink_id = assetlink.id
assetlink_linked = assetlink.linked
assetlink_debug_string = assetlink.debug_string
assetlink_relation_extensions = assetlink.relation_extensions
assetlink_error_code = assetlink.error_code
assetlink_max_age = assetlink.max_age
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple statement resources
statement_0 = provider.digitalassetlinks_api.Statement {
}
statement_1 = provider.digitalassetlinks_api.Statement {
}
statement_2 = provider.digitalassetlinks_api.Statement {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    statement = provider.digitalassetlinks_api.Statement {
    }
```

---

## Related Documentation

- [GCP Digitalassetlinks_api Documentation](https://cloud.google.com/digitalassetlinks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
