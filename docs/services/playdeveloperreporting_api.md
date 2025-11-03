# Playdeveloperreporting_api Service



**Resources**: 24

---

## Overview

The playdeveloperreporting_api service provides access to 24 resource types:

- [Stuckbackgroundwakelockrate](#stuckbackgroundwakelockrate) [CR]
- [App](#app) [R]
- [Lmkrate](#lmkrate) [CR]
- [Anomalie](#anomalie) [R]
- [Excessivewakeuprate](#excessivewakeuprate) [CR]
- [Crashrate](#crashrate) [CR]
- [Anrrate](#anrrate) [CR]
- [Slowrenderingrate](#slowrenderingrate) [CR]
- [Slowstartrate](#slowstartrate) [CR]
- [Issue](#issue) [R]
- [Count](#count) [CR]
- [Report](#report) [R]
- [Report](#report) [R]
- [App](#app) [R]
- [Lmkrate](#lmkrate) [CR]
- [Excessivewakeuprate](#excessivewakeuprate) [CR]
- [Crashrate](#crashrate) [CR]
- [Anrrate](#anrrate) [CR]
- [Issue](#issue) [R]
- [Count](#count) [CR]
- [Slowrenderingrate](#slowrenderingrate) [CR]
- [Slowstartrate](#slowstartrate) [CR]
- [Stuckbackgroundwakelockrate](#stuckbackgroundwakelockrate) [CR]
- [Anomalie](#anomalie) [R]

---

## Resources


### Stuckbackgroundwakelockrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `stuckBgWakelockRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a wakelock held in the background for longer than 1 hour. * `stuckBgWakelockRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `stuckBgWakelockRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `stuckBgWakelockRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `stuckBgWakelockRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `stuckBgWakelockRate` metric. A user is counted in this metric if they app was doing any work on the device, i.e., not just active foreground usage but also background work. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stuckbackgroundwakelockrate
stuckbackgroundwakelockrate = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet
}

# Access stuckbackgroundwakelockrate outputs
stuckbackgroundwakelockrate_id = stuckbackgroundwakelockrate.id
stuckbackgroundwakelockrate_freshness_info = stuckbackgroundwakelockrate.freshness_info
stuckbackgroundwakelockrate_name = stuckbackgroundwakelockrate.name
```

---


### App

Describes filtering options for releases.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tracks` | Vec<String> | List of tracks to filter releases over. Provides the grouping of version codes under releases and tracks. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_tracks = app.tracks
```

---


### Lmkrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimensions` | Vec<String> |  | Optional. Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `timeline_spec` | String |  | Optional. Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. |
| `user_cohort` | String |  | Optional. User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `filter` | String |  | Optional. Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `page_token` | String |  | Optional. A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `page_size` | i64 |  | Optional. Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `metrics` | Vec<String> |  | Optional. Metrics to aggregate. **Supported metrics:** * `userPerceivedLmkRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one LMK while they were actively using your app (a user-perceived LMK). An app is considered to be in active use if it is displaying any activity or executing any foreground service. * `userPerceivedLmkRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedLmkRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `userPerceivedLmkRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedLmkRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `userPerceivedLmkRate` metrics. A user is counted in this metric if they used the app in the foreground during the aggregation period. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/lmkRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/lmkRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lmkrate
lmkrate = provider.playdeveloperreporting_api.Lmkrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/lmkRateMetricSet
}

# Access lmkrate outputs
lmkrate_id = lmkrate.id
lmkrate_freshness_info = lmkrate.freshness_info
lmkrate_name = lmkrate.name
```

---


### Anomalie

Lists anomalies in any of the datasets.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anomalies` | Vec<String> | Anomalies that were found. |
| `next_page_token` | String | Continuation token to fetch the next page of data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access anomalie outputs
anomalie_id = anomalie.id
anomalie_anomalies = anomalie.anomalies
anomalie_next_page_token = anomalie.next_page_token
```

---


### Excessivewakeuprate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `excessiveWakeupRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had more than 10 wakeups per hour. * `excessiveWakeupRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `excessiveWakeupRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `excessiveWakeupRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `excessiveWakeupRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `excessiveWakeupRate` metric. A user is counted in this metric if they app was doing any work on the device, i.e., not just active foreground usage but also background work. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create excessivewakeuprate
excessivewakeuprate = provider.playdeveloperreporting_api.Excessivewakeuprate {
    name = "value"  # Required. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet
}

# Access excessivewakeuprate outputs
excessivewakeuprate_id = excessivewakeuprate.id
excessivewakeuprate_name = excessivewakeuprate.name
excessivewakeuprate_freshness_info = excessivewakeuprate.freshness_info
```

---


### Crashrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimensions` | Vec<String> |  | Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `crashRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one crash. * `crashRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `crashRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `crashRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `crashRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedCrashRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one crash while they were actively using your app (a user-perceived crash). An app is considered to be in active use if it is displaying any activity or executing any foreground service. * `userPerceivedCrashRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedCrashRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedCrashRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedCrashRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `crashRate` and `userPerceivedCrashRate` metrics. A user is counted in this metric if they used the app actively during the aggregation period. An app is considered to be in active use if it is displaying any activity or executing any foreground service. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. * HOURLY: metrics are aggregated in hourly intervals. The default and only supported timezone is `UTC`. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/crashRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/crashRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create crashrate
crashrate = provider.playdeveloperreporting_api.Crashrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/crashRateMetricSet
}

# Access crashrate outputs
crashrate_id = crashrate.id
crashrate_name = crashrate.name
crashrate_freshness_info = crashrate.freshness_info
```

---


### Anrrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. * HOURLY: metrics are aggregated in hourly intervals. The default and only supported timezone is `UTC`. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `anrRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one ANR. * `anrRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `anrRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `anrRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `anrRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedAnrRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one user-perceived ANR. User-perceived ANRs are currently those of 'Input dispatching' type. * `userPerceivedAnrRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedAnrRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedAnrRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedAnrRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not . supported in HOURLY granularity. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `anrRate` and `userPerceivedAnrRate` metrics. A user is counted in this metric if they used the app in the foreground during the aggregation period. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `dimensions` | Vec<String> |  | Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/anrRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/anrRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create anrrate
anrrate = provider.playdeveloperreporting_api.Anrrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/anrRateMetricSet
}

# Access anrrate outputs
anrrate_id = anrrate.id
anrrate_freshness_info = anrrate.freshness_info
anrrate_name = anrrate.name
```

---


### Slowrenderingrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `slowRenderingRate20Fps` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow rendering. * `slowRenderingRate20Fps7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate20Fps` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate20Fps28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate20Fps` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate30Fps` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow rendering. * `slowRenderingRate30Fps7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate30Fps` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate30Fps28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate30Fps` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `slowRenderingRate20Fps`/`slowRenderingRate30Fps` metric. A user is counted in this metric if their app was launched in the device. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/slowRenderingRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/slowRenderingRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slowrenderingrate
slowrenderingrate = provider.playdeveloperreporting_api.Slowrenderingrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/slowRenderingRateMetricSet
}

# Access slowrenderingrate outputs
slowrenderingrate_id = slowrenderingrate.id
slowrenderingrate_name = slowrenderingrate.name
slowrenderingrate_freshness_info = slowrenderingrate.freshness_info
```

---


### Slowstartrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `slowStartRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow start. * `slowStartRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowStartRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowStartRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowStartRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `slowStartRate` metric. A user is counted in this metric if their app was launched in the device. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/slowStartRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/slowStartRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slowstartrate
slowstartrate = provider.playdeveloperreporting_api.Slowstartrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/slowStartRateMetricSet
}

# Access slowstartrate outputs
slowstartrate_id = slowstartrate.id
slowstartrate_name = slowstartrate.name
slowstartrate_freshness_info = slowstartrate.freshness_info
```

---


### Issue

Searches all error issues in which reports have been grouped.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_issues` | Vec<String> | ErrorIssues that were found. |
| `next_page_token` | String | Continuation token to fetch the next page of data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access issue outputs
issue_id = issue.id
issue_error_issues = issue.error_issues
issue_next_page_token = issue.next_page_token
```

---


### Count

Queries the metrics in the metrics set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceModel` (string): unique identifier of the user's device model. * `deviceType` (string): identifier of the device's form factor, e.g., PHONE. * `reportType` (string): the type of error. The value should correspond to one of the possible values in ErrorType. * `issueId` (string): the id an error was assigned to. The value should correspond to the `{issue}` component of the issue name. * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. The default and only supported timezone is `America/Los_Angeles`. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions and: * `isUserPerceived` (string): denotes whether error is user perceived or not, USER_PERCEIVED or NOT_USER_PERCEIVED. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `errorReportCount` (`google.type.Decimal`): Absolute count of individual error reports that have been received for an app. * `distinctUsers` (`google.type.Decimal`): Count of distinct users for which reports have been received. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. This value is not rounded, however it may be an approximation. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/errorCountMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | The resource name. Format: apps/{app}/errorCountMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create count
count = provider.playdeveloperreporting_api.Count {
    name = "value"  # Required. The resource name. Format: apps/{app}/errorCountMetricSet
}

# Access count outputs
count_id = count.id
count_freshness_info = count.freshness_info
count_name = count.name
```

---


### Report

Searches all error reports received for an app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_reports` | Vec<String> | Error reports that were found. |
| `next_page_token` | String | Page token to fetch the next page of reports. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_error_reports = report.error_reports
report_next_page_token = report.next_page_token
```

---


### Report

Searches all error reports received for an app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Page token to fetch the next page of reports. |
| `error_reports` | Vec<String> | Error reports that were found. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_next_page_token = report.next_page_token
report_error_reports = report.error_reports
```

---


### App

Searches for Apps accessible by the user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apps` | Vec<String> | The apps accessible to the user calling the endpoint. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_apps = app.apps
app_next_page_token = app.next_page_token
```

---


### Lmkrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional. Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `dimensions` | Vec<String> |  | Optional. Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `filter` | String |  | Optional. Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `page_token` | String |  | Optional. A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `user_cohort` | String |  | Optional. User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `timeline_spec` | String |  | Optional. Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. |
| `metrics` | Vec<String> |  | Optional. Metrics to aggregate. **Supported metrics:** * `userPerceivedLmkRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one LMK while they were actively using your app (a user-perceived LMK). An app is considered to be in active use if it is displaying any activity or executing any foreground service. * `userPerceivedLmkRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedLmkRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `userPerceivedLmkRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedLmkRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `userPerceivedLmkRate` metrics. A user is counted in this metric if they used the app in the foreground during the aggregation period. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/lmkRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/lmkRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lmkrate
lmkrate = provider.playdeveloperreporting_api.Lmkrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/lmkRateMetricSet
}

# Access lmkrate outputs
lmkrate_id = lmkrate.id
lmkrate_freshness_info = lmkrate.freshness_info
lmkrate_name = lmkrate.name
```

---


### Excessivewakeuprate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `excessiveWakeupRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had more than 10 wakeups per hour. * `excessiveWakeupRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `excessiveWakeupRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `excessiveWakeupRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `excessiveWakeupRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `excessiveWakeupRate` metric. A user is counted in this metric if they app was doing any work on the device, i.e., not just active foreground usage but also background work. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create excessivewakeuprate
excessivewakeuprate = provider.playdeveloperreporting_api.Excessivewakeuprate {
    name = "value"  # Required. The resource name. Format: apps/{app}/excessiveWakeupRateMetricSet
}

# Access excessivewakeuprate outputs
excessivewakeuprate_id = excessivewakeuprate.id
excessivewakeuprate_name = excessivewakeuprate.name
excessivewakeuprate_freshness_info = excessivewakeuprate.freshness_info
```

---


### Crashrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. * HOURLY: metrics are aggregated in hourly intervals. The default and only supported timezone is `UTC`. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `crashRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one crash. * `crashRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `crashRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `crashRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `crashRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedCrashRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one crash while they were actively using your app (a user-perceived crash). An app is considered to be in active use if it is displaying any activity or executing any foreground service. * `userPerceivedCrashRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedCrashRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedCrashRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedCrashRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `crashRate` and `userPerceivedCrashRate` metrics. A user is counted in this metric if they used the app actively during the aggregation period. An app is considered to be in active use if it is displaying any activity or executing any foreground service. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `dimensions` | Vec<String> |  | Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/crashRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/crashRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create crashrate
crashrate = provider.playdeveloperreporting_api.Crashrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/crashRateMetricSet
}

# Access crashrate outputs
crashrate_id = crashrate.id
crashrate_name = crashrate.name
crashrate_freshness_info = crashrate.freshness_info
```

---


### Anrrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. **Supported values:** * `OS_PUBLIC` To select data from all publicly released Android versions. This is the default. Supports all the above dimensions. * `APP_TESTERS` To select data from users who have opted in to be testers. Supports all the above dimensions. * `OS_BETA` To select data from beta android versions only, excluding data from released android versions. Only the following dimensions are supported: * `versionCode` (int64): version of the app that was running on the user's device. * `osBuild` (string): OS build of the user's device, e.g., "T1B2.220916.004". |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `anrRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one ANR. * `anrRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `anrRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `anrRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `anrRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedAnrRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that experienced at least one user-perceived ANR. User-perceived ANRs are currently those of 'Input dispatching' type. * `userPerceivedAnrRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedAnrRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. Not supported in HOURLY granularity. * `userPerceivedAnrRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `userPerceivedAnrRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. Not . supported in HOURLY granularity. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `anrRate` and `userPerceivedAnrRate` metrics. A user is counted in this metric if they used the app in the foreground during the aggregation period. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100,000; values above 100,000 will be coerced to 100,000. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `dimensions` | Vec<String> |  | Dimensions to slice the metrics by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the default and only supported timezone is `America/Los_Angeles`. * HOURLY: metrics are aggregated in hourly intervals. The default and only supported timezone is `UTC`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/anrRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/anrRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create anrrate
anrrate = provider.playdeveloperreporting_api.Anrrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/anrRateMetricSet
}

# Access anrrate outputs
anrrate_id = anrrate.id
anrrate_freshness_info = anrrate.freshness_info
anrrate_name = anrrate.name
```

---


### Issue

Searches all error issues in which reports have been grouped.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token to fetch the next page of data. |
| `error_issues` | Vec<String> | ErrorIssues that were found. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access issue outputs
issue_id = issue.id
issue_next_page_token = issue.next_page_token
issue_error_issues = issue.error_issues
```

---


### Count

Queries the metrics in the metrics set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceModel` (string): unique identifier of the user's device model. * `deviceType` (string): identifier of the device's form factor, e.g., PHONE. * `reportType` (string): the type of error. The value should correspond to one of the possible values in ErrorType. * `issueId` (string): the id an error was assigned to. The value should correspond to the `{issue}` component of the issue name. * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions and: * `isUserPerceived` (string): denotes whether error is user perceived or not, USER_PERCEIVED or NOT_USER_PERCEIVED. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `errorReportCount` (`google.type.Decimal`): Absolute count of individual error reports that have been received for an app. * `distinctUsers` (`google.type.Decimal`): Count of distinct users for which reports have been received. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. This value is not rounded, however it may be an approximation. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. The default and only supported timezone is `America/Los_Angeles`. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/errorCountMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name. Format: apps/{app}/errorCountMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create count
count = provider.playdeveloperreporting_api.Count {
    name = "value"  # Required. The resource name. Format: apps/{app}/errorCountMetricSet
}

# Access count outputs
count_id = count.id
count_name = count.name
count_freshness_info = count.freshness_info
```

---


### Slowrenderingrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `slowRenderingRate20Fps` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow rendering. * `slowRenderingRate20Fps7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate20Fps` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate20Fps28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate20Fps` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate30Fps` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow rendering. * `slowRenderingRate30Fps7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate30Fps` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowRenderingRate30Fps28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowRenderingRate30Fps` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `slowRenderingRate20Fps`/`slowRenderingRate30Fps` metric. A user is counted in this metric if their app was launched in the device. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/slowRenderingRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `freshness_info` | String | Summary about data freshness in this resource. |
| `name` | String | Identifier. The resource name. Format: apps/{app}/slowRenderingRateMetricSet |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slowrenderingrate
slowrenderingrate = provider.playdeveloperreporting_api.Slowrenderingrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/slowRenderingRateMetricSet
}

# Access slowrenderingrate outputs
slowrenderingrate_id = slowrenderingrate.id
slowrenderingrate_freshness_info = slowrenderingrate.freshness_info
slowrenderingrate_name = slowrenderingrate.name
```

---


### Slowstartrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `slowStartRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a slow start. * `slowStartRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowStartRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `slowStartRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `slowStartRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `slowStartRate` metric. A user is counted in this metric if their app was launched in the device. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/slowStartRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/slowStartRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slowstartrate
slowstartrate = provider.playdeveloperreporting_api.Slowstartrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/slowStartRateMetricSet
}

# Access slowstartrate outputs
slowstartrate_id = slowstartrate.id
slowstartrate_name = slowstartrate.name
slowstartrate_freshness_info = slowstartrate.freshness_info
```

---


### Stuckbackgroundwakelockrate

Queries the metrics in the metric set.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Maximum size of the returned data. If unspecified, at most 1000 rows will be returned. The maximum value is 100000; values above 100000 will be coerced to 100000. |
| `filter` | String |  | Filters to apply to data. The filtering expression follows [AIP-160](https://google.aip.dev/160) standard and supports filtering by equality of all breakdown dimensions. |
| `page_token` | String |  | A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to the request must match the call that provided the page token. |
| `user_cohort` | String |  | User view to select. The output data will correspond to the selected view. The only supported value is `OS_PUBLIC`. |
| `dimensions` | Vec<String> |  | Dimensions to slice the data by. **Supported dimensions:** * `apiLevel` (string): the API level of Android that was running on the user's device, e.g., 26. * `versionCode` (int64): version of the app that was running on the user's device. * `deviceModel` (string): unique identifier of the user's device model. The form of the identifier is 'deviceBrand/device', where deviceBrand corresponds to Build.BRAND and device corresponds to Build.DEVICE, e.g., google/coral. * `deviceBrand` (string): unique identifier of the user's device brand, e.g., google. * `deviceType` (string): the type (also known as form factor) of the user's device, e.g., PHONE. * `countryCode` (string): the country or region of the user's device based on their IP address, represented as a 2-letter ISO-3166 code (e.g. US for the United States). * `deviceRamBucket` (int64): RAM of the device, in MB, in buckets (3GB, 4GB, etc.). * `deviceSocMake` (string): Make of the device's primary system-on-chip, e.g., Samsung. [Reference](https://developer.android.com/reference/android/os/Build#SOC_MANUFACTURER) * `deviceSocModel` (string): Model of the device's primary system-on-chip, e.g., "Exynos 2100". [Reference](https://developer.android.com/reference/android/os/Build#SOC_MODEL) * `deviceCpuMake` (string): Make of the device's CPU, e.g., Qualcomm. * `deviceCpuModel` (string): Model of the device's CPU, e.g., "Kryo 240". * `deviceGpuMake` (string): Make of the device's GPU, e.g., ARM. * `deviceGpuModel` (string): Model of the device's GPU, e.g., Mali. * `deviceGpuVersion` (string): Version of the device's GPU, e.g., T750. * `deviceVulkanVersion` (string): Vulkan version of the device, e.g., "4198400". * `deviceGlEsVersion` (string): OpenGL ES version of the device, e.g., "196610". * `deviceScreenSize` (string): Screen size of the device, e.g., NORMAL, LARGE. * `deviceScreenDpi` (string): Screen density of the device, e.g., mdpi, hdpi. |
| `metrics` | Vec<String> |  | Metrics to aggregate. **Supported metrics:** * `stuckBgWakelockRate` (`google.type.Decimal`): Percentage of distinct users in the aggregation period that had a wakelock held in the background for longer than 1 hour. * `stuckBgWakelockRate7dUserWeighted` (`google.type.Decimal`): Rolling average value of `stuckBgWakelockRate` in the last 7 days. The daily values are weighted by the count of distinct users for the day. * `stuckBgWakelockRate28dUserWeighted` (`google.type.Decimal`): Rolling average value of `stuckBgWakelockRate` in the last 28 days. The daily values are weighted by the count of distinct users for the day. * `distinctUsers` (`google.type.Decimal`): Count of distinct users in the aggregation period that were used as normalization value for the `stuckBgWakelockRate` metric. A user is counted in this metric if they app was doing any work on the device, i.e., not just active foreground usage but also background work. Care must be taken not to aggregate this count further, as it may result in users being counted multiple times. The value is rounded to the nearest multiple of 10, 100, 1,000 or 1,000,000, depending on the magnitude of the value. |
| `timeline_spec` | String |  | Specification of the timeline aggregation parameters. **Supported aggregation periods:** * DAILY: metrics are aggregated in calendar date intervals. Due to historical constraints, the only supported timezone is `America/Los_Angeles`. |
| `name` | String | ✅ | Required. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet |
| `freshness_info` | String | Summary about data freshness in this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stuckbackgroundwakelockrate
stuckbackgroundwakelockrate = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
    name = "value"  # Required. The resource name. Format: apps/{app}/stuckBackgroundWakelockRateMetricSet
}

# Access stuckbackgroundwakelockrate outputs
stuckbackgroundwakelockrate_id = stuckbackgroundwakelockrate.id
stuckbackgroundwakelockrate_name = stuckbackgroundwakelockrate.name
stuckbackgroundwakelockrate_freshness_info = stuckbackgroundwakelockrate.freshness_info
```

---


### Anomalie

Lists anomalies in any of the datasets.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anomalies` | Vec<String> | Anomalies that were found. |
| `next_page_token` | String | Continuation token to fetch the next page of data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access anomalie outputs
anomalie_id = anomalie.id
anomalie_anomalies = anomalie.anomalies
anomalie_next_page_token = anomalie.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple stuckbackgroundwakelockrate resources
stuckbackgroundwakelockrate_0 = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
    name = "value-0"
}
stuckbackgroundwakelockrate_1 = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
    name = "value-1"
}
stuckbackgroundwakelockrate_2 = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    stuckbackgroundwakelockrate = provider.playdeveloperreporting_api.Stuckbackgroundwakelockrate {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Playdeveloperreporting_api Documentation](https://cloud.google.com/playdeveloperreporting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
