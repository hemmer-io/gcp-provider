# Bigqueryreservation_api Service



**Resources**: 14

---

## Overview

The bigqueryreservation_api service provides access to 14 resource types:

- [Reservation_group](#reservation_group) [CRD]
- [Capacity_commitment](#capacity_commitment) [CRUD]
- [Location](#location) [RU]
- [Reservation](#reservation) [CRUD]
- [Assignment](#assignment) [CRUD]
- [Slot_pool](#slot_pool) [RD]
- [Reservation_grant](#reservation_grant) [CRD]
- [Location](#location) [R]
- [Reservation](#reservation) [CRUD]
- [Operation](#operation) [CR]
- [Reservation](#reservation) [CRUD]
- [Capacity_commitment](#capacity_commitment) [CRUD]
- [Location](#location) [RU]
- [Assignment](#assignment) [CRUD]

---

## Resources


### Reservation_group

Creates a new reservation group.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the reservation group, e.g., `projects/*/locations/*/reservationGroups/team1-prod`. The reservation_group_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `parent` | String | ✅ | Required. Project, location. E.g., `projects/myproject/locations/US` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the reservation group, e.g., `projects/*/locations/*/reservationGroups/team1-prod`. The reservation_group_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation_group
reservation_group = provider.bigqueryreservation_api.Reservation_group {
    parent = "value"  # Required. Project, location. E.g., `projects/myproject/locations/US`
}

# Access reservation_group outputs
reservation_group_id = reservation_group.id
reservation_group_name = reservation_group.name
```

---


### Capacity_commitment

Creates a new capacity commitment resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commitment_start_time` | String |  | Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments. Note after the commitment is renewed, commitment_start_time won't be changed. It refers to the start time of the original commitment. |
| `plan` | String |  | Optional. Capacity commitment commitment plan. |
| `state` | String |  | Output only. State of the commitment. |
| `name` | String |  | Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `failure_status` | String |  | Output only. For FAILED commitment plan, provides the reason of failure. |
| `commitment_end_time` | String |  | Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments. Note after renewal, commitment_end_time is the time the renewed commitment expires. So itwould be at a time after commitment_start_time + committed period, because we don't change commitment_start_time , |
| `is_flat_rate` | bool |  | Output only. If true, the commitment is a flat-rate commitment, otherwise, it's an edition commitment. |
| `multi_region_auxiliary` | bool |  | Applicable only for commitments located within one of the BigQuery multi-regions (US or EU). If set to true, this commitment is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this commitment is placed in the organization's default region. NOTE: this is a preview feature. Project must be allow-listed in order to set this field. |
| `renewal_plan` | String |  | Optional. The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL and TRIAL commitments. |
| `edition` | String |  | Optional. Edition of the capacity commitment. |
| `slot_count` | String |  | Optional. Number of slots in this commitment. |
| `parent` | String | ✅ | Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `commitment_start_time` | String | Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments. Note after the commitment is renewed, commitment_start_time won't be changed. It refers to the start time of the original commitment. |
| `plan` | String | Optional. Capacity commitment commitment plan. |
| `state` | String | Output only. State of the commitment. |
| `name` | String | Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `failure_status` | String | Output only. For FAILED commitment plan, provides the reason of failure. |
| `commitment_end_time` | String | Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments. Note after renewal, commitment_end_time is the time the renewed commitment expires. So itwould be at a time after commitment_start_time + committed period, because we don't change commitment_start_time , |
| `is_flat_rate` | bool | Output only. If true, the commitment is a flat-rate commitment, otherwise, it's an edition commitment. |
| `multi_region_auxiliary` | bool | Applicable only for commitments located within one of the BigQuery multi-regions (US or EU). If set to true, this commitment is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this commitment is placed in the organization's default region. NOTE: this is a preview feature. Project must be allow-listed in order to set this field. |
| `renewal_plan` | String | Optional. The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL and TRIAL commitments. |
| `edition` | String | Optional. Edition of the capacity commitment. |
| `slot_count` | String | Optional. Number of slots in this commitment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create capacity_commitment
capacity_commitment = provider.bigqueryreservation_api.Capacity_commitment {
    parent = "value"  # Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US`
}

# Access capacity_commitment outputs
capacity_commitment_id = capacity_commitment.id
capacity_commitment_commitment_start_time = capacity_commitment.commitment_start_time
capacity_commitment_plan = capacity_commitment.plan
capacity_commitment_state = capacity_commitment.state
capacity_commitment_name = capacity_commitment.name
capacity_commitment_failure_status = capacity_commitment.failure_status
capacity_commitment_commitment_end_time = capacity_commitment.commitment_end_time
capacity_commitment_is_flat_rate = capacity_commitment.is_flat_rate
capacity_commitment_multi_region_auxiliary = capacity_commitment.multi_region_auxiliary
capacity_commitment_renewal_plan = capacity_commitment.renewal_plan
capacity_commitment_edition = capacity_commitment.edition
capacity_commitment_slot_count = capacity_commitment.slot_count
```

---


### Location

Deprecated: Looks up assignments for a specified resource for a particular region. If the request is about a project: 1. Assignments created on the project will be returned if they exist. 2. Otherwise assignments created on the closest ancestor will be returned. 3. Assignments for different JobTypes will all be returned. The same logic applies if the request is about a folder. If the request is about an organization, then assignments created on the organization will be returned (organization doesn't have ancestors). Comparing to ListAssignments, there are some behavior differences: 1. permission on the assignee will be verified in this API. 2. Hierarchy lookup (project->folder->organization) happens in this API. 3. Parent here is `projects/*/locations/*`, instead of `projects/*/locations/*reservations/*`. **Note** "-" cannot be used for projects nor locations.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `preferred_tables` | Vec<String> |  | Optional. Preferred tables to use BI capacity for. |
| `size` | String |  | Optional. Size of a reservation, in bytes. |
| `update_time` | String |  | Output only. The last update timestamp of a reservation. |
| `name` | String |  | Identifier. The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`. |
| `name` | String | ✅ | Identifier. The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `assignments` | Vec<String> | List of assignments visible to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_next_page_token = location.next_page_token
location_assignments = location.assignments
```

---


### Reservation

Creates a new reservation resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `slot_capacity` | String |  | Optional. Baseline slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false, or autoscaling is enabled. The total slot_capacity of the reservation and its siblings may exceed the total slot_count of capacity commitments. In that case, the exceeding slots will be charged with the autoscale SKU. You can increase the number of baseline slots in a reservation every few minutes. If you want to decrease your baseline slots, you are limited to once an hour if you have recently changed your baseline slot capacity and your baseline slots exceed your committed slots. Otherwise, you can decrease your baseline slots every few minutes. |
| `edition` | String |  | Optional. Edition of the reservation. |
| `ignore_idle_slots` | bool |  | Optional. If false, any query or pipeline job using this reservation will use idle slots from other reservations within the same admin project. If true, a query or pipeline job using this reservation will execute with the slot capacity specified in the slot_capacity field at most. |
| `scaling_mode` | String |  | Optional. The scaling mode for the reservation. If the field is present but max_slots is not present, requests will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. |
| `original_primary_location` | String |  | Output only. The location where the reservation was originally created. This is set only during the failover reservation's creation. All billing charges for the failover reservation will be applied to this location. |
| `max_slots` | String |  | Optional. The overall max slots for the reservation, covering slot_capacity (baseline), idle slots (if ignore_idle_slots is false) and scaled slots. If present, the reservation won't use more than the specified number of slots, even if there is demand and supply (from idle slots). NOTE: capping a reservation's idle slot usage is best effort and its usage may exceed the max_slots value. However, in terms of autoscale.current_slots (which accounts for the additional added slots), it will never exceed the max_slots - baseline. This field must be set together with the scaling_mode enum value, otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. If the max_slots and scaling_mode are set, the autoscale or autoscale.max_slots field must be unset. Otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. However, the autoscale field may still be in the output. The autopscale.max_slots will always show as 0 and the autoscaler.current_slots will represent the current slots from autoscaler excluding idle slots. For example, if the max_slots is 1000 and scaling_mode is AUTOSCALE_ONLY, then in the output, the autoscaler.max_slots will be 0 and the autoscaler.current_slots may be any value between 0 and 1000. If the max_slots is 1000, scaling_mode is ALL_SLOTS, the baseline is 100 and idle slots usage is 200, then in the output, the autoscaler.max_slots will be 0 and the autoscaler.current_slots will not be higher than 700. If the max_slots is 1000, scaling_mode is IDLE_SLOTS_ONLY, then in the output, the autoscaler field will be null. If the max_slots and scaling_mode are set, then the ignore_idle_slots field must be aligned with the scaling_mode enum value.(See details in ScalingMode comments). Otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. Please note, the max_slots is for user to manage the part of slots greater than the baseline. Therefore, we don't allow users to set max_slots smaller or equal to the baseline as it will not be meaningful. If the field is present and slot_capacity>=max_slots, requests will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. Please note that if max_slots is set to 0, we will treat it as unset. Customers can set max_slots to 0 and set scaling_mode to SCALING_MODE_UNSPECIFIED to disable the max_slots feature. |
| `creation_time` | String |  | Output only. Creation time of the reservation. |
| `primary_location` | String |  | Output only. The current location of the reservation's primary replica. This field is only set for reservations using the managed disaster recovery feature. |
| `autoscale` | String |  | Optional. The configuration parameters for the auto scaling feature. |
| `secondary_location` | String |  | Optional. The current location of the reservation's secondary replica. This field is only set for reservations using the managed disaster recovery feature. Users can set this in create reservation calls to create a failover reservation or in update reservation calls to convert a non-failover reservation to a failover reservation(or vice versa). |
| `scheduling_policy` | String |  | Optional. The scheduling policy to use for jobs and queries running under this reservation. The scheduling policy controls how the reservation's resources are distributed. This feature is not yet generally available. |
| `multi_region_auxiliary` | bool |  | Applicable only for reservations located within one of the BigQuery multi-regions (US or EU). If set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region. NOTE: this is a preview feature. Project must be allow-listed in order to set this field. |
| `concurrency` | String |  | Optional. Job concurrency target which sets a soft upper bound on the number of jobs that can run concurrently in this reservation. This is a soft target due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency target will be automatically computed by the system. NOTE: this field is exposed as target job concurrency in the Information Schema, DDL and BigQuery CLI. |
| `reservation_group` | String |  | Optional. The reservation group that this reservation belongs to. You can set this property when you create or update a reservation. Reservations do not need to belong to a reservation group. Format: projects/{project}/locations/{location}/reservationGroups/{reservation_group} or just {reservation_group} |
| `labels` | HashMap<String, String> |  | Optional. The labels associated with this reservation. You can use these to organize and group your reservations. You can set this property when you create or update a reservation. |
| `replication_status` | String |  | Output only. The Disaster Recovery(DR) replication status of the reservation. This is only available for the primary replicas of DR/failover reservations and provides information about the both the staleness of the secondary and the last error encountered while trying to replicate changes from the primary to the secondary. If this field is blank, it means that the reservation is either not a DR reservation or the reservation is a DR secondary or that any replication operations on the reservation have succeeded. |
| `update_time` | String |  | Output only. Last update time of the reservation. |
| `parent` | String | ✅ | Required. Project, location. E.g., `projects/myproject/locations/US` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `slot_capacity` | String | Optional. Baseline slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false, or autoscaling is enabled. The total slot_capacity of the reservation and its siblings may exceed the total slot_count of capacity commitments. In that case, the exceeding slots will be charged with the autoscale SKU. You can increase the number of baseline slots in a reservation every few minutes. If you want to decrease your baseline slots, you are limited to once an hour if you have recently changed your baseline slot capacity and your baseline slots exceed your committed slots. Otherwise, you can decrease your baseline slots every few minutes. |
| `edition` | String | Optional. Edition of the reservation. |
| `ignore_idle_slots` | bool | Optional. If false, any query or pipeline job using this reservation will use idle slots from other reservations within the same admin project. If true, a query or pipeline job using this reservation will execute with the slot capacity specified in the slot_capacity field at most. |
| `scaling_mode` | String | Optional. The scaling mode for the reservation. If the field is present but max_slots is not present, requests will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. |
| `original_primary_location` | String | Output only. The location where the reservation was originally created. This is set only during the failover reservation's creation. All billing charges for the failover reservation will be applied to this location. |
| `max_slots` | String | Optional. The overall max slots for the reservation, covering slot_capacity (baseline), idle slots (if ignore_idle_slots is false) and scaled slots. If present, the reservation won't use more than the specified number of slots, even if there is demand and supply (from idle slots). NOTE: capping a reservation's idle slot usage is best effort and its usage may exceed the max_slots value. However, in terms of autoscale.current_slots (which accounts for the additional added slots), it will never exceed the max_slots - baseline. This field must be set together with the scaling_mode enum value, otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. If the max_slots and scaling_mode are set, the autoscale or autoscale.max_slots field must be unset. Otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. However, the autoscale field may still be in the output. The autopscale.max_slots will always show as 0 and the autoscaler.current_slots will represent the current slots from autoscaler excluding idle slots. For example, if the max_slots is 1000 and scaling_mode is AUTOSCALE_ONLY, then in the output, the autoscaler.max_slots will be 0 and the autoscaler.current_slots may be any value between 0 and 1000. If the max_slots is 1000, scaling_mode is ALL_SLOTS, the baseline is 100 and idle slots usage is 200, then in the output, the autoscaler.max_slots will be 0 and the autoscaler.current_slots will not be higher than 700. If the max_slots is 1000, scaling_mode is IDLE_SLOTS_ONLY, then in the output, the autoscaler field will be null. If the max_slots and scaling_mode are set, then the ignore_idle_slots field must be aligned with the scaling_mode enum value.(See details in ScalingMode comments). Otherwise the request will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. Please note, the max_slots is for user to manage the part of slots greater than the baseline. Therefore, we don't allow users to set max_slots smaller or equal to the baseline as it will not be meaningful. If the field is present and slot_capacity>=max_slots, requests will be rejected with error code `google.rpc.Code.INVALID_ARGUMENT`. Please note that if max_slots is set to 0, we will treat it as unset. Customers can set max_slots to 0 and set scaling_mode to SCALING_MODE_UNSPECIFIED to disable the max_slots feature. |
| `creation_time` | String | Output only. Creation time of the reservation. |
| `primary_location` | String | Output only. The current location of the reservation's primary replica. This field is only set for reservations using the managed disaster recovery feature. |
| `autoscale` | String | Optional. The configuration parameters for the auto scaling feature. |
| `secondary_location` | String | Optional. The current location of the reservation's secondary replica. This field is only set for reservations using the managed disaster recovery feature. Users can set this in create reservation calls to create a failover reservation or in update reservation calls to convert a non-failover reservation to a failover reservation(or vice versa). |
| `scheduling_policy` | String | Optional. The scheduling policy to use for jobs and queries running under this reservation. The scheduling policy controls how the reservation's resources are distributed. This feature is not yet generally available. |
| `multi_region_auxiliary` | bool | Applicable only for reservations located within one of the BigQuery multi-regions (US or EU). If set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region. NOTE: this is a preview feature. Project must be allow-listed in order to set this field. |
| `concurrency` | String | Optional. Job concurrency target which sets a soft upper bound on the number of jobs that can run concurrently in this reservation. This is a soft target due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency target will be automatically computed by the system. NOTE: this field is exposed as target job concurrency in the Information Schema, DDL and BigQuery CLI. |
| `reservation_group` | String | Optional. The reservation group that this reservation belongs to. You can set this property when you create or update a reservation. Reservations do not need to belong to a reservation group. Format: projects/{project}/locations/{location}/reservationGroups/{reservation_group} or just {reservation_group} |
| `labels` | HashMap<String, String> | Optional. The labels associated with this reservation. You can use these to organize and group your reservations. You can set this property when you create or update a reservation. |
| `replication_status` | String | Output only. The Disaster Recovery(DR) replication status of the reservation. This is only available for the primary replicas of DR/failover reservations and provides information about the both the staleness of the secondary and the last error encountered while trying to replicate changes from the primary to the secondary. If this field is blank, it means that the reservation is either not a DR reservation or the reservation is a DR secondary or that any replication operations on the reservation have succeeded. |
| `update_time` | String | Output only. Last update time of the reservation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation
reservation = provider.bigqueryreservation_api.Reservation {
    parent = "value"  # Required. Project, location. E.g., `projects/myproject/locations/US`
}

# Access reservation outputs
reservation_id = reservation.id
reservation_name = reservation.name
reservation_slot_capacity = reservation.slot_capacity
reservation_edition = reservation.edition
reservation_ignore_idle_slots = reservation.ignore_idle_slots
reservation_scaling_mode = reservation.scaling_mode
reservation_original_primary_location = reservation.original_primary_location
reservation_max_slots = reservation.max_slots
reservation_creation_time = reservation.creation_time
reservation_primary_location = reservation.primary_location
reservation_autoscale = reservation.autoscale
reservation_secondary_location = reservation.secondary_location
reservation_scheduling_policy = reservation.scheduling_policy
reservation_multi_region_auxiliary = reservation.multi_region_auxiliary
reservation_concurrency = reservation.concurrency
reservation_reservation_group = reservation.reservation_group
reservation_labels = reservation.labels
reservation_replication_status = reservation.replication_status
reservation_update_time = reservation.update_time
```

---


### Assignment

Creates an assignment object which allows the given project to submit jobs of a certain type using slots from the specified reservation. Currently a resource (project, folder, organization) can only have one assignment per each (job_type, location) combination, and that reservation will be used for all jobs of the matching type. Different assignments can be created on different levels of the projects, folders or organization hierarchy. During query execution, the assignment is looked up at the project, folder and organization levels in that order. The first assignment found is applied to the query. When creating assignments, it does not matter if other assignments exist at higher levels. Example: * The organization `organizationA` contains two projects, `project1` and `project2`. * Assignments for all three entities (`organizationA`, `project1`, and `project2`) could all be created and mapped to the same or different reservations. "None" assignments represent an absence of the assignment. Projects assigned to None use on-demand pricing. To create a "None" assignment, use "none" as a reservation_id in the parent. Example parent: `projects/myproject/locations/US/reservations/none`. Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have 'bigquery.admin' permissions on the project using the reservation and the project that owns this reservation. Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment does not match location of the reservation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assignee` | String |  | Optional. The resource which will use the reservation. E.g. `projects/myproject`, `folders/123`, or `organizations/456`. |
| `enable_gemini_in_bigquery` | bool |  | Optional. This field controls if "Gemini in BigQuery" (https://cloud.google.com/gemini/docs/bigquery/overview) features should be enabled for this reservation assignment, which is not on by default. "Gemini in BigQuery" has a distinct compliance posture from BigQuery. If this field is set to true, the assignment job type is QUERY, and the parent reservation edition is ENTERPRISE_PLUS, then the assignment will give the grantee project/organization access to "Gemini in BigQuery" features. |
| `job_type` | String |  | Optional. Which type of jobs will use the reservation. |
| `name` | String |  | Output only. Name of the resource. E.g.: `projects/myproject/locations/US/reservations/team1-prod/assignments/123`. The assignment_id must only contain lower case alphanumeric characters or dashes and the max length is 64 characters. |
| `state` | String |  | Output only. State of the assignment. |
| `scheduling_policy` | String |  | Optional. The scheduling policy to use for jobs and queries of this assignee when running under the associated reservation. The scheduling policy controls how the reservation's resources are distributed. This overrides the default scheduling policy specified on the reservation. This feature is not yet generally available. |
| `parent` | String | ✅ | Required. The parent resource name of the assignment E.g. `projects/myproject/locations/US/reservations/team1-prod` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignments` | Vec<String> | List of assignments visible to the user. |
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assignment
assignment = provider.bigqueryreservation_api.Assignment {
    parent = "value"  # Required. The parent resource name of the assignment E.g. `projects/myproject/locations/US/reservations/team1-prod`
}

# Access assignment outputs
assignment_id = assignment.id
assignment_assignments = assignment.assignments
assignment_next_page_token = assignment.next_page_token
```

---


### Slot_pool

Returns information about the slot pool.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. |
| `slot_count` | String | Number of slots in this pool. |
| `commitment_end_time` | String | Output only. The end of the commitment period. Slot pool cannot be removed before commitment_end_time. It is applicable only for ACTIVE slot pools and is computed as a combination of the plan and the time when the slot pool became ACTIVE. |
| `failure_status` | String | Output only. For FAILED slot pool, provides the reason of failure. |
| `name` | String | Output only. The resource name of the slot pool, e.g., projects/myproject/locations/us-central1/reservations/myreservation/slotPools/123 |
| `plan` | String | Slot pool commitment plan. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access slot_pool outputs
slot_pool_id = slot_pool.id
slot_pool_state = slot_pool.state
slot_pool_slot_count = slot_pool.slot_count
slot_pool_commitment_end_time = slot_pool.commitment_end_time
slot_pool_failure_status = slot_pool.failure_status
slot_pool_name = slot_pool.name
slot_pool_plan = slot_pool.plan
```

---


### Reservation_grant

Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have 'bigquery.admin' permissions on the project using the reservation and the project that owns this reservation. Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the grant does not match location of the reservation.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reservation` | String |  | Resource name of the reservation. E.g., projects/myproject/locations/eu/reservations/my_reservation. This reservation must be in the same location as the grant. This reservation should belong to the same parent project. |
| `state` | String |  | Output only. State of the ReservationGrant. |
| `name` | String |  | Output only. Name of the resource. E.g.: projects/myproject/locations/eu/reservationGrants/123. |
| `grantee` | String |  | The resource which will use the reservation. E.g. projects/myproject, folders/123, organizations/456. |
| `job_type` | String |  | Which type of jobs will use the reservation. |
| `parent` | String | ✅ | The parent resource name of the reservation grant E.g.: projects/myproject/location/eu. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `reservation_grants` | Vec<String> | List of reservation grants visible to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation_grant
reservation_grant = provider.bigqueryreservation_api.Reservation_grant {
    parent = "value"  # The parent resource name of the reservation grant E.g.: projects/myproject/location/eu.
}

# Access reservation_grant outputs
reservation_grant_id = reservation_grant.id
reservation_grant_next_page_token = reservation_grant.next_page_token
reservation_grant_reservation_grants = reservation_grant.reservation_grants
```

---


### Location

Look up grants for a specified resource for a particular region. If the request is about a project: 1) Grants created on the project will be returned if they exist. 2) Otherwise grants created on the closest ancestor will be returned. 3) Grants for different JobTypes will all be returned. Same logic applies if the request is about a folder. If the request is about an organization, then grants created on the organization will be returned (organization doesn't have ancestors). Comparing to ListReservationGrants, there are two behavior differences: 1) permission on the grantee will be verified in this API. 2) Hierarchy lookup (project->folder->organization) happens in this API.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `reservation_grants` | Vec<String> | List of reservation grants visible to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_next_page_token = location.next_page_token
location_reservation_grants = location.reservation_grants
```

---


### Reservation

Creates a new reservation resource. Multiple reservations are created if the ancestor reservations do not exist.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `use_parent_reservation` | bool |  | If true, any query using this reservation will also be submitted to the parent reservation. This allows the query to share the additional slot capacity of the parent with other queries in the parent reservation. If the parent also has this field set to true, then this process will continue until it encounters a reservation for which this is false. If false, a query using this reservation will execute with the maximum slot capacity as specified above. If not specified, default value is true. Ignored for top-level reservation. |
| `name` | String |  | The resource name of the reservation, e.g., "projects/*/locations/*/reservations/dev/team/product". Reservation names (e.g., "dev/team/product") exceeding a depth of six will fail with `google.rpc.Code.INVALID_ARGUMENT`. |
| `slot_capacity` | String |  | Maximum slots available to this reservation and its children. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. In a scan of a multi-partitioned table, a single slot operates on a single partition of the table. If the new reservation's slot capacity exceed the parent's slot capacity or if total slot capacity of the new reservation and its siblings exceeds the parent's slot capacity, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`. |
| `parent` | String | ✅ | Project, location, and (optionally) reservation name. E.g., projects/myproject/locations/us-central1/reservations/parent |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `use_parent_reservation` | bool | If true, any query using this reservation will also be submitted to the parent reservation. This allows the query to share the additional slot capacity of the parent with other queries in the parent reservation. If the parent also has this field set to true, then this process will continue until it encounters a reservation for which this is false. If false, a query using this reservation will execute with the maximum slot capacity as specified above. If not specified, default value is true. Ignored for top-level reservation. |
| `name` | String | The resource name of the reservation, e.g., "projects/*/locations/*/reservations/dev/team/product". Reservation names (e.g., "dev/team/product") exceeding a depth of six will fail with `google.rpc.Code.INVALID_ARGUMENT`. |
| `slot_capacity` | String | Maximum slots available to this reservation and its children. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. In a scan of a multi-partitioned table, a single slot operates on a single partition of the table. If the new reservation's slot capacity exceed the parent's slot capacity or if total slot capacity of the new reservation and its siblings exceeds the parent's slot capacity, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation
reservation = provider.bigqueryreservation_api.Reservation {
    parent = "value"  # Project, location, and (optionally) reservation name. E.g., projects/myproject/locations/us-central1/reservations/parent
}

# Access reservation outputs
reservation_id = reservation.id
reservation_use_parent_reservation = reservation.use_parent_reservation
reservation_name = reservation.name
reservation_slot_capacity = reservation.slot_capacity
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.bigqueryreservation_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
```

---


### Reservation

Creates a new reservation resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ignore_idle_slots` | bool |  | If false, any query or pipeline job using this reservation will use idle slots from other reservations within the same admin project. If true, a query or pipeline job using this reservation will execute with the slot capacity specified in the slot_capacity field at most. |
| `multi_region_auxiliary` | bool |  | Applicable only for reservations located within one of the BigQuery multi-regions (US or EU). If set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region. |
| `concurrency` | String |  | Job concurrency target which sets a soft upper bound on the number of jobs that can run concurrently in this reservation. This is a soft target due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency target will be automatically computed by the system. NOTE: this field is exposed as `target_job_concurrency` in the Information Schema, DDL and BQ CLI. |
| `name` | String |  | The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `creation_time` | String |  | Output only. Creation time of the reservation. |
| `slot_capacity` | String |  | Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false. If the new reservation's slot capacity exceeds the project's slot capacity or if total slot capacity of the new reservation and its siblings exceeds the project's slot capacity, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`. NOTE: for reservations in US or EU multi-regions, slot capacity constraints are checked separately for default and auxiliary regions. See multi_region_auxiliary flag for more details. |
| `update_time` | String |  | Output only. Last update time of the reservation. |
| `parent` | String | ✅ | Required. Project, location. E.g., `projects/myproject/locations/US` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ignore_idle_slots` | bool | If false, any query or pipeline job using this reservation will use idle slots from other reservations within the same admin project. If true, a query or pipeline job using this reservation will execute with the slot capacity specified in the slot_capacity field at most. |
| `multi_region_auxiliary` | bool | Applicable only for reservations located within one of the BigQuery multi-regions (US or EU). If set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region. |
| `concurrency` | String | Job concurrency target which sets a soft upper bound on the number of jobs that can run concurrently in this reservation. This is a soft target due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency target will be automatically computed by the system. NOTE: this field is exposed as `target_job_concurrency` in the Information Schema, DDL and BQ CLI. |
| `name` | String | The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `creation_time` | String | Output only. Creation time of the reservation. |
| `slot_capacity` | String | Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false. If the new reservation's slot capacity exceeds the project's slot capacity or if total slot capacity of the new reservation and its siblings exceeds the project's slot capacity, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`. NOTE: for reservations in US or EU multi-regions, slot capacity constraints are checked separately for default and auxiliary regions. See multi_region_auxiliary flag for more details. |
| `update_time` | String | Output only. Last update time of the reservation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reservation
reservation = provider.bigqueryreservation_api.Reservation {
    parent = "value"  # Required. Project, location. E.g., `projects/myproject/locations/US`
}

# Access reservation outputs
reservation_id = reservation.id
reservation_ignore_idle_slots = reservation.ignore_idle_slots
reservation_multi_region_auxiliary = reservation.multi_region_auxiliary
reservation_concurrency = reservation.concurrency
reservation_name = reservation.name
reservation_creation_time = reservation.creation_time
reservation_slot_capacity = reservation.slot_capacity
reservation_update_time = reservation.update_time
```

---


### Capacity_commitment

Creates a new capacity commitment resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the commitment. |
| `commitment_start_time` | String |  | Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments. |
| `commitment_end_time` | String |  | Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments. |
| `failure_status` | String |  | Output only. For FAILED commitment plan, provides the reason of failure. |
| `renewal_plan` | String |  | The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL commitments. |
| `name` | String |  | Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `plan` | String |  | Capacity commitment commitment plan. |
| `multi_region_auxiliary` | bool |  | Applicable only for commitments located within one of the BigQuery multi-regions (US or EU). If set to true, this commitment is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this commitment is placed in the organization's default region. |
| `slot_count` | String |  | Number of slots in this commitment. |
| `parent` | String | ✅ | Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the commitment. |
| `commitment_start_time` | String | Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments. |
| `commitment_end_time` | String | Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments. |
| `failure_status` | String | Output only. For FAILED commitment plan, provides the reason of failure. |
| `renewal_plan` | String | The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL commitments. |
| `name` | String | Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters. |
| `plan` | String | Capacity commitment commitment plan. |
| `multi_region_auxiliary` | bool | Applicable only for commitments located within one of the BigQuery multi-regions (US or EU). If set to true, this commitment is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this commitment is placed in the organization's default region. |
| `slot_count` | String | Number of slots in this commitment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create capacity_commitment
capacity_commitment = provider.bigqueryreservation_api.Capacity_commitment {
    parent = "value"  # Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US`
}

# Access capacity_commitment outputs
capacity_commitment_id = capacity_commitment.id
capacity_commitment_state = capacity_commitment.state
capacity_commitment_commitment_start_time = capacity_commitment.commitment_start_time
capacity_commitment_commitment_end_time = capacity_commitment.commitment_end_time
capacity_commitment_failure_status = capacity_commitment.failure_status
capacity_commitment_renewal_plan = capacity_commitment.renewal_plan
capacity_commitment_name = capacity_commitment.name
capacity_commitment_plan = capacity_commitment.plan
capacity_commitment_multi_region_auxiliary = capacity_commitment.multi_region_auxiliary
capacity_commitment_slot_count = capacity_commitment.slot_count
```

---


### Location

Looks up assignments for a specified resource for a particular region. If the request is about a project: 1. Assignments created on the project will be returned if they exist. 2. Otherwise assignments created on the closest ancestor will be returned. 3. Assignments for different JobTypes will all be returned. The same logic applies if the request is about a folder. If the request is about an organization, then assignments created on the organization will be returned (organization doesn't have ancestors). Comparing to ListAssignments, there are some behavior differences: 1. permission on the assignee will be verified in this API. 2. Hierarchy lookup (project->folder->organization) happens in this API. 3. Parent here is `projects/*/locations/*`, instead of `projects/*/locations/*reservations/*`. **Note** "-" cannot be used for projects nor locations.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last update timestamp of a reservation. |
| `size` | String |  | Size of a reservation, in bytes. |
| `preferred_tables` | Vec<String> |  | Preferred tables to use BI capacity for. |
| `name` | String |  | The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`. |
| `name` | String | ✅ | The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `assignments` | Vec<String> | List of assignments visible to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_next_page_token = location.next_page_token
location_assignments = location.assignments
```

---


### Assignment

Creates an assignment object which allows the given project to submit jobs of a certain type using slots from the specified reservation. Currently a resource (project, folder, organization) can only have one assignment per each (job_type, location) combination, and that reservation will be used for all jobs of the matching type. Different assignments can be created on different levels of the projects, folders or organization hierarchy. During query execution, the assignment is looked up at the project, folder and organization levels in that order. The first assignment found is applied to the query. When creating assignments, it does not matter if other assignments exist at higher levels. Example: * The organization `organizationA` contains two projects, `project1` and `project2`. * Assignments for all three entities (`organizationA`, `project1`, and `project2`) could all be created and mapped to the same or different reservations. "None" assignments represent an absence of the assignment. Projects assigned to None use on-demand pricing. To create a "None" assignment, use "none" as a reservation_id in the parent. Example parent: `projects/myproject/locations/US/reservations/none`. Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have 'bigquery.admin' permissions on the project using the reservation and the project that owns this reservation. Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment does not match location of the reservation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_type` | String |  | Which type of jobs will use the reservation. |
| `assignee` | String |  | The resource which will use the reservation. E.g. `projects/myproject`, `folders/123`, or `organizations/456`. |
| `name` | String |  | Output only. Name of the resource. E.g.: `projects/myproject/locations/US/reservations/team1-prod/assignments/123`. The assignment_id must only contain lower case alphanumeric characters or dashes and the max length is 64 characters. |
| `state` | String |  | Output only. State of the assignment. |
| `parent` | String | ✅ | Required. The parent resource name of the assignment E.g. `projects/myproject/locations/US/reservations/team1-prod` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `assignments` | Vec<String> | List of assignments visible to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create assignment
assignment = provider.bigqueryreservation_api.Assignment {
    parent = "value"  # Required. The parent resource name of the assignment E.g. `projects/myproject/locations/US/reservations/team1-prod`
}

# Access assignment outputs
assignment_id = assignment.id
assignment_next_page_token = assignment.next_page_token
assignment_assignments = assignment.assignments
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple reservation_group resources
reservation_group_0 = provider.bigqueryreservation_api.Reservation_group {
    parent = "value-0"
}
reservation_group_1 = provider.bigqueryreservation_api.Reservation_group {
    parent = "value-1"
}
reservation_group_2 = provider.bigqueryreservation_api.Reservation_group {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    reservation_group = provider.bigqueryreservation_api.Reservation_group {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigqueryreservation_api Documentation](https://cloud.google.com/bigqueryreservation_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
