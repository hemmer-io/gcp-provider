# Binaryauthorization_api Service



**Resources**: 9

---

## Overview

The binaryauthorization_api service provides access to 9 resource types:

- [Project](#project) [RU]
- [Attestor](#attestor) [CRUD]
- [Policy](#policy) [CR]
- [Policie](#policie) [CRUD]
- [Systempolicy](#systempolicy) [R]
- [Systempolicy](#systempolicy) [R]
- [Attestor](#attestor) [CRUD]
- [Project](#project) [RU]
- [Policy](#policy) [CR]

---

## Resources


### Project

A policy specifies the attestors that must attest to a container image, before the project is allowed to deploy that image. There is at most one policy per project. All image admission requests are permitted if a project has no policy. Gets the policy for this project. Returns a default policy if the project does not have one.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admission_whitelist_patterns` | Vec<String> |  | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `etag` | String |  | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `istio_service_identity_admission_rules` | HashMap<String, String> |  | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `update_time` | String |  | Output only. Time when the policy was last updated. |
| `global_policy_evaluation_mode` | String |  | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `name` | String |  | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `cluster_admission_rules` | HashMap<String, String> |  | Optional. A valid policy has only one of the following rule maps non-empty, i.e. only one of `cluster_admission_rules`, `kubernetes_namespace_admission_rules`, `kubernetes_service_account_admission_rules`, or `istio_service_identity_admission_rules` can be non-empty. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `default_admission_rule` | String |  | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> |  | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> |  | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |
| `description` | String |  | Optional. A descriptive comment. |
| `name` | String | ✅ | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admission_whitelist_patterns` | Vec<String> | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `istio_service_identity_admission_rules` | HashMap<String, String> | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `update_time` | String | Output only. Time when the policy was last updated. |
| `global_policy_evaluation_mode` | String | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `name` | String | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `cluster_admission_rules` | HashMap<String, String> | Optional. A valid policy has only one of the following rule maps non-empty, i.e. only one of `cluster_admission_rules`, `kubernetes_namespace_admission_rules`, `kubernetes_service_account_admission_rules`, or `istio_service_identity_admission_rules` can be non-empty. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `default_admission_rule` | String | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |
| `description` | String | Optional. A descriptive comment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_admission_whitelist_patterns = project.admission_whitelist_patterns
project_etag = project.etag
project_istio_service_identity_admission_rules = project.istio_service_identity_admission_rules
project_update_time = project.update_time
project_global_policy_evaluation_mode = project.global_policy_evaluation_mode
project_name = project.name
project_cluster_admission_rules = project.cluster_admission_rules
project_default_admission_rule = project.default_admission_rule
project_kubernetes_namespace_admission_rules = project.kubernetes_namespace_admission_rules
project_kubernetes_service_account_admission_rules = project.kubernetes_service_account_admission_rules
project_description = project.description
```

---


### Attestor

Creates an attestor, and returns a copy of the new attestor. Returns `NOT_FOUND` if the project does not exist, `INVALID_ARGUMENT` if the request is malformed, `ALREADY_EXISTS` if the attestor already exists.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The resource name, in the format: `projects/*/attestors/*`. This field may not be updated. |
| `etag` | String |  | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the attestor has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `update_time` | String |  | Output only. Time when the attestor was last updated. |
| `description` | String |  | Optional. A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs. |
| `user_owned_grafeas_note` | String |  | This specifies how an attestation will be read, and how it will be used during policy enforcement. |
| `parent` | String | ✅ | Required. The parent of this attestor. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The resource name, in the format: `projects/*/attestors/*`. This field may not be updated. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the attestor has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `update_time` | String | Output only. Time when the attestor was last updated. |
| `description` | String | Optional. A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs. |
| `user_owned_grafeas_note` | String | This specifies how an attestation will be read, and how it will be used during policy enforcement. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attestor
attestor = provider.binaryauthorization_api.Attestor {
    parent = "value"  # Required. The parent of this attestor.
}

# Access attestor outputs
attestor_id = attestor.id
attestor_name = attestor.name
attestor_etag = attestor.etag
attestor_update_time = attestor.update_time
attestor_description = attestor.description
attestor_user_owned_grafeas_note = attestor.user_owned_grafeas_note
```

---


### Policy

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy
policy = provider.binaryauthorization_api.Policy {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access policy outputs
policy_id = policy.id
policy_bindings = policy.bindings
policy_etag = policy.etag
policy_version = policy.version
```

---


### Policie

Creates a platform policy, and returns a copy of it. Returns `NOT_FOUND` if the project or platform doesn't exist, `INVALID_ARGUMENT` if the request is malformed, `ALREADY_EXISTS` if the policy already exists, and `INVALID_ARGUMENT` if the policy contains a platform-specific policy that does not match the platform value specified in the URL.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. Used to prevent updating the policy when another request has updated it since it was retrieved. |
| `name` | String |  | Output only. The relative resource name of the Binary Authorization platform policy, in the form of `projects/*/platforms/*/policies/*`. |
| `update_time` | String |  | Output only. Time when the policy was last updated. |
| `description` | String |  | Optional. A description comment about the policy. |
| `gke_policy` | String |  | Optional. GKE platform-specific policy. |
| `parent` | String | ✅ | Required. The parent of this platform policy. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. Used to prevent updating the policy when another request has updated it since it was retrieved. |
| `name` | String | Output only. The relative resource name of the Binary Authorization platform policy, in the form of `projects/*/platforms/*/policies/*`. |
| `update_time` | String | Output only. Time when the policy was last updated. |
| `description` | String | Optional. A description comment about the policy. |
| `gke_policy` | String | Optional. GKE platform-specific policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.binaryauthorization_api.Policie {
    parent = "value"  # Required. The parent of this platform policy.
}

# Access policie outputs
policie_id = policie.id
policie_etag = policie.etag
policie_name = policie.name
policie_update_time = policie.update_time
policie_description = policie.description
policie_gke_policy = policie.gke_policy
```

---


### Systempolicy

Gets the current system policy in the specified location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admission_whitelist_patterns` | Vec<String> | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `istio_service_identity_admission_rules` | HashMap<String, String> | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `update_time` | String | Output only. Time when the policy was last updated. |
| `global_policy_evaluation_mode` | String | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `name` | String | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `cluster_admission_rules` | HashMap<String, String> | Optional. A valid policy has only one of the following rule maps non-empty, i.e. only one of `cluster_admission_rules`, `kubernetes_namespace_admission_rules`, `kubernetes_service_account_admission_rules`, or `istio_service_identity_admission_rules` can be non-empty. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `default_admission_rule` | String | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |
| `description` | String | Optional. A descriptive comment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access systempolicy outputs
systempolicy_id = systempolicy.id
systempolicy_admission_whitelist_patterns = systempolicy.admission_whitelist_patterns
systempolicy_etag = systempolicy.etag
systempolicy_istio_service_identity_admission_rules = systempolicy.istio_service_identity_admission_rules
systempolicy_update_time = systempolicy.update_time
systempolicy_global_policy_evaluation_mode = systempolicy.global_policy_evaluation_mode
systempolicy_name = systempolicy.name
systempolicy_cluster_admission_rules = systempolicy.cluster_admission_rules
systempolicy_default_admission_rule = systempolicy.default_admission_rule
systempolicy_kubernetes_namespace_admission_rules = systempolicy.kubernetes_namespace_admission_rules
systempolicy_kubernetes_service_account_admission_rules = systempolicy.kubernetes_service_account_admission_rules
systempolicy_description = systempolicy.description
```

---


### Systempolicy

Gets the current system policy in the specified location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_admission_rule` | String | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `cluster_admission_rules` | HashMap<String, String> | Optional. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `global_policy_evaluation_mode` | String | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `istio_service_identity_admission_rules` | HashMap<String, String> | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `name` | String | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `description` | String | Optional. A descriptive comment. |
| `admission_whitelist_patterns` | Vec<String> | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `update_time` | String | Output only. Time when the policy was last updated. |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access systempolicy outputs
systempolicy_id = systempolicy.id
systempolicy_default_admission_rule = systempolicy.default_admission_rule
systempolicy_etag = systempolicy.etag
systempolicy_cluster_admission_rules = systempolicy.cluster_admission_rules
systempolicy_global_policy_evaluation_mode = systempolicy.global_policy_evaluation_mode
systempolicy_istio_service_identity_admission_rules = systempolicy.istio_service_identity_admission_rules
systempolicy_name = systempolicy.name
systempolicy_description = systempolicy.description
systempolicy_admission_whitelist_patterns = systempolicy.admission_whitelist_patterns
systempolicy_kubernetes_namespace_admission_rules = systempolicy.kubernetes_namespace_admission_rules
systempolicy_update_time = systempolicy.update_time
systempolicy_kubernetes_service_account_admission_rules = systempolicy.kubernetes_service_account_admission_rules
```

---


### Attestor

Creates an attestor, and returns a copy of the new attestor. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT if the request is malformed, ALREADY_EXISTS if the attestor already exists.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs. |
| `user_owned_drydock_note` | String |  | A Drydock ATTESTATION_AUTHORITY Note, created by the user. |
| `etag` | String |  | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the attestor has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `name` | String |  | Required. The resource name, in the format: `projects/*/attestors/*`. This field may not be updated. |
| `update_time` | String |  | Output only. Time when the attestor was last updated. |
| `parent` | String | ✅ | Required. The parent of this attestor. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs. |
| `user_owned_drydock_note` | String | A Drydock ATTESTATION_AUTHORITY Note, created by the user. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the attestor has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `name` | String | Required. The resource name, in the format: `projects/*/attestors/*`. This field may not be updated. |
| `update_time` | String | Output only. Time when the attestor was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attestor
attestor = provider.binaryauthorization_api.Attestor {
    parent = "value"  # Required. The parent of this attestor.
}

# Access attestor outputs
attestor_id = attestor.id
attestor_description = attestor.description
attestor_user_owned_drydock_note = attestor.user_owned_drydock_note
attestor_etag = attestor.etag
attestor_name = attestor.name
attestor_update_time = attestor.update_time
```

---


### Project

A policy specifies the attestors that must attest to a container image, before the project is allowed to deploy that image. There is at most one policy per project. All image admission requests are permitted if a project has no policy. Gets the policy for this project. Returns a default policy if the project does not have one.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_admission_rule` | String |  | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `etag` | String |  | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `cluster_admission_rules` | HashMap<String, String> |  | Optional. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `global_policy_evaluation_mode` | String |  | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `istio_service_identity_admission_rules` | HashMap<String, String> |  | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `name` | String |  | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `description` | String |  | Optional. A descriptive comment. |
| `admission_whitelist_patterns` | Vec<String> |  | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> |  | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `update_time` | String |  | Output only. Time when the policy was last updated. |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> |  | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |
| `name` | String | ✅ | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_admission_rule` | String | Required. Default admission rule for a cluster without a per-cluster, per- kubernetes-service-account, or per-istio-service-identity admission rule. |
| `etag` | String | Optional. A checksum, returned by the server, that can be sent on update requests to ensure the policy has an up-to-date value before attempting to update it. See https://google.aip.dev/154. |
| `cluster_admission_rules` | HashMap<String, String> | Optional. Per-cluster admission rules. Cluster spec format: `location.clusterId`. There can be at most one admission rule per cluster spec. A `location` is either a compute zone (e.g. us-central1-a) or a region (e.g. us-central1). For `clusterId` syntax restrictions see https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters. |
| `global_policy_evaluation_mode` | String | Optional. Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not covered by the global policy will be subject to the project admission policy. This setting has no effect when specified inside a global admission policy. |
| `istio_service_identity_admission_rules` | HashMap<String, String> | Optional. Per-istio-service-identity admission rules. Istio service identity spec format: `spiffe:///ns//sa/` or `/ns//sa/` e.g. `spiffe://example.com/ns/test-ns/sa/default` |
| `name` | String | Output only. The resource name, in the format `projects/*/policy`. There is at most one policy per project. |
| `description` | String | Optional. A descriptive comment. |
| `admission_whitelist_patterns` | Vec<String> | Optional. Admission policy allowlisting. A matching admission request will always be permitted. This feature is typically used to exclude Google or third-party infrastructure images from Binary Authorization policies. |
| `kubernetes_namespace_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format: `[a-z.-]+`, e.g. `some-namespace` |
| `update_time` | String | Output only. Time when the policy was last updated. |
| `kubernetes_service_account_admission_rules` | HashMap<String, String> | Optional. Per-kubernetes-service-account admission rules. Service account spec format: `namespace:serviceaccount`. e.g. `test-ns:default` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_default_admission_rule = project.default_admission_rule
project_etag = project.etag
project_cluster_admission_rules = project.cluster_admission_rules
project_global_policy_evaluation_mode = project.global_policy_evaluation_mode
project_istio_service_identity_admission_rules = project.istio_service_identity_admission_rules
project_name = project.name
project_description = project.description
project_admission_whitelist_patterns = project.admission_whitelist_patterns
project_kubernetes_namespace_admission_rules = project.kubernetes_namespace_admission_rules
project_update_time = project.update_time
project_kubernetes_service_account_admission_rules = project.kubernetes_service_account_admission_rules
```

---


### Policy

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy
policy = provider.binaryauthorization_api.Policy {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access policy outputs
policy_id = policy.id
policy_bindings = policy.bindings
policy_version = policy.version
policy_etag = policy.etag
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple project resources
project_0 = provider.binaryauthorization_api.Project {
    name = "value-0"
}
project_1 = provider.binaryauthorization_api.Project {
    name = "value-1"
}
project_2 = provider.binaryauthorization_api.Project {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.binaryauthorization_api.Project {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Binaryauthorization_api Documentation](https://cloud.google.com/binaryauthorization_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
