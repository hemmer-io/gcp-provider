# Gmailpostmastertools_api Service



**Resources**: 4

---

## Overview

The gmailpostmastertools_api service provides access to 4 resource types:

- [Domain](#domain) [R]
- [Traffic_stat](#traffic_stat) [R]
- [Domain](#domain) [R]
- [Traffic_stat](#traffic_stat) [R]

---

## Resources


### Domain

Gets a specific domain registered by the client. Returns NOT_FOUND if the domain does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Timestamp when the user registered this domain. Assigned by the server. |
| `name` | String | The resource name of the Domain. Domain names have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com). |
| `permission` | String | User’s permission for this domain. Assigned by the server. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access domain outputs
domain_id = domain.id
domain_create_time = domain.create_time
domain_name = domain.name
domain_permission = domain.permission
```

---


### Traffic_stat

Get traffic statistics for a domain on a specific date. Returns PERMISSION_DENIED if user does not have permission to access TrafficStats for the domain.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_reported_spam_ratio` | f64 | The ratio of user-report spam vs. email that was sent to the inbox. This is potentially inexact -- users may want to refer to the description of the interval fields userReportedSpamRatioLowerBound and userReportedSpamRatioUpperBound for more explicit accuracy guarantees. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `dkim_success_ratio` | f64 | The ratio of mail that successfully authenticated with DKIM vs. all mail that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed mail is excluded. |
| `user_reported_spam_ratio_lower_bound` | f64 | The lower bound of the confidence interval for the user reported spam ratio. If this field is set, then the value of userReportedSpamRatio is set to the midpoint of this interval and is thus inexact. However, the true ratio is guaranteed to be in between this lower bound and the corresponding upper bound 95% of the time. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `delivery_errors` | Vec<String> | Delivery errors for the domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |
| `spammy_feedback_loops` | Vec<String> | Spammy [Feedback loop identifiers] (https://support.google.com/mail/answer/6254652) with their individual spam rates. This metric only pertains to traffic that is authenticated by [DKIM](http://www.dkim.org/). |
| `inbound_encryption_ratio` | f64 | The ratio of incoming mail (to Gmail), that passed secure transport (TLS) vs all mail received from that domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |
| `name` | String | The resource name of the traffic statistics. Traffic statistic names have the form `domains/{domain}/trafficStats/{date}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com) of the domain this traffic statistics pertains to and date is the date in yyyymmdd format that these statistics corresponds to. For example: domains/mymail.mydomain.com/trafficStats/20160807 |
| `ip_reputations` | Vec<String> | Reputation information pertaining to the IP addresses of the email servers for the domain. There is exactly one entry for each reputation category except REPUTATION_CATEGORY_UNSPECIFIED. |
| `spf_success_ratio` | f64 | The ratio of mail that successfully authenticated with SPF vs. all mail that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed mail is excluded. |
| `domain_reputation` | String | Reputation of the domain. |
| `user_reported_spam_ratio_upper_bound` | f64 | The upper bound of the confidence interval for the user reported spam ratio. If this field is set, then the value of userReportedSpamRatio is set to the midpoint of this interval and is thus inexact. However, the true ratio is guaranteed to be in between this upper bound and the corresponding lower bound 95% of the time. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `outbound_encryption_ratio` | f64 | The ratio of outgoing mail (from Gmail) that was accepted over secure transport (TLS). |
| `dmarc_success_ratio` | f64 | The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks vs all mail received from the domain that successfully authenticated with either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access traffic_stat outputs
traffic_stat_id = traffic_stat.id
traffic_stat_user_reported_spam_ratio = traffic_stat.user_reported_spam_ratio
traffic_stat_dkim_success_ratio = traffic_stat.dkim_success_ratio
traffic_stat_user_reported_spam_ratio_lower_bound = traffic_stat.user_reported_spam_ratio_lower_bound
traffic_stat_delivery_errors = traffic_stat.delivery_errors
traffic_stat_spammy_feedback_loops = traffic_stat.spammy_feedback_loops
traffic_stat_inbound_encryption_ratio = traffic_stat.inbound_encryption_ratio
traffic_stat_name = traffic_stat.name
traffic_stat_ip_reputations = traffic_stat.ip_reputations
traffic_stat_spf_success_ratio = traffic_stat.spf_success_ratio
traffic_stat_domain_reputation = traffic_stat.domain_reputation
traffic_stat_user_reported_spam_ratio_upper_bound = traffic_stat.user_reported_spam_ratio_upper_bound
traffic_stat_outbound_encryption_ratio = traffic_stat.outbound_encryption_ratio
traffic_stat_dmarc_success_ratio = traffic_stat.dmarc_success_ratio
```

---


### Domain

Gets a specific domain registered by the client. Returns NOT_FOUND if the domain does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the Domain. Domain names have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com). |
| `create_time` | String | Timestamp when the user registered this domain. Assigned by the server. |
| `permission` | String | User’s permission for this domain. Assigned by the server. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access domain outputs
domain_id = domain.id
domain_name = domain.name
domain_create_time = domain.create_time
domain_permission = domain.permission
```

---


### Traffic_stat

Get traffic statistics for a domain on a specific date. Returns PERMISSION_DENIED if user does not have permission to access TrafficStats for the domain.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spf_success_ratio` | f64 | The ratio of mail that successfully authenticated with SPF vs. all mail that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed mail is excluded. |
| `name` | String | The resource name of the traffic statistics. Traffic statistic names have the form `domains/{domain}/trafficStats/{date}`, where domain_name is the fully qualified domain name (i.e., mymail.mydomain.com) of the domain this traffic statistics pertains to and date is the date in yyyymmdd format that these statistics corresponds to. For example: domains/mymail.mydomain.com/trafficStats/20160807 |
| `spammy_feedback_loops` | Vec<String> | Spammy [Feedback loop identifiers] (https://support.google.com/mail/answer/6254652) with their individual spam rates. This metric only pertains to traffic that is authenticated by [DKIM](http://www.dkim.org/). |
| `delivery_errors` | Vec<String> | Delivery errors for the domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |
| `user_reported_spam_ratio` | f64 | The ratio of user-report spam vs. email that was sent to the inbox. This is potentially inexact -- users may want to refer to the description of the interval fields userReportedSpamRatioLowerBound and userReportedSpamRatioUpperBound for more explicit accuracy guarantees. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `domain_reputation` | String | Reputation of the domain. |
| `dmarc_success_ratio` | f64 | The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks vs all mail received from the domain that successfully authenticated with either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |
| `user_reported_spam_ratio_lower_bound` | f64 | The lower bound of the confidence interval for the user reported spam ratio. If this field is set, then the value of userReportedSpamRatio is set to the midpoint of this interval and is thus inexact. However, the true ratio is guaranteed to be in between this lower bound and the corresponding upper bound 95% of the time. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `user_reported_spam_ratio_upper_bound` | f64 | The upper bound of the confidence interval for the user reported spam ratio. If this field is set, then the value of userReportedSpamRatio is set to the midpoint of this interval and is thus inexact. However, the true ratio is guaranteed to be in between this upper bound and the corresponding lower bound 95% of the time. This metric only pertains to emails authenticated by [DKIM](http://www.dkim.org/). |
| `outbound_encryption_ratio` | f64 | The ratio of outgoing mail (from Gmail) that was accepted over secure transport (TLS). |
| `dkim_success_ratio` | f64 | The ratio of mail that successfully authenticated with DKIM vs. all mail that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed mail is excluded. |
| `inbound_encryption_ratio` | f64 | The ratio of incoming mail (to Gmail), that passed secure transport (TLS) vs all mail received from that domain. This metric only pertains to traffic that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/). |
| `ip_reputations` | Vec<String> | Reputation information pertaining to the IP addresses of the email servers for the domain. There is exactly one entry for each reputation category except REPUTATION_CATEGORY_UNSPECIFIED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access traffic_stat outputs
traffic_stat_id = traffic_stat.id
traffic_stat_spf_success_ratio = traffic_stat.spf_success_ratio
traffic_stat_name = traffic_stat.name
traffic_stat_spammy_feedback_loops = traffic_stat.spammy_feedback_loops
traffic_stat_delivery_errors = traffic_stat.delivery_errors
traffic_stat_user_reported_spam_ratio = traffic_stat.user_reported_spam_ratio
traffic_stat_domain_reputation = traffic_stat.domain_reputation
traffic_stat_dmarc_success_ratio = traffic_stat.dmarc_success_ratio
traffic_stat_user_reported_spam_ratio_lower_bound = traffic_stat.user_reported_spam_ratio_lower_bound
traffic_stat_user_reported_spam_ratio_upper_bound = traffic_stat.user_reported_spam_ratio_upper_bound
traffic_stat_outbound_encryption_ratio = traffic_stat.outbound_encryption_ratio
traffic_stat_dkim_success_ratio = traffic_stat.dkim_success_ratio
traffic_stat_inbound_encryption_ratio = traffic_stat.inbound_encryption_ratio
traffic_stat_ip_reputations = traffic_stat.ip_reputations
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple domain resources
domain_0 = provider.gmailpostmastertools_api.Domain {
}
domain_1 = provider.gmailpostmastertools_api.Domain {
}
domain_2 = provider.gmailpostmastertools_api.Domain {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domain = provider.gmailpostmastertools_api.Domain {
    }
```

---

## Related Documentation

- [GCP Gmailpostmastertools_api Documentation](https://cloud.google.com/gmailpostmastertools_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
