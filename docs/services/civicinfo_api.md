# Civicinfo_api Service



**Resources**: 2

---

## Overview

The civicinfo_api service provides access to 2 resource types:

- [Election](#election) [R]
- [Division](#division) [R]

---

## Resources


### Election

Looks up information relevant to a voter based on the voter's registered address.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `polling_locations` | Vec<String> | Locations where the voter is eligible to vote on election day. |
| `drop_off_locations` | Vec<String> | Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field. |
| `precinct_id` | String |  |
| `precincts` | Vec<String> | The precincts that match this voter's address. Will only be returned for project IDs which have been allowlisted as "partner projects". |
| `early_vote_sites` | Vec<String> | Locations where the voter is eligible to vote early, prior to election day. |
| `state` | Vec<String> | Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array. |
| `mail_only` | bool | Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well). |
| `election` | String | The election that was queried. |
| `normalized_input` | String | The normalized version of the requested address |
| `contests` | Vec<String> | Contests that will appear on the voter's ballot. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "civicinfo#voterInfoResponse". |
| `other_elections` | Vec<String> | When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access election outputs
election_id = election.id
election_polling_locations = election.polling_locations
election_drop_off_locations = election.drop_off_locations
election_precinct_id = election.precinct_id
election_precincts = election.precincts
election_early_vote_sites = election.early_vote_sites
election_state = election.state
election_mail_only = election.mail_only
election_election = election.election
election_normalized_input = election.normalized_input
election_contests = election.contests
election_kind = election.kind
election_other_elections = election.other_elections
```

---


### Division

Searches for political divisions by their natural name or OCD ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> |  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "civicinfo#divisionSearchResponse". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access division outputs
division_id = division.id
division_results = division.results
division_kind = division.kind
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple election resources
election_0 = provider.civicinfo_api.Election {
}
election_1 = provider.civicinfo_api.Election {
}
election_2 = provider.civicinfo_api.Election {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    election = provider.civicinfo_api.Election {
    }
```

---

## Related Documentation

- [GCP Civicinfo_api Documentation](https://cloud.google.com/civicinfo_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
