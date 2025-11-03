# Getting Started

This guide will help you get started using the Gcp provider for Hemmer.

---

## Prerequisites

- Hemmer CLI installed
- Gcp provider installed ([Installation Guide](installation.md))
- GCP credentials configured (GOOGLE_APPLICATION_CREDENTIALS)

---

## Basic Usage

### 1. Initialize Provider

Create a new Rust project and add the provider dependency:

```bash
cargo new my-gcp-app
cd my-gcp-app
```

Add to `Cargo.toml`:

```toml
[dependencies]
hemmer-gcp-provider = "*"
hemmer-core = "*"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### 2. Basic Example

```kcl
# main.k
import gcp

# Create provider instance
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create a glossarie
glossarie = provider.translate_api.Glossarie {
    parent = "value"
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    glossarie = provider.translate_api.Glossarie {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
glossarie = provider.translate_api.Glossarie {
    # configuration
}

# Reference its outputs
output_value = glossarie.id
```

---

## Available Services

This provider includes 348 services:

### 1. Translate_api

**Resources**: 18

- Glossarie [CRD]
- Location [CR]
- Operation [CRD]
- Project [CR]
- Detection [CR]
- Translation [CR]
- Language [R]
- Project [CR]
- Adaptive_mt_sentence [R]
- Dataset [CRD]
- Example [R]
- Glossary_entrie [CRUD]
- Operation [CRD]
- Adaptive_mt_dataset [CRD]
- Glossarie [CRUD]
- Location [CR]
- Adaptive_mt_file [RD]
- Model [CRD]

📖 [Full translate_api documentation](services/translate_api.md)

### 2. Area120tables_api

**Resources**: 3

- Row [CRUD]
- Workspace [R]
- Table [R]

📖 [Full area120tables_api documentation](services/area120tables_api.md)

### 3. Abusiveexperiencereport_api

**Resources**: 2

- Site [R]
- Violating_site [R]

📖 [Full abusiveexperiencereport_api documentation](services/abusiveexperiencereport_api.md)

### 4. Securesourcemanager_api

**Resources**: 10

- Pull_request_comment [CRUD]
- Branch_rule [CRUD]
- Issue_comment [CRUD]
- Repositorie [CRUD]
- Instance [CRD]
- Hook [CRUD]
- Location [R]
- Issue [CRUD]
- Operation [CRD]
- Pull_request [CRU]

📖 [Full securesourcemanager_api documentation](services/securesourcemanager_api.md)

### 5. File_api

**Resources**: 11

- Operation [CRD]
- Location [R]
- Instance [CRUD]
- Backup [CRUD]
- Snapshot [CRUD]
- Snapshot [CRUD]
- Location [R]
- Backup [CRUD]
- Share [CRUD]
- Instance [CRUD]
- Operation [CRD]

📖 [Full file_api documentation](services/file_api.md)

### 6. Oauth2_api

**Resources**: 6

- Me [R]
- Oauth2 [C]
- Userinfo [R]
- Userinfo [R]
- Me [R]
- Oauth2 [C]

📖 [Full oauth2_api documentation](services/oauth2_api.md)

### 7. Serviceuser_api

**Resources**: 1

- Service [CR]

📖 [Full serviceuser_api documentation](services/serviceuser_api.md)

### 8. Bigquerydatatransfer_api

**Resources**: 6

- Location [CR]
- Transfer_config [CRUD]
- Data_source [CR]
- Project [C]
- Run [RD]
- Transfer_log [R]

📖 [Full bigquerydatatransfer_api documentation](services/bigquerydatatransfer_api.md)

### 9. Cloudfunctions_api

**Resources**: 18

- Function [CRUD]
- Operation [R]
- Location [R]
- Location [R]
- Function [CRUD]
- Operation [R]
- Operation [R]
- Function [CRUD]
- Location [R]
- Runtime [R]
- Function [CRUD]
- Location [R]
- Operation [R]
- Runtime [R]
- Location [R]
- Runtime [R]
- Operation [R]
- Function [CRUD]

📖 [Full cloudfunctions_api documentation](services/cloudfunctions_api.md)

### 10. Networkconnectivity_api

**Resources**: 22

- Multicloud_data_transfer_supported_service [R]
- Service_connection_token [CRD]
- Operation [CRD]
- Spoke [CRUD]
- Group [CRU]
- Hub [CRUD]
- Service_classe [CRUD]
- Route [R]
- Location [CR]
- Internal_range [CRUD]
- Service_connection_map [CRUD]
- Regional_endpoint [CRD]
- Destination [CRUD]
- Service_connection_policie [CRUD]
- Policy_based_route [CRD]
- Multicloud_data_transfer_config [CRUD]
- Route_table [R]
- Operation [CRD]
- Hub [CRUD]
- Internal_range [CRUD]
- Location [R]
- Spoke [CRUD]

📖 [Full networkconnectivity_api documentation](services/networkconnectivity_api.md)

### 11. Mybusinessverifications_api

**Resources**: 3

- Verification [CR]
- Location [CR]
- Verification_token [C]

📖 [Full mybusinessverifications_api documentation](services/mybusinessverifications_api.md)

### 12. Qpxexpress_api

**Resources**: 1

- Trip [C]

📖 [Full qpxexpress_api documentation](services/qpxexpress_api.md)

### 13. Dataproc_api

**Resources**: 15

- Node_group [CR]
- Batche [CRD]
- Spark_application [CR]
- Workflow_template [CRUD]
- Cluster [CRUD]
- Job [CRUD]
- Autoscaling_policie [CRUD]
- Session_template [CRUD]
- Session [CRD]
- Operation [CRD]
- Operation [CRD]
- Job [CRUD]
- Autoscaling_policie [CRUD]
- Cluster [CRUD]
- Workflow_template [CRUD]

📖 [Full dataproc_api documentation](services/dataproc_api.md)

### 14. Playgrouping_api

**Resources**: 2

- Tag [C]
- Token [C]

📖 [Full playgrouping_api documentation](services/playgrouping_api.md)

### 15. Mirror_api

**Resources**: 7

- Location [R]
- Contact [CRUD]
- Timeline [CRUD]
- Setting [R]
- Account [C]
- Attachment [CRD]
- Subscription [CRUD]

📖 [Full mirror_api documentation](services/mirror_api.md)

### 16. Adexchangeseller_api

**Resources**: 27

- Adclient [R]
- Urlchannel [R]
- Adunit [R]
- Report [R]
- Saved [R]
- Customchannel [R]
- Account [R]
- Report [R]
- Metric [R]
- Alert [R]
- Urlchannel [R]
- Dimension [R]
- Customchannel [R]
- Saved [R]
- Adclient [R]
- Preferreddeal [R]
- Urlchannel [R]
- Alert [R]
- Adunit [R]
- Customchannel [R]
- Saved [R]
- Metric [R]
- Adclient [R]
- Account [R]
- Dimension [R]
- Report [R]
- Preferreddeal [R]

📖 [Full adexchangeseller_api documentation](services/adexchangeseller_api.md)

### 17. Binaryauthorization_api

**Resources**: 9

- Policy [CR]
- Systempolicy [R]
- Policie [CRUD]
- Attestor [CRUD]
- Project [RU]
- Attestor [CRUD]
- Systempolicy [R]
- Project [RU]
- Policy [CR]

📖 [Full binaryauthorization_api documentation](services/binaryauthorization_api.md)

### 18. Marketingplatformadmin_api

**Resources**: 2

- Organization [CR]
- Analytics_account_link [CRD]

📖 [Full marketingplatformadmin_api documentation](services/marketingplatformadmin_api.md)

### 19. Cloudprivatecatalogproducer_api

**Resources**: 6

- Operation [CRD]
- Icon [C]
- Catalog [CRUD]
- Product [CRUD]
- Version [CRUD]
- Association [CRD]

📖 [Full cloudprivatecatalogproducer_api documentation](services/cloudprivatecatalogproducer_api.md)

### 20. Pubsublite_api

**Resources**: 5

- Topic [CRUD]
- Cursor [R]
- Operation [CRD]
- Reservation [CRUD]
- Subscription [CRUD]

📖 [Full pubsublite_api documentation](services/pubsublite_api.md)

### 21. Slides_api

**Resources**: 2

- Page [R]
- Presentation [CR]

📖 [Full slides_api documentation](services/slides_api.md)

### 22. Billingbudgets_api

**Resources**: 2

- Budget [CRUD]
- Budget [CRUD]

📖 [Full billingbudgets_api documentation](services/billingbudgets_api.md)

### 23. Workstations_api

**Resources**: 9

- Workstation_cluster [CRUD]
- Operation [CRD]
- Workstation [CRUD]
- Location [R]
- Workstation_config [CRUD]
- Workstation_config [CRUD]
- Workstation_cluster [CRUD]
- Operation [CRD]
- Workstation [CRUD]

📖 [Full workstations_api documentation](services/workstations_api.md)

### 24. Civicinfo_api

**Resources**: 2

- Election [R]
- Division [R]

📖 [Full civicinfo_api documentation](services/civicinfo_api.md)

### 25. Spectrum_api

**Resources**: 1

- Paw [C]

📖 [Full spectrum_api documentation](services/spectrum_api.md)

### 26. Surveys_api

**Resources**: 2

- Survey [CRUD]
- Result [R]

📖 [Full surveys_api documentation](services/surveys_api.md)

### 27. Dataplex_api

**Resources**: 33

- Entrie [CRUD]
- Governance_rule [CR]
- Encryption_config [CRUD]
- Action [R]
- Data_asset [C]
- Glossarie [CRUD]
- Partition [CRD]
- Job [CR]
- Environment [CRUD]
- Content [CRUD]
- Zone [CRUD]
- Data_attribute_binding [CRUD]
- Entry_link [CRD]
- Aspect_type [CRUD]
- Entry_link_type [CR]
- Term [CRUD]
- Session [R]
- Operation [CRD]
- Change_request [CR]
- Entitie [CRUD]
- Lake [CRUD]
- Task [CRUD]
- Data_taxonomie [CRUD]
- Entry_group [CRUD]
- Contentitem [CRUD]
- Data_scan [CRUD]
- Attribute [CRUD]
- Location [CR]
- Entry_type [CRUD]
- Metadata_job [CR]
- Categorie [CRUD]
- Data_product [C]
- Asset [CRUD]

📖 [Full dataplex_api documentation](services/dataplex_api.md)

### 28. Redis_api

**Resources**: 12

- Operation [CRD]
- Instance [CRUD]
- Cluster [CRUD]
- Backup_collection [R]
- Backup [CRD]
- Location [R]
- Instance [CRUD]
- Cluster [CRUD]
- Location [R]
- Backup_collection [R]
- Operation [CRD]
- Backup [CRD]

📖 [Full redis_api documentation](services/redis_api.md)

### 29. Addressvalidation_api

**Resources**: 1

- Addressvalidation [C]

📖 [Full addressvalidation_api documentation](services/addressvalidation_api.md)

### 30. Sourcerepo_api

**Resources**: 2

- Repo [CRUD]
- Project [RU]

📖 [Full sourcerepo_api documentation](services/sourcerepo_api.md)

### 31. Osconfig_api

**Resources**: 24

- Os_policy_assignment [CRUD]
- Global [RU]
- Operation [CR]
- Inventorie [R]
- Patch_job [CR]
- Report [R]
- Vulnerability_report [R]
- Instance_detail [R]
- Patch_deployment [CRUD]
- Vulnerability_report [R]
- Report [R]
- Os_policy_assignment [CRUD]
- Instance_os_policies_compliance [R]
- Operation [CR]
- Inventorie [R]
- Operation [CRD]
- Policy_orchestrator [CRUD]
- Operation [CRD]
- Policy_orchestrator [CRUD]
- Guest_policie [CRUD]
- Patch_job [CR]
- Instance_detail [R]
- Instance [C]
- Patch_deployment [CRUD]

📖 [Full osconfig_api documentation](services/osconfig_api.md)

### 32. Domainsrdap_api

**Resources**: 6

- Autnum [R]
- Domain [R]
- Entity [R]
- Nameserver [R]
- Ip [R]
- Domainsrdap [R]

📖 [Full domainsrdap_api documentation](services/domainsrdap_api.md)

### 33. Speech_api

**Resources**: 12

- Phrase_set [CRUD]
- Custom_classe [CRUD]
- Operation [R]
- Speech [C]
- Operation [R]
- Operation [R]
- Speech [C]
- Operation [R]
- Speech [C]
- Custom_classe [CRUD]
- Phrase_set [CRUD]
- Operation [R]

📖 [Full speech_api documentation](services/speech_api.md)

### 34. Trafficdirector_api

**Resources**: 2

- Discovery [C]
- Discovery [C]

📖 [Full trafficdirector_api documentation](services/trafficdirector_api.md)

### 35. Indexing_api

**Resources**: 1

- Url_notification [CR]

📖 [Full indexing_api documentation](services/indexing_api.md)

### 36. Ondemandscanning_api

**Resources**: 6

- Scan [C]
- Vulnerabilitie [R]
- Operation [CRD]
- Vulnerabilitie [R]
- Scan [C]
- Operation [CRD]

📖 [Full ondemandscanning_api documentation](services/ondemandscanning_api.md)

### 37. Drive_api

**Resources**: 27

- Teamdrive [CRUD]
- Comment [CRUD]
- About [R]
- Children [CRD]
- App [R]
- File [CRUD]
- Propertie [CRUD]
- Drive [CRUD]
- Permission [CRUD]
- Channel [C]
- Parent [CRD]
- Revision [RUD]
- Change [CR]
- Replie [CRUD]
- Change [CR]
- Revision [RUD]
- App [R]
- Accessproposal [CR]
- Replie [CRUD]
- Permission [CRUD]
- Channel [C]
- Drive [CRUD]
- Operation [R]
- Comment [CRUD]
- About [R]
- Teamdrive [CRUD]
- File [CRUD]

📖 [Full drive_api documentation](services/drive_api.md)

### 38. Mybusinessnotifications_api

**Resources**: 1

- Account [RU]

📖 [Full mybusinessnotifications_api documentation](services/mybusinessnotifications_api.md)

### 39. Workflowexecutions_api

**Resources**: 5

- Callback [R]
- Workflow [C]
- Execution [CR]
- Step_entrie [R]
- Execution [CR]

📖 [Full workflowexecutions_api documentation](services/workflowexecutions_api.md)

### 40. Developerconnect_api

**Resources**: 7

- Account_connector [CRUD]
- User [CRD]
- Connection [CRUD]
- Git_repository_link [CRD]
- Operation [CRD]
- Location [R]
- Insights_config [CRUD]

📖 [Full developerconnect_api documentation](services/developerconnect_api.md)

### 41. Analyticsadmin_api

**Resources**: 39

- Reporting_data_annotation [CRUD]
- Measurement_protocol_secret [CRUD]
- Audience [CRU]
- Propertie [CRUD]
- Subproperty_sync_config [RU]
- Search_ads360_link [CRUD]
- Display_video360_advertiser_link_proposal [CRD]
- Account [CRUD]
- Calculated_metric [CRUD]
- Access_binding [CRUD]
- Ad_sense_link [CRD]
- Subproperty_event_filter [CRUD]
- Display_video360_advertiser_link [CRUD]
- Custom_dimension [CRU]
- Rollup_property_source_link [CRD]
- Channel_group [CRUD]
- Expanded_data_set [CRUD]
- Key_event [CRUD]
- Custom_metric [CRU]
- Event_edit_rule [CRUD]
- Event_create_rule [CRUD]
- Sk_ad_network_conversion_value_schema [CRUD]
- Big_query_link [CRUD]
- Google_ads_link [CRUD]
- Account_summarie [R]
- Firebase_link [CRD]
- Conversion_event [CRUD]
- Data_stream [CRUD]
- Measurement_protocol_secret [CRUD]
- Account_summarie [R]
- Custom_metric [CRU]
- Data_stream [CRUD]
- Propertie [CRUD]
- Google_ads_link [CRUD]
- Account [CRUD]
- Custom_dimension [CRU]
- Key_event [CRUD]
- Conversion_event [CRUD]
- Firebase_link [CRD]

📖 [Full analyticsadmin_api documentation](services/analyticsadmin_api.md)

### 42. Playdeveloperreporting_api

**Resources**: 24

- Stuckbackgroundwakelockrate [CR]
- App [R]
- Lmkrate [CR]
- Anomalie [R]
- Excessivewakeuprate [CR]
- Crashrate [CR]
- Anrrate [CR]
- Slowrenderingrate [CR]
- Slowstartrate [CR]
- Issue [R]
- Count [CR]
- Report [R]
- Report [R]
- App [R]
- Lmkrate [CR]
- Excessivewakeuprate [CR]
- Crashrate [CR]
- Anrrate [CR]
- Issue [R]
- Count [CR]
- Slowrenderingrate [CR]
- Slowstartrate [CR]
- Stuckbackgroundwakelockrate [CR]
- Anomalie [R]

📖 [Full playdeveloperreporting_api documentation](services/playdeveloperreporting_api.md)

### 43. Prod_tt_sasportal_api

**Resources**: 6

- Deployment [CRUD]
- Policie [CR]
- Installer [C]
- Customer [CRU]
- Node [CRUD]
- Device [CRUD]

📖 [Full prod_tt_sasportal_api documentation](services/prod_tt_sasportal_api.md)

### 44. Searchconsole_api

**Resources**: 5

- Sitemap [RUD]
- Searchanalytic [C]
- Index [C]
- Site [RUD]
- Mobile_friendly_test [C]

📖 [Full searchconsole_api documentation](services/searchconsole_api.md)

### 45. Firebaseremoteconfig_api

**Resources**: 1

- Project [RU]

📖 [Full firebaseremoteconfig_api documentation](services/firebaseremoteconfig_api.md)

### 46. Publicca_api

**Resources**: 3

- External_account_key [C]
- External_account_key [C]
- External_account_key [C]

📖 [Full publicca_api documentation](services/publicca_api.md)

### 47. Discovery_api

**Resources**: 1

- Api [R]

📖 [Full discovery_api documentation](services/discovery_api.md)

### 48. Firebasehosting_api

**Resources**: 9

- Operation [CRD]
- Release [CR]
- Custom_domain [CRUD]
- Site [CRUD]
- Operation [R]
- File [R]
- Domain [CRUD]
- Channel [CRUD]
- Version [CRUD]

📖 [Full firebasehosting_api documentation](services/firebasehosting_api.md)

### 49. Memcache_api

**Resources**: 6

- Location [R]
- Instance [CRUD]
- Operation [CRD]
- Operation [CRD]
- Location [R]
- Instance [CRUD]

📖 [Full memcache_api documentation](services/memcache_api.md)

### 50. Playablelocations_api

**Resources**: 1

- Playablelocation [C]

📖 [Full playablelocations_api documentation](services/playablelocations_api.md)

### 51. Jobs_api

**Resources**: 16

- Job [CRUD]
- Companie [CRUD]
- Job [CRUD]
- Client_event [C]
- Companie [CRUD]
- Operation [R]
- Tenant [CRUD]
- Client_event [C]
- Companie [CRUD]
- Project [R]
- Job [CRUD]
- Job [CRUD]
- Companie [CRUD]
- Project [R]
- Client_event [C]
- Operation [R]

📖 [Full jobs_api documentation](services/jobs_api.md)

### 52. Iam_api

**Resources**: 20

- Namespace [CRUD]
- Permission [C]
- Scim_tenant [CRUD]
- Operation [R]
- Token [CRUD]
- Service_account [CRUD]
- Oauth_client [CRUD]
- Managed_identitie [CRUD]
- Provider [CRUD]
- Key [CRD]
- Iam_policie [C]
- Subject [CD]
- Workload_identity_pool [CRUD]
- Credential [CRUD]
- Role [CRUD]
- Workforce_pool [CRUD]
- Operation [R]
- Policie [CRUD]
- Operation [R]
- Policie [CRUD]

📖 [Full iam_api documentation](services/iam_api.md)

### 53. Reseller_api

**Resources**: 3

- Customer [CRU]
- Resellernotify [CR]
- Subscription [CRD]

📖 [Full reseller_api documentation](services/reseller_api.md)

### 54. Realtimebidding_api

**Resources**: 8

- Pretargeting_config [CRUD]
- User_list [CRU]
- Publisher_connection [CR]
- Creative [CRU]
- Bidder [R]
- Buyer [R]
- Endpoint [RU]
- Bidding_function [CR]

📖 [Full realtimebidding_api documentation](services/realtimebidding_api.md)

### 55. Datamigration_api

**Resources**: 12

- Conversion_workspace [CRUD]
- Location [R]
- Connection_profile [CRUD]
- Operation [CRD]
- Migration_job [CRUD]
- Mapping_rule [CRD]
- Object [CR]
- Private_connection [CRD]
- Connection_profile [CRUD]
- Migration_job [CRUD]
- Location [R]
- Operation [CRD]

📖 [Full datamigration_api documentation](services/datamigration_api.md)

### 56. Safebrowsing_api

**Resources**: 9

- Hash_list [R]
- Hashe [R]
- Threat_list [R]
- Threat_hit [C]
- Threat_list_update [C]
- Threat_matche [C]
- Full_hashe [C]
- Encoded_update [R]
- Encoded_full_hashe [R]

📖 [Full safebrowsing_api documentation](services/safebrowsing_api.md)

### 57. Libraryagent_api

**Resources**: 2

- Book [CR]
- Shelve [R]

📖 [Full libraryagent_api documentation](services/libraryagent_api.md)

### 58. Css_api

**Resources**: 5

- Label [CRUD]
- Account [CR]
- Css_product [R]
- Css_product_input [CUD]
- Quota [R]

📖 [Full css_api documentation](services/css_api.md)

### 59. Androidpublisher_api

**Resources**: 36

- Inapppurchase [R]
- Product [R]
- Voidedpurchase [R]
- Grant [CUD]
- Track [CRU]
- Application [C]
- Variant [CR]
- Internalappsharingartifact [C]
- Product [CR]
- Bundle [CR]
- Expansionfile [CRU]
- Subscriptionsv2 [CR]
- Inappproduct [CRUD]
- Detail [RU]
- User [CRUD]
- Listing [RUD]
- Externaltransaction [CR]
- Monetization [C]
- Base_plan [CD]
- Voidedpurchase [R]
- Device_tier_config [CR]
- Productsv2 [R]
- Countryavailability [R]
- Subscription [CRUD]
- Order [CR]
- Image [CRD]
- Review [CR]
- Tester [RU]
- Offer [CRUD]
- Generatedapk [R]
- Apprecovery [CR]
- Purchase_option [C]
- Deobfuscationfile [C]
- Onetimeproduct [CRUD]
- Apk [CR]
- Edit [CRD]

📖 [Full androidpublisher_api documentation](services/androidpublisher_api.md)

### 60. Datacatalog_api

**Resources**: 20

- Entrie [CRUD]
- Policy_tag [CRUD]
- Tag_template [CRUD]
- Enum_value [C]
- Taxonomie [CRUD]
- Catalog [C]
- Field [CUD]
- Entry_group [CRUD]
- Tag [CRUD]
- Operation [CRD]
- Location [CR]
- Entrie [CRUD]
- Taxonomie [CRUD]
- Tag [CRUD]
- Tag_template [CRUD]
- Policy_tag [CRUD]
- Field [CUD]
- Catalog [C]
- Enum_value [C]
- Entry_group [CRUD]

📖 [Full datacatalog_api documentation](services/datacatalog_api.md)

### 61. Cloudlocationfinder_api

**Resources**: 4

- Cloud_location [R]
- Location [R]
- Location [R]
- Cloud_location [R]

📖 [Full cloudlocationfinder_api documentation](services/cloudlocationfinder_api.md)

### 62. Storagetransfer_api

**Resources**: 4

- Agent_pool [CRUD]
- Google_service_account [R]
- Transfer_job [CRUD]
- Transfer_operation [CR]

📖 [Full storagetransfer_api documentation](services/storagetransfer_api.md)

### 63. Resourcesettings_api

**Resources**: 1

- Setting [RU]

📖 [Full resourcesettings_api documentation](services/resourcesettings_api.md)

### 64. Firebasedatabase_api

**Resources**: 1

- Instance [CRD]

📖 [Full firebasedatabase_api documentation](services/firebasedatabase_api.md)

### 65. Analyticshub_api

**Resources**: 6

- Subscription [CRD]
- Query_template [CRUD]
- Data_exchange [CRUD]
- Listing [CRUD]
- Data_exchange [CRUD]
- Listing [CRUD]

📖 [Full analyticshub_api documentation](services/analyticshub_api.md)

### 66. Http_body

**Resources**: 1

- Fhir [CR]

📖 [Full http_body documentation](services/http_body.md)

### 67. Managedkafka_api

**Resources**: 18

- Consumer_group [RUD]
- Subject [CRD]
- Mode [RUD]
- Cluster [CRUD]
- Connector [CRUD]
- Config [RUD]
- Type [R]
- Schema_registrie [CRD]
- Compatibility [C]
- Version [CRD]
- Schema [R]
- Topic [CRUD]
- Context [R]
- Operation [CRD]
- Location [R]
- Connect_cluster [CRUD]
- Acl [CRUD]
- Referencedby [R]

📖 [Full managedkafka_api documentation](services/managedkafka_api.md)

### 68. Poly_api

**Resources**: 2

- Asset [R]
- Likedasset [R]

📖 [Full poly_api documentation](services/poly_api.md)

### 69. Orgpolicy_api

**Resources**: 3

- Constraint [R]
- Policie [CRUD]
- Custom_constraint [CRUD]

📖 [Full orgpolicy_api documentation](services/orgpolicy_api.md)

### 70. Certificatemanager_api

**Resources**: 8

- Trust_config [CRUD]
- Dns_authorization [CRUD]
- Certificate_issuance_config [CRUD]
- Certificate [CRUD]
- Certificate_map [CRUD]
- Certificate_map_entrie [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full certificatemanager_api documentation](services/certificatemanager_api.md)

### 71. Apigateway_api

**Resources**: 20

- Location [R]
- Gateway [CRUD]
- Operation [CRD]
- Api [CRUD]
- Config [CRUD]
- Location [R]
- Gateway [CR]
- Config [CR]
- Operation [CRD]
- Api [CR]
- Config [CRUD]
- Gateway [CRUD]
- Location [R]
- Api [CRUD]
- Operation [CRD]
- Gateway [CR]
- Api [CR]
- Location [R]
- Config [CR]
- Operation [CRD]

📖 [Full apigateway_api documentation](services/apigateway_api.md)

### 72. Datastore_api

**Resources**: 5

- Project [C]
- Project [C]
- Operation [CRD]
- Indexe [CRD]
- Project [C]

📖 [Full datastore_api documentation](services/datastore_api.md)

### 73. Cloudtasks_api

**Resources**: 9

- Queue [CRUD]
- Task [CRD]
- Location [RU]
- Task [CRD]
- Queue [CRUD]
- Location [RU]
- Task [CRD]
- Location [RU]
- Queue [CRUD]

📖 [Full cloudtasks_api documentation](services/cloudtasks_api.md)

### 74. Firebaseappdistribution_api

**Resources**: 16

- Media [C]
- Group [CRUD]
- App [R]
- Tester [CRU]
- Release [CRU]
- Feedback_report [RD]
- Operation [CRD]
- Release [C]
- Tester [R]
- Test_case [CRUD]
- Note [C]
- Release_by_hash [R]
- Project [R]
- Test [CR]
- Upload_statu [R]
- App [RU]

📖 [Full firebaseappdistribution_api documentation](services/firebaseappdistribution_api.md)

### 75. Domains_api

**Resources**: 9

- Location [R]
- Operation [R]
- Registration [CRUD]
- Location [R]
- Registration [CRUD]
- Operation [R]
- Location [R]
- Registration [CRUD]
- Operation [R]

📖 [Full domains_api documentation](services/domains_api.md)

### 76. Policysimulator_api

**Resources**: 12

- Org_policy_violation [R]
- Operation [R]
- Replay [CR]
- Org_policy_violations_preview [CR]
- Result [R]
- Operation [R]
- Replay [CR]
- Result [R]
- Operation [R]
- Org_policy_violation [R]
- Org_policy_violations_preview [CR]
- Operation [R]

📖 [Full policysimulator_api documentation](services/policysimulator_api.md)

### 77. Apihub_api

**Resources**: 19

- Instance [CRUD]
- Operation [CRUD]
- External_api [CRUD]
- Location [CR]
- Discovered_api_operation [R]
- Spec [CRUD]
- Version [CRUD]
- Host_project_registration [CR]
- Api [CRUD]
- Discovered_api_observation [R]
- Dependencie [CRUD]
- Plugin [CRUD]
- Api_hub_instance [CRD]
- Attribute [CRUD]
- Style_guide [R]
- Definition [R]
- Runtime_project_attachment [CRD]
- Curation [CRUD]
- Deployment [CRUD]

📖 [Full apihub_api documentation](services/apihub_api.md)

### 78. Mybusinessaccountmanagement_api

**Resources**: 4

- Location [C]
- Account [CRU]
- Invitation [CR]
- Admin [CRUD]

📖 [Full mybusinessaccountmanagement_api documentation](services/mybusinessaccountmanagement_api.md)

### 79. Smartdevicemanagement_api

**Resources**: 3

- Structure [R]
- Room [R]
- Device [CR]

📖 [Full smartdevicemanagement_api documentation](services/smartdevicemanagement_api.md)

### 80. Tracing_api

**Resources**: 2

- Span [C]
- Trace [CR]

📖 [Full tracing_api documentation](services/tracing_api.md)

### 81. Gamesmanagement_api

**Resources**: 5

- Application [R]
- Player [CD]
- Score [C]
- Event [C]
- Achievement [C]

📖 [Full gamesmanagement_api documentation](services/gamesmanagement_api.md)

### 82. Firestore_api

**Resources**: 15

- User_cred [CRD]
- Database [CRUD]
- Backup_schedule [CRUD]
- Field [RU]
- Location [R]
- Backup [RD]
- Operation [CRD]
- Document [CRUD]
- Indexe [CRD]
- Database [C]
- Indexe [CRD]
- Field [RU]
- Indexe [CRD]
- Database [C]
- Document [CRUD]

📖 [Full firestore_api documentation](services/firestore_api.md)

### 83. Language_api

**Resources**: 4

- Document [C]
- Document [C]
- Document [C]
- Document [C]

📖 [Full language_api documentation](services/language_api.md)

### 84. Json_body

**Resources**: 6

- Model [CRUD]
- Version [CRUD]
- Job [CRU]
- Project [CR]
- Operation [CRD]
- Location [R]

📖 [Full json_body documentation](services/json_body.md)

### 85. Replicapoolupdater_api

**Resources**: 2

- Zone_operation [R]
- Rolling_update [CR]

📖 [Full replicapoolupdater_api documentation](services/replicapoolupdater_api.md)

### 86. Androidmanagement_api

**Resources**: 11

- Signup_url [C]
- Device [CRUD]
- Provisioning_info [R]
- Application [R]
- Operation [CR]
- Web_app [CRUD]
- Enrollment_token [CRD]
- Web_token [C]
- Migration_token [CR]
- Enterprise [CRUD]
- Policie [CRUD]

📖 [Full androidmanagement_api documentation](services/androidmanagement_api.md)

### 87. Licensing_api

**Resources**: 1

- License_assignment [CRUD]

📖 [Full licensing_api documentation](services/licensing_api.md)

### 88. Sasportal_api

**Resources**: 6

- Customer [CRU]
- Device [CRUD]
- Installer [C]
- Policie [CR]
- Node [CRUD]
- Deployment [CRUD]

📖 [Full sasportal_api documentation](services/sasportal_api.md)

### 89. Vmwareengine_api

**Resources**: 20

- Hcx_activation_key [CR]
- External_access_rule [CRUD]
- Upgrade [RU]
- Vmware_engine_network [CRUD]
- Management_dns_zone_binding [CRUD]
- Announcement [R]
- External_addresse [CRUD]
- Subnet [RU]
- Network_policie [CRUD]
- Network_peering [CRUD]
- Dns_bind_permission [C]
- Operation [RD]
- Private_connection [CRUD]
- Cluster [CRUD]
- Node_type [R]
- Location [R]
- Peering_route [R]
- Private_cloud [CRUD]
- Logging_server [CRUD]
- Node [R]

📖 [Full vmwareengine_api documentation](services/vmwareengine_api.md)

### 90. Pollen_api

**Resources**: 2

- Forecast [R]
- Heatmap_tile [R]

📖 [Full pollen_api documentation](services/pollen_api.md)

### 91. Cloudsupport_api

**Resources**: 10

- Case [CRU]
- Case_classification [R]
- Attachment [R]
- Comment [CR]
- Media [CR]
- Case [CRU]
- Media [CR]
- Comment [CR]
- Attachment [R]
- Case_classification [R]

📖 [Full cloudsupport_api documentation](services/cloudsupport_api.md)

### 92. Servicemanagement_api

**Resources**: 5

- Rollout [CR]
- Consumer [C]
- Service [CRD]
- Config [CR]
- Operation [R]

📖 [Full servicemanagement_api documentation](services/servicemanagement_api.md)

### 93. Cloudiot_api

**Resources**: 5

- Registrie [CRUD]
- State [R]
- Group [C]
- Device [CRUD]
- Config_version [R]

📖 [Full cloudiot_api documentation](services/cloudiot_api.md)

### 94. Androiddeviceprovisioning_api

**Resources**: 6

- Dpc [R]
- Device [CR]
- Customer [CR]
- Operation [R]
- Vendor [R]
- Configuration [CRUD]

📖 [Full androiddeviceprovisioning_api documentation](services/androiddeviceprovisioning_api.md)

### 95. Gamesconfiguration_api

**Resources**: 2

- Leaderboard_configuration [CRUD]
- Achievement_configuration [CRUD]

📖 [Full gamesconfiguration_api documentation](services/gamesconfiguration_api.md)

### 96. Areainsights_api

**Resources**: 1

- Areainsight [C]

📖 [Full areainsights_api documentation](services/areainsights_api.md)

### 97. Meet_api

**Resources**: 7

- Space [CRU]
- Recording [R]
- Conference_record [R]
- Entrie [R]
- Participant_session [R]
- Transcript [R]
- Participant [R]

📖 [Full meet_api documentation](services/meet_api.md)

### 98. Adsense_api

**Resources**: 34

- Payment [R]
- Customchannel [CRUD]
- Adunit [CRU]
- Account [R]
- Alert [R]
- Adclient [R]
- Saved [R]
- Urlchannel [R]
- Policy_issue [R]
- Site [R]
- Report [R]
- Account [R]
- Adunit [R]
- Saved [R]
- Adclient [R]
- Metric [R]
- Savedadstyle [R]
- Alert [RD]
- Report [R]
- Urlchannel [R]
- Payment [R]
- Dimension [R]
- Customchannel [R]
- Savedadstyle [R]
- Urlchannel [R]
- Adclient [R]
- Report [R]
- Metric [R]
- Alert [R]
- Dimension [R]
- Saved [R]
- Customchannel [R]
- Adunit [R]
- Account [R]

📖 [Full adsense_api documentation](services/adsense_api.md)

### 99. Toolresults_api

**Resources**: 11

- Perf_sample_serie [CR]
- Execution [CRU]
- Sample [CR]
- Perf_metrics_summary [C]
- Project [CR]
- Cluster [R]
- Environment [R]
- Historie [CR]
- Thumbnail [R]
- Test_case [R]
- Step [CRU]

📖 [Full toolresults_api documentation](services/toolresults_api.md)

### 100. Artifactregistry_api

**Resources**: 37

- Yum_artifact [C]
- Package [RUD]
- Version [CRUD]
- Googet_artifact [C]
- Generic_artifact [C]
- Kfp_artifact [C]
- Rule [CRUD]
- Docker_image [R]
- Npm_package [R]
- Operation [R]
- Attachment [CRD]
- Project [RU]
- Repositorie [CRUD]
- Location [RU]
- Python_package [R]
- Maven_artifact [R]
- Apt_artifact [C]
- Tag [CRUD]
- Go_module [C]
- File [CRUD]
- File [R]
- Operation [R]
- Location [R]
- Apt_artifact [C]
- Package [RUD]
- Project [RU]
- Version [RD]
- Tag [CRUD]
- Yum_artifact [C]
- Repositorie [CRUD]
- File [R]
- Operation [R]
- Repositorie [CRUD]
- Version [RD]
- Tag [CRUD]
- Package [RD]
- Location [R]

📖 [Full artifactregistry_api documentation](services/artifactregistry_api.md)

### 101. Cloudcommerceprocurement_api

**Resources**: 2

- Entitlement [CRU]
- Account [CR]

📖 [Full cloudcommerceprocurement_api documentation](services/cloudcommerceprocurement_api.md)

### 102. Alertcenter_api

**Resources**: 3

- Feedback [CR]
- Alert [CRD]
- Alertcenter [RU]

📖 [Full alertcenter_api documentation](services/alertcenter_api.md)

### 103. Mybusinessbusinessinformation_api

**Resources**: 5

- Google_location [C]
- Categorie [R]
- Chain [R]
- Attribute [R]
- Location [CRUD]

📖 [Full mybusinessbusinessinformation_api documentation](services/mybusinessbusinessinformation_api.md)

### 104. Bigquerydatapolicy_api

**Resources**: 2

- Data_policie [CRUD]
- Data_policie [CRUD]

📖 [Full bigquerydatapolicy_api documentation](services/bigquerydatapolicy_api.md)

### 105. Androidenterprise_api

**Resources**: 17

- Serviceaccountkey [CRD]
- Managedconfigurationssetting [R]
- Permission [R]
- Storelayoutpage [CRUD]
- Webapp [CRUD]
- Storelayoutcluster [CRUD]
- Managedconfigurationsforuser [RUD]
- Enrollment_token [C]
- Managedconfigurationsfordevice [RUD]
- Install [RUD]
- Enterprise [CRU]
- User [CRUD]
- Product [CR]
- Entitlement [RUD]
- Grouplicenseuser [R]
- Grouplicense [R]
- Device [CRU]

📖 [Full androidenterprise_api documentation](services/androidenterprise_api.md)

### 106. Playmoviespartner_api

**Resources**: 4

- Country [R]
- Order [R]
- Avail [R]
- Store_info [R]

📖 [Full playmoviespartner_api documentation](services/playmoviespartner_api.md)

### 107. Oracledatabase_api

**Resources**: 24

- Odb_network [CRD]
- Db_server [R]
- Cloud_exadata_infrastructure [CRD]
- Database [R]
- Cloud_vm_cluster [CRD]
- Autonomous_database_backup [R]
- Odb_subnet [CRD]
- Db_system_initial_storage_size [R]
- Db_node [R]
- Db_system [CRD]
- Autonomous_database_character_set [R]
- Db_system_shape [R]
- Operation [CRD]
- Db_version [R]
- Database_character_set [R]
- Autonomous_database [CRUD]
- Pluggable_database [R]
- Autonomous_db_version [R]
- Exascale_db_storage_vault [CRD]
- Minor_version [R]
- Gi_version [R]
- Location [R]
- Entitlement [R]
- Exadb_vm_cluster [CRUD]

📖 [Full oracledatabase_api documentation](services/oracledatabase_api.md)

### 108. Composer_api

**Resources**: 12

- Environment [CRUD]
- User_workloads_secret [CRUD]
- User_workloads_config_map [CRUD]
- Workload [R]
- Operation [RD]
- Image_version [R]
- Environment [CRUD]
- User_workloads_config_map [CRUD]
- User_workloads_secret [CRUD]
- Operation [RD]
- Image_version [R]
- Workload [R]

📖 [Full composer_api documentation](services/composer_api.md)

### 109. Dataform_api

**Resources**: 20

- Release_config [CRUD]
- Operation [CRD]
- Folder [CR]
- Workflow_invocation [CRD]
- Team_folder [CR]
- Workflow_config [CRUD]
- Compilation_result [CR]
- Location [RU]
- Repositorie [CRUD]
- Workspace [CRD]
- Operation [CRD]
- Location [RU]
- Workspace [CRD]
- Workflow_config [CRUD]
- Compilation_result [CR]
- Folder [CR]
- Repositorie [CRUD]
- Release_config [CRUD]
- Team_folder [CR]
- Workflow_invocation [CRD]

📖 [Full dataform_api documentation](services/dataform_api.md)

### 110. Adsensehost_api

**Resources**: 7

- Customchannel [CRUD]
- Urlchannel [CRD]
- Account [R]
- Adclient [R]
- Adunit [CRUD]
- Associationsession [R]
- Report [R]

📖 [Full adsensehost_api documentation](services/adsensehost_api.md)

### 111. Deploymentmanager_api

**Resources**: 19

- Operation [R]
- Deployment [CRUD]
- Resource [R]
- Manifest [R]
- Composite_type [CRUD]
- Type [R]
- Type_provider [CRUD]
- Type_provider [CRUD]
- Operation [R]
- Deployment [CRUD]
- Type [R]
- Manifest [R]
- Composite_type [CRUD]
- Resource [R]
- Type [R]
- Deployment [CRUD]
- Operation [R]
- Resource [R]
- Manifest [R]

📖 [Full deploymentmanager_api documentation](services/deploymentmanager_api.md)

### 112. Displayvideo_api

**Resources**: 144

- Advertiser [CRUD]
- Floodlight_group [RU]
- Operation [R]
- Line_item [CRUD]
- Inventory_source_group [CRUD]
- Campaign [CRUD]
- Site [CRD]
- Creative [CRUD]
- Channel [CRU]
- Media [CR]
- Location_list [CRU]
- Manual_trigger [CRU]
- Google_audience [R]
- User [CRUD]
- Custom_list [R]
- Negative_keyword [CRD]
- Guaranteed_order [CRU]
- Asset [C]
- Inventory_source [CRU]
- Assigned_targeting_option [CRD]
- Custom_bidding_algorithm [CRU]
- Script [CR]
- Assigned_inventory_source [CRD]
- Partner [CR]
- First_and_third_party_audience [CRU]
- Assigned_location [CRD]
- Insertion_order [CRUD]
- Combined_audience [R]
- Targeting_option [CR]
- Negative_keyword_list [CRUD]
- Invoice [R]
- Sdfdownloadtask [C]
- Media [R]
- Operation [R]
- Guaranteed_order [CRU]
- Custom_bidding_algorithm [CRU]
- Negative_keyword_list [CRUD]
- Custom_list [R]
- Inventory_source [CRU]
- Negative_keyword [CRD]
- Floodlight_activitie [R]
- Assigned_location [CRD]
- Youtube_ad_group_ad [R]
- Combined_audience [R]
- Partner [CR]
- Media [CR]
- Advertiser [CRUD]
- Insertion_order [CRUD]
- Channel [CRU]
- Operation [R]
- Campaign [CRUD]
- Targeting_option [CR]
- Site [CRD]
- Manual_trigger [CRU]
- User [CRUD]
- Creative [CRUD]
- Assigned_inventory_source [CRD]
- Line_item [CRUD]
- Sdfdownloadtask [C]
- Location_list [CRU]
- Google_audience [R]
- Invoice [R]
- Asset [C]
- Youtube_ad_group [R]
- Assigned_targeting_option [CRD]
- Floodlight_group [RU]
- Inventory_source_group [CRUD]
- Script [CR]
- Media [R]
- Operation [R]
- Assigned_targeting_option [CRD]
- Campaign [CRUD]
- Inventory_source_group [CRUD]
- Floodlight_activitie [R]
- Asset [C]
- Operation [R]
- Assigned_inventory_source [CRD]
- Ad_group [R]
- Line_item [CRUD]
- Guaranteed_order [CRU]
- Creative [CRUD]
- First_party_and_partner_audience [CRU]
- Ad_asset [CR]
- Targeting_option [CR]
- Insertion_order [CRUD]
- Negative_keyword_list [CRUD]
- Inventory_source [CRU]
- Youtube_asset_association [CRD]
- User [CRUD]
- Rule [CR]
- Custom_list [R]
- Assigned_location [CRD]
- Google_audience [R]
- Floodlight_group [RU]
- Partner [CR]
- Custom_bidding_algorithm [CRU]
- Media [CR]
- Advertiser [CRUD]
- Sdfdownloadtask [C]
- Location_list [CRU]
- Ad_group_ad [R]
- Site [CRD]
- Negative_keyword [CRD]
- Combined_audience [R]
- Invoice [R]
- Channel [CRU]
- Script [CR]
- Custom_bidding_algorithm [CRU]
- Floodlight_activitie [R]
- User [CRUD]
- Combined_audience [R]
- Assigned_inventory_source [CRD]
- Assigned_targeting_option [CRD]
- Channel [CRU]
- Assigned_location [CRD]
- Asset [C]
- Site [CRD]
- Creative [CRUD]
- Negative_keyword_list [CRUD]
- Rule [CR]
- Media [CR]
- Floodlight_group [RU]
- Google_audience [R]
- Sdfdownloadtask [C]
- Operation [R]
- First_and_third_party_audience [CRU]
- Targeting_option [CR]
- Custom_list [R]
- Inventory_source_group [CRUD]
- Guaranteed_order [CRU]
- Ad_group [R]
- Invoice [R]
- Ad_group_ad [R]
- Script [CR]
- Advertiser [CRUD]
- Insertion_order [CRUD]
- Campaign [CRUD]
- Location_list [CRU]
- Partner [CR]
- Inventory_source [CRU]
- Line_item [CRUD]
- Negative_keyword [CRD]
- Operation [R]
- Media [R]

📖 [Full displayvideo_api documentation](services/displayvideo_api.md)

### 113. Chromewebstore_api

**Resources**: 3

- Item [CRU]
- Media [C]
- Item [CR]

📖 [Full chromewebstore_api documentation](services/chromewebstore_api.md)

### 114. Cloudkms_api

**Resources**: 12

- Import_job [CR]
- Ekm_connection [CRU]
- Crypto_key_version [CRU]
- Key_ring [CR]
- Crypto_key [CRU]
- Organization [RU]
- Project [RU]
- Key_handle [CR]
- Operation [R]
- Ekm_config [CR]
- Folder [RU]
- Location [CRU]

📖 [Full cloudkms_api documentation](services/cloudkms_api.md)

### 115. Bigtableadmin_api

**Resources**: 12

- Instance [CRUD]
- Schema_bundle [CRUD]
- Cluster [CRUD]
- Backup [CRUD]
- Hot_tablet [R]
- Logical_view [CRUD]
- Table [CRUD]
- Operation [R]
- Authorized_view [CRUD]
- App_profile [CRUD]
- Materialized_view [CRUD]
- Location [R]

📖 [Full bigtableadmin_api documentation](services/bigtableadmin_api.md)

### 116. Drivelabels_api

**Resources**: 12

- Limit [R]
- Label [CRUD]
- Lock [R]
- Permission [CRD]
- Revision [U]
- User [R]
- User [R]
- Permission [CRD]
- Limit [R]
- Lock [R]
- Revision [U]
- Label [CRUD]

📖 [Full drivelabels_api documentation](services/drivelabels_api.md)

### 117. Vision_api

**Resources**: 10

- Image [C]
- Operation [CRD]
- Product_set [CRUD]
- Product [CRUD]
- File [C]
- Reference_image [CRD]
- Image [C]
- File [C]
- Image [C]
- File [C]

📖 [Full vision_api documentation](services/vision_api.md)

### 118. Streetviewpublish_api

**Resources**: 2

- Photo [CRUD]
- Photo_sequence [CRD]

📖 [Full streetviewpublish_api documentation](services/streetviewpublish_api.md)

### 119. Clouddeploy_api

**Resources**: 11

- Custom_target_type [CRUD]
- Job_run [CR]
- Rollout [CR]
- Deploy_policie [CRUD]
- Automation_run [CR]
- Target [CRUD]
- Delivery_pipeline [CRUD]
- Operation [CRD]
- Release [CR]
- Automation [CRUD]
- Location [R]

📖 [Full clouddeploy_api documentation](services/clouddeploy_api.md)

### 120. Biglake_api

**Resources**: 3

- Database [CRUD]
- Catalog [CRD]
- Table [CRUD]

📖 [Full biglake_api documentation](services/biglake_api.md)

### 121. Eventarc_api

**Resources**: 14

- Location [RU]
- Enrollment [CRUD]
- Google_api_source [CRUD]
- Channel [CRUD]
- Message_buse [CRUD]
- Pipeline [CRUD]
- Channel_connection [CRD]
- Provider [R]
- Operation [CRD]
- Trigger [CRUD]
- Kafka_source [CR]
- Operation [CRD]
- Trigger [CRUD]
- Location [R]

📖 [Full eventarc_api documentation](services/eventarc_api.md)

### 122. Apigeeregistry_api

**Resources**: 10

- Runtime [CR]
- Location [R]
- Deployment [CRUD]
- Artifact [CRUD]
- Document [CR]
- Version [CRUD]
- Api [CRUD]
- Instance [CRD]
- Spec [CRUD]
- Operation [CRD]

📖 [Full apigeeregistry_api documentation](services/apigeeregistry_api.md)

### 123. Cloudbuild_api

**Resources**: 22

- Cloudbuild [C]
- Worker_pool [CRUD]
- Bitbucket_server_config [CRUD]
- Github_enterprise_config [CRUD]
- Connected_repositorie [C]
- Github_dot_com_webhook [C]
- Trigger [CRUD]
- Repo [R]
- Build [CR]
- Location [CR]
- Operation [CR]
- Git_lab_config [CRUD]
- Operation [CR]
- Worker_pool [CRUD]
- Repositorie [CRD]
- Connection [CRUD]
- Location [R]
- Operation [CR]
- Operation [CR]
- Worker_pool [CRUD]
- Operation [CR]
- Worker_pool [CRUD]

📖 [Full cloudbuild_api documentation](services/cloudbuild_api.md)

### 124. Secretmanager_api

**Resources**: 9

- Location [R]
- Version [CR]
- Secret [CRUD]
- Location [R]
- Secret [CRUD]
- Version [CR]
- Secret [CRUD]
- Version [CR]
- Location [R]

📖 [Full secretmanager_api documentation](services/secretmanager_api.md)

### 125. Sheets_api

**Resources**: 4

- Spreadsheet [CR]
- Sheet [C]
- Developer_metadata [CR]
- Value [CRU]

📖 [Full sheets_api documentation](services/sheets_api.md)

### 126. Recommendationengine_api

**Resources**: 6

- Operation [R]
- Catalog_item [CRUD]
- Placement [C]
- Catalog [RU]
- Prediction_api_key_registration [CRD]
- User_event [CR]

📖 [Full recommendationengine_api documentation](services/recommendationengine_api.md)

### 127. Appengine_api

**Resources**: 39

- Instance [CRD]
- Location [R]
- Version [CRUD]
- Operation [R]
- App [CRU]
- Module [RUD]
- Domain_mapping [CRUD]
- Instance [CRD]
- App [CRU]
- Ingress_rule [CRUD]
- Service [RUD]
- Authorized_domain [R]
- Authorized_certificate [CRUD]
- Application [U]
- Version [CRUD]
- Operation [R]
- Location [R]
- Location [R]
- Version [CRUD]
- App [CRU]
- Service [RUD]
- Instance [CRD]
- Operation [R]
- Location [R]
- Domain_mapping [CRUD]
- Operation [R]
- Authorized_certificate [CRUD]
- Authorized_domain [R]
- Authorized_domain [R]
- Domain_mapping [CRUD]
- Authorized_certificate [CRUD]
- Location [R]
- Operation [R]
- Instance [CRD]
- Application [U]
- App [CRU]
- Ingress_rule [CRUD]
- Version [CRUD]
- Service [RUD]

📖 [Full appengine_api documentation](services/appengine_api.md)

### 128. Dataportability_api

**Resources**: 8

- Access_type [C]
- Portability_archive [C]
- Archive_job [CR]
- Authorization [C]
- Portability_archive [C]
- Authorization [C]
- Access_type [C]
- Archive_job [CR]

📖 [Full dataportability_api documentation](services/dataportability_api.md)

### 129. Remotebuildexecution_api

**Resources**: 10

- Operation [CRD]
- Media [CR]
- Workerpool [CRUD]
- Instance [CRUD]
- Operation [R]
- Blob [CR]
- Action [C]
- Remotebuildexecution [R]
- Action_result [RU]
- Operation [C]

📖 [Full remotebuildexecution_api documentation](services/remotebuildexecution_api.md)

### 130. Containeranalysis_api

**Resources**: 10

- Note [CRUD]
- Resource [C]
- Occurrence [CRUD]
- Resource [C]
- Note [CRUD]
- Occurrence [CRUD]
- Operation [CU]
- Occurrence [CRUD]
- Scan_config [RU]
- Note [CRUD]

📖 [Full containeranalysis_api documentation](services/containeranalysis_api.md)

### 131. Workloadmanager_api

**Resources**: 9

- Execution [CRD]
- Discoveredprofile [R]
- Scanned_resource [R]
- Evaluation [CRUD]
- Rule [R]
- Operation [CRD]
- Insight [CD]
- Result [R]
- Location [R]

📖 [Full workloadmanager_api documentation](services/workloadmanager_api.md)

### 132. Cloudcontrolspartner_api

**Resources**: 10

- Access_approval_request [R]
- Location [R]
- Customer [CRUD]
- Violation [R]
- Workload [R]
- Location [R]
- Violation [R]
- Workload [R]
- Customer [CRUD]
- Access_approval_request [R]

📖 [Full cloudcontrolspartner_api documentation](services/cloudcontrolspartner_api.md)

### 133. Monitoring_api

**Resources**: 22

- Metrics_scope [R]
- V1 [C]
- Metadata [R]
- Operation [R]
- Label [R]
- Dashboard [CRUD]
- Project [CD]
- Service [CRUD]
- Metric_descriptor [CRD]
- Snooze [CRU]
- Member [R]
- Collectd_time_serie [C]
- Alert [R]
- Alert_policie [CRUD]
- Uptime_check_config [CRUD]
- Uptime_check_ip [R]
- Notification_channel_descriptor [R]
- Service_level_objective [CRUD]
- Notification_channel [CRUD]
- Group [CRUD]
- Time_serie [CR]
- Monitored_resource_descriptor [R]

📖 [Full monitoring_api documentation](services/monitoring_api.md)

### 134. Privateca_api

**Resources**: 12

- Location [R]
- Ca_pool [CRUD]
- Certificate_authoritie [CRUD]
- Certificate_template [CRUD]
- Operation [CRD]
- Certificate_revocation_list [CRU]
- Certificate [CRU]
- Certificate_authoritie [CR]
- Certificate_revocation_list [CR]
- Location [R]
- Reusable_config [CR]
- Operation [CRD]

📖 [Full privateca_api documentation](services/privateca_api.md)

### 135. Container_api

**Resources**: 14

- Zone [R]
- Node_pool [CRUD]
- Well_known [R]
- Location [R]
- Operation [CR]
- Cluster [CRUD]
- Usable_subnetwork [R]
- Usable_subnetwork [R]
- Node_pool [CRUD]
- Operation [CR]
- Zone [R]
- Cluster [CRUD]
- Location [R]
- Well_known [R]

📖 [Full container_api documentation](services/container_api.md)

### 136. Merchantapi_api

**Resources**: 82

- File_upload [R]
- Data_source [CRUD]
- Regional_inventorie [CRD]
- Local_inventorie [CRD]
- Quota [R]
- Product [R]
- Product_input [CUD]
- Order_tracking_signal [C]
- File_upload [R]
- Data_source [CRUD]
- Lfp_merchant_state [R]
- Lfp_inventorie [C]
- Lfp_store [CRD]
- Lfp_sale [C]
- Conversion_source [CRUD]
- Aggregate_product_statuse [R]
- Issueresolution [C]
- Order_tracking_signal [C]
- Promotion [CR]
- Report [C]
- Aggregate_product_statuse [R]
- Issueresolution [C]
- Promotion [CR]
- Terms_of_service_agreement_state [R]
- Email_preference [RU]
- Shipping_setting [CR]
- Developer_registration [CR]
- Account [CRUD]
- Relationship [RU]
- Gbp_account [CR]
- Business_info [RU]
- Homepage [CRU]
- Online_return_policie [CRD]
- Terms_of_service [CR]
- Omnichannel_setting [CRU]
- Automatic_improvement [RU]
- Program [CR]
- Service [CR]
- Business_identity [RU]
- User [CRUD]
- Autofeed_setting [RU]
- Issue [R]
- Checkout_setting [CRUD]
- Lfp_provider [CR]
- Region [CRUD]
- Product [R]
- Product_input [CUD]
- Notificationsubscription [CRUD]
- Account [CRUD]
- Terms_of_service [CR]
- Omnichannel_setting [CRU]
- Business_identity [RU]
- Autofeed_setting [RU]
- Checkout_setting [CRUD]
- Automatic_improvement [RU]
- Service [CR]
- Program [CR]
- Developer_registration [CR]
- Region [CRUD]
- Email_preference [RU]
- Gbp_account [CR]
- Business_info [RU]
- User [CRUD]
- Homepage [CRU]
- Online_return_policie [CRUD]
- Terms_of_service_agreement_state [R]
- Shipping_setting [CR]
- Relationship [RU]
- Lfp_provider [CR]
- Issue [R]
- Merchant_review [CRD]
- Product_review [CRD]
- Quota [R]
- Conversion_source [CRUD]
- Local_inventorie [CRD]
- Regional_inventorie [CRD]
- Notificationsubscription [CRUD]
- Lfp_merchant_state [R]
- Lfp_sale [C]
- Lfp_store [CRD]
- Lfp_inventorie [C]
- Report [C]

📖 [Full merchantapi_api documentation](services/merchantapi_api.md)

### 137. Datapipelines_api

**Resources**: 2

- Pipeline [CRUD]
- Job [R]

📖 [Full datapipelines_api documentation](services/datapipelines_api.md)

### 138. Sts_api

**Resources**: 2

- St [C]
- St [C]

📖 [Full sts_api documentation](services/sts_api.md)

### 139. Observability_api

**Resources**: 4

- Scope [RU]
- Trace_scope [CRUD]
- Location [R]
- Operation [CRD]

📖 [Full observability_api documentation](services/observability_api.md)

### 140. Dataflow_api

**Resources**: 10

- Project [CD]
- Work_item [C]
- Location [C]
- Stage [R]
- Template [CR]
- Snapshot [RD]
- Message [R]
- Debug [C]
- Flex_template [C]
- Job [CRU]

📖 [Full dataflow_api documentation](services/dataflow_api.md)

### 141. Cloudresourcemanager_api

**Resources**: 24

- Folder [C]
- Lien [CRD]
- Operation [R]
- Organization [CR]
- Project [CRUD]
- Operation [R]
- Folder [CRUD]
- Operation [R]
- Folder [CRUD]
- Project [CRUD]
- Organization [CRU]
- Effective_tag [R]
- Tag_binding [CRD]
- Tag_hold [CRD]
- Tag_value [CRUD]
- Project [CRUD]
- Operation [R]
- Organization [CR]
- Tag_binding_collection [RU]
- Lien [CRD]
- Effective_tag_binding_collection [R]
- Capabilitie [RU]
- Folder [CRUD]
- Tag_key [CRUD]

📖 [Full cloudresourcemanager_api documentation](services/cloudresourcemanager_api.md)

### 142. Firebaseapphosting_api

**Resources**: 14

- Build [CRD]
- Location [R]
- Domain [CRUD]
- Operation [CRD]
- Traffic [RU]
- Rollout [CR]
- Backend [CRUD]
- Location [R]
- Traffic [RU]
- Domain [CRUD]
- Rollout [CR]
- Build [CRD]
- Operation [CRD]
- Backend [CRUD]

📖 [Full firebaseapphosting_api documentation](services/firebaseapphosting_api.md)

### 143. Verifiedaccess_api

**Resources**: 2

- Challenge [C]
- Challenge [C]

📖 [Full verifiedaccess_api documentation](services/verifiedaccess_api.md)

### 144. Accessapproval_api

**Resources**: 4

- Approval_request [CR]
- Project [RUD]
- Folder [RUD]
- Organization [RUD]

📖 [Full accessapproval_api documentation](services/accessapproval_api.md)

### 145. Firebasedataconnect_api

**Resources**: 10

- Location [R]
- Schema [CRUD]
- Operation [CRD]
- Connector [CRUD]
- Service [CRUD]
- Schema [CRUD]
- Operation [CRD]
- Service [CRUD]
- Connector [CRUD]
- Location [R]

📖 [Full firebasedataconnect_api documentation](services/firebasedataconnect_api.md)

### 146. Assuredworkloads_api

**Resources**: 8

- Workload [CRUD]
- Violation [CR]
- Update [CR]
- Operation [R]
- Workload [CRUD]
- Violation [CR]
- Operation [R]
- Update [CR]

📖 [Full assuredworkloads_api documentation](services/assuredworkloads_api.md)

### 147. Datafusion_api

**Resources**: 11

- Instance [CRUD]
- Location [R]
- Dns_peering [CRD]
- Operation [CRD]
- Version [R]
- Instance [CRUD]
- Namespace [CR]
- Operation [CRD]
- Location [CR]
- Dns_peering [CRD]
- Version [R]

📖 [Full datafusion_api documentation](services/datafusion_api.md)

### 148. Doubleclicksearch_api

**Resources**: 3

- Saved_column [R]
- Conversion [CRU]
- Report [CR]

📖 [Full doubleclicksearch_api documentation](services/doubleclicksearch_api.md)

### 149. Ideahub_api

**Resources**: 10

- Idea [R]
- Locale [R]
- Topic_state [U]
- Idea_state [U]
- Idea_activitie [C]
- Idea_state [U]
- Idea [R]
- Locale [R]
- Topic_state [U]
- Idea_activitie [C]

📖 [Full ideahub_api documentation](services/ideahub_api.md)

### 150. Alloydb_api

**Resources**: 21

- Cluster [CRUD]
- Operation [CRD]
- Supported_database_flag [R]
- Location [R]
- Backup [CRUD]
- User [CRUD]
- Instance [CRUD]
- Operation [CRD]
- Location [R]
- Cluster [CRUD]
- Backup [CRUD]
- Instance [CRUD]
- Supported_database_flag [R]
- User [CRUD]
- Location [R]
- Cluster [CRUD]
- Instance [CRUD]
- User [CRUD]
- Supported_database_flag [R]
- Backup [CRUD]
- Operation [CRD]

📖 [Full alloydb_api documentation](services/alloydb_api.md)

### 151. Doubleclickbidmanager_api

**Resources**: 4

- Report [R]
- Querie [CRD]
- Report [R]
- Querie [CRD]

📖 [Full doubleclickbidmanager_api documentation](services/doubleclickbidmanager_api.md)

### 152. Bigqueryreservation_api

**Resources**: 14

- Location [RU]
- Reservation_group [CRD]
- Capacity_commitment [CRUD]
- Reservation [CRUD]
- Assignment [CRUD]
- Operation [CR]
- Reservation_grant [CRD]
- Reservation [CRUD]
- Location [R]
- Slot_pool [RD]
- Capacity_commitment [CRUD]
- Reservation [CRUD]
- Location [RU]
- Assignment [CRUD]

📖 [Full bigqueryreservation_api documentation](services/bigqueryreservation_api.md)

### 153. Notebooks_api

**Resources**: 10

- Runtime [CRUD]
- Location [R]
- Instance [CRUD]
- Operation [CRD]
- Execution [CRD]
- Environment [CRD]
- Schedule [CRD]
- Location [R]
- Instance [CRUD]
- Operation [CRD]

📖 [Full notebooks_api documentation](services/notebooks_api.md)

### 154. Readerrevenuesubscriptionlinking_api

**Resources**: 1

- Reader [RUD]

📖 [Full readerrevenuesubscriptionlinking_api documentation](services/readerrevenuesubscriptionlinking_api.md)

### 155. Authorizedbuyersmarketplace_api

**Resources**: 16

- Deal [CRU]
- Auction_package [CR]
- Publisher_profile [R]
- Client [CRU]
- Proposal [CRU]
- User [CRD]
- Finalized_deal [CR]
- User [CRD]
- Proposal [CRU]
- Finalized_deal [CR]
- Data_segment [CRU]
- Client [CRU]
- Deal [CRU]
- Auction_package [CR]
- Publisher_profile [R]
- Data_segment [CRU]

📖 [Full authorizedbuyersmarketplace_api documentation](services/authorizedbuyersmarketplace_api.md)

### 156. Adexchangebuyer2_api

**Resources**: 19

- User [RU]
- Filtered_bid [R]
- Filtered_bid_request [R]
- Proposal [CRU]
- Creative [CRU]
- Invitation [CR]
- Bid_metric [R]
- Client [CRU]
- Finalized_proposal [CR]
- Product [R]
- Non_billable_winning_bid [R]
- Bid_responses_without_bid [R]
- Deal_association [CR]
- Filter_set [CRD]
- Bid_response_error [R]
- Detail [R]
- Losing_bid [R]
- Impression_metric [R]
- Publisher_profile [R]

📖 [Full adexchangebuyer2_api documentation](services/adexchangebuyer2_api.md)

### 157. Adsenseplatform_api

**Resources**: 8

- Site [CRD]
- Account [CR]
- Event [C]
- Group [RU]
- Account [CR]
- Site [CRUD]
- Platform [R]
- Event [C]

📖 [Full adsenseplatform_api documentation](services/adsenseplatform_api.md)

### 158. Mybusinessplaceactions_api

**Resources**: 2

- Place_action_link [CRUD]
- Place_action_type_metadata [R]

📖 [Full mybusinessplaceactions_api documentation](services/mybusinessplaceactions_api.md)

### 159. Firebasestorage_api

**Resources**: 3

- Default_bucket [C]
- Project [RD]
- Bucket [CR]

📖 [Full firebasestorage_api documentation](services/firebasestorage_api.md)

### 160. Calendar_api

**Resources**: 8

- Setting [CR]
- Acl [CRUD]
- Calendar_list [CRUD]
- Calendar [CRUD]
- Freebusy [C]
- Channel [C]
- Color [R]
- Event [CRUD]

📖 [Full calendar_api documentation](services/calendar_api.md)

### 161. Networkmanagement_api

**Resources**: 8

- Vpc_flow_logs_config [CRUD]
- Connectivity_test [CRUD]
- Operation [CRD]
- Location [R]
- Vpc_flow_logs_config [CRUD]
- Connectivity_test [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full networkmanagement_api documentation](services/networkmanagement_api.md)

### 162. Admin_api

**Resources**: 34

- Privilege [R]
- Schema [CRUD]
- Group [CRUD]
- Role_assignment [CRD]
- Aliase [CRD]
- Domain [CRD]
- Orgunit [CRUD]
- Channel [C]
- Mobiledevice [CRD]
- Photo [RUD]
- Token [RD]
- Role [CRUD]
- Building [CRUD]
- Chromeosdevice [CRU]
- Asp [RD]
- Domain_aliase [CRD]
- Feature [CRUD]
- Print_server [CRUD]
- Command [R]
- Calendar [CRUD]
- Two_step_verification [C]
- Verification_code [CR]
- Customer [RU]
- Printer [CRUD]
- User [CRUD]
- Member [CRUD]
- Chromeo [C]
- Application [R]
- Transfer [CR]
- Activitie [CR]
- User_usage_report [R]
- Entity_usage_report [R]
- Channel [C]
- Customer_usage_report [R]

📖 [Full admin_api documentation](services/admin_api.md)

### 163. Acceleratedmobilepageurl_api

**Resources**: 1

- Amp_url [C]

📖 [Full acceleratedmobilepageurl_api documentation](services/acceleratedmobilepageurl_api.md)

### 164. Fusiontables_api

**Resources**: 12

- Template [CRUD]
- Query [CR]
- Column [CRUD]
- Task [RD]
- Table [CRUD]
- Style [CRUD]
- Column [CRUD]
- Query [CR]
- Template [CRUD]
- Table [CRUD]
- Style [CRUD]
- Task [RD]

📖 [Full fusiontables_api documentation](services/fusiontables_api.md)

### 165. Chromepolicy_api

**Resources**: 6

- Network [C]
- Policie [C]
- Media [C]
- Policy_schema [R]
- Group [C]
- Orgunit [C]

📖 [Full chromepolicy_api documentation](services/chromepolicy_api.md)

### 166. Securitycenter_api

**Resources**: 38

- Custom_module [CRUD]
- External_system [U]
- Big_query_export [CRUD]
- Finding [CRU]
- Organization [RU]
- Notification_config [CRUD]
- Simulation [R]
- Attack_path [R]
- Event_threat_detection_setting [C]
- Resource_value_config [CRUD]
- Effective_custom_module [R]
- Asset [CRU]
- Source [CRU]
- Operation [CRD]
- Valued_resource [R]
- Mute_config [CRUD]
- Operation [CRD]
- Cluster [RU]
- Project [RU]
- Event_threat_detection_setting [R]
- Security_health_analytics_setting [R]
- Organization [RU]
- Folder [RU]
- Container_threat_detection_setting [R]
- Virtual_machine_threat_detection_setting [R]
- Rapid_vulnerability_detection_setting [R]
- Web_security_scanner_setting [R]
- Organization [RU]
- Source [CRU]
- Asset [CRU]
- Finding [CRU]
- Operation [CRD]
- Source [CRU]
- Finding [CRU]
- Asset [CRU]
- Organization [RU]
- Operation [CRD]
- Notification_config [CRUD]

📖 [Full securitycenter_api documentation](services/securitycenter_api.md)

### 167. Analyticsreporting_api

**Resources**: 2

- User_activity [C]
- Report [C]

📖 [Full analyticsreporting_api documentation](services/analyticsreporting_api.md)

### 168. Iap_api

**Resources**: 5

- Identity_aware_proxy_client [CRD]
- Brand [CR]
- Iap [CRU]
- Dest_group [CRUD]
- Iap [C]

📖 [Full iap_api documentation](services/iap_api.md)

### 169. Consumersurveys_api

**Resources**: 3

- Result [R]
- Survey [CRUD]
- Mobileapppanel [RU]

📖 [Full consumersurveys_api documentation](services/consumersurveys_api.md)

### 170. Siteverification_api

**Resources**: 1

- Web_resource [CRUD]

📖 [Full siteverification_api documentation](services/siteverification_api.md)

### 171. Versionhistory_api

**Resources**: 4

- Channel [R]
- Platform [R]
- Release [R]
- Version [R]

📖 [Full versionhistory_api documentation](services/versionhistory_api.md)

### 172. Storagebatchoperations_api

**Resources**: 3

- Operation [CRD]
- Location [R]
- Job [CRD]

📖 [Full storagebatchoperations_api documentation](services/storagebatchoperations_api.md)

### 173. Acmedns_api

**Resources**: 1

- Acme_challenge_set [CR]

📖 [Full acmedns_api documentation](services/acmedns_api.md)

### 174. Storage_api

**Resources**: 23

- Bucket_access_control [CRUD]
- Default_object_access_control [CRUD]
- Operation [CR]
- Folder [CRD]
- Object_access_control [CRUD]
- Managed_folder [CRUD]
- Channel [C]
- Service_account [R]
- Notification [CRD]
- Object [CRUD]
- Hmac_key [CRUD]
- Bucket [CRUD]
- Anywhere_cache [CRU]
- Bucket_access_control [CRUD]
- Bucket [CRUD]
- Default_object_access_control [CRUD]
- Channel [C]
- Object_access_control [CRUD]
- Object [CRUD]
- Bucket_access_control [CRUD]
- Object [CRUD]
- Bucket [CRUD]
- Object_access_control [CRUD]

📖 [Full storage_api documentation](services/storage_api.md)

### 175. Networkservices_api

**Resources**: 40

- Gateway [CRUD]
- Endpoint_policie [CRUD]
- Service_lb_policie [CRUD]
- Lb_traffic_extension [CRUD]
- Tls_route [CRUD]
- Wasm_plugin [CRUD]
- Version [CRD]
- Edge_cache_service [CR]
- Lb_edge_extension [CRUD]
- Authz_extension [CRUD]
- Edge_cache_origin [CR]
- Location [R]
- Edge_cache_keyset [CR]
- Meshe [CRUD]
- Operation [CRD]
- Grpc_route [CRUD]
- Http_route [CRUD]
- Lb_route_extension [CRUD]
- Service_binding [CRUD]
- Tcp_route [CRUD]
- Route_view [R]
- Gateway [CRUD]
- Lb_traffic_extension [CRUD]
- Endpoint_policie [CRUD]
- Location [R]
- Service_binding [CRUD]
- Lb_route_extension [CRUD]
- Tls_route [CRUD]
- Lb_tcp_extension [CRUD]
- Tcp_route [CRUD]
- Http_route [CRUD]
- Meshe [CRUD]
- Authz_extension [CRUD]
- Route_view [R]
- Service_lb_policie [CRUD]
- Lb_edge_extension [CRUD]
- Operation [CRD]
- Wasm_plugin [CRUD]
- Grpc_route [CRUD]
- Version [CRD]

📖 [Full networkservices_api documentation](services/networkservices_api.md)

### 176. Dfareporting_api

**Resources**: 506

- Targetable_remarketing_list [R]
- Placement_group [CRU]
- Floodlight_activitie [CRUD]
- Compatible_field [C]
- Placement_strategie [CRUD]
- Creative [CRU]
- Postal_code [R]
- User_role_permission [R]
- Creative_asset [C]
- Floodlight_configuration [RU]
- Countrie [R]
- Size [CR]
- Order_document [R]
- Advertiser [CRU]
- Mobile_carrier [R]
- Dimension_value [C]
- Dynamic_targeting_key [CRD]
- Account_active_ad_summarie [R]
- Operating_system_version [R]
- Placement [CRU]
- Account_permission_group [R]
- File [R]
- Connection_type [R]
- Subaccount [CRU]
- Campaign_creative_association [CR]
- Landing_page [CRUD]
- Change_log [R]
- Content_categorie [CRUD]
- Account_permission [R]
- Metro [R]
- Browser [R]
- Advertiser_group [CRUD]
- Language [R]
- Platform_type [R]
- Order [R]
- Report [CRUD]
- Region [R]
- Operating_system [R]
- User_profile [R]
- Ad [CRU]
- Video_format [R]
- Citie [R]
- Creative_group [CRU]
- Directory_site_contact [R]
- Conversion [C]
- Account_user_profile [CRU]
- Event_tag [CRUD]
- Inventory_item [R]
- Project [R]
- User_role_permission_group [R]
- Floodlight_activity_group [CRU]
- Site [CRU]
- Targeting_template [CRU]
- Creative_field_value [CRUD]
- Directory_site [CR]
- Account [RU]
- User_role [CRUD]
- Creative_field [CRUD]
- Remarketing_list [CRU]
- Remarketing_list_share [RU]
- Campaign [CRU]
- Account_permission [R]
- Site [CRU]
- File [R]
- Placement_strategie [CRUD]
- Creative_asset [C]
- Dynamic_targeting_key [CRD]
- Citie [R]
- Account_active_ad_summarie [R]
- Content_categorie [CRUD]
- User_role_permission_group [R]
- User_role_permission [R]
- Platform_type [R]
- Targetable_remarketing_list [R]
- Campaign_creative_association [CR]
- Metro [R]
- Subaccount [CRU]
- Creative_field_value [CRUD]
- User_profile [R]
- Remarketing_list_share [RU]
- Account_permission_group [R]
- Operating_system_version [R]
- Creative_field [CRUD]
- Project [R]
- Inventory_item [R]
- Mobile_carrier [R]
- Advertiser_group [CRUD]
- Compatible_field [C]
- Creative_group [CRU]
- Creative [CRU]
- Mobile_app [R]
- Language [R]
- Campaign [CRU]
- Order_document [R]
- Event_tag [CRUD]
- Account [RU]
- Change_log [R]
- Ad [CRU]
- Report [CRUD]
- Advertiser_landing_page [CRU]
- Operating_system [R]
- Countrie [R]
- Order [R]
- Remarketing_list [CRU]
- Placement [CRU]
- User_role [CRUD]
- Directory_site [CR]
- Placement_group [CRU]
- Dimension_value [C]
- Advertiser [CRU]
- Video_format [R]
- Floodlight_activitie [CRUD]
- Browser [R]
- Conversion [C]
- Size [CR]
- Postal_code [R]
- Floodlight_configuration [RU]
- Floodlight_activity_group [CRU]
- Connection_type [R]
- Targeting_template [CRU]
- Region [R]
- Account_user_profile [CRU]
- Creative_asset [C]
- Floodlight_activitie [CRUD]
- Placement [CRU]
- User_role_permission [R]
- Browser [R]
- Custom_event [C]
- Ad [CRU]
- Citie [R]
- Size [CR]
- Order_document [R]
- Subaccount [CRU]
- User_profile [R]
- Dynamic_targeting_key [CRD]
- Compatible_field [C]
- Account_permission [R]
- Event_tag [CRUD]
- Targetable_remarketing_list [R]
- Report [CRUD]
- Change_log [R]
- Account_user_profile [CRU]
- Floodlight_configuration [RU]
- Advertiser [CRU]
- Account [RU]
- Creative_field_value [CRUD]
- Platform_type [R]
- Language [R]
- Inventory_item [R]
- User_role [CRUD]
- Mobile_carrier [R]
- Postal_code [R]
- Account_active_ad_summarie [R]
- File [R]
- Advertiser_landing_page [CRU]
- Metro [R]
- Content_categorie [CRUD]
- Dimension_value [C]
- Placement_strategie [CRUD]
- Project [R]
- Region [R]
- Advertiser_group [CRUD]
- Site [CRU]
- Creative_field [CRUD]
- Conversion [C]
- Directory_site [CR]
- Account_permission_group [R]
- Creative_group [CRU]
- Floodlight_activity_group [CRU]
- Campaign [CRU]
- Targeting_template [CRU]
- Creative [CRU]
- Order [R]
- Placement_group [CRU]
- Operating_system_version [R]
- Countrie [R]
- Campaign_creative_association [CR]
- Operating_system [R]
- User_role_permission_group [R]
- Remarketing_list [CRU]
- Remarketing_list_share [RU]
- Connection_type [R]
- Video_format [R]
- Mobile_app [R]
- Media [C]
- Subaccount [CRU]
- Account_permission_group [R]
- Content_categorie [CRUD]
- Creative_field [CRUD]
- User_role [CRUD]
- User_profile [R]
- Ad [CRU]
- Metro [R]
- Placement [CRU]
- Citie [R]
- Size [CR]
- Creative_asset [C]
- Compatible_field [C]
- Account_permission [R]
- Advertiser [CRU]
- Directory_site_contact [R]
- Project [R]
- Dimension_value [C]
- Inventory_item [R]
- Countrie [R]
- Account_active_ad_summarie [R]
- Region [R]
- Postal_code [R]
- Advertiser_group [CRUD]
- Platform_type [R]
- Operating_system [R]
- Order_document [R]
- File [R]
- User_role_permission_group [R]
- Floodlight_configuration [RU]
- Floodlight_activity_group [CRU]
- Directory_site [CR]
- Order [R]
- User_role_permission [R]
- Remarketing_list [CRU]
- Operating_system_version [R]
- Connection_type [R]
- Mobile_app [R]
- Video_format [R]
- Report [CRUD]
- Placement_strategie [CRUD]
- Account_user_profile [CRU]
- Conversion [C]
- Creative [CRU]
- Language [R]
- Change_log [R]
- Mobile_carrier [R]
- Creative_field_value [CRUD]
- Dynamic_targeting_key [CRD]
- Site [CRU]
- Campaign [CRU]
- Targetable_remarketing_list [R]
- Floodlight_activitie [CRUD]
- Campaign_creative_association [CR]
- Advertiser_landing_page [CRU]
- Event_tag [CRUD]
- Browser [R]
- Account [RU]
- Remarketing_list_share [RU]
- Placement_group [CRU]
- Targeting_template [CRU]
- Creative_group [CRU]
- Account_permission [R]
- User_profile [R]
- Countrie [R]
- Tv_campaign_detail [R]
- Creative_field [CRUD]
- Dynamic_profile [CRU]
- Floodlight_activitie [CRUD]
- Dynamic_feed [CRU]
- User_role_permission_group [R]
- Account_permission_group [R]
- Conversion [C]
- Creative_field_value [CRUD]
- Mobile_carrier [R]
- Site [CRU]
- Creative_group [CRU]
- Creative [CRU]
- Remarketing_list [CRU]
- Account_user_profile [CRU]
- Subaccount [CRU]
- Placement_strategie [CRUD]
- Directory_site [CR]
- Floodlight_activity_group [CRU]
- Video_format [R]
- User_role [CRUD]
- Placement [CRU]
- Region [R]
- Targetable_remarketing_list [R]
- Ad [CRU]
- Postal_code [R]
- Account [RU]
- Account_active_ad_summarie [R]
- Content_categorie [CRUD]
- Connection_type [R]
- Change_log [R]
- Advertiser_landing_page [CRU]
- Operating_system [R]
- Advertiser [CRU]
- Operating_system_version [R]
- Mobile_app [R]
- Metro [R]
- Dimension_value [C]
- Campaign_creative_association [CR]
- Compatible_field [C]
- Size [CR]
- Platform_type [R]
- Campaign [CRU]
- Event_tag [CRUD]
- Creative_asset [C]
- Targeting_template [CRU]
- Dynamic_targeting_key [CRD]
- Advertiser_group [CRUD]
- Billing_profile [RU]
- Browser [R]
- Advertiser_invoice [R]
- File [R]
- User_role_permission [R]
- Studio_creative [CR]
- Report [CRUD]
- Studio_creative_asset [C]
- Floodlight_configuration [RU]
- Tv_campaign_summarie [R]
- Billing_rate [R]
- Billing_assignment [CR]
- Remarketing_list_share [RU]
- Language [R]
- Citie [R]
- Placement_group [CRU]
- Dimension_value [C]
- Account_permission_group [R]
- Inventory_item [R]
- Billing_assignment [CR]
- Dynamic_feed [CRU]
- User_role_permission [R]
- Advertiser_landing_page [CRU]
- Mobile_carrier [R]
- Browser [R]
- Ad [CRU]
- Remarketing_list [CRU]
- Targeting_template [CRU]
- Creative_field_value [CRUD]
- Change_log [R]
- Metro [R]
- Floodlight_activity_group [CRU]
- Studio_creative_asset [C]
- Tv_campaign_detail [R]
- Dynamic_profile [CRU]
- Remarketing_list_share [RU]
- Operating_system_version [R]
- Mobile_app [R]
- Creative_group [CRU]
- Creative_field [CRUD]
- Account_user_profile [CRU]
- Creative_asset [C]
- Compatible_field [C]
- User_profile [R]
- Content_categorie [CRUD]
- Advertiser_group [CRUD]
- Creative [CRU]
- Countrie [R]
- Site [CRU]
- Region [R]
- Targetable_remarketing_list [R]
- Campaign [CRU]
- Operating_system [R]
- Placement [CRU]
- User_role_permission_group [R]
- Floodlight_configuration [RU]
- Billing_profile [RU]
- Order [R]
- File [R]
- Postal_code [R]
- Advertiser_invoice [R]
- Platform_type [R]
- Citie [R]
- Event_tag [CRUD]
- Account_permission [R]
- Conversion [C]
- Billing_rate [R]
- Placement_strategie [CRUD]
- Subaccount [CRU]
- Studio_creative [CR]
- Campaign_creative_association [CR]
- Account [RU]
- Directory_site [CR]
- Advertiser [CRU]
- Report [CRUD]
- Placement_group [CRU]
- Language [R]
- Project [R]
- Video_format [R]
- Account_active_ad_summarie [R]
- User_role [CRUD]
- Tv_campaign_summarie [R]
- Dynamic_targeting_key [CRD]
- Floodlight_activitie [CRUD]
- Connection_type [R]
- Size [CR]
- Project [R]
- File [R]
- Creative_field_value [CRUD]
- User_role [CRUD]
- User_role_permission [R]
- Creative_group [CRU]
- Remarketing_list [CRU]
- Creative [CRU]
- Dynamic_targeting_key [CRD]
- Video_format [R]
- Creative_field [CRUD]
- Campaign_creative_association [CR]
- Content_categorie [CRUD]
- Account_active_ad_summarie [R]
- Placement_strategie [CRUD]
- Language [R]
- User_profile [R]
- Countrie [R]
- Placement [CRU]
- Account_permission_group [R]
- Advertiser_landing_page [CRU]
- User_role_permission_group [R]
- Metro [R]
- Inventory_item [R]
- Targeting_template [CRU]
- Site [CRU]
- Subaccount [CRU]
- Size [CR]
- Account_user_profile [CRU]
- Report [CRUD]
- Remarketing_list_share [RU]
- Region [R]
- Advertiser_group [CRUD]
- Mobile_carrier [R]
- Operating_system [R]
- Postal_code [R]
- Event_tag [CRUD]
- Compatible_field [C]
- Placement_group [CRU]
- Ad [CRU]
- Advertiser [CRU]
- Change_log [R]
- Browser [R]
- Operating_system_version [R]
- Order_document [R]
- Floodlight_activitie [CRUD]
- Floodlight_activity_group [CRU]
- Directory_site_contact [R]
- Account [RU]
- Campaign [CRU]
- Citie [R]
- Targetable_remarketing_list [R]
- Creative_asset [C]
- Floodlight_configuration [RU]
- Account_permission [R]
- Directory_site [CR]
- Conversion [C]
- Dimension_value [C]
- Order [R]
- Connection_type [R]
- Platform_type [R]
- Site [CRU]
- Operating_system [R]
- Citie [R]
- User_role_permission_group [R]
- Dynamic_targeting_key [CRD]
- Directory_site [CR]
- Mobile_app [R]
- Postal_code [R]
- Language [R]
- Placement_group [CRU]
- Report [CRUD]
- User_role_permission [R]
- Operating_system_version [R]
- Ad [CRU]
- Video_format [R]
- Creative [CRU]
- Region [R]
- Project [R]
- Advertiser_group [CRUD]
- Event_tag [CRUD]
- File [R]
- Account [RU]
- Size [CR]
- Connection_type [R]
- Targetable_remarketing_list [R]
- Creative_field [CRUD]
- Order_document [R]
- Account_permission_group [R]
- Floodlight_configuration [RU]
- Content_categorie [CRUD]
- Targeting_template [CRU]
- Creative_field_value [CRUD]
- Account_permission [R]
- Remarketing_list [CRU]
- Mobile_carrier [R]
- User_profile [R]
- Advertiser_landing_page [CRU]
- Remarketing_list_share [RU]
- Placement_strategie [CRUD]
- Subaccount [CRU]
- Account_user_profile [CRU]
- Dimension_value [C]
- Creative_asset [C]
- Browser [R]
- Campaign [CRU]
- Platform_type [R]
- Order [R]
- Countrie [R]
- Floodlight_activitie [CRUD]
- User_role [CRUD]
- Floodlight_activity_group [CRU]
- Creative_group [CRU]
- Conversion [C]
- Inventory_item [R]
- Placement [CRU]
- Change_log [R]
- Campaign_creative_association [CR]
- Advertiser [CRU]
- Account_active_ad_summarie [R]
- Compatible_field [C]
- Metro [R]

📖 [Full dfareporting_api documentation](services/dfareporting_api.md)

### 177. Webfonts_api

**Resources**: 1

- Webfont [R]

📖 [Full webfonts_api documentation](services/webfonts_api.md)

### 178. Apim_api

**Resources**: 6

- Api_observation [CR]
- Operation [CRD]
- Observation_source [CRD]
- Observation_job [CRD]
- Api_operation [R]
- Location [R]

📖 [Full apim_api documentation](services/apim_api.md)

### 179. Chromeuxreport_api

**Resources**: 1

- Record [C]

📖 [Full chromeuxreport_api documentation](services/chromeuxreport_api.md)

### 180. Aiplatform_api

**Resources**: 133

- Data_item [R]
- Tuning_job [CR]
- Notebook_runtime [CRD]
- Index_endpoint [CRUD]
- Tensorboard [CRUD]
- Featurestore [CRUD]
- Feature [CRUD]
- Evaluation_set [CRUD]
- Custom_job [CRD]
- Openapi [C]
- Persistent_resource [CRUD]
- Operation [CRD]
- Endpoint [CRUD]
- Evaluation_item [CRD]
- Saved_querie [RD]
- Dataset_version [CRUD]
- Run [CRUD]
- Evaluation_run [CRD]
- Model [CRUD]
- Dataset [CRUD]
- Metadata_store [CRD]
- Slice [CR]
- Metadata_schema [CR]
- Pipeline_job [CRD]
- Cached_content [CRUD]
- Chat [C]
- Invoke [C]
- Studie [CRD]
- Indexe [CRUD]
- Training_pipeline [CRD]
- Project [RU]
- Time_serie [CRUD]
- Feature_online_store [CRUD]
- Rag_file [CRD]
- Execution [CRUD]
- Rag_corpora [CRUD]
- Entity_type [CRUD]
- Nas_trial_detail [R]
- Trial [CRD]
- Migratable_resource [C]
- Specialist_pool [CRUD]
- Reasoning_engine [CRUD]
- Annotation [R]
- Notebook_execution_job [CRD]
- Batch_prediction_job [CRD]
- Artifact [CRUD]
- Media [C]
- Schedule [CRUD]
- Feature_view [CRUD]
- Hyperparameter_tuning_job [CRD]
- Feature_view_sync [R]
- Feature_group [CRUD]
- Data_labeling_job [CRD]
- Nas_job [CRD]
- Model_deployment_monitoring_job [CRUD]
- Experiment [CRUD]
- Notebook_runtime_template [CRUD]
- Context [CRUD]
- Annotation_spec [R]
- Evaluation [CR]
- Deployment_resource_pool [CRUD]
- Location [CRU]
- Schedule [CRUD]
- Custom_job [CRD]
- Featurestore [CRUD]
- Feature_view [CRUD]
- Training_pipeline [CRD]
- Data_item [R]
- Annotation_spec [R]
- Session [CRUD]
- Feature_group [CRUD]
- Specialist_pool [CRUD]
- Experiment [CRUD]
- Sandbox_environment [CRD]
- Dataset [CRUD]
- Tuning_job [CR]
- Tensorboard [CRUD]
- Trial [CRD]
- Media [C]
- Entity_type [CRUD]
- Evaluation_run [CRD]
- Execution [CRUD]
- Studie [CRD]
- Rag_corpora [CRUD]
- Example_store [CRUD]
- Model_deployment_monitoring_job [CRUD]
- Pipeline_job [CRD]
- Model_garden_eula [C]
- Operation [CRD]
- Project [CRU]
- Chat [C]
- Feature_monitor_job [CR]
- Extension [CRUD]
- Index_endpoint [CRUD]
- Hyperparameter_tuning_job [CRD]
- Model_monitoring_job [CRD]
- Indexe [CRUD]
- Batch_prediction_job [CRD]
- Nas_trial_detail [R]
- Evaluation [CR]
- Migratable_resource [C]
- Context [CRUD]
- Rag_file [CRD]
- Feature_view_sync [R]
- Artifact [CRUD]
- Location [CRU]
- Event [R]
- Model [CRUD]
- Feature [CRUD]
- Notebook_execution_job [CRD]
- Evaluation_item [CRD]
- Model_monitor [CRUD]
- Annotation [R]
- Endpoint [CRUD]
- Slice [CR]
- Persistent_resource [CRUD]
- Evaluation_set [CRUD]
- Feature_online_store [CRUD]
- Metadata_schema [CR]
- Dataset_version [CRUD]
- Notebook_runtime [CRD]
- Memorie [CRUD]
- Metadata_store [CRD]
- Nas_job [CRD]
- Saved_querie [RD]
- Data_labeling_job [CRD]
- Deployment_resource_pool [CRUD]
- Notebook_runtime_template [CRUD]
- Cached_content [CRUD]
- Reasoning_engine [CRUD]
- Time_serie [CRUD]
- Feature_monitor [CRUD]
- Run [CRUD]

📖 [Full aiplatform_api documentation](services/aiplatform_api.md)

### 181. Script_api

**Resources**: 5

- Deployment [CRUD]
- Project [CRU]
- Script [C]
- Processe [R]
- Version [CR]

📖 [Full script_api documentation](services/script_api.md)

### 182. Contentwarehouse_api

**Resources**: 9

- Rule_set [CRUD]
- Document [CRUD]
- Document_schema [CRUD]
- Synonym_set [CRUD]
- Operation [R]
- Location [CR]
- Document_link [CD]
- Reference_id [RUD]
- Project [C]

📖 [Full contentwarehouse_api documentation](services/contentwarehouse_api.md)

### 183. Batch_api

**Resources**: 5

- Operation [CRD]
- State [C]
- Location [R]
- Job [CRD]
- Task [R]

📖 [Full batch_api documentation](services/batch_api.md)

### 184. Kmsinventory_api

**Resources**: 2

- Crypto_key [R]
- Protected_resource [R]

📖 [Full kmsinventory_api documentation](services/kmsinventory_api.md)

### 185. Run_api

**Resources**: 23

- Operation [CRD]
- Domainmapping [CRD]
- Authorizeddomain [R]
- Route [R]
- Execution [CRD]
- Job [CRUD]
- Service [CRUD]
- Configuration [R]
- Workerpool [CRUD]
- Task [R]
- Revision [RD]
- Location [R]
- Worker_pool [CRUD]
- Revision [RD]
- Operation [CRD]
- Task [R]
- Build [C]
- Location [CR]
- Job [CRUD]
- Execution [CRD]
- Service [CRUD]
- Customresourcedefinition [R]
- Job [CRD]

📖 [Full run_api documentation](services/run_api.md)

### 186. Servicenetworking_api

**Resources**: 11

- Peered_dns_domain [CRD]
- Network [RU]
- Connection [CRU]
- Service [CU]
- Dns_record_set [CRU]
- Role [C]
- Dns_zone [CR]
- Operation [CRD]
- Service [CU]
- Connection [CR]
- Operation [R]

📖 [Full servicenetworking_api documentation](services/servicenetworking_api.md)

### 187. Cloudscheduler_api

**Resources**: 6

- Job [CRUD]
- Location [RU]
- Operation [CRD]
- Operation [CRD]
- Job [CRUD]
- Location [R]

📖 [Full cloudscheduler_api documentation](services/cloudscheduler_api.md)

### 188. Cloudsearch_api

**Resources**: 12

- Stat [R]
- Setting [RU]
- Lro [R]
- Item [CRD]
- Datasource [CRUD]
- Searchapplication [CRUD]
- Query [C]
- Unmappedid [R]
- Media [C]
- Cloudsearch [C]
- Source [R]
- Operation [R]

📖 [Full cloudsearch_api documentation](services/cloudsearch_api.md)

### 189. Admob_api

**Resources**: 16

- App [R]
- Account [R]
- Network_report [C]
- Mediation_report [C]
- Ad_unit [R]
- Adapter [R]
- Network_report [C]
- Ad_unit_mapping [CR]
- App [CR]
- Ad_source [R]
- Mediation_ab_experiment [C]
- Campaign_report [C]
- Mediation_report [C]
- Account [R]
- Mediation_group [CRU]
- Ad_unit [CR]

📖 [Full admob_api documentation](services/admob_api.md)

### 190. Workspaceevents_api

**Resources**: 2

- Operation [R]
- Subscription [CRUD]

📖 [Full workspaceevents_api documentation](services/workspaceevents_api.md)

### 191. Metastore_api

**Resources**: 35

- Migration_execution [RD]
- Location [R]
- Metadata_import [CRU]
- Table [CR]
- Backup [CRD]
- Database [CR]
- Federation [CRUD]
- Operation [CRD]
- Service [CRUD]
- Federation [CRUD]
- Location [R]
- Database [CR]
- Table [CR]
- Service [CRUD]
- Backup [CRD]
- Metadata_import [CRU]
- Migration_execution [RD]
- Operation [CRD]
- Backup [CRD]
- Migration_execution [RD]
- Service [CRUD]
- Service [CRUD]
- Backup [CRD]
- Metadata_import [CRU]
- Backup [CRD]
- Federation [CRUD]
- Database [CR]
- Operation [CRD]
- Location [R]
- Service [CRUD]
- Migration_execution [RD]
- Table [CR]
- Service [CRUD]
- Backup [CRD]
- Migration_execution [RD]

📖 [Full metastore_api documentation](services/metastore_api.md)

### 192. Policytroubleshooter_api

**Resources**: 2

- Iam [C]
- Iam [C]

📖 [Full policytroubleshooter_api documentation](services/policytroubleshooter_api.md)

### 193. Analyticsdata_api

**Resources**: 4

- Propertie [CR]
- Analyticsdata [C]
- Propertie [CR]
- Audience_export [CR]

📖 [Full analyticsdata_api documentation](services/analyticsdata_api.md)

### 194. Contactcenteraiplatform_api

**Resources**: 3

- Contact_center [CRUD]
- Location [R]
- Operation [CRD]

📖 [Full contactcenteraiplatform_api documentation](services/contactcenteraiplatform_api.md)

### 195. Forms_api

**Resources**: 3

- Form [CR]
- Response [R]
- Watche [CRD]

📖 [Full forms_api documentation](services/forms_api.md)

### 196. Clouderrorreporting_api

**Resources**: 5

- Location [D]
- Group_stat [R]
- Project [D]
- Event [CR]
- Group [RU]

📖 [Full clouderrorreporting_api documentation](services/clouderrorreporting_api.md)

### 197. Pagespeedonline_api

**Resources**: 4

- Pagespeedapi [R]
- Pagespeedapi [R]
- Pagespeedapi [R]
- Pagespeedapi [R]

📖 [Full pagespeedonline_api documentation](services/pagespeedonline_api.md)

### 198. Parallelstore_api

**Resources**: 6

- Instance [CRUD]
- Location [R]
- Operation [CRD]
- Instance [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full parallelstore_api documentation](services/parallelstore_api.md)

### 199. Cloudbilling_api

**Resources**: 9

- Project [RU]
- Sub_account [CR]
- Service [R]
- Sku [R]
- Billing_account [CRU]
- Sku_group [R]
- Price [R]
- Sku [R]
- Service [R]

📖 [Full cloudbilling_api documentation](services/cloudbilling_api.md)

### 200. Cloudidentity_api

**Resources**: 22

- Userinvitation [CR]
- Device_user [CRD]
- Idp_credential [CRD]
- Policie [R]
- Inbound_saml_sso_profile [CRUD]
- Client_state [RU]
- Group [CRUD]
- Device [CRD]
- Membership [CRD]
- Inbound_oidc_sso_profile [CRUD]
- Inbound_sso_assignment [CRUD]
- Membership [CRD]
- Inbound_sso_assignment [CRUD]
- Userinvitation [CR]
- Device [CRD]
- Group [CRUD]
- Policie [CRUD]
- Inbound_saml_sso_profile [CRUD]
- Client_state [RU]
- Inbound_oidc_sso_profile [CRUD]
- Idp_credential [CRD]
- Device_user [CRD]

📖 [Full cloudidentity_api documentation](services/cloudidentity_api.md)

### 201. Appsactivity_api

**Resources**: 1

- Activitie [R]

📖 [Full appsactivity_api documentation](services/appsactivity_api.md)

### 202. Groupssettings_api

**Resources**: 1

- Group [RU]

📖 [Full groupssettings_api documentation](services/groupssettings_api.md)

### 203. Adexchangebuyer_api

**Resources**: 21

- Marketplaceprivateauction [C]
- Product [R]
- Proposal [CRU]
- Performance_report [R]
- Billing_info [R]
- Account [RU]
- Pubprofile [R]
- Marketplacenote [CR]
- Creative [CR]
- Pretargeting_config [CRUD]
- Budget [RU]
- Marketplacedeal [CRUD]
- Billing_info [R]
- Budget [RU]
- Direct_deal [R]
- Creative [CR]
- Account [RU]
- Performance_report [R]
- Pretargeting_config [CRUD]
- Account [RU]
- Creative [CR]

📖 [Full adexchangebuyer_api documentation](services/adexchangebuyer_api.md)

### 204. Youtube_api

**Resources**: 33

- Video_trainability [R]
- Live_chat_moderator [CRD]
- Super_chat_event [R]
- Video_abuse_report_reason [R]
- Channel_section [CRUD]
- Live_chat_ban [CD]
- I18n_region [R]
- Playlist [CRUD]
- Abuse_report [C]
- Channel [RU]
- Watermark [C]
- Live_broadcast [CRUD]
- Message [R]
- V3 [U]
- Thumbnail [C]
- Video_categorie [R]
- Live_chat_message [CRD]
- Activitie [R]
- Video [CRUD]
- I18n_language [R]
- Member [R]
- Third_party_link [CRUD]
- Comment [CRUD]
- Caption [CRUD]
- Memberships_level [R]
- Playlist_image [CRUD]
- Search [R]
- Test [C]
- Channel_banner [C]
- Comment_thread [CR]
- Playlist_item [CRUD]
- Live_stream [CRUD]
- Subscription [CRD]

📖 [Full youtube_api documentation](services/youtube_api.md)

### 205. Looker_api

**Resources**: 4

- Backup [CRD]
- Instance [CRUD]
- Location [R]
- Operation [CRD]

📖 [Full looker_api documentation](services/looker_api.md)

### 206. Gkebackup_api

**Resources**: 12

- Backup_channel [CRUD]
- Volume_restore [CR]
- Restore_plan_binding [R]
- Operation [CRD]
- Restore [CRUD]
- Backup [CRUD]
- Restore_channel [CRUD]
- Location [R]
- Restore_plan [CRUD]
- Backup_plan [CRUD]
- Volume_backup [CR]
- Backup_plan_binding [R]

📖 [Full gkebackup_api documentation](services/gkebackup_api.md)

### 207. Cloudasset_api

**Resources**: 16

- Effective_iam_policie [R]
- Saved_querie [CRUD]
- Asset [R]
- Feed [CRUD]
- Cloudasset [CR]
- Operation [R]
- Asset [R]
- Cloudasset [CR]
- Cloudasset [C]
- Operation [R]
- Organization [CR]
- Folder [C]
- Project [CR]
- Operation [R]
- Iam_policie [R]
- Resource [R]

📖 [Full cloudasset_api documentation](services/cloudasset_api.md)

### 208. People_api

**Resources**: 5

- Contact_group [CRUD]
- People [CRUD]
- Connection [R]
- Other_contact [CR]
- Member [C]

📖 [Full people_api documentation](services/people_api.md)

### 209. Mybusinessqanda_api

**Resources**: 2

- Answer [CRD]
- Question [CRUD]

📖 [Full mybusinessqanda_api documentation](services/mybusinessqanda_api.md)

### 210. Beyondcorp_api

**Resources**: 20

- Security_gateway [CRUD]
- App_connector [CRUD]
- App_gateway [CRD]
- Application [CRUD]
- Location [R]
- Operation [CRD]
- App_connection [CRUD]
- Connector [CRUD]
- Insight [R]
- Partner_tenant [RD]
- Subscription [CRU]
- App_gateway [CRD]
- App_connector [CRUD]
- Connection [CRUD]
- Location [R]
- Application_domain [CR]
- Security_gateway [CRUD]
- Application [CRUD]
- Operation [CRD]
- App_connection [CRUD]

📖 [Full beyondcorp_api documentation](services/beyondcorp_api.md)

### 211. Pubsub_api

**Resources**: 8

- Schema [CRD]
- Snapshot [CRUD]
- Topic [CRUD]
- Subscription [CRUD]
- Subscription [CRD]
- Topic [CRD]
- Subscription [CRD]
- Topic [CRD]

📖 [Full pubsub_api documentation](services/pubsub_api.md)

### 212. Servicebroker_api

**Resources**: 13

- Servicebroker [CR]
- Instance [R]
- Catalog [R]
- Service_binding [CRD]
- Service_instance [CRUD]
- Broker [CRD]
- Binding [R]
- Servicebroker [CR]
- Servicebroker [CR]
- Service_binding [CRD]
- Service_instance [CRUD]
- Catalog [R]
- Instance [R]

📖 [Full servicebroker_api documentation](services/servicebroker_api.md)

### 213. Contactcenterinsights_api

**Resources**: 23

- Operation [CR]
- View [CRUD]
- Conversation [CRUD]
- Dataset [CRUD]
- Phrase_matcher [CRUD]
- Qa_question_tag [CRUD]
- Feedback_label [CRUD]
- Note [CRUD]
- Authorized_view_set [CRUD]
- Authorized_view [CRUD]
- Qa_question [CRUD]
- Qa_scorecard [CRUD]
- Assessment [CRD]
- Issue [CRUD]
- Encryption_spec [C]
- Analysis_rule [CRUD]
- Segment [C]
- Assessment_rule [CRUD]
- Revision [CRD]
- Location [CRU]
- Insightsdata [C]
- Issue_model [CRUD]
- Analyse [CRD]

📖 [Full contactcenterinsights_api documentation](services/contactcenterinsights_api.md)

### 214. Accesscontextmanager_api

**Resources**: 11

- Service_perimeter [CRUD]
- Gcp_user_access_binding [CRUD]
- Service [R]
- Operation [CRD]
- Access_policie [CRUD]
- Access_level [CRUD]
- Authorized_orgs_desc [CRUD]
- Access_policie [CRUD]
- Access_level [CRUD]
- Operation [R]
- Service_perimeter [CRUD]

📖 [Full accesscontextmanager_api documentation](services/accesscontextmanager_api.md)

### 215. Searchads360_api

**Resources**: 4

- Customer [R]
- Search_ads360 [C]
- Search_ads360_field [CR]
- Custom_column [R]

📖 [Full searchads360_api documentation](services/searchads360_api.md)

### 216. Oslogin_api

**Resources**: 14

- User [CR]
- Ssh_public_key [CRUD]
- Location [C]
- Project [D]
- User [CR]
- Ssh_public_key [CRUD]
- Zone [C]
- Project [CD]
- Location [C]
- User [CR]
- Location [C]
- Ssh_public_key [CRUD]
- Zone [C]
- Project [CD]

📖 [Full oslogin_api documentation](services/oslogin_api.md)

### 217. Playcustomapp_api

**Resources**: 1

- Custom_app [C]

📖 [Full playcustomapp_api documentation](services/playcustomapp_api.md)

### 218. Webmasters_api

**Resources**: 3

- Searchanalytic [C]
- Sitemap [RUD]
- Site [RUD]

📖 [Full webmasters_api documentation](services/webmasters_api.md)

### 219. Fitness_api

**Resources**: 4

- Data_source [CRUD]
- Dataset [CRUD]
- Session [RUD]
- Data_point_change [R]

📖 [Full fitness_api documentation](services/fitness_api.md)

### 220. Parametermanager_api

**Resources**: 3

- Location [R]
- Parameter [CRUD]
- Version [CRUD]

📖 [Full parametermanager_api documentation](services/parametermanager_api.md)

### 221. Businessprofileperformance_api

**Resources**: 2

- Location [R]
- Monthly [R]

📖 [Full businessprofileperformance_api documentation](services/businessprofileperformance_api.md)

### 222. Videointelligence_api

**Resources**: 6

- Operation [CRD]
- Video [C]
- Video [C]
- Video [C]
- Video [C]
- Video [C]

📖 [Full videointelligence_api documentation](services/videointelligence_api.md)

### 223. Replicapool_api

**Resources**: 2

- Pool [CRD]
- Replica [CRD]

📖 [Full replicapool_api documentation](services/replicapool_api.md)

### 224. Dlp_api

**Resources**: 14

- Project_data_profile [R]
- Column_data_profile [R]
- Discovery_config [CRUD]
- Image [C]
- Stored_info_type [CRUD]
- Job_trigger [CRUD]
- Table_data_profile [RD]
- Content [C]
- Info_type [R]
- Inspect_template [CRUD]
- Dlp_job [CRD]
- File_store_data_profile [RD]
- Connection [CRUD]
- Deidentify_template [CRUD]

📖 [Full dlp_api documentation](services/dlp_api.md)

### 225. Migrationcenter_api

**Resources**: 27

- Report [CRD]
- Operation [CRD]
- Preference_set [CRUD]
- Import_data_file [CRD]
- Report_config [CRD]
- Discovery_client [CRUD]
- Source [CRUD]
- Error_frame [R]
- Asset [CRUD]
- Group [CRUD]
- Location [RU]
- Import_job [CRUD]
- Relation [R]
- Report_config [CRD]
- Source [CRUD]
- Asset [CRUD]
- Error_frame [R]
- Preference_set [CRUD]
- Operation [CRD]
- Import_job [CRUD]
- Import_data_file [CRD]
- Relation [R]
- Assets_export_job [CRD]
- Discovery_client [CRUD]
- Group [CRUD]
- Location [RU]
- Report [CRD]

📖 [Full migrationcenter_api documentation](services/migrationcenter_api.md)

### 226. Kgsearch_api

**Resources**: 1

- Entitie [R]

📖 [Full kgsearch_api documentation](services/kgsearch_api.md)

### 227. Cloudprofiler_api

**Resources**: 1

- Profile [CRU]

📖 [Full cloudprofiler_api documentation](services/cloudprofiler_api.md)

### 228. Healthcare_api

**Resources**: 60

- Consent [CRUD]
- Dicom_store [CRUD]
- Message [CRUD]
- Nlp [C]
- Fhir [CRUD]
- User_data_mapping [CRUD]
- Instance [RD]
- Fhir_store [CRUD]
- Attribute_definition [CRUD]
- Operation [CRD]
- Bulkdata [R]
- Data_mapper_workspace [CR]
- Studie [CRD]
- Location [R]
- Serie [RD]
- Consent_artifact [CRD]
- Hl7_v2_store [CRUD]
- Dataset [CRUD]
- Frame [R]
- Consent_store [CRUD]
- Operation [R]
- Hl7_v2_store [CR]
- Dataset [CR]
- Location [R]
- Dicom_store [CR]
- Annotation [CRUD]
- Operation [R]
- Hl7_v2_store [CRUD]
- Serie [RD]
- Dataset [CRUD]
- Dicom_store [CRUD]
- Fhir_store [CRUD]
- Fhir [CRUD]
- Location [R]
- Studie [CRD]
- Instance [RD]
- Annotation_store [CRUD]
- Dicom_web [CR]
- Message [CRUD]
- Frame [R]
- Attribute_definition [CRUD]
- Studie [CRUD]
- User_data_mapping [CRUD]
- Consent_artifact [CRD]
- Message [CRUD]
- Serie [RUD]
- Instance [RUD]
- Frame [R]
- Consent_store [CRUD]
- Location [R]
- Data_mapper_workspace [CR]
- Fhir_store [CRUD]
- Hl7_v2_store [CRUD]
- Dataset [CRUD]
- Consent [CRUD]
- Dicom_store [CRUD]
- Fhir [CRUD]
- Bulkdata [R]
- Nlp [C]
- Operation [CRD]

📖 [Full healthcare_api documentation](services/healthcare_api.md)

### 229. Firebaseappcheck_api

**Resources**: 26

- Play_integrity_config [RU]
- Oauth_client [C]
- Recaptcha_enterprise_config [RU]
- App [C]
- Safety_net_config [RU]
- Service [CRU]
- Resource_policie [CRUD]
- Debug_token [CRUD]
- Jwk [R]
- App_attest_config [RU]
- Device_check_config [RU]
- Recaptcha_v3_config [RU]
- Project [C]
- Recaptcha_v3_config [RU]
- Resource_policie [CRUD]
- App [C]
- Service [CRU]
- Play_integrity_config [RU]
- Recaptcha_config [RU]
- Jwk [R]
- Device_check_config [RU]
- Debug_token [CRUD]
- Safety_net_config [RU]
- App_attest_config [RU]
- Recaptcha_enterprise_config [RU]
- Oauth_client [C]

📖 [Full firebaseappcheck_api documentation](services/firebaseappcheck_api.md)

### 230. Gkeonprem_api

**Resources**: 8

- Vmware_admin_cluster [CRUD]
- Bare_metal_admin_cluster [CRUD]
- Vmware_node_pool [CRUD]
- Bare_metal_node_pool [CRUD]
- Operation [CRD]
- Location [R]
- Bare_metal_cluster [CRUD]
- Vmware_cluster [CRUD]

📖 [Full gkeonprem_api documentation](services/gkeonprem_api.md)

### 231. Repeated_any_query_error

**Resources**: 1

- Entrie [C]

📖 [Full repeated_any_query_error documentation](services/repeated_any_query_error.md)

### 232. Transcoder_api

**Resources**: 4

- Job_template [CRD]
- Job [CRD]
- Job_template [CRD]
- Job [CRD]

📖 [Full transcoder_api documentation](services/transcoder_api.md)

### 233. Firebaserules_api

**Resources**: 3

- Release [CRUD]
- Project [C]
- Ruleset [CRD]

📖 [Full firebaserules_api documentation](services/firebaserules_api.md)

### 234. Plusdomains_api

**Resources**: 6

- Circle [R]
- Media [C]
- Activitie [R]
- People [R]
- Comment [R]
- Audience [R]

📖 [Full plusdomains_api documentation](services/plusdomains_api.md)

### 235. Proximitybeacon_api

**Resources**: 6

- Beaconinfo [C]
- Beacon [CRUD]
- Diagnostic [R]
- Attachment [CRD]
- Proximitybeacon [R]
- Namespace [RU]

📖 [Full proximitybeacon_api documentation](services/proximitybeacon_api.md)

### 236. Firebase_api

**Resources**: 9

- Web_app [CRU]
- Available_location [R]
- Project [CRU]
- Sha [CRD]
- Android_app [CRU]
- Default_location [C]
- Ios_app [CRU]
- Operation [R]
- Available_project [R]

📖 [Full firebase_api documentation](services/firebase_api.md)

### 237. Resource_named_service

**Resources**: 6

- Version [CRUD]
- Instance [CRD]
- Operation [R]
- Location [R]
- App [CR]
- Service [RUD]

📖 [Full resource_named_service documentation](services/resource_named_service.md)

### 238. Config_api

**Resources**: 9

- Preview [CRD]
- Deployment [CRUD]
- Resource_change [R]
- Operation [CRD]
- Resource_drift [R]
- Location [R]
- Resource [R]
- Revision [CR]
- Terraform_version [R]

📖 [Full config_api documentation](services/config_api.md)

### 239. Games_api

**Resources**: 13

- Score [CR]
- Recall [CR]
- Achievement_definition [R]
- Leaderboard [R]
- Stat [R]
- Snapshot [R]
- Metagame [R]
- Player [R]
- Event [CR]
- Achievement [CR]
- Accesstoken [C]
- Application [CR]
- Revision [R]

📖 [Full games_api documentation](services/games_api.md)

### 240. Appstate_api

**Resources**: 1

- State [CRUD]

📖 [Full appstate_api documentation](services/appstate_api.md)

### 241. Saasservicemgmt_api

**Resources**: 10

- Unit_kind [CRUD]
- Release [CRUD]
- Tenant [CRUD]
- Unit [CRUD]
- Rollout_kind [CRUD]
- Rollout [CRUD]
- Replications_internal [CRUD]
- Unit_operation [CRUD]
- Location [R]
- Saa [CRUD]

📖 [Full saasservicemgmt_api documentation](services/saasservicemgmt_api.md)

### 242. Test_api

**Resources**: 2

- Bucket [R]
- Oauth2 [R]

📖 [Full test_api documentation](services/test_api.md)

### 243. Gkehub_api

**Resources**: 42

- Membership [CRUD]
- Binding [CRUD]
- Namespace [CRUD]
- Operation [CRD]
- Fleet [CRUD]
- Location [R]
- Feature [CRUD]
- Scope [CRUD]
- Rbacrolebinding [CRUD]
- Scope [CRUD]
- Namespace [CRUD]
- Binding [CRUD]
- Feature [CRUD]
- Fleet [CRUD]
- Rbacrolebinding [CRUD]
- Location [R]
- Membership [CRUD]
- Operation [CRD]
- Operation [CRD]
- Location [R]
- Membership [CRUD]
- Location [R]
- Operation [CR]
- Feature [CRUD]
- Location [R]
- Feature [CRUD]
- Operation [CR]
- Membership [CRUD]
- Rbacrolebinding [CRUD]
- Namespace [CRUD]
- Feature [CRUD]
- Fleet [CRUD]
- Operation [CRD]
- Location [R]
- Scope [CRUD]
- Binding [CRUD]
- Operation [CRD]
- Membership [CRUD]
- Location [R]
- Location [R]
- Operation [CR]
- Feature [CRUD]

📖 [Full gkehub_api documentation](services/gkehub_api.md)

### 244. Analytics_api

**Resources**: 32

- Segment [R]
- Profile_user_link [CRUD]
- Custom_metric [CRU]
- Experiment [CRUD]
- Web_property_ad_words_link [CRUD]
- Client_id [C]
- Custom_data_source [R]
- Webproperty_user_link [CRUD]
- Remarketing_audience [CRUD]
- Account [R]
- Account_summarie [R]
- Mcf [R]
- Column [R]
- Profile_filter_link [CRUD]
- Upload [CR]
- Ga [R]
- Filter [CRUD]
- Realtime [R]
- Goal [CRU]
- Account_user_link [CRUD]
- Webpropertie [CRU]
- Unsampled_report [CRD]
- Custom_dimension [CRU]
- Profile [CRUD]
- Provisioning [C]
- User_deletion_request [C]
- Segment [R]
- Profile [R]
- Account [R]
- Goal [R]
- Webpropertie [R]
- Data [R]

📖 [Full analytics_api documentation](services/analytics_api.md)

### 245. Discoveryengine_api

**Resources**: 112

- Sitemap [CRD]
- Collection [RUD]
- User_event [CR]
- Project [C]
- Location [CRU]
- Serving_config [CRU]
- Ranking_config [C]
- Branche [R]
- Completion_suggestion [C]
- Answer [R]
- Schema [CRUD]
- Custom_model [R]
- License_config [CRU]
- User_license [R]
- Session [CRUD]
- Conversation [CRUD]
- Operation [CR]
- Suggestion_deny_list_entrie [C]
- Grounding_config [C]
- Completion_config [C]
- Cmek_config [RUD]
- Site_search_engine [CR]
- Media [R]
- Engine [CRUD]
- Data_store [CRUD]
- Control [CRUD]
- User_store [CRUD]
- Document [CRUD]
- Target_site [CRUD]
- Identity_mapping_store [CRD]
- Assistant [CRU]
- Source [CR]
- Engine [CRUD]
- Assistant [CRU]
- Sitemap [CRD]
- Cmek_config [RUD]
- Location [CRU]
- Sample_querie [CRUD]
- Evaluation [CR]
- Grounding_config [C]
- Identity_mapping_store [CRD]
- Branche [R]
- Target_site [CRUD]
- Site_search_engine [CR]
- Completion_config [C]
- Custom_model [R]
- Suggestion_deny_list_entrie [C]
- Completion_suggestion [C]
- Schema [CRUD]
- User_store [CRUD]
- Requirement [C]
- Project [CRU]
- Canned_querie [CRUD]
- Chunk [R]
- Sample_query_set [CRUD]
- Control [CRUD]
- Data_store [CRUD]
- Notebook [CR]
- File [CR]
- Billing_account_license_config [CR]
- Conversation [CRUD]
- Data_connector [CR]
- Document [CRUD]
- Widget_config [RU]
- Session [CRUD]
- User_license [R]
- Ranking_config [C]
- Media [CR]
- Audio_overview [CD]
- Agent [CRUD]
- Connector_run [R]
- User_event [CR]
- License_config [CRU]
- Serving_config [CRU]
- Authorization [CRUD]
- Answer [R]
- Analytic [C]
- Collection [RUD]
- Operation [CR]
- Site_search_engine [CR]
- User_store [CRUD]
- User_event [CR]
- Cmek_config [RUD]
- Schema [CRUD]
- License_config [CRU]
- Target_site [CRUD]
- Engine [CRUD]
- Operation [CR]
- Custom_model [R]
- Sample_query_set [CRUD]
- Project [C]
- Document [CRUD]
- Branche [R]
- Control [CRUD]
- Evaluation [CR]
- Completion_config [C]
- Grounding_config [C]
- User_license [R]
- Ranking_config [C]
- Conversation [CRUD]
- Completion_suggestion [C]
- Identity_mapping_store [CRD]
- Sample_querie [CRUD]
- Sitemap [CRD]
- Suggestion_deny_list_entrie [C]
- Data_store [CRUD]
- Session [CRUD]
- Media [R]
- Answer [R]
- Location [CRU]
- Assistant [CRU]
- Serving_config [CRU]

📖 [Full discoveryengine_api documentation](services/discoveryengine_api.md)

### 246. Playintegrity_api

**Resources**: 2

- Device_recall [C]
- Playintegrity [C]

📖 [Full playintegrity_api documentation](services/playintegrity_api.md)

### 247. Homegraph_api

**Resources**: 2

- Agent_user [D]
- Device [C]

📖 [Full homegraph_api documentation](services/homegraph_api.md)

### 248. Apikeys_api

**Resources**: 2

- Operation [R]
- Key [CRUD]

📖 [Full apikeys_api documentation](services/apikeys_api.md)

### 249. Serviceconsumermanagement_api

**Resources**: 8

- Tenancy_unit [CRD]
- Operation [CRD]
- Service [R]
- Operation [R]
- Limit [R]
- Producer_quota_policie [CRUD]
- Consumer_quota_metric [CR]
- Producer_override [CRUD]

📖 [Full serviceconsumermanagement_api documentation](services/serviceconsumermanagement_api.md)

### 250. Datalineage_api

**Resources**: 5

- Lineage_event [CRD]
- Run [CRUD]
- Operation [CRD]
- Location [C]
- Processe [CRUD]

📖 [Full datalineage_api documentation](services/datalineage_api.md)

### 251. Manufacturers_api

**Resources**: 2

- Product_certification [RUD]
- Product [RUD]

📖 [Full manufacturers_api documentation](services/manufacturers_api.md)

### 252. Vectortile_api

**Resources**: 2

- Terraintile [R]
- Featuretile [R]

📖 [Full vectortile_api documentation](services/vectortile_api.md)

### 253. Chat_api

**Resources**: 10

- Reaction [CRD]
- Space_event [R]
- Custom_emoji [CRD]
- Message [CRUD]
- Thread [R]
- Media [CR]
- Space_notification_setting [RU]
- Space [CRUD]
- Member [CRUD]
- Attachment [R]

📖 [Full chat_api documentation](services/chat_api.md)

### 254. Bigquery_api

**Resources**: 8

- Dataset [CRUD]
- Routine [CRUD]
- Row_access_policie [CRUD]
- Job [CRD]
- Model [RUD]
- Project [R]
- Tabledata [CR]
- Table [CRUD]

📖 [Full bigquery_api documentation](services/bigquery_api.md)

### 255. Plus_api

**Resources**: 3

- Activitie [R]
- Comment [R]
- People [R]

📖 [Full plus_api documentation](services/plus_api.md)

### 256. Bigqueryconnection_api

**Resources**: 2

- Connection [CRUD]
- Connection [CRUD]

📖 [Full bigqueryconnection_api documentation](services/bigqueryconnection_api.md)

### 257. Places_api

**Resources**: 2

- Photo [R]
- Place [CR]

📖 [Full places_api documentation](services/places_api.md)

### 258. Logging_api

**Resources**: 23

- Entrie [C]
- Metric [CRUD]
- Monitored_resource_descriptor [R]
- Sink [CRUD]
- Billing_account [R]
- Sink [CRUD]
- Saved_querie [CRUD]
- Link [CRD]
- View [CRUD]
- Logging [RU]
- Location [R]
- Project [R]
- Exclusion [CRUD]
- Organization [RU]
- Recent_querie [R]
- Monitored_resource_descriptor [R]
- Operation [CR]
- Entrie [C]
- Bucket [CRUD]
- Log_scope [CRUD]
- Folder [RU]
- Log [RD]
- Metric [CRUD]

📖 [Full logging_api documentation](services/logging_api.md)

### 259. Webrisk_api

**Resources**: 5

- Threat_list [R]
- Operation [CRD]
- Uri [R]
- Submission [C]
- Hashe [R]

📖 [Full webrisk_api documentation](services/webrisk_api.md)

### 260. Vpcaccess_api

**Resources**: 6

- Location [R]
- Operation [R]
- Connector [CRUD]
- Location [R]
- Connector [CRUD]
- Operation [R]

📖 [Full vpcaccess_api documentation](services/vpcaccess_api.md)

### 261. Urlshortener_api

**Resources**: 1

- Url [CR]

📖 [Full urlshortener_api documentation](services/urlshortener_api.md)

### 262. Sqladmin_api

**Resources**: 20

- User [CRUD]
- Tier [R]
- Database [CRUD]
- Backup_run [CRD]
- Backup [CRUD]
- Operation [CR]
- Flag [R]
- Instance [CRUD]
- Ssl_cert [CRD]
- Connect [CR]
- Operation [CR]
- Ssl_cert [CRD]
- User [CRUD]
- Backup_run [CRD]
- Instance [CRUD]
- Database [CRUD]
- Connect [CR]
- Tier [R]
- Backup [CRUD]
- Flag [R]

📖 [Full sqladmin_api documentation](services/sqladmin_api.md)

### 263. Keep_api

**Resources**: 3

- Media [R]
- Note [CRD]
- Permission [C]

📖 [Full keep_api documentation](services/keep_api.md)

### 264. Walletobjects_api

**Resources**: 20

- Loyaltyobject [CRU]
- Genericobject [CRU]
- Media [CR]
- Permission [RU]
- Flightobject [CRU]
- Offerclas [CRU]
- Eventticketobject [CRU]
- Issuer [CRU]
- Smarttap [C]
- Eventticketclas [CRU]
- Transitclas [CRU]
- Jwt [C]
- Giftcardobject [CRU]
- Transitobject [CRU]
- Flightclas [CRU]
- Genericclas [CRU]
- Offerobject [CRU]
- Giftcardclas [CRU]
- Private_content [C]
- Loyaltyclas [CRU]

📖 [Full walletobjects_api documentation](services/walletobjects_api.md)

### 265. Apphub_api

**Resources**: 16

- Operation [CRD]
- Location [CR]
- Application [CRUD]
- Discovered_workload [R]
- Discovered_service [R]
- Workload [CRUD]
- Service [CRUD]
- Service_project_attachment [CRD]
- Location [CR]
- Service [CRUD]
- Service_project_attachment [CRD]
- Discovered_service [R]
- Workload [CRUD]
- Application [CRUD]
- Discovered_workload [R]
- Operation [CRD]

📖 [Full apphub_api documentation](services/apphub_api.md)

### 266. Vmmigration_api

**Resources**: 28

- Image_import [CRD]
- Target_project [CRUD]
- Group [CRUD]
- Location [R]
- Operation [CRD]
- Image_import_job [CR]
- Source [CRUD]
- Datacenter_connector [CRD]
- Utilization_report [CRD]
- Migrating_vm [CRUD]
- Disk_migration_job [CRUD]
- Replication_cycle [R]
- Clone_job [CR]
- Cutover_job [CR]
- Datacenter_connector [CRD]
- Migrating_vm [CRUD]
- Replication_cycle [R]
- Image_import_job [CR]
- Source [CRUD]
- Utilization_report [CRD]
- Disk_migration_job [CRUD]
- Target_project [CRUD]
- Operation [CRD]
- Location [R]
- Cutover_job [CR]
- Image_import [CRD]
- Group [CRUD]
- Clone_job [CR]

📖 [Full vmmigration_api documentation](services/vmmigration_api.md)

### 267. Airquality_api

**Resources**: 4

- Forecast [C]
- Heatmap_tile [R]
- Current_condition [C]
- History [C]

📖 [Full airquality_api documentation](services/airquality_api.md)

### 268. Chromemanagement_api

**Resources**: 14

- App [R]
- Command [CR]
- Profile [RD]
- Chrome [R]
- Report [R]
- Android [R]
- Event [R]
- Device [R]
- User [R]
- Third_party_profile_user [C]
- Notification_config [CRD]
- Operation [CRD]
- Web [R]
- Certificate_provisioning_processe [CR]

📖 [Full chromemanagement_api documentation](services/chromemanagement_api.md)

### 269. Adexperiencereport_api

**Resources**: 2

- Site [R]
- Violating_site [R]

📖 [Full adexperiencereport_api documentation](services/adexperiencereport_api.md)

### 270. Recommender_api

**Resources**: 9

- Insight_type [RU]
- Insight [CR]
- Recommendation [CR]
- Recommender [RU]
- Location [R]
- Recommender [RU]
- Insight_type [RU]
- Insight [CR]
- Recommendation [CR]

📖 [Full recommender_api documentation](services/recommender_api.md)

### 271. Driveactivity_api

**Resources**: 1

- Activity [C]

📖 [Full driveactivity_api documentation](services/driveactivity_api.md)

### 272. Dns_api

**Resources**: 34

- Project [R]
- Change [CR]
- Response_policy_rule [CRUD]
- Managed_zone_operation [R]
- Policie [CRUD]
- Response_policie [CRUD]
- Resource_record_set [CRUD]
- Managed_zone [CRUD]
- Dns_key [R]
- Managed_zone_operation [R]
- Response_policie [CRUD]
- Policie [CRUD]
- Response_policy_rule [CRUD]
- Resource_record_set [CRUD]
- Dns_key [R]
- Change [CR]
- Project [R]
- Managed_zone [CRUD]
- Resource_record_set [R]
- Project [R]
- Managed_zone_operation [R]
- Managed_zone [CRUD]
- Change [CR]
- Policie [CRUD]
- Dns_key [R]
- Change [CR]
- Dns_key [R]
- Managed_zone_operation [R]
- Resource_record_set [CRUD]
- Response_policy_rule [CRUD]
- Response_policie [CRUD]
- Managed_zone [CRUD]
- Project [R]
- Policie [CRUD]

📖 [Full dns_api documentation](services/dns_api.md)

### 273. Cloudshell_api

**Resources**: 4

- Environment [CR]
- Operation [CRD]
- Environment [CRU]
- Public_key [CD]

📖 [Full cloudshell_api documentation](services/cloudshell_api.md)

### 274. Content_api

**Resources**: 52

- Returncarrier [CRUD]
- Ordertrackingsignal [C]
- Collectionstatuse [R]
- Freelistingsprogram [CR]
- Promotion [CR]
- Credential [C]
- Merchantsupport [C]
- Label [CRUD]
- Accountstatuse [CR]
- Accounttax [CRU]
- Datafeed [CRUD]
- Returnaddres [CRD]
- Csse [CR]
- Shippingsetting [CRU]
- Returnpolicyonline [CRUD]
- Pubsubnotificationsetting [RU]
- Liasetting [CRU]
- Product [CRUD]
- Productdeliverytime [CRD]
- Po [CRD]
- Recommendation [CR]
- Returnpolicy [CRD]
- Checkoutsetting [CRD]
- Productstatuse [CR]
- Region [CRUD]
- Collection [CRD]
- Datafeedstatuse [CR]
- Conversionsource [CRUD]
- Quota [R]
- Regionalinventory [C]
- Account [CRUD]
- Report [C]
- Localinventory [C]
- Shoppingadsprogram [CR]
- Orderpayment [C]
- Order [CR]
- Orderreturn [R]
- Orderinvoice [C]
- Order [CR]
- Datafeed [CRUD]
- Orderreport [R]
- Productstatuse [CR]
- Datafeedstatuse [CR]
- Accountstatuse [CR]
- Product [CRD]
- Accounttax [CRU]
- Liasetting [CRU]
- Account [CRUD]
- Orderreturn [R]
- Shippingsetting [CRU]
- Po [CRD]
- Orderinvoice [C]

📖 [Full content_api documentation](services/content_api.md)

### 275. Commentanalyzer_api

**Resources**: 1

- Comment [C]

📖 [Full commentanalyzer_api documentation](services/commentanalyzer_api.md)

### 276. Backupdr_api

**Resources**: 13

- Backup_plan [CRUD]
- Backup_plan_association [CRUD]
- Operation [CRD]
- Backup_vault [CRUD]
- Trial [C]
- Backup [CRUD]
- Management_server [CRD]
- Service_config [C]
- Data_source [CRU]
- Revision [R]
- Resource_backup_config [R]
- Location [R]
- Data_source_reference [R]

📖 [Full backupdr_api documentation](services/backupdr_api.md)

### 277. Groupsmigration_api

**Resources**: 1

- Archive [C]

📖 [Full groupsmigration_api documentation](services/groupsmigration_api.md)

### 278. Connectors_api

**Resources**: 23

- Custom_connector_version [CRD]
- Runtime_entity_schema [R]
- End_user_authentication [CRUD]
- Version [R]
- Connection [CRUD]
- Operation [CRD]
- Connector [R]
- Runtime_action_schema [R]
- Eventtype [R]
- Global [RU]
- Custom_connector [CRUD]
- Endpoint_attachment [CRUD]
- Managed_zone [CRUD]
- Provider [CR]
- Event_subscription [CRUD]
- Location [RU]
- Connection_schema_metadata [CR]
- Tool [CR]
- Connection [CR]
- Entity_type [R]
- Resource [R]
- Entitie [CRUD]
- Action [CR]

📖 [Full connectors_api documentation](services/connectors_api.md)

### 279. Solar_api

**Resources**: 3

- Building_insight [R]
- Data_layer [R]
- Geo_tiff [R]

📖 [Full solar_api documentation](services/solar_api.md)

### 280. Fcm_api

**Resources**: 1

- Message [C]

📖 [Full fcm_api documentation](services/fcm_api.md)

### 281. Networksecurity_api

**Resources**: 51

- Security_profile_group [CRUD]
- Firewall_endpoint [CRUD]
- Intercept_deployment_group [CRUD]
- Intercept_deployment [CRUD]
- Intercept_endpoint_group [CRUD]
- Operation [CRD]
- Location [R]
- Authz_policie [CRUD]
- Mirroring_endpoint_group_association [CRUD]
- Mirroring_deployment [CRUD]
- Intercept_endpoint_group_association [CRUD]
- Url_list [CRUD]
- Firewall_endpoint_association [CRUD]
- Client_tls_policie [CRUD]
- Backend_authentication_config [CRUD]
- Authorization_policie [CRUD]
- Address_group [CRUD]
- Mirroring_deployment_group [CRUD]
- Rule [CRUD]
- Security_profile [CRUD]
- Gateway_security_policie [CRUD]
- Server_tls_policie [CRUD]
- Tls_inspection_policie [CRUD]
- Mirroring_endpoint_group [CRUD]
- Operation [CRD]
- Address_group [CRUD]
- Firewall_endpoint [CRUD]
- Location [R]
- Sac_realm [CRD]
- Security_profile_group [CRUD]
- Server_tls_policie [CRUD]
- Mirroring_endpoint_group [CRUD]
- Firewall_endpoint_association [CRUD]
- Security_profile [CRUD]
- Dns_threat_detector [CRUD]
- Authorization_policie [CRUD]
- Intercept_deployment_group [CRUD]
- Gateway_security_policie [CRUD]
- Rule [CRUD]
- Mirroring_deployment_group [CRUD]
- Mirroring_endpoint_group_association [CRUD]
- Authz_policie [CRUD]
- Sac_attachment [CRD]
- Intercept_endpoint_group [CRUD]
- Intercept_endpoint_group_association [CRUD]
- Mirroring_deployment [CRUD]
- Url_list [CRUD]
- Tls_inspection_policie [CRUD]
- Intercept_deployment [CRUD]
- Client_tls_policie [CRUD]
- Backend_authentication_config [CRUD]

📖 [Full networksecurity_api documentation](services/networksecurity_api.md)

### 282. Managedidentities_api

**Resources**: 18

- Domain [CRUD]
- Operation [CRD]
- Sql_integration [R]
- Location [R]
- Peering [CRUD]
- Backup [CRUD]
- Operation [CRD]
- Peering [CRUD]
- Sql_integration [R]
- Location [R]
- Domain [CRUD]
- Backup [CRUD]
- Peering [CRUD]
- Sql_integration [R]
- Operation [CRD]
- Domain [CRUD]
- Location [R]
- Backup [CRUD]

📖 [Full managedidentities_api documentation](services/managedidentities_api.md)

### 283. Serviceusage_api

**Resources**: 8

- Service [CR]
- Operation [CRD]
- Operation [R]
- Limit [R]
- Consumer_override [CRUD]
- Admin_override [CRUD]
- Consumer_quota_metric [CR]
- Service [CR]

📖 [Full serviceusage_api documentation](services/serviceusage_api.md)

### 284. Digitalassetlinks_api

**Resources**: 2

- Statement [R]
- Assetlink [CR]

📖 [Full digitalassetlinks_api documentation](services/digitalassetlinks_api.md)

### 285. Clouddebugger_api

**Resources**: 2

- Debuggee [CR]
- Breakpoint [CRUD]

📖 [Full clouddebugger_api documentation](services/clouddebugger_api.md)

### 286. Datamanager_api

**Resources**: 3

- Event [C]
- Request_statu [R]
- Audience_member [C]

📖 [Full datamanager_api documentation](services/datamanager_api.md)

### 287. Ids_api

**Resources**: 3

- Location [R]
- Operation [CRD]
- Endpoint [CRUD]

📖 [Full ids_api documentation](services/ids_api.md)

### 288. Workflows_api

**Resources**: 6

- Operation [RD]
- Location [R]
- Workflow [CRUD]
- Workflow [CRUD]
- Location [R]
- Operation [RD]

📖 [Full workflows_api documentation](services/workflows_api.md)

### 289. Cloudtrace_api

**Resources**: 5

- Project [U]
- Trace [R]
- Trace_sink [CRUD]
- Span [C]
- Trace [C]

📖 [Full cloudtrace_api documentation](services/cloudtrace_api.md)

### 290. Advisorynotifications_api

**Resources**: 2

- Location [RU]
- Notification [R]

📖 [Full advisorynotifications_api documentation](services/advisorynotifications_api.md)

### 291. Fcmdata_api

**Resources**: 1

- Delivery_data [R]

📖 [Full fcmdata_api documentation](services/fcmdata_api.md)

### 292. Checks_api

**Resources**: 6

- Media [C]
- Report [R]
- Scan [CR]
- App [R]
- Operation [CRD]
- Aisafety [C]

📖 [Full checks_api documentation](services/checks_api.md)

### 293. Blogger_3

**Resources**: 8

- Page_view [R]
- Blog [R]
- User [R]
- Comment [CRD]
- Post [CRUD]
- Page [CRUD]
- Blog_user_info [R]
- Post_user_info [R]

📖 [Full blogger_3 documentation](services/blogger_3.md)

### 294. Mediaupload

**Resources**: 1

- Caption [C]

📖 [Full mediaupload documentation](services/mediaupload.md)

### 295. Paymentsresellersubscription_api

**Resources**: 5

- Subscription [CR]
- Line_item [U]
- Promotion [CR]
- Product [R]
- User_session [C]

📖 [Full paymentsresellersubscription_api documentation](services/paymentsresellersubscription_api.md)

### 296. Spanner_api

**Resources**: 14

- Backup_schedule [CRUD]
- Database_role [CR]
- Database [CRUD]
- Session [CRD]
- Instance_partition [CRUD]
- Instance [CRUD]
- Database_operation [R]
- Instance_config_operation [R]
- Operation [CRD]
- Backup_operation [R]
- Instance_config [CRUD]
- Scan [R]
- Instance_partition_operation [R]
- Backup [CRUD]

📖 [Full spanner_api documentation](services/spanner_api.md)

### 297. Cloudchannel_api

**Resources**: 15

- Customer_repricing_config [CRUD]
- Customer [CRUD]
- Sku_group [R]
- Integrator [CR]
- Sku [R]
- Channel_partner_repricing_config [CRUD]
- Offer [R]
- Operation [CRD]
- Report_job [C]
- Report [CR]
- Product [R]
- Entitlement [CR]
- Channel_partner_link [CRU]
- Account [CR]
- Billable_sku [R]

📖 [Full cloudchannel_api documentation](services/cloudchannel_api.md)

### 298. Gameservices_api

**Resources**: 6

- Operation [CRD]
- Game_server_deployment [CR]
- Location [R]
- Game_server_deployment [CR]
- Location [R]
- Operation [CRD]

📖 [Full gameservices_api documentation](services/gameservices_api.md)

### 299. Cloudprivatecatalog_api

**Resources**: 3

- Version [R]
- Catalog [R]
- Product [R]

📖 [Full cloudprivatecatalog_api documentation](services/cloudprivatecatalog_api.md)

### 300. Gmailpostmastertools_api

**Resources**: 4

- Domain [R]
- Traffic_stat [R]
- Domain [R]
- Traffic_stat [R]

📖 [Full gmailpostmastertools_api documentation](services/gmailpostmastertools_api.md)

### 301. Partners_api

**Resources**: 10

- Offer [R]
- History [R]
- User_event [C]
- User [RUD]
- User_state [R]
- Analytic [R]
- Lead [CR]
- Client_message [C]
- Partner [RU]
- Companie [R]

📖 [Full partners_api documentation](services/partners_api.md)

### 302. Any

**Resources**: 5

- Indexe [R]
- Log_service [R]
- Log [RD]
- Entrie [C]
- Sink [CRUD]

📖 [Full any documentation](services/any.md)

### 303. Runtimeconfig_api

**Resources**: 5

- Operation [CRD]
- Operation [CR]
- Variable [CRUD]
- Waiter [CRD]
- Config [CRUD]

📖 [Full runtimeconfig_api documentation](services/runtimeconfig_api.md)

### 304. Retail_api

**Resources**: 38

- User_event [C]
- Attributes_config [C]
- Control [CRUD]
- Serving_config [CRUD]
- Model [CRUD]
- Catalog [CRU]
- Product [CRUD]
- Placement [C]
- Project [RU]
- Operation [R]
- Completion_data [C]
- Generative_question [CR]
- Serving_config [CRUD]
- Product [CRUD]
- Completion_data [C]
- Attributes_config [C]
- Placement [C]
- Generative_question [CR]
- Model [CRUD]
- Operation [R]
- User_event [C]
- Catalog [CRU]
- Control [CRUD]
- Catalog [CRU]
- Control [CRUD]
- User_event [C]
- Model [CRUD]
- Merchant_center_account_link [CRD]
- Product [CRUD]
- Branche [R]
- Serving_config [CRUD]
- Generative_question [CR]
- Project [CRU]
- Placement [C]
- Retail_project [C]
- Attributes_config [C]
- Completion_data [C]
- Operation [R]

📖 [Full retail_api documentation](services/retail_api.md)

### 305. Books_api

**Resources**: 21

- Readingposition [CR]
- Myconfig [CR]
- Recommended [CR]
- Annotation_data [R]
- Promooffer [CR]
- Useruploaded [R]
- Volume_annotation [R]
- Volume [R]
- Personalizedstream [R]
- Associated [R]
- Dictionary [R]
- Familysharing [CR]
- Annotation [CRUD]
- Membership [R]
- Notification [R]
- Layer [R]
- Mybook [R]
- Bookshelve [CR]
- Serie [R]
- Onboarding [R]
- Cloudloading [C]

📖 [Full books_api documentation](services/books_api.md)

### 306. Websecurityscanner_api

**Resources**: 15

- Finding [R]
- Scan_run [CR]
- Scan_config [CRUD]
- Crawled_url [R]
- Finding_type_stat [R]
- Scan_run [CR]
- Finding [R]
- Scan_config [CRUD]
- Crawled_url [R]
- Finding_type_stat [R]
- Crawled_url [R]
- Finding [R]
- Finding_type_stat [R]
- Scan_config [CRUD]
- Scan_run [CR]

📖 [Full websecurityscanner_api documentation](services/websecurityscanner_api.md)

### 307. Localservices_api

**Resources**: 2

- Account_report [R]
- Detailed_lead_report [R]

📖 [Full localservices_api documentation](services/localservices_api.md)

### 308. Servicedirectory_api

**Resources**: 9

- Namespace [CRUD]
- Location [R]
- Service [CRUD]
- Endpoint [CRUD]
- Endpoint [CRUD]
- Namespace [CRUD]
- Workload [C]
- Service [CRUD]
- Location [R]

📖 [Full servicedirectory_api documentation](services/servicedirectory_api.md)

### 309. Customsearch_api

**Resources**: 2

- Cse [R]
- Siterestrict [R]

📖 [Full customsearch_api documentation](services/customsearch_api.md)

### 310. Policyanalyzer_api

**Resources**: 2

- Activitie [R]
- Activitie [R]

📖 [Full policyanalyzer_api documentation](services/policyanalyzer_api.md)

### 311. Mybusinessbusinesscalls_api

**Resources**: 2

- Businesscallsinsight [R]
- Location [RU]

📖 [Full mybusinessbusinesscalls_api documentation](services/mybusinessbusinesscalls_api.md)

### 312. Ml_api

**Resources**: 8

- Trial [CRD]
- Location [R]
- Model [CRUD]
- Studie [CRD]
- Project [CR]
- Version [CRUD]
- Job [CRU]
- Operation [CR]

📖 [Full ml_api documentation](services/ml_api.md)

### 313. Firebaseml_api

**Resources**: 4

- Operation [CRD]
- Model [CRUD]
- Operation [R]
- Model [C]

📖 [Full firebaseml_api documentation](services/firebaseml_api.md)

### 314. Blogger_api

**Resources**: 13

- Comment [R]
- Page [R]
- Post [R]
- Blog [R]
- User [R]
- Comment [CRD]
- User [R]
- Post [CRUD]
- Page [CRUD]
- Blog [R]
- Page_view [R]
- Post_user_info [R]
- Blog_user_info [R]

📖 [Full blogger_api documentation](services/blogger_api.md)

### 315. Vault_api

**Resources**: 6

- Account [CRD]
- Export [CRD]
- Operation [CRD]
- Matter [CRUD]
- Saved_querie [CRD]
- Hold [CRUD]

📖 [Full vault_api documentation](services/vault_api.md)

### 316. Baremetalsolution_api

**Resources**: 15

- Operation [CRD]
- Nfs_share [CRUD]
- Location [R]
- Provisioning_config [CRU]
- Instance [CRU]
- Ssh_key [CRD]
- Network [CRU]
- Os_image [R]
- Operation [R]
- Provisioning_quota [R]
- Volume [CRU]
- Snapshot [CRD]
- Lun [CR]
- Provisioning_quota [R]
- Location [C]

📖 [Full baremetalsolution_api documentation](services/baremetalsolution_api.md)

### 317. Testing_api

**Resources**: 4

- Test_matrice [CR]
- Device_session [CRU]
- Test_environment_catalog [R]
- Application_detail_service [C]

📖 [Full testing_api documentation](services/testing_api.md)

### 318. Recaptchaenterprise_api

**Resources**: 6

- Key [CRUD]
- Assessment [C]
- Membership [R]
- Relatedaccountgroupmembership [C]
- Relatedaccountgroup [R]
- Firewallpolicie [CRUD]

📖 [Full recaptchaenterprise_api documentation](services/recaptchaenterprise_api.md)

### 319. Servicecontrol_api

**Resources**: 2

- Service [C]
- Service [C]

📖 [Full servicecontrol_api documentation](services/servicecontrol_api.md)

### 320. Documentai_api

**Resources**: 21

- Operation [CR]
- Evaluation [R]
- Location [R]
- Processor_type [R]
- Human_review_config [C]
- Processor [CRUD]
- Schema_version [CRUD]
- Schema [CRUD]
- Processor_version [CRD]
- Dataset [CRU]
- Processor [CRD]
- Processor_type [R]
- Human_review_config [C]
- Location [R]
- Processor_version [CRD]
- Schema_version [CRUD]
- Operation [CRD]
- Evaluation [R]
- Schema [CRUD]
- Operation [R]
- Document [C]

📖 [Full documentai_api documentation](services/documentai_api.md)

### 321. Firebasedynamiclinks_api

**Resources**: 3

- Short_link [C]
- Firebasedynamiclink [CR]
- Managed_short_link [C]

📖 [Full firebasedynamiclinks_api documentation](services/firebasedynamiclinks_api.md)

### 322. Apigee_api

**Resources**: 64

- Reference [CRUD]
- Security_incident [CRU]
- Data [R]
- Security_monitoring_condition [CRUD]
- Apidoc [CRUD]
- Datastore [CRUD]
- Api [CRUD]
- Resourcefile [CRUD]
- Stat [R]
- Security_report [CR]
- Developer [CRUD]
- Subscription [CR]
- Security_assessment_result [C]
- Targetserver [CRUD]
- Optimized_host_stat [R]
- Querie [CR]
- Security_stat [C]
- Optimized_stat [R]
- Envgroup [CRUD]
- Entrie [CRUD]
- Archive_deployment [CRUD]
- Apicategorie [CRUD]
- Host_querie [CR]
- Nat_addresse [CRD]
- Attribute [CRD]
- Security_action [CRUD]
- Debugsession [CRD]
- App [CRUD]
- Issuer [R]
- Keyvaluemap [CRUD]
- Project [C]
- Admin [R]
- Security_feedback [CRUD]
- Appgroup [CRUD]
- Addons_config [C]
- Rateplan [CRUD]
- Sharedflow [CRD]
- Organization [CRUD]
- Export [CR]
- Space [CRUD]
- Apiproduct [CRUD]
- Create [C]
- Key [CRUD]
- Environment [CRUD]
- Datacollector [CRUD]
- Endpoint_attachment [CRD]
- Keystore [CRD]
- Flowhook [RUD]
- Cache [D]
- Host_security_report [CR]
- Operation [R]
- Revision [CRD]
- Attachment [CRD]
- Balance [C]
- Canaryevaluation [CR]
- Dns_zone [CRD]
- Override [CRUD]
- Aliase [CRUD]
- Security_profile [CRUD]
- Instance [CRUD]
- Host_stat [R]
- Security_profiles_v2 [CRUD]
- Report [CRUD]
- Deployment [CR]

📖 [Full apigee_api documentation](services/apigee_api.md)

### 323. Netapp_api

**Resources**: 24

- Operation [CRD]
- Location [R]
- Volume [CRUD]
- Quota_rule [CRUD]
- Replication [CRUD]
- Backup_vault [CRUD]
- Kms_config [CRUD]
- Snapshot [CRUD]
- Active_directorie [CRUD]
- Backup [CRUD]
- Backup_policie [CRUD]
- Storage_pool [CRUD]
- Volume [CRUD]
- Replication [CRUD]
- Backup_policie [CRUD]
- Active_directorie [CRUD]
- Backup_vault [CRUD]
- Kms_config [CRUD]
- Operation [CRD]
- Backup [CRUD]
- Storage_pool [CRUD]
- Snapshot [CRUD]
- Quota_rule [CRUD]
- Location [R]

📖 [Full netapp_api documentation](services/netapp_api.md)

### 324. Identitytoolkit_api

**Resources**: 12

- Default_supported_idp_config [CRUD]
- Account [C]
- Identity_platform [C]
- Default_supported_idp [R]
- Oauth_idp_config [CRUD]
- Identitytoolkit [R]
- Inbound_saml_config [CRUD]
- Mfa_sign_in [C]
- Mfa_enrollment [C]
- Project [RU]
- Tenant [CRUD]
- Relyingparty [CR]

📖 [Full identitytoolkit_api documentation](services/identitytoolkit_api.md)

### 325. Datastream_api

**Resources**: 14

- Route [CRD]
- Stream [CRUD]
- Object [CR]
- Location [R]
- Private_connection [CRD]
- Connection_profile [CRUD]
- Operation [CRD]
- Operation [CRD]
- Route [CRD]
- Stream [CRUD]
- Location [R]
- Connection_profile [CRUD]
- Private_connection [CRD]
- Object [CR]

📖 [Full datastream_api documentation](services/datastream_api.md)

### 326. Iamcredentials_api

**Resources**: 1

- Service_account [C]

📖 [Full iamcredentials_api documentation](services/iamcredentials_api.md)

### 327. Datalabeling_api

**Resources**: 15

- Feedback_thread [RD]
- Video [C]
- Annotated_dataset [RD]
- Text [C]
- Example [R]
- Image [C]
- Example_comparison [C]
- Annotation_spec_set [CRD]
- Instruction [CRD]
- Dataset [CRD]
- Data_item [R]
- Evaluation [R]
- Feedback_message [CRD]
- Operation [RD]
- Evaluation_job [CRUD]

📖 [Full datalabeling_api documentation](services/datalabeling_api.md)

### 328. Youtubereporting_api

**Resources**: 4

- Report [R]
- Job [CRD]
- Media [R]
- Report_type [R]

📖 [Full youtubereporting_api documentation](services/youtubereporting_api.md)

### 329. Genomics_api

**Resources**: 5

- Pipeline [CRUD]
- Operation [CR]
- Pipeline [C]
- Worker [C]
- Operation [CR]

📖 [Full genomics_api documentation](services/genomics_api.md)

### 330. Lifesciences_api

**Resources**: 3

- Pipeline [C]
- Location [R]
- Operation [CR]

📖 [Full lifesciences_api documentation](services/lifesciences_api.md)

### 331. Compute_api

**Resources**: 371

- Instant_snapshot [CRD]
- Region_ssl_policie [CRUD]
- Region_instance_template [CRD]
- Vpn_tunnel [CRD]
- Region_ssl_certificate [CRD]
- Wire_group [CRUD]
- Target_grpc_proxie [CRUD]
- Network_attachment [CRUD]
- Region_disk_type [R]
- Interconnect_attachment [CRUD]
- Region_network_endpoint_group [CRD]
- Ssl_certificate [CRD]
- Subnetwork [CRUD]
- Interconnect [CRUD]
- Interconnect_attachment_group [CRUD]
- Target_vpn_gateway [CRD]
- Forwarding_rule [CRUD]
- Https_health_check [CRUD]
- Region_target_tcp_proxie [CRD]
- Vpn_gateway [CRD]
- Region_url_map [CRUD]
- Region_health_check_service [CRUD]
- Interconnect_group [CRUD]
- External_vpn_gateway [CRD]
- Instance_group_manager_resize_request [CRD]
- Instance_group_manager [CRUD]
- License_code [CR]
- Security_policie [CRUD]
- Reservation [CRUD]
- Service_attachment [CRUD]
- Node_type [R]
- Interconnect_location [R]
- Autoscaler [CRUD]
- Image_family_view [R]
- Snapshot_setting [RU]
- Backend_bucket [CRUD]
- Storage_pool [CRUD]
- Packet_mirroring [CRUD]
- Node_group [CRUD]
- Url_map [CRUD]
- Disk [CRUD]
- Target_pool [CRD]
- Region_health_check [CRUD]
- Global_operation [CRD]
- Public_advertised_prefixe [CRUD]
- Region_instance_group_manager [CRUD]
- Http_health_check [CRUD]
- Network_edge_security_service [CRUD]
- Target_instance [CRD]
- Region_security_policie [CRUD]
- Global_network_endpoint_group [CRD]
- Resource_policie [CRUD]
- Region_instant_snapshot [CRD]
- License [CRUD]
- Ssl_policie [CRUD]
- Global_forwarding_rule [CRUD]
- Zone [R]
- Zone_operation [CRD]
- Region_operation [CRD]
- Reservation_sub_block [CR]
- Reservation_block [CR]
- Region_instance [C]
- Cross_site_network [CRUD]
- Region_backend_service [CRUD]
- Disk_type [R]
- Machine_image [CRD]
- Target_tcp_proxie [CRD]
- Target_https_proxie [CRUD]
- Addresse [CRD]
- Organization_security_policie [CRUD]
- Network_profile [R]
- Global_addresse [CRD]
- Preview_feature [RU]
- Accelerator_type [R]
- Region_target_http_proxie [CRD]
- Firewall_policie [CRUD]
- Instance_group [CRD]
- Firewall [CRUD]
- Global_organization_operation [RD]
- Region_target_https_proxie [CRUD]
- Region_commitment [CRU]
- Route [CRD]
- Node_template [CRD]
- Interconnect_remote_location [R]
- Region_disk [CRUD]
- Project [CR]
- Region_instance_group [CR]
- Router [CRUD]
- Network_firewall_policie [CRUD]
- Storage_pool_type [R]
- Image [CRUD]
- Region_network_firewall_policie [CRUD]
- Backend_service [CRUD]
- Health_check [CRUD]
- Region_zone [R]
- Global_public_delegated_prefixe [CRUD]
- Instance_setting [RU]
- Future_reservation [CRUD]
- Target_http_proxie [CRUD]
- Instance [CRUD]
- Snapshot [CRD]
- Instance_template [CRD]
- Region_notification_endpoint [CRD]
- Network_endpoint_group [CRD]
- Region_autoscaler [CRUD]
- Target_ssl_proxie [CRD]
- Region [R]
- Network [CRUD]
- Machine_type [R]
- Public_delegated_prefixe [CRUD]
- Network_attachment [CRUD]
- Region_autoscaler [CRUD]
- Vpn_gateway [CRD]
- Region_notification_endpoint [CRD]
- Preview_feature [RU]
- License_code [CR]
- Region_instance_template [CRD]
- Machine_type [R]
- Reservation_sub_block [CR]
- Target_pool [CRD]
- Packet_mirroring [CRUD]
- Interconnect_location [R]
- Security_policie [CRUD]
- Zone [R]
- Health_check [CRUD]
- Global_forwarding_rule [CRUD]
- Interconnect_group [CRUD]
- Region_backend_bucket [CRUD]
- Region_health_aggregation_policie [CRUD]
- External_vpn_gateway [CRD]
- Region_backend_service [CRUD]
- Global_vm_extension_policie [CRUD]
- Node_group [CRUD]
- Region_ssl_policie [CRUD]
- Snapshot_setting [RU]
- Instance_group_manager_resize_request [CRD]
- Disk_type [R]
- Subnetwork [CRUD]
- Region_zone [R]
- Instance [CRUD]
- Region_target_http_proxie [CRD]
- Disk_setting [RU]
- Target_ssl_proxie [CRD]
- Reservation [CRUD]
- Addresse [CRD]
- Region [R]
- Network [CRUD]
- Instance_group_manager [CRUD]
- Snapshot [CRD]
- Region_security_policie [CRUD]
- Http_health_check [CRUD]
- License [CRUD]
- Region_instance_group_manager_resize_request [CRD]
- Zone_folder_operation [R]
- Global_operation [CRD]
- Route [CRD]
- Global_folder_operation [R]
- Machine_image [CRD]
- Zone_vm_extension_policie [CRUD]
- Instant_snapshot_group [CRD]
- Target_grpc_proxie [CRUD]
- Zone_queued_resource [CRD]
- Region_commitment [CRU]
- Reservation_block [CR]
- Region_multi_mig_member [R]
- Vpn_tunnel [CRD]
- Network_edge_security_service [CRUD]
- Region_instance_group_manager [CRUD]
- Snapshot_group [CRD]
- Node_template [CRD]
- Zone_organization_operation [R]
- Interconnect_attachment_group [CRUD]
- Advice [C]
- Region_composite_health_check [CRUD]
- Target_https_proxie [CRUD]
- Region_instant_snapshot [CRD]
- Region_snapshot_setting [RU]
- Instance_template [CRD]
- Instant_snapshot [CRD]
- Storage_pool_type [R]
- Global_organization_operation [RD]
- Zone_operation [CRD]
- Target_instance [CRD]
- Network_profile [R]
- Service_attachment [CRUD]
- Region_ssl_certificate [CRD]
- Global_addresse [CRD]
- Global_network_endpoint_group [CRD]
- Interconnect_attachment [CRUD]
- Project [CR]
- Region_url_map [CRUD]
- Region_target_tcp_proxie [CRD]
- Https_health_check [CRUD]
- Cross_site_network [CRUD]
- Resource_policie [CRUD]
- Image [CRUD]
- Backend_bucket [CRUD]
- Autoscaler [CRUD]
- Region_disk_setting [RU]
- Rollout_plan [CRD]
- Region_instance_group [CR]
- Rollout [RUD]
- Router [CRUD]
- Recoverable_snapshot [CRD]
- Firewall_policie [CRUD]
- Region_disk [CRUD]
- Target_tcp_proxie [CRD]
- Region_multi_mig [CRD]
- Instance_setting [RU]
- Url_map [CRUD]
- Node_type [R]
- Interconnect_remote_location [R]
- Ssl_policie [CRUD]
- Region_health_check_service [CRUD]
- Region_instant_snapshot_group [CRD]
- Region_disk_type [R]
- Region_health_check [CRUD]
- Wire_group [CRUD]
- Target_http_proxie [CRUD]
- Forwarding_rule [CRUD]
- Public_advertised_prefixe [CRUD]
- Ha_controller [CRUD]
- Region_target_https_proxie [CRUD]
- Backend_service [CRUD]
- Disk [CRUD]
- Reliability_risk [R]
- Instance_group [CRD]
- Target_vpn_gateway [CRD]
- Future_reservation [CRUD]
- Region_network_policie [CRUD]
- Network_endpoint_group [CRD]
- Region_network_firewall_policie [CRUD]
- Interconnect [CRUD]
- Region_instance [C]
- Global_public_delegated_prefixe [CRUD]
- Public_delegated_prefixe [CRUD]
- Region_snapshot [CRD]
- Region_network_endpoint_group [CRD]
- Organization_security_policie [CRUD]
- Accelerator_type [R]
- Region_health_source [CRUD]
- Region_operation [CRD]
- Storage_pool [CRUD]
- Ssl_certificate [CRD]
- Firewall [CRUD]
- Image_family_view [R]
- Network_firewall_policie [CRUD]
- Interconnect_location [R]
- Cross_site_network [CRUD]
- Region_ssl_certificate [CRD]
- Zone_operation [CRD]
- Region_composite_health_check [CRUD]
- Region_disk_setting [RU]
- Global_forwarding_rule [CRUD]
- Public_delegated_prefixe [CRUD]
- External_vpn_gateway [CRD]
- Region_security_policie [CRUD]
- Interconnect_attachment [CRUD]
- Global_network_endpoint_group [CRD]
- Global_public_delegated_prefixe [CRUD]
- Target_vpn_gateway [CRD]
- Router [CRUD]
- License_code [CR]
- Security_policie [CRUD]
- Region_backend_bucket [CRUD]
- Http_health_check [CRUD]
- Global_operation [CRD]
- Region_target_http_proxie [CRD]
- Region_instance [C]
- Region_instance_group_manager [CRUD]
- Instance_group_manager [CRUD]
- Health_check [CRUD]
- Snapshot_setting [RU]
- Snapshot [CRD]
- Region_url_map [CRUD]
- Network [CRUD]
- Subnetwork [CRUD]
- Machine_type [R]
- Interconnect_remote_location [R]
- Instant_snapshot [CRD]
- Ssl_policie [CRUD]
- Instance_setting [RU]
- Ssl_certificate [CRD]
- Disk_type [R]
- Region_network_firewall_policie [CRUD]
- Region_target_https_proxie [CRUD]
- Advice [C]
- Region_notification_endpoint [CRD]
- Future_reservation [CRUD]
- Node_template [CRD]
- Route [CRD]
- Target_instance [CRD]
- Region_snapshot_setting [RU]
- Region_snapshot [CRD]
- Region [R]
- Global_addresse [CRD]
- Disk_setting [RU]
- Resource_policie [CRUD]
- Storage_pool [CRUD]
- Target_pool [CRD]
- Forwarding_rule [CRUD]
- Node_type [R]
- Packet_mirroring [CRUD]
- Region_health_check [CRUD]
- Firewall_policie [CRUD]
- Region_instance_template [CRD]
- Backend_service [CRUD]
- License [CRUD]
- Addresse [CRD]
- Interconnect_group [CRUD]
- Public_advertised_prefixe [CRUD]
- Autoscaler [CRUD]
- Instance_group [CRD]
- Interconnect_attachment_group [CRUD]
- Accelerator_type [R]
- Vpn_tunnel [CRD]
- Network_edge_security_service [CRUD]
- Region_zone [R]
- Reservation_block [CR]
- Instance_template [CRD]
- Region_instance_group [CR]
- Target_http_proxie [CRUD]
- Instance [CRUD]
- Service_attachment [CRUD]
- Preview_feature [RU]
- Region_disk [CRUD]
- Target_grpc_proxie [CRUD]
- Backend_bucket [CRUD]
- Interconnect [CRUD]
- Url_map [CRUD]
- Project [CR]
- Region_target_tcp_proxie [CRD]
- Region_network_endpoint_group [CRD]
- Region_network_policie [CRUD]
- Region_health_source [CRUD]
- Target_tcp_proxie [CRD]
- Global_organization_operation [RD]
- Disk [CRUD]
- Region_instance_group_manager_resize_request [CRD]
- Region_health_check_service [CRUD]
- Storage_pool_type [R]
- Reservation [CRUD]
- Region_backend_service [CRUD]
- Network_endpoint_group [CRD]
- Wire_group [CRUD]
- Target_ssl_proxie [CRD]
- Zone_vm_extension_policie [CRUD]
- Region_operation [CRD]
- Node_group [CRUD]
- Region_multi_mig [CRD]
- Machine_image [CRD]
- Image [CRUD]
- Https_health_check [CRUD]
- Region_health_aggregation_policie [CRUD]
- Region_disk_type [R]
- Firewall [CRUD]
- Reservation_sub_block [CR]
- Network_attachment [CRUD]
- Region_instant_snapshot [CRD]
- Region_autoscaler [CRUD]
- Region_ssl_policie [CRUD]
- Global_vm_extension_policie [R]
- Instance_group_manager_resize_request [CRD]
- Target_https_proxie [CRUD]
- Vpn_gateway [CRD]
- Network_profile [R]
- Region_commitment [CRU]
- Zone [R]
- Network_firewall_policie [CRUD]
- Organization_security_policie [CRUD]
- Image_family_view [R]

📖 [Full compute_api documentation](services/compute_api.md)

### 332. Youtubeanalytics_api

**Resources**: 6

- Report [R]
- Group [CRUD]
- Group_item [CRD]
- Group [CRUD]
- Group_item [CRD]
- Report [R]

📖 [Full youtubeanalytics_api documentation](services/youtubeanalytics_api.md)

### 333. Factchecktools_api

**Resources**: 2

- Page [CRUD]
- Claim [R]

📖 [Full factchecktools_api documentation](services/factchecktools_api.md)

### 334. Dialogflow_api

**Resources**: 101

- Agent [CRUD]
- Security_setting [CRUD]
- Location [R]
- Tool [CRUD]
- Changelog [R]
- Flow [CRUD]
- Conversation [RD]
- Example [CRUD]
- Result [R]
- Environment [CRUD]
- Continuous_test_result [R]
- Version [CRUD]
- Operation [CR]
- Deployment [R]
- Test_case [CRU]
- Experiment [CRUD]
- Session [C]
- Entity_type [CRUD]
- Webhook [CRUD]
- Playbook [CRUD]
- Page [CRUD]
- Intent [CRUD]
- Transition_route_group [CRUD]
- Generator [CRUD]
- Message [CR]
- Stateless_suggestion [C]
- Knowledge_base [CRUD]
- Project [CRD]
- Participant [CRU]
- Version [CRUD]
- Conversation [CR]
- Encryption_spec [C]
- Document [CRUD]
- Entity_type [CRUD]
- Session [CD]
- Agent [CRU]
- Environment [CRUD]
- Intent [CRUD]
- Suggestion [CR]
- Context [CRUD]
- Entitie [C]
- Answer_record [RU]
- Operation [CR]
- Phone_number [CRUD]
- Sip_trunk [CRUD]
- Location [CRD]
- Tool [CRUD]
- Generator [CRUD]
- Conversation_profile [CRUD]
- Evaluation [CRD]
- Knowledge_base [CRUD]
- Document [CRUD]
- Encryption_spec [C]
- Answer_record [RU]
- Conversation [CR]
- Operation [CR]
- Location [CRD]
- Version [CRUD]
- Session [CD]
- Evaluation [CRD]
- Agent [CRU]
- Intent [CRUD]
- Conversation_dataset [CRD]
- Generator [CRUD]
- Entity_type [CRUD]
- Stateless_suggestion [C]
- Conversation_profile [CRUD]
- Message [R]
- Tool [CRUD]
- Project [CRD]
- Entitie [C]
- Suggestion [C]
- Environment [CRUD]
- Context [CRUD]
- Participant [CRU]
- Conversation_model [CRD]
- Sip_trunk [CRUD]
- Version [CRUD]
- Flow [CRUD]
- Playbook [CRUD]
- Example [CRUD]
- Continuous_test_result [R]
- Intent [CRUD]
- Session [C]
- Result [R]
- Environment [CRUD]
- Deployment [R]
- Tool [CRUD]
- Experiment [CRUD]
- Location [R]
- Changelog [R]
- Transition_route_group [CRUD]
- Generator [CRUD]
- Security_setting [CRUD]
- Agent [CRUD]
- Webhook [CRUD]
- Entity_type [CRUD]
- Page [CRUD]
- Test_case [CRU]
- Operation [CR]
- Operation [CR]

📖 [Full dialogflow_api documentation](services/dialogflow_api.md)

### 335. Docs_api

**Resources**: 1

- Document [CR]

📖 [Full docs_api documentation](services/docs_api.md)

### 336. Tagmanager_api

**Resources**: 30

- Entitie [R]
- Move_folder [U]
- Variable [CRUD]
- Container [CRUD]
- Folder [CRUD]
- Trigger [CRUD]
- Environment [CRUD]
- Reauthorize_environment [U]
- Version [CRUD]
- Permission [CRUD]
- Account [RU]
- Tag [CRUD]
- Version [CRUD]
- Folder [CRUD]
- Zone [CRUD]
- Variable [CRUD]
- Environment [CRUD]
- Transformation [CRUD]
- Version_header [R]
- Container [CRUD]
- Gtag_config [CRUD]
- Tag [CRUD]
- Client [CRUD]
- Destination [CR]
- Template [CRUD]
- Trigger [CRUD]
- User_permission [CRUD]
- Account [RU]
- Workspace [CRUD]
- Built_in_variable [CRD]

📖 [Full tagmanager_api documentation](services/tagmanager_api.md)

### 337. Texttospeech_api

**Resources**: 8

- Voice [R]
- Operation [CRD]
- Location [C]
- Text [C]
- Location [C]
- Text [C]
- Operation [R]
- Voice [R]

📖 [Full texttospeech_api documentation](services/texttospeech_api.md)

### 338. Gmail_api

**Resources**: 15

- Smime_info [CRD]
- Filter [CRD]
- Delegate [CRD]
- Send_a [CRUD]
- Label [CRUD]
- History [R]
- Keypair [CR]
- Identitie [CRUD]
- Draft [CRUD]
- Attachment [R]
- Message [CRD]
- Forwarding_addresse [CRD]
- Setting [RU]
- Thread [CRD]
- User [CR]

📖 [Full gmail_api documentation](services/gmail_api.md)

### 339. Tasks_api

**Resources**: 2

- Tasklist [CRUD]
- Task [CRUD]

📖 [Full tasks_api documentation](services/tasks_api.md)

### 340. Securityposture_api

**Resources**: 6

- Posture_deployment [CRUD]
- Operation [CRD]
- Location [R]
- Posture [CRUD]
- Posture_template [R]
- Report [CR]

📖 [Full securityposture_api documentation](services/securityposture_api.md)

### 341. Sql_api

**Resources**: 8

- Instance [CRUD]
- Operation [R]
- User [CRUD]
- Tier [R]
- Backup_run [CRD]
- Database [CRUD]
- Flag [R]
- Ssl_cert [CRD]

📖 [Full sql_api documentation](services/sql_api.md)

### 342. Integrations_api

**Resources**: 34

- Project [R]
- Apps_script_project [C]
- Connector_platform_region [R]
- Location [R]
- Integration [CRD]
- Cloud_function [C]
- Sfdc_instance [CRUD]
- Suspension [CR]
- Connection [R]
- Runtime_action_schema [R]
- Product [CR]
- Execution [CR]
- Runtime_entity_schema [R]
- Certificate [CRUD]
- Client [C]
- Version [CRUD]
- Callback [R]
- Executionsnapshot [R]
- Sfdc_channel [CRUD]
- Auth_config [CRUD]
- Callback [R]
- Runtime_entity_schema [R]
- Apps_script_project [C]
- Execution [CR]
- Auth_config [CRUD]
- Runtime_action_schema [R]
- Integration [CRD]
- Suspension [CR]
- Sfdc_instance [CRUD]
- Connection [R]
- Version [CRUD]
- Connector_platform_region [R]
- Certificate [CRUD]
- Sfdc_channel [CRUD]

📖 [Full integrations_api documentation](services/integrations_api.md)

### 343. Essentialcontacts_api

**Resources**: 1

- Contact [CRUD]

📖 [Full essentialcontacts_api documentation](services/essentialcontacts_api.md)

### 344. Tpu_api

**Resources**: 23

- Operation [CRD]
- Accelerator_type [R]
- Location [R]
- Tensorflow_version [R]
- Node [CRD]
- Location [CR]
- Operation [CRD]
- Node [CRUD]
- Queued_resource [CRD]
- Accelerator_type [R]
- Runtime_version [R]
- Location [R]
- Operation [CRD]
- Tensorflow_version [R]
- Accelerator_type [R]
- Node [CRD]
- Runtime_version [R]
- Reservation [R]
- Accelerator_type [R]
- Node [CRUD]
- Location [CR]
- Operation [CRD]
- Queued_resource [CRD]

📖 [Full tpu_api documentation](services/tpu_api.md)

### 345. Classroom_api

**Resources**: 17

- Add_on_attachment [CRUD]
- Guardian [RD]
- Announcement [CRUD]
- Course [CRUD]
- Guardian_invitation [CRU]
- User_profile [R]
- Invitation [CRD]
- Topic [CRUD]
- Post [R]
- Student_submission [CRU]
- Teacher [CRD]
- Course_work_material [CRUD]
- Student [CRD]
- Aliase [CRD]
- Registration [CD]
- Rubric [CRUD]
- Course_work [CRUD]

📖 [Full classroom_api documentation](services/classroom_api.md)

### 346. Mybusinesslodging_api

**Resources**: 2

- Location [RU]
- Lodging [R]

📖 [Full mybusinesslodging_api documentation](services/mybusinesslodging_api.md)

### 347. Rapidmigrationassessment_api

**Resources**: 4

- Collector [CRUD]
- Location [R]
- Operation [CRD]
- Annotation [CR]

📖 [Full rapidmigrationassessment_api documentation](services/rapidmigrationassessment_api.md)

### 348. Blockchainnodeengine_api

**Resources**: 3

- Operation [CRD]
- Location [R]
- Blockchain_node [CRUD]

📖 [Full blockchainnodeengine_api documentation](services/blockchainnodeengine_api.md)


---

## Example: Complete Workflow

Here's a complete example showing a typical workflow:

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
    region = "us-central1"
}

# Create glossarie
glossarie = provider.translate_api.Glossarie {
    parent = "example-value"
}

# Use resource outputs
glossarie_id = glossarie.id
glossarie_end_time = glossarie.end_time
glossarie_entry_count = glossarie.entry_count
glossarie_input_config = glossarie.input_config
glossarie_language_codes_set = glossarie.language_codes_set
glossarie_name = glossarie.name
glossarie_language_pair = glossarie.language_pair
glossarie_submit_time = glossarie.submit_time

```

---

## Configuration

### Environment Variables

Configure GCP credentials using a service account:

```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account-key.json
export GCP_PROJECT=your-project-id
```

### KCL Configuration

```kcl
# Configure provider in your KCL code
provider = gcp.GcpProvider {
    project = "my-project-id"
    region = "us-west1"
    # Credentials from GOOGLE_APPLICATION_CREDENTIALS
}
```

---

## Next Steps

- 📚 [Service Documentation](services/) - Detailed docs for each service
- 📖 [Installation Guide](installation.md) - Installation options
- ⬅️ [Back to README](../README.md)

---

## Need Help?

- 📖 Check service-specific documentation in `docs/services/`
- 🐛 [Report issues](https://github.com/YOUR_ORG/hemmer-provider-gcp/issues)
- 💬 [Join discussions](https://github.com/YOUR_ORG/hemmer-provider-gcp/discussions)
