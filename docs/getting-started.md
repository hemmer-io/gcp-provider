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

# Create a caption
caption = provider.mediaupload.Caption {
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    caption = provider.mediaupload.Caption {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
caption = provider.mediaupload.Caption {
    # configuration
}

# Reference its outputs
output_value = caption.id
```

---

## Available Services

This provider includes 348 services:

### 1. Mediaupload

**Resources**: 1

- Caption [C]

📖 [Full mediaupload documentation](services/mediaupload.md)

### 2. Driveactivity_api

**Resources**: 1

- Activity [C]

📖 [Full driveactivity_api documentation](services/driveactivity_api.md)

### 3. Commentanalyzer_api

**Resources**: 1

- Comment [C]

📖 [Full commentanalyzer_api documentation](services/commentanalyzer_api.md)

### 4. Streetviewpublish_api

**Resources**: 2

- Photo [CRUD]
- Photo_sequence [CRD]

📖 [Full streetviewpublish_api documentation](services/streetviewpublish_api.md)

### 5. Script_api

**Resources**: 5

- Version [CR]
- Project [CRU]
- Script [C]
- Deployment [CRUD]
- Processe [R]

📖 [Full script_api documentation](services/script_api.md)

### 6. Webmasters_api

**Resources**: 3

- Sitemap [RUD]
- Site [RUD]
- Searchanalytic [C]

📖 [Full webmasters_api documentation](services/webmasters_api.md)

### 7. Marketingplatformadmin_api

**Resources**: 2

- Organization [CR]
- Analytics_account_link [CRD]

📖 [Full marketingplatformadmin_api documentation](services/marketingplatformadmin_api.md)

### 8. Clouddebugger_api

**Resources**: 2

- Breakpoint [CRUD]
- Debuggee [CR]

📖 [Full clouddebugger_api documentation](services/clouddebugger_api.md)

### 9. Gamesconfiguration_api

**Resources**: 2

- Achievement_configuration [CRUD]
- Leaderboard_configuration [CRUD]

📖 [Full gamesconfiguration_api documentation](services/gamesconfiguration_api.md)

### 10. Cloudasset_api

**Resources**: 16

- Operation [R]
- Saved_querie [CRUD]
- Feed [CRUD]
- Asset [R]
- Effective_iam_policie [R]
- Cloudasset [CR]
- Asset [R]
- Cloudasset [CR]
- Cloudasset [C]
- Operation [R]
- Operation [R]
- Organization [CR]
- Project [CR]
- Folder [C]
- Resource [R]
- Iam_policie [R]

📖 [Full cloudasset_api documentation](services/cloudasset_api.md)

### 11. Apigeeregistry_api

**Resources**: 10

- Version [CRUD]
- Spec [CRUD]
- Location [R]
- Deployment [CRUD]
- Runtime [CR]
- Api [CRUD]
- Document [CR]
- Instance [CRD]
- Operation [CRD]
- Artifact [CRUD]

📖 [Full apigeeregistry_api documentation](services/apigeeregistry_api.md)

### 12. Notebooks_api

**Resources**: 10

- Instance [CRUD]
- Runtime [CRUD]
- Environment [CRD]
- Execution [CRD]
- Schedule [CRD]
- Operation [CRD]
- Location [R]
- Location [R]
- Instance [CRUD]
- Operation [CRD]

📖 [Full notebooks_api documentation](services/notebooks_api.md)

### 13. Contactcenterinsights_api

**Resources**: 23

- Issue_model [CRUD]
- Issue [CRUD]
- Authorized_view_set [CRUD]
- Dataset [CRUD]
- Assessment_rule [CRUD]
- Qa_scorecard [CRUD]
- Analyse [CRD]
- Operation [CR]
- Qa_question [CRUD]
- Qa_question_tag [CRUD]
- Authorized_view [CRUD]
- Note [CRUD]
- Analysis_rule [CRUD]
- Revision [CRD]
- Location [CRU]
- Encryption_spec [C]
- Feedback_label [CRUD]
- Segment [C]
- Assessment [CRD]
- View [CRUD]
- Insightsdata [C]
- Phrase_matcher [CRUD]
- Conversation [CRUD]

📖 [Full contactcenterinsights_api documentation](services/contactcenterinsights_api.md)

### 14. Books_api

**Resources**: 21

- Promooffer [CR]
- Bookshelve [CR]
- Mybook [R]
- Annotation_data [R]
- Associated [R]
- Useruploaded [R]
- Recommended [CR]
- Volume [R]
- Onboarding [R]
- Cloudloading [C]
- Serie [R]
- Membership [R]
- Dictionary [R]
- Volume_annotation [R]
- Readingposition [CR]
- Myconfig [CR]
- Personalizedstream [R]
- Notification [R]
- Familysharing [CR]
- Layer [R]
- Annotation [CRUD]

📖 [Full books_api documentation](services/books_api.md)

### 15. Servicemanagement_api

**Resources**: 5

- Operation [R]
- Consumer [C]
- Config [CR]
- Service [CRD]
- Rollout [CR]

📖 [Full servicemanagement_api documentation](services/servicemanagement_api.md)

### 16. Oslogin_api

**Resources**: 14

- Ssh_public_key [CRUD]
- Project [D]
- Location [C]
- User [CR]
- Zone [C]
- User [CR]
- Location [C]
- Project [CD]
- Ssh_public_key [CRUD]
- Project [CD]
- Zone [C]
- User [CR]
- Location [C]
- Ssh_public_key [CRUD]

📖 [Full oslogin_api documentation](services/oslogin_api.md)

### 17. Bigqueryreservation_api

**Resources**: 14

- Reservation_group [CRD]
- Capacity_commitment [CRUD]
- Location [RU]
- Reservation [CRUD]
- Assignment [CRUD]
- Slot_pool [RD]
- Reservation_grant [CRD]
- Location [R]
- Reservation [CRUD]
- Operation [CR]
- Reservation [CRUD]
- Capacity_commitment [CRUD]
- Location [RU]
- Assignment [CRUD]

📖 [Full bigqueryreservation_api documentation](services/bigqueryreservation_api.md)

### 18. Urlshortener_api

**Resources**: 1

- Url [CR]

📖 [Full urlshortener_api documentation](services/urlshortener_api.md)

### 19. Spanner_api

**Resources**: 14

- Database_role [CR]
- Database_operation [R]
- Instance_config [CRUD]
- Backup_operation [R]
- Operation [CRD]
- Session [CRD]
- Backup_schedule [CRUD]
- Instance_partition_operation [R]
- Instance_config_operation [R]
- Instance [CRUD]
- Database [CRUD]
- Instance_partition [CRUD]
- Scan [R]
- Backup [CRUD]

📖 [Full spanner_api documentation](services/spanner_api.md)

### 20. Servicecontrol_api

**Resources**: 2

- Service [C]
- Service [C]

📖 [Full servicecontrol_api documentation](services/servicecontrol_api.md)

### 21. Pubsub_api

**Resources**: 8

- Schema [CRD]
- Snapshot [CRUD]
- Topic [CRUD]
- Subscription [CRUD]
- Topic [CRD]
- Subscription [CRD]
- Topic [CRD]
- Subscription [CRD]

📖 [Full pubsub_api documentation](services/pubsub_api.md)

### 22. Androiddeviceprovisioning_api

**Resources**: 6

- Dpc [R]
- Customer [CR]
- Device [CR]
- Operation [R]
- Configuration [CRUD]
- Vendor [R]

📖 [Full androiddeviceprovisioning_api documentation](services/androiddeviceprovisioning_api.md)

### 23. Servicenetworking_api

**Resources**: 11

- Dns_record_set [CRU]
- Service [CU]
- Network [RU]
- Role [C]
- Dns_zone [CR]
- Peered_dns_domain [CRD]
- Connection [CRU]
- Operation [CRD]
- Connection [CR]
- Service [CU]
- Operation [R]

📖 [Full servicenetworking_api documentation](services/servicenetworking_api.md)

### 24. Vision_api

**Resources**: 10

- File [C]
- Product [CRUD]
- Image [C]
- Product_set [CRUD]
- Reference_image [CRD]
- Operation [CRD]
- Image [C]
- File [C]
- Image [C]
- File [C]

📖 [Full vision_api documentation](services/vision_api.md)

### 25. Genomics_api

**Resources**: 5

- Pipeline [CRUD]
- Operation [CR]
- Worker [C]
- Operation [CR]
- Pipeline [C]

📖 [Full genomics_api documentation](services/genomics_api.md)

### 26. Run_api

**Resources**: 23

- Execution [CRD]
- Location [R]
- Job [CRUD]
- Workerpool [CRUD]
- Service [CRUD]
- Revision [RD]
- Operation [CRD]
- Authorizeddomain [R]
- Route [R]
- Task [R]
- Domainmapping [CRD]
- Configuration [R]
- Operation [CRD]
- Task [R]
- Job [CRUD]
- Worker_pool [CRUD]
- Service [CRUD]
- Execution [CRD]
- Location [CR]
- Build [C]
- Revision [RD]
- Customresourcedefinition [R]
- Job [CRD]

📖 [Full run_api documentation](services/run_api.md)

### 27. Gkeonprem_api

**Resources**: 8

- Vmware_node_pool [CRUD]
- Bare_metal_node_pool [CRUD]
- Location [R]
- Vmware_cluster [CRUD]
- Vmware_admin_cluster [CRUD]
- Bare_metal_admin_cluster [CRUD]
- Operation [CRD]
- Bare_metal_cluster [CRUD]

📖 [Full gkeonprem_api documentation](services/gkeonprem_api.md)

### 28. Youtube_api

**Resources**: 33

- Playlist_image [CRUD]
- Subscription [CRD]
- V3 [U]
- Channel [RU]
- Member [R]
- Watermark [C]
- Video_trainability [R]
- Live_chat_moderator [CRD]
- Playlist [CRUD]
- Search [R]
- Abuse_report [C]
- Super_chat_event [R]
- Thumbnail [C]
- Activitie [R]
- Live_stream [CRUD]
- Live_chat_message [CRD]
- Caption [CRUD]
- Video_abuse_report_reason [R]
- Channel_section [CRUD]
- Comment [CRUD]
- Third_party_link [CRUD]
- Video_categorie [R]
- I18n_region [R]
- I18n_language [R]
- Memberships_level [R]
- Video [CRUD]
- Channel_banner [C]
- Test [C]
- Playlist_item [CRUD]
- Comment_thread [CR]
- Live_broadcast [CRUD]
- Live_chat_ban [CD]
- Message [R]

📖 [Full youtube_api documentation](services/youtube_api.md)

### 29. Contactcenteraiplatform_api

**Resources**: 3

- Operation [CRD]
- Location [R]
- Contact_center [CRUD]

📖 [Full contactcenteraiplatform_api documentation](services/contactcenteraiplatform_api.md)

### 30. Ondemandscanning_api

**Resources**: 6

- Vulnerabilitie [R]
- Operation [CRD]
- Scan [C]
- Vulnerabilitie [R]
- Scan [C]
- Operation [CRD]

📖 [Full ondemandscanning_api documentation](services/ondemandscanning_api.md)

### 31. Appstate_api

**Resources**: 1

- State [CRUD]

📖 [Full appstate_api documentation](services/appstate_api.md)

### 32. Licensing_api

**Resources**: 1

- License_assignment [CRUD]

📖 [Full licensing_api documentation](services/licensing_api.md)

### 33. Clouddeploy_api

**Resources**: 11

- Operation [CRD]
- Release [CR]
- Job_run [CR]
- Custom_target_type [CRUD]
- Location [R]
- Target [CRUD]
- Automation [CRUD]
- Delivery_pipeline [CRUD]
- Automation_run [CR]
- Deploy_policie [CRUD]
- Rollout [CR]

📖 [Full clouddeploy_api documentation](services/clouddeploy_api.md)

### 34. Cloudcommerceprocurement_api

**Resources**: 2

- Account [CR]
- Entitlement [CRU]

📖 [Full cloudcommerceprocurement_api documentation](services/cloudcommerceprocurement_api.md)

### 35. Vpcaccess_api

**Resources**: 6

- Location [R]
- Operation [R]
- Connector [CRUD]
- Location [R]
- Connector [CRUD]
- Operation [R]

📖 [Full vpcaccess_api documentation](services/vpcaccess_api.md)

### 36. Vmwareengine_api

**Resources**: 20

- Private_connection [CRUD]
- Private_cloud [CRUD]
- Cluster [CRUD]
- External_access_rule [CRUD]
- Dns_bind_permission [C]
- Location [R]
- Node [R]
- Announcement [R]
- Node_type [R]
- Upgrade [RU]
- Network_peering [CRUD]
- Hcx_activation_key [CR]
- Management_dns_zone_binding [CRUD]
- Network_policie [CRUD]
- Logging_server [CRUD]
- Peering_route [R]
- External_addresse [CRUD]
- Vmware_engine_network [CRUD]
- Operation [RD]
- Subnet [RU]

📖 [Full vmwareengine_api documentation](services/vmwareengine_api.md)

### 37. Http_body

**Resources**: 1

- Fhir [CR]

📖 [Full http_body documentation](services/http_body.md)

### 38. Mybusinessqanda_api

**Resources**: 2

- Answer [CRD]
- Question [CRUD]

📖 [Full mybusinessqanda_api documentation](services/mybusinessqanda_api.md)

### 39. Prod_tt_sasportal_api

**Resources**: 6

- Device [CRUD]
- Policie [CR]
- Deployment [CRUD]
- Customer [CRU]
- Node [CRUD]
- Installer [C]

📖 [Full prod_tt_sasportal_api documentation](services/prod_tt_sasportal_api.md)

### 40. Adexchangebuyer_api

**Resources**: 21

- Proposal [CRU]
- Product [R]
- Marketplacenote [CR]
- Pubprofile [R]
- Pretargeting_config [CRUD]
- Billing_info [R]
- Budget [RU]
- Creative [CR]
- Marketplacedeal [CRUD]
- Marketplaceprivateauction [C]
- Performance_report [R]
- Account [RU]
- Performance_report [R]
- Budget [RU]
- Direct_deal [R]
- Pretargeting_config [CRUD]
- Account [RU]
- Billing_info [R]
- Creative [CR]
- Creative [CR]
- Account [RU]

📖 [Full adexchangebuyer_api documentation](services/adexchangebuyer_api.md)

### 41. Mybusinesslodging_api

**Resources**: 2

- Location [RU]
- Lodging [R]

📖 [Full mybusinesslodging_api documentation](services/mybusinesslodging_api.md)

### 42. Serviceconsumermanagement_api

**Resources**: 8

- Service [R]
- Tenancy_unit [CRD]
- Operation [CRD]
- Operation [R]
- Limit [R]
- Producer_override [CRUD]
- Producer_quota_policie [CRUD]
- Consumer_quota_metric [CR]

📖 [Full serviceconsumermanagement_api documentation](services/serviceconsumermanagement_api.md)

### 43. Integrations_api

**Resources**: 34

- Sfdc_instance [CRUD]
- Executionsnapshot [R]
- Location [R]
- Auth_config [CRUD]
- Project [R]
- Cloud_function [C]
- Runtime_action_schema [R]
- Runtime_entity_schema [R]
- Version [CRUD]
- Integration [CRD]
- Product [CR]
- Suspension [CR]
- Apps_script_project [C]
- Callback [R]
- Client [C]
- Sfdc_channel [CRUD]
- Execution [CR]
- Connection [R]
- Certificate [CRUD]
- Connector_platform_region [R]
- Apps_script_project [C]
- Callback [R]
- Certificate [CRUD]
- Execution [CR]
- Connector_platform_region [R]
- Suspension [CR]
- Sfdc_instance [CRUD]
- Runtime_action_schema [R]
- Version [CRUD]
- Auth_config [CRUD]
- Runtime_entity_schema [R]
- Connection [R]
- Integration [CRD]
- Sfdc_channel [CRUD]

📖 [Full integrations_api documentation](services/integrations_api.md)

### 44. Playablelocations_api

**Resources**: 1

- Playablelocation [C]

📖 [Full playablelocations_api documentation](services/playablelocations_api.md)

### 45. Chromewebstore_api

**Resources**: 3

- Item [CRU]
- Media [C]
- Item [CR]

📖 [Full chromewebstore_api documentation](services/chromewebstore_api.md)

### 46. Backupdr_api

**Resources**: 13

- Data_source [CRU]
- Data_source_reference [R]
- Trial [C]
- Location [R]
- Service_config [C]
- Operation [CRD]
- Revision [R]
- Backup [CRUD]
- Backup_plan_association [CRUD]
- Backup_vault [CRUD]
- Backup_plan [CRUD]
- Management_server [CRD]
- Resource_backup_config [R]

📖 [Full backupdr_api documentation](services/backupdr_api.md)

### 47. Observability_api

**Resources**: 4

- Trace_scope [CRUD]
- Location [R]
- Operation [CRD]
- Scope [RU]

📖 [Full observability_api documentation](services/observability_api.md)

### 48. Recaptchaenterprise_api

**Resources**: 6

- Key [CRUD]
- Relatedaccountgroupmembership [C]
- Assessment [C]
- Relatedaccountgroup [R]
- Membership [R]
- Firewallpolicie [CRUD]

📖 [Full recaptchaenterprise_api documentation](services/recaptchaenterprise_api.md)

### 49. Replicapool_api

**Resources**: 2

- Pool [CRD]
- Replica [CRD]

📖 [Full replicapool_api documentation](services/replicapool_api.md)

### 50. Apigateway_api

**Resources**: 20

- Api [CRUD]
- Config [CRUD]
- Operation [CRD]
- Gateway [CRUD]
- Location [R]
- Api [CR]
- Gateway [CR]
- Operation [CRD]
- Config [CR]
- Location [R]
- Api [CRUD]
- Location [R]
- Operation [CRD]
- Config [CRUD]
- Gateway [CRUD]
- Operation [CRD]
- Config [CR]
- Api [CR]
- Location [R]
- Gateway [CR]

📖 [Full apigateway_api documentation](services/apigateway_api.md)

### 51. Dlp_api

**Resources**: 14

- Deidentify_template [CRUD]
- Column_data_profile [R]
- Discovery_config [CRUD]
- Inspect_template [CRUD]
- Dlp_job [CRD]
- Connection [CRUD]
- Job_trigger [CRUD]
- Project_data_profile [R]
- Table_data_profile [RD]
- Content [C]
- Image [C]
- Stored_info_type [CRUD]
- File_store_data_profile [RD]
- Info_type [R]

📖 [Full dlp_api documentation](services/dlp_api.md)

### 52. Adsense_api

**Resources**: 34

- Account [R]
- Adclient [R]
- Adunit [CRU]
- Customchannel [CRUD]
- Alert [R]
- Payment [R]
- Site [R]
- Policy_issue [R]
- Saved [R]
- Urlchannel [R]
- Report [R]
- Metric [R]
- Dimension [R]
- Account [R]
- Savedadstyle [R]
- Adunit [R]
- Urlchannel [R]
- Report [R]
- Adclient [R]
- Customchannel [R]
- Alert [RD]
- Saved [R]
- Payment [R]
- Customchannel [R]
- Account [R]
- Savedadstyle [R]
- Report [R]
- Saved [R]
- Adclient [R]
- Dimension [R]
- Metric [R]
- Urlchannel [R]
- Adunit [R]
- Alert [R]

📖 [Full adsense_api documentation](services/adsense_api.md)

### 53. Oracledatabase_api

**Resources**: 24

- Database [R]
- Database_character_set [R]
- Operation [CRD]
- Db_system_shape [R]
- Pluggable_database [R]
- Cloud_vm_cluster [CRD]
- Autonomous_database_character_set [R]
- Odb_network [CRD]
- Exadb_vm_cluster [CRUD]
- Cloud_exadata_infrastructure [CRD]
- Autonomous_db_version [R]
- Db_server [R]
- Exascale_db_storage_vault [CRD]
- Db_system [CRD]
- Gi_version [R]
- Odb_subnet [CRD]
- Db_system_initial_storage_size [R]
- Db_version [R]
- Autonomous_database [CRUD]
- Autonomous_database_backup [R]
- Entitlement [R]
- Minor_version [R]
- Location [R]
- Db_node [R]

📖 [Full oracledatabase_api documentation](services/oracledatabase_api.md)

### 54. Toolresults_api

**Resources**: 11

- Step [CRU]
- Historie [CR]
- Execution [CRU]
- Cluster [R]
- Environment [R]
- Test_case [R]
- Thumbnail [R]
- Perf_sample_serie [CR]
- Project [CR]
- Perf_metrics_summary [C]
- Sample [CR]

📖 [Full toolresults_api documentation](services/toolresults_api.md)

### 55. Binaryauthorization_api

**Resources**: 9

- Project [RU]
- Attestor [CRUD]
- Policy [CR]
- Policie [CRUD]
- Systempolicy [R]
- Systempolicy [R]
- Attestor [CRUD]
- Project [RU]
- Policy [CR]

📖 [Full binaryauthorization_api documentation](services/binaryauthorization_api.md)

### 56. Config_api

**Resources**: 9

- Operation [CRD]
- Resource [R]
- Deployment [CRUD]
- Terraform_version [R]
- Resource_drift [R]
- Revision [CR]
- Location [R]
- Preview [CRD]
- Resource_change [R]

📖 [Full config_api documentation](services/config_api.md)

### 57. Androidmanagement_api

**Resources**: 11

- Enterprise [CRUD]
- Device [CRUD]
- Operation [CR]
- Signup_url [C]
- Web_token [C]
- Enrollment_token [CRD]
- Migration_token [CR]
- Web_app [CRUD]
- Policie [CRUD]
- Application [R]
- Provisioning_info [R]

📖 [Full androidmanagement_api documentation](services/androidmanagement_api.md)

### 58. Fusiontables_api

**Resources**: 12

- Column [CRUD]
- Query [CR]
- Template [CRUD]
- Table [CRUD]
- Style [CRUD]
- Task [RD]
- Task [RD]
- Template [CRUD]
- Style [CRUD]
- Query [CR]
- Column [CRUD]
- Table [CRUD]

📖 [Full fusiontables_api documentation](services/fusiontables_api.md)

### 59. Policysimulator_api

**Resources**: 12

- Replay [CR]
- Operation [R]
- Org_policy_violations_preview [CR]
- Org_policy_violation [R]
- Result [R]
- Operation [R]
- Org_policy_violations_preview [CR]
- Operation [R]
- Result [R]
- Replay [CR]
- Org_policy_violation [R]
- Operation [R]

📖 [Full policysimulator_api documentation](services/policysimulator_api.md)

### 60. Searchads360_api

**Resources**: 4

- Search_ads360 [C]
- Custom_column [R]
- Search_ads360_field [CR]
- Customer [R]

📖 [Full searchads360_api documentation](services/searchads360_api.md)

### 61. Eventarc_api

**Resources**: 14

- Message_buse [CRUD]
- Provider [R]
- Kafka_source [CR]
- Trigger [CRUD]
- Location [RU]
- Channel_connection [CRD]
- Operation [CRD]
- Channel [CRUD]
- Enrollment [CRUD]
- Google_api_source [CRUD]
- Pipeline [CRUD]
- Location [R]
- Trigger [CRUD]
- Operation [CRD]

📖 [Full eventarc_api documentation](services/eventarc_api.md)

### 62. Adsensehost_api

**Resources**: 7

- Adunit [CRUD]
- Urlchannel [CRD]
- Report [R]
- Account [R]
- Customchannel [CRUD]
- Adclient [R]
- Associationsession [R]

📖 [Full adsensehost_api documentation](services/adsensehost_api.md)

### 63. Cloudprivatecatalog_api

**Resources**: 3

- Catalog [R]
- Version [R]
- Product [R]

📖 [Full cloudprivatecatalog_api documentation](services/cloudprivatecatalog_api.md)

### 64. Servicebroker_api

**Resources**: 13

- Servicebroker [CR]
- Instance [R]
- Binding [R]
- Service_binding [CRD]
- Service_instance [CRUD]
- Catalog [R]
- Broker [CRD]
- Servicebroker [CR]
- Service_binding [CRD]
- Service_instance [CRUD]
- Servicebroker [CR]
- Catalog [R]
- Instance [R]

📖 [Full servicebroker_api documentation](services/servicebroker_api.md)

### 65. Accessapproval_api

**Resources**: 4

- Organization [RUD]
- Folder [RUD]
- Project [RUD]
- Approval_request [CR]

📖 [Full accessapproval_api documentation](services/accessapproval_api.md)

### 66. Datalabeling_api

**Resources**: 15

- Annotated_dataset [RD]
- Image [C]
- Text [C]
- Operation [RD]
- Evaluation [R]
- Example_comparison [C]
- Example [R]
- Data_item [R]
- Feedback_message [CRD]
- Instruction [CRD]
- Feedback_thread [RD]
- Annotation_spec_set [CRD]
- Video [C]
- Evaluation_job [CRUD]
- Dataset [CRD]

📖 [Full datalabeling_api documentation](services/datalabeling_api.md)

### 67. Bigquerydatatransfer_api

**Resources**: 6

- Run [RD]
- Transfer_config [CRUD]
- Data_source [CR]
- Transfer_log [R]
- Project [C]
- Location [CR]

📖 [Full bigquerydatatransfer_api documentation](services/bigquerydatatransfer_api.md)

### 68. Mybusinessverifications_api

**Resources**: 3

- Verification_token [C]
- Location [CR]
- Verification [CR]

📖 [Full mybusinessverifications_api documentation](services/mybusinessverifications_api.md)

### 69. Localservices_api

**Resources**: 2

- Account_report [R]
- Detailed_lead_report [R]

📖 [Full localservices_api documentation](services/localservices_api.md)

### 70. Bigtableadmin_api

**Resources**: 12

- App_profile [CRUD]
- Location [R]
- Authorized_view [CRUD]
- Hot_tablet [R]
- Logical_view [CRUD]
- Table [CRUD]
- Schema_bundle [CRUD]
- Materialized_view [CRUD]
- Cluster [CRUD]
- Instance [CRUD]
- Operation [R]
- Backup [CRUD]

📖 [Full bigtableadmin_api documentation](services/bigtableadmin_api.md)

### 71. Cloudtasks_api

**Resources**: 9

- Task [CRD]
- Queue [CRUD]
- Location [RU]
- Queue [CRUD]
- Location [RU]
- Task [CRD]
- Task [CRD]
- Location [RU]
- Queue [CRUD]

📖 [Full cloudtasks_api documentation](services/cloudtasks_api.md)

### 72. Siteverification_api

**Resources**: 1

- Web_resource [CRUD]

📖 [Full siteverification_api documentation](services/siteverification_api.md)

### 73. Cloudiot_api

**Resources**: 5

- Group [C]
- Config_version [R]
- Registrie [CRUD]
- State [R]
- Device [CRUD]

📖 [Full cloudiot_api documentation](services/cloudiot_api.md)

### 74. Indexing_api

**Resources**: 1

- Url_notification [CR]

📖 [Full indexing_api documentation](services/indexing_api.md)

### 75. Networkmanagement_api

**Resources**: 8

- Vpc_flow_logs_config [CRUD]
- Operation [CRD]
- Location [R]
- Connectivity_test [CRUD]
- Vpc_flow_logs_config [CRUD]
- Operation [CRD]
- Connectivity_test [CRUD]
- Location [R]

📖 [Full networkmanagement_api documentation](services/networkmanagement_api.md)

### 76. Sheets_api

**Resources**: 4

- Spreadsheet [CR]
- Value [CRU]
- Developer_metadata [CR]
- Sheet [C]

📖 [Full sheets_api documentation](services/sheets_api.md)

### 77. Looker_api

**Resources**: 4

- Instance [CRUD]
- Operation [CRD]
- Location [R]
- Backup [CRD]

📖 [Full looker_api documentation](services/looker_api.md)

### 78. Dataform_api

**Resources**: 20

- Repositorie [CRUD]
- Workflow_config [CRUD]
- Team_folder [CR]
- Location [RU]
- Workspace [CRD]
- Release_config [CRUD]
- Compilation_result [CR]
- Operation [CRD]
- Folder [CR]
- Workflow_invocation [CRD]
- Workspace [CRD]
- Release_config [CRUD]
- Workflow_config [CRUD]
- Compilation_result [CR]
- Workflow_invocation [CRD]
- Operation [CRD]
- Team_folder [CR]
- Folder [CR]
- Repositorie [CRUD]
- Location [RU]

📖 [Full dataform_api documentation](services/dataform_api.md)

### 79. Drive_api

**Resources**: 27

- Change [CR]
- Revision [RUD]
- App [R]
- About [R]
- Teamdrive [CRUD]
- Drive [CRUD]
- Propertie [CRUD]
- Parent [CRD]
- Channel [C]
- Replie [CRUD]
- Children [CRD]
- Permission [CRUD]
- Comment [CRUD]
- File [CRUD]
- Drive [CRUD]
- About [R]
- Revision [RUD]
- Replie [CRUD]
- Permission [CRUD]
- Operation [R]
- Accessproposal [CR]
- Channel [C]
- File [CRUD]
- Teamdrive [CRUD]
- App [R]
- Comment [CRUD]
- Change [CR]

📖 [Full drive_api documentation](services/drive_api.md)

### 80. Container_api

**Resources**: 14

- Operation [CR]
- Zone [R]
- Usable_subnetwork [R]
- Location [R]
- Node_pool [CRUD]
- Cluster [CRUD]
- Well_known [R]
- Usable_subnetwork [R]
- Zone [R]
- Location [R]
- Well_known [R]
- Operation [CR]
- Cluster [CRUD]
- Node_pool [CRUD]

📖 [Full container_api documentation](services/container_api.md)

### 81. Netapp_api

**Resources**: 24

- Kms_config [CRUD]
- Backup_vault [CRUD]
- Snapshot [CRUD]
- Storage_pool [CRUD]
- Backup_policie [CRUD]
- Quota_rule [CRUD]
- Active_directorie [CRUD]
- Backup [CRUD]
- Location [R]
- Volume [CRUD]
- Operation [CRD]
- Replication [CRUD]
- Operation [CRD]
- Quota_rule [CRUD]
- Backup_policie [CRUD]
- Location [R]
- Replication [CRUD]
- Volume [CRUD]
- Kms_config [CRUD]
- Active_directorie [CRUD]
- Backup_vault [CRUD]
- Storage_pool [CRUD]
- Backup [CRUD]
- Snapshot [CRUD]

📖 [Full netapp_api documentation](services/netapp_api.md)

### 82. Datastream_api

**Resources**: 14

- Operation [CRD]
- Object [CR]
- Stream [CRUD]
- Private_connection [CRD]
- Route [CRD]
- Connection_profile [CRUD]
- Location [R]
- Connection_profile [CRUD]
- Location [R]
- Private_connection [CRD]
- Stream [CRUD]
- Operation [CRD]
- Route [CRD]
- Object [CR]

📖 [Full datastream_api documentation](services/datastream_api.md)

### 83. Parametermanager_api

**Resources**: 3

- Location [R]
- Parameter [CRUD]
- Version [CRUD]

📖 [Full parametermanager_api documentation](services/parametermanager_api.md)

### 84. Firebase_api

**Resources**: 9

- Default_location [C]
- Project [CRU]
- Android_app [CRU]
- Operation [R]
- Sha [CRD]
- Available_location [R]
- Available_project [R]
- Web_app [CRU]
- Ios_app [CRU]

📖 [Full firebase_api documentation](services/firebase_api.md)

### 85. Bigquery_api

**Resources**: 8

- Routine [CRUD]
- Project [R]
- Tabledata [CR]
- Dataset [CRUD]
- Table [CRUD]
- Job [CRD]
- Model [RUD]
- Row_access_policie [CRUD]

📖 [Full bigquery_api documentation](services/bigquery_api.md)

### 86. Apphub_api

**Resources**: 16

- Service_project_attachment [CRD]
- Discovered_workload [R]
- Operation [CRD]
- Location [CR]
- Discovered_service [R]
- Application [CRUD]
- Service [CRUD]
- Workload [CRUD]
- Operation [CRD]
- Application [CRUD]
- Workload [CRUD]
- Discovered_workload [R]
- Discovered_service [R]
- Location [CR]
- Service_project_attachment [CRD]
- Service [CRUD]

📖 [Full apphub_api documentation](services/apphub_api.md)

### 87. Apigee_api

**Resources**: 64

- Environment [CRUD]
- Apiproduct [CRUD]
- Cache [D]
- Addons_config [C]
- Project [C]
- Override [CRUD]
- Sharedflow [CRD]
- Apidoc [CRUD]
- Subscription [CR]
- Optimized_host_stat [R]
- Datastore [CRUD]
- Keyvaluemap [CRUD]
- Rateplan [CRUD]
- Querie [CR]
- Deployment [CR]
- Export [CR]
- Dns_zone [CRD]
- Attachment [CRD]
- Security_monitoring_condition [CRUD]
- Create [C]
- Appgroup [CRUD]
- Envgroup [CRUD]
- Resourcefile [CRUD]
- Optimized_stat [R]
- Organization [CRUD]
- Targetserver [CRUD]
- Apicategorie [CRUD]
- Security_assessment_result [C]
- Reference [CRUD]
- Security_profiles_v2 [CRUD]
- Security_profile [CRUD]
- Security_stat [C]
- Security_report [CR]
- Operation [R]
- Security_incident [CRU]
- Canaryevaluation [CR]
- Host_querie [CR]
- Instance [CRUD]
- Report [CRUD]
- Endpoint_attachment [CRD]
- Nat_addresse [CRD]
- Security_feedback [CRUD]
- Key [CRUD]
- Issuer [R]
- Space [CRUD]
- Api [CRUD]
- Host_stat [R]
- Flowhook [RUD]
- Aliase [CRUD]
- Archive_deployment [CRUD]
- Stat [R]
- Entrie [CRUD]
- Datacollector [CRUD]
- Debugsession [CRD]
- Security_action [CRUD]
- Keystore [CRD]
- Revision [CRD]
- Data [R]
- Developer [CRUD]
- Attribute [CRD]
- Admin [R]
- App [CRUD]
- Host_security_report [CR]
- Balance [C]

📖 [Full apigee_api documentation](services/apigee_api.md)

### 88. Transcoder_api

**Resources**: 4

- Job [CRD]
- Job_template [CRD]
- Job [CRD]
- Job_template [CRD]

📖 [Full transcoder_api documentation](services/transcoder_api.md)

### 89. Metastore_api

**Resources**: 35

- Backup [CRD]
- Service [CRUD]
- Location [R]
- Metadata_import [CRU]
- Database [CR]
- Operation [CRD]
- Federation [CRUD]
- Migration_execution [RD]
- Table [CR]
- Migration_execution [RD]
- Operation [CRD]
- Location [R]
- Service [CRUD]
- Backup [CRD]
- Federation [CRUD]
- Table [CR]
- Metadata_import [CRU]
- Database [CR]
- Service [CRUD]
- Migration_execution [RD]
- Backup [CRD]
- Service [CRUD]
- Backup [CRD]
- Service [CRUD]
- Database [CR]
- Location [R]
- Backup [CRD]
- Table [CR]
- Federation [CRUD]
- Migration_execution [RD]
- Metadata_import [CRU]
- Operation [CRD]
- Migration_execution [RD]
- Backup [CRD]
- Service [CRUD]

📖 [Full metastore_api documentation](services/metastore_api.md)

### 90. Apikeys_api

**Resources**: 2

- Operation [R]
- Key [CRUD]

📖 [Full apikeys_api documentation](services/apikeys_api.md)

### 91. Trafficdirector_api

**Resources**: 2

- Discovery [C]
- Discovery [C]

📖 [Full trafficdirector_api documentation](services/trafficdirector_api.md)

### 92. Libraryagent_api

**Resources**: 2

- Shelve [R]
- Book [CR]

📖 [Full libraryagent_api documentation](services/libraryagent_api.md)

### 93. Securitycenter_api

**Resources**: 38

- Asset [CRU]
- Notification_config [CRUD]
- External_system [U]
- Operation [CRD]
- Effective_custom_module [R]
- Finding [CRU]
- Organization [RU]
- Attack_path [R]
- Big_query_export [CRUD]
- Event_threat_detection_setting [C]
- Valued_resource [R]
- Custom_module [CRUD]
- Simulation [R]
- Source [CRU]
- Mute_config [CRUD]
- Resource_value_config [CRUD]
- Operation [CRD]
- Cluster [RU]
- Virtual_machine_threat_detection_setting [R]
- Project [RU]
- Organization [RU]
- Rapid_vulnerability_detection_setting [R]
- Folder [RU]
- Event_threat_detection_setting [R]
- Web_security_scanner_setting [R]
- Security_health_analytics_setting [R]
- Container_threat_detection_setting [R]
- Organization [RU]
- Finding [CRU]
- Source [CRU]
- Operation [CRD]
- Asset [CRU]
- Finding [CRU]
- Asset [CRU]
- Notification_config [CRUD]
- Source [CRU]
- Operation [CRD]
- Organization [RU]

📖 [Full securitycenter_api documentation](services/securitycenter_api.md)

### 94. Firebaseappcheck_api

**Resources**: 26

- Resource_policie [CRUD]
- Recaptcha_v3_config [RU]
- Jwk [R]
- App_attest_config [RU]
- App [C]
- Service [CRU]
- Device_check_config [RU]
- Debug_token [CRUD]
- Play_integrity_config [RU]
- Recaptcha_enterprise_config [RU]
- Safety_net_config [RU]
- Oauth_client [C]
- Recaptcha_enterprise_config [RU]
- Debug_token [CRUD]
- App [C]
- Project [C]
- Jwk [R]
- Safety_net_config [RU]
- Service [CRU]
- Resource_policie [CRUD]
- App_attest_config [RU]
- Recaptcha_v3_config [RU]
- Play_integrity_config [RU]
- Device_check_config [RU]
- Recaptcha_config [RU]
- Oauth_client [C]

📖 [Full firebaseappcheck_api documentation](services/firebaseappcheck_api.md)

### 95. Poly_api

**Resources**: 2

- Likedasset [R]
- Asset [R]

📖 [Full poly_api documentation](services/poly_api.md)

### 96. Androidpublisher_api

**Resources**: 36

- Inapppurchase [R]
- Product [R]
- Voidedpurchase [R]
- Review [CR]
- Countryavailability [R]
- User [CRUD]
- Expansionfile [CRU]
- Application [C]
- Image [CRD]
- Subscription [CRUD]
- Inappproduct [CRUD]
- Variant [CR]
- Tester [RU]
- Order [CR]
- Internalappsharingartifact [C]
- Monetization [C]
- Product [CR]
- Externaltransaction [CR]
- Edit [CRD]
- Purchase_option [C]
- Base_plan [CD]
- Productsv2 [R]
- Listing [RUD]
- Grant [CUD]
- Apk [CR]
- Apprecovery [CR]
- Detail [RU]
- Bundle [CR]
- Track [CRU]
- Device_tier_config [CR]
- Generatedapk [R]
- Offer [CRUD]
- Subscriptionsv2 [CR]
- Deobfuscationfile [C]
- Onetimeproduct [CRUD]
- Voidedpurchase [R]

📖 [Full androidpublisher_api documentation](services/androidpublisher_api.md)

### 97. Civicinfo_api

**Resources**: 2

- Election [R]
- Division [R]

📖 [Full civicinfo_api documentation](services/civicinfo_api.md)

### 98. Secretmanager_api

**Resources**: 9

- Version [CR]
- Location [R]
- Secret [CRUD]
- Secret [CRUD]
- Version [CR]
- Location [R]
- Version [CR]
- Location [R]
- Secret [CRUD]

📖 [Full secretmanager_api documentation](services/secretmanager_api.md)

### 99. Blockchainnodeengine_api

**Resources**: 3

- Location [R]
- Operation [CRD]
- Blockchain_node [CRUD]

📖 [Full blockchainnodeengine_api documentation](services/blockchainnodeengine_api.md)

### 100. Gmail_api

**Resources**: 15

- Draft [CRUD]
- Setting [RU]
- Identitie [CRUD]
- User [CR]
- Delegate [CRD]
- History [R]
- Send_a [CRUD]
- Label [CRUD]
- Thread [CRD]
- Message [CRD]
- Smime_info [CRD]
- Attachment [R]
- Forwarding_addresse [CRD]
- Filter [CRD]
- Keypair [CR]

📖 [Full gmail_api documentation](services/gmail_api.md)

### 101. Mirror_api

**Resources**: 7

- Setting [R]
- Location [R]
- Contact [CRUD]
- Attachment [CRD]
- Timeline [CRUD]
- Account [C]
- Subscription [CRUD]

📖 [Full mirror_api documentation](services/mirror_api.md)

### 102. Factchecktools_api

**Resources**: 2

- Claim [R]
- Page [CRUD]

📖 [Full factchecktools_api documentation](services/factchecktools_api.md)

### 103. Acmedns_api

**Resources**: 1

- Acme_challenge_set [CR]

📖 [Full acmedns_api documentation](services/acmedns_api.md)

### 104. Playintegrity_api

**Resources**: 2

- Device_recall [C]
- Playintegrity [C]

📖 [Full playintegrity_api documentation](services/playintegrity_api.md)

### 105. Runtimeconfig_api

**Resources**: 5

- Operation [CRD]
- Operation [CR]
- Config [CRUD]
- Variable [CRUD]
- Waiter [CRD]

📖 [Full runtimeconfig_api documentation](services/runtimeconfig_api.md)

### 106. Proximitybeacon_api

**Resources**: 6

- Proximitybeacon [R]
- Beaconinfo [C]
- Attachment [CRD]
- Namespace [RU]
- Beacon [CRUD]
- Diagnostic [R]

📖 [Full proximitybeacon_api documentation](services/proximitybeacon_api.md)

### 107. Discoveryengine_api

**Resources**: 112

- Cmek_config [RUD]
- Answer [R]
- Identity_mapping_store [CRD]
- Document [CRUD]
- Schema [CRUD]
- Operation [CR]
- Location [CRU]
- Conversation [CRUD]
- Control [CRUD]
- Sitemap [CRD]
- Media [R]
- User_license [R]
- Collection [RUD]
- Completion_suggestion [C]
- Session [CRUD]
- License_config [CRU]
- Serving_config [CRU]
- Project [C]
- User_store [CRUD]
- Suggestion_deny_list_entrie [C]
- Grounding_config [C]
- Branche [R]
- Completion_config [C]
- Engine [CRUD]
- Data_store [CRUD]
- Custom_model [R]
- Target_site [CRUD]
- Assistant [CRU]
- Ranking_config [C]
- User_event [CR]
- Site_search_engine [CR]
- Site_search_engine [CR]
- Grounding_config [C]
- Requirement [C]
- Operation [CR]
- Connector_run [R]
- Engine [CRUD]
- Evaluation [CR]
- Sitemap [CRD]
- Completion_config [C]
- Session [CRUD]
- Answer [R]
- Source [CR]
- Ranking_config [C]
- Serving_config [CRU]
- Control [CRUD]
- Project [CRU]
- Canned_querie [CRUD]
- Sample_querie [CRUD]
- Notebook [CR]
- Branche [R]
- Collection [RUD]
- Agent [CRUD]
- Suggestion_deny_list_entrie [C]
- Schema [CRUD]
- User_store [CRUD]
- User_event [CR]
- File [CR]
- Cmek_config [RUD]
- User_license [R]
- Audio_overview [CD]
- Location [CRU]
- Target_site [CRUD]
- Billing_account_license_config [CR]
- Authorization [CRUD]
- Conversation [CRUD]
- Identity_mapping_store [CRD]
- Media [CR]
- Data_store [CRUD]
- Data_connector [CR]
- Assistant [CRU]
- Sample_query_set [CRUD]
- Analytic [C]
- Widget_config [RU]
- Chunk [R]
- License_config [CRU]
- Completion_suggestion [C]
- Document [CRUD]
- Custom_model [R]
- Session [CRUD]
- Evaluation [CR]
- Sitemap [CRD]
- Target_site [CRUD]
- Sample_query_set [CRUD]
- Identity_mapping_store [CRD]
- Control [CRUD]
- Data_store [CRUD]
- Serving_config [CRU]
- Suggestion_deny_list_entrie [C]
- Branche [R]
- User_license [R]
- Cmek_config [RUD]
- Operation [CR]
- Answer [R]
- Document [CRUD]
- License_config [CRU]
- Sample_querie [CRUD]
- User_store [CRUD]
- Conversation [CRUD]
- Location [CRU]
- Media [R]
- Assistant [CRU]
- Project [C]
- Ranking_config [C]
- Completion_config [C]
- Custom_model [R]
- Engine [CRUD]
- Site_search_engine [CR]
- User_event [CR]
- Completion_suggestion [C]
- Grounding_config [C]
- Schema [CRUD]

📖 [Full discoveryengine_api documentation](services/discoveryengine_api.md)

### 108. Doubleclickbidmanager_api

**Resources**: 4

- Querie [CRD]
- Report [R]
- Report [R]
- Querie [CRD]

📖 [Full doubleclickbidmanager_api documentation](services/doubleclickbidmanager_api.md)

### 109. Sourcerepo_api

**Resources**: 2

- Repo [CRUD]
- Project [RU]

📖 [Full sourcerepo_api documentation](services/sourcerepo_api.md)

### 110. Plusdomains_api

**Resources**: 6

- Activitie [R]
- Audience [R]
- Circle [R]
- Comment [R]
- People [R]
- Media [C]

📖 [Full plusdomains_api documentation](services/plusdomains_api.md)

### 111. Blogger_api

**Resources**: 13

- Blog [R]
- Post [R]
- Comment [R]
- Page [R]
- User [R]
- Blog [R]
- User [R]
- Blog_user_info [R]
- Comment [CRD]
- Post [CRUD]
- Page_view [R]
- Page [CRUD]
- Post_user_info [R]

📖 [Full blogger_api documentation](services/blogger_api.md)

### 112. Checks_api

**Resources**: 6

- Aisafety [C]
- App [R]
- Media [C]
- Scan [CR]
- Operation [CRD]
- Report [R]

📖 [Full checks_api documentation](services/checks_api.md)

### 113. Baremetalsolution_api

**Resources**: 15

- Operation [CRD]
- Lun [CR]
- Os_image [R]
- Provisioning_quota [R]
- Nfs_share [CRUD]
- Provisioning_config [CRU]
- Location [R]
- Snapshot [CRD]
- Operation [R]
- Instance [CRU]
- Ssh_key [CRD]
- Volume [CRU]
- Network [CRU]
- Location [C]
- Provisioning_quota [R]

📖 [Full baremetalsolution_api documentation](services/baremetalsolution_api.md)

### 114. Saasservicemgmt_api

**Resources**: 10

- Tenant [CRUD]
- Saa [CRUD]
- Unit_operation [CRUD]
- Location [R]
- Rollout_kind [CRUD]
- Unit [CRUD]
- Release [CRUD]
- Rollout [CRUD]
- Replications_internal [CRUD]
- Unit_kind [CRUD]

📖 [Full saasservicemgmt_api documentation](services/saasservicemgmt_api.md)

### 115. Cloudtrace_api

**Resources**: 5

- Project [U]
- Trace [R]
- Trace_sink [CRUD]
- Trace [C]
- Span [C]

📖 [Full cloudtrace_api documentation](services/cloudtrace_api.md)

### 116. Mybusinessnotifications_api

**Resources**: 1

- Account [RU]

📖 [Full mybusinessnotifications_api documentation](services/mybusinessnotifications_api.md)

### 117. Consumersurveys_api

**Resources**: 3

- Mobileapppanel [RU]
- Survey [CRUD]
- Result [R]

📖 [Full consumersurveys_api documentation](services/consumersurveys_api.md)

### 118. Displayvideo_api

**Resources**: 144

- Operation [R]
- Sdfdownloadtask [C]
- Guaranteed_order [CRU]
- Assigned_targeting_option [CRD]
- Inventory_source_group [CRUD]
- User [CRUD]
- Media [CR]
- Targeting_option [CR]
- Partner [CR]
- Location_list [CRU]
- First_and_third_party_audience [CRU]
- Manual_trigger [CRU]
- Google_audience [R]
- Custom_bidding_algorithm [CRU]
- Campaign [CRUD]
- Line_item [CRUD]
- Custom_list [R]
- Invoice [R]
- Floodlight_group [RU]
- Insertion_order [CRUD]
- Asset [C]
- Site [CRD]
- Creative [CRUD]
- Negative_keyword [CRD]
- Assigned_inventory_source [CRD]
- Advertiser [CRUD]
- Negative_keyword_list [CRUD]
- Script [CR]
- Combined_audience [R]
- Inventory_source [CRU]
- Channel [CRU]
- Assigned_location [CRD]
- Operation [R]
- Media [R]
- Line_item [CRUD]
- Script [CR]
- Combined_audience [R]
- Assigned_targeting_option [CRD]
- Inventory_source [CRU]
- Campaign [CRUD]
- Advertiser [CRUD]
- Asset [C]
- Negative_keyword [CRD]
- Site [CRD]
- Insertion_order [CRUD]
- Partner [CR]
- Creative [CRUD]
- Negative_keyword_list [CRUD]
- Custom_list [R]
- Channel [CRU]
- Inventory_source_group [CRUD]
- Manual_trigger [CRU]
- Custom_bidding_algorithm [CRU]
- Sdfdownloadtask [C]
- Guaranteed_order [CRU]
- Youtube_ad_group [R]
- Targeting_option [CR]
- Youtube_ad_group_ad [R]
- User [CRUD]
- Invoice [R]
- Location_list [CRU]
- Floodlight_activitie [R]
- Assigned_location [CRD]
- Media [CR]
- Assigned_inventory_source [CRD]
- Google_audience [R]
- Operation [R]
- Floodlight_group [RU]
- Operation [R]
- Media [R]
- Sdfdownloadtask [C]
- Invoice [R]
- Youtube_asset_association [CRD]
- Insertion_order [CRUD]
- Assigned_targeting_option [CRD]
- Custom_bidding_algorithm [CRU]
- Negative_keyword [CRD]
- Operation [R]
- First_party_and_partner_audience [CRU]
- Google_audience [R]
- Floodlight_group [RU]
- Media [CR]
- Ad_group [R]
- Negative_keyword_list [CRUD]
- Asset [C]
- Site [CRD]
- Targeting_option [CR]
- Line_item [CRUD]
- Custom_list [R]
- Advertiser [CRUD]
- Creative [CRUD]
- Channel [CRU]
- Combined_audience [R]
- Inventory_source [CRU]
- Campaign [CRUD]
- Location_list [CRU]
- Guaranteed_order [CRU]
- User [CRUD]
- Ad_asset [CR]
- Inventory_source_group [CRUD]
- Rule [CR]
- Ad_group_ad [R]
- Assigned_location [CRD]
- Floodlight_activitie [R]
- Script [CR]
- Assigned_inventory_source [CRD]
- Partner [CR]
- Custom_list [R]
- Sdfdownloadtask [C]
- Targeting_option [CR]
- Google_audience [R]
- Assigned_inventory_source [CRD]
- Ad_group [R]
- Guaranteed_order [CRU]
- Advertiser [CRUD]
- Negative_keyword [CRD]
- Floodlight_group [RU]
- Inventory_source [CRU]
- Negative_keyword_list [CRUD]
- User [CRUD]
- Floodlight_activitie [R]
- Inventory_source_group [CRUD]
- Operation [R]
- Rule [CR]
- Partner [CR]
- Line_item [CRUD]
- Site [CRD]
- Asset [C]
- Assigned_location [CRD]
- Ad_group_ad [R]
- Campaign [CRUD]
- Channel [CRU]
- Insertion_order [CRUD]
- Combined_audience [R]
- Custom_bidding_algorithm [CRU]
- Location_list [CRU]
- Assigned_targeting_option [CRD]
- Script [CR]
- Media [CR]
- Creative [CRUD]
- Invoice [R]
- First_and_third_party_audience [CRU]
- Media [R]
- Operation [R]

📖 [Full displayvideo_api documentation](services/displayvideo_api.md)

### 119. Dataplex_api

**Resources**: 33

- Task [CRUD]
- Encryption_config [CRUD]
- Data_taxonomie [CRUD]
- Operation [CRD]
- Action [R]
- Lake [CRUD]
- Entitie [CRUD]
- Data_scan [CRUD]
- Zone [CRUD]
- Contentitem [CRUD]
- Categorie [CRUD]
- Attribute [CRUD]
- Data_attribute_binding [CRUD]
- Data_product [C]
- Glossarie [CRUD]
- Environment [CRUD]
- Data_asset [C]
- Session [R]
- Entry_link [CRD]
- Job [CR]
- Metadata_job [CR]
- Entry_link_type [CR]
- Entrie [CRUD]
- Governance_rule [CR]
- Aspect_type [CRUD]
- Entry_type [CRUD]
- Partition [CRD]
- Term [CRUD]
- Entry_group [CRUD]
- Location [CR]
- Content [CRUD]
- Change_request [CR]
- Asset [CRUD]

📖 [Full dataplex_api documentation](services/dataplex_api.md)

### 120. Policytroubleshooter_api

**Resources**: 2

- Iam [C]
- Iam [C]

📖 [Full policytroubleshooter_api documentation](services/policytroubleshooter_api.md)

### 121. Meet_api

**Resources**: 7

- Recording [R]
- Transcript [R]
- Participant [R]
- Entrie [R]
- Conference_record [R]
- Participant_session [R]
- Space [CRU]

📖 [Full meet_api documentation](services/meet_api.md)

### 122. Area120tables_api

**Resources**: 3

- Workspace [R]
- Table [R]
- Row [CRUD]

📖 [Full area120tables_api documentation](services/area120tables_api.md)

### 123. Adexchangeseller_api

**Resources**: 27

- Saved [R]
- Adclient [R]
- Urlchannel [R]
- Adunit [R]
- Report [R]
- Customchannel [R]
- Saved [R]
- Metric [R]
- Account [R]
- Urlchannel [R]
- Report [R]
- Adclient [R]
- Preferreddeal [R]
- Dimension [R]
- Customchannel [R]
- Alert [R]
- Alert [R]
- Dimension [R]
- Urlchannel [R]
- Preferreddeal [R]
- Account [R]
- Adclient [R]
- Saved [R]
- Metric [R]
- Report [R]
- Adunit [R]
- Customchannel [R]

📖 [Full adexchangeseller_api documentation](services/adexchangeseller_api.md)

### 124. Firebasedataconnect_api

**Resources**: 10

- Location [R]
- Schema [CRUD]
- Service [CRUD]
- Connector [CRUD]
- Operation [CRD]
- Connector [CRUD]
- Service [CRUD]
- Location [R]
- Operation [CRD]
- Schema [CRUD]

📖 [Full firebasedataconnect_api documentation](services/firebasedataconnect_api.md)

### 125. Partners_api

**Resources**: 10

- History [R]
- User [RUD]
- Companie [R]
- Offer [R]
- Partner [RU]
- Lead [CR]
- User_event [C]
- User_state [R]
- Client_message [C]
- Analytic [R]

📖 [Full partners_api documentation](services/partners_api.md)

### 126. Billingbudgets_api

**Resources**: 2

- Budget [CRUD]
- Budget [CRUD]

📖 [Full billingbudgets_api documentation](services/billingbudgets_api.md)

### 127. Areainsights_api

**Resources**: 1

- Areainsight [C]

📖 [Full areainsights_api documentation](services/areainsights_api.md)

### 128. Calendar_api

**Resources**: 8

- Channel [C]
- Acl [CRUD]
- Freebusy [C]
- Calendar [CRUD]
- Color [R]
- Event [CRUD]
- Calendar_list [CRUD]
- Setting [CR]

📖 [Full calendar_api documentation](services/calendar_api.md)

### 129. Securityposture_api

**Resources**: 6

- Posture [CRUD]
- Location [R]
- Posture_deployment [CRUD]
- Posture_template [R]
- Operation [CRD]
- Report [CR]

📖 [Full securityposture_api documentation](services/securityposture_api.md)

### 130. Slides_api

**Resources**: 2

- Page [R]
- Presentation [CR]

📖 [Full slides_api documentation](services/slides_api.md)

### 131. Cloudsupport_api

**Resources**: 10

- Case_classification [R]
- Attachment [R]
- Comment [CR]
- Media [CR]
- Case [CRU]
- Attachment [R]
- Comment [CR]
- Case [CRU]
- Case_classification [R]
- Media [CR]

📖 [Full cloudsupport_api documentation](services/cloudsupport_api.md)

### 132. Datafusion_api

**Resources**: 11

- Dns_peering [CRD]
- Location [R]
- Instance [CRUD]
- Operation [CRD]
- Version [R]
- Location [CR]
- Instance [CRUD]
- Version [R]
- Namespace [CR]
- Operation [CRD]
- Dns_peering [CRD]

📖 [Full datafusion_api documentation](services/datafusion_api.md)

### 133. Firebasehosting_api

**Resources**: 9

- Operation [CRD]
- Operation [R]
- Release [CR]
- Site [CRUD]
- Version [CRUD]
- Domain [CRUD]
- Custom_domain [CRUD]
- Channel [CRUD]
- File [R]

📖 [Full firebasehosting_api documentation](services/firebasehosting_api.md)

### 134. Games_api

**Resources**: 13

- Revision [R]
- Recall [CR]
- Application [CR]
- Accesstoken [C]
- Score [CR]
- Leaderboard [R]
- Metagame [R]
- Snapshot [R]
- Player [R]
- Achievement_definition [R]
- Event [CR]
- Stat [R]
- Achievement [CR]

📖 [Full games_api documentation](services/games_api.md)

### 135. Iam_api

**Resources**: 20

- Permission [C]
- Namespace [CRUD]
- Provider [CRUD]
- Token [CRUD]
- Role [CRUD]
- Managed_identitie [CRUD]
- Subject [CD]
- Key [CRD]
- Iam_policie [C]
- Service_account [CRUD]
- Workforce_pool [CRUD]
- Credential [CRUD]
- Oauth_client [CRUD]
- Workload_identity_pool [CRUD]
- Operation [R]
- Scim_tenant [CRUD]
- Policie [CRUD]
- Operation [R]
- Policie [CRUD]
- Operation [R]

📖 [Full iam_api documentation](services/iam_api.md)

### 136. Reseller_api

**Resources**: 3

- Customer [CRU]
- Subscription [CRD]
- Resellernotify [CR]

📖 [Full reseller_api documentation](services/reseller_api.md)

### 137. File_api

**Resources**: 11

- Backup [CRUD]
- Snapshot [CRUD]
- Operation [CRD]
- Instance [CRUD]
- Location [R]
- Snapshot [CRUD]
- Share [CRUD]
- Backup [CRUD]
- Location [R]
- Operation [CRD]
- Instance [CRUD]

📖 [Full file_api documentation](services/file_api.md)

### 138. Serviceusage_api

**Resources**: 8

- Operation [CRD]
- Service [CR]
- Admin_override [CRUD]
- Service [CR]
- Consumer_quota_metric [CR]
- Operation [R]
- Consumer_override [CRUD]
- Limit [R]

📖 [Full serviceusage_api documentation](services/serviceusage_api.md)

### 139. Managedkafka_api

**Resources**: 18

- Schema_registrie [CRD]
- Operation [CRD]
- Cluster [CRUD]
- Schema [R]
- Compatibility [C]
- Version [CRD]
- Config [RUD]
- Referencedby [R]
- Type [R]
- Acl [CRUD]
- Location [R]
- Topic [CRUD]
- Context [R]
- Connect_cluster [CRUD]
- Consumer_group [RUD]
- Subject [CRD]
- Connector [CRUD]
- Mode [RUD]

📖 [Full managedkafka_api documentation](services/managedkafka_api.md)

### 140. Css_api

**Resources**: 5

- Quota [R]
- Css_product [R]
- Css_product_input [CUD]
- Label [CRUD]
- Account [CR]

📖 [Full css_api documentation](services/css_api.md)

### 141. Resourcesettings_api

**Resources**: 1

- Setting [RU]

📖 [Full resourcesettings_api documentation](services/resourcesettings_api.md)

### 142. Logging_api

**Resources**: 23

- Sink [CRUD]
- Metric [CRUD]
- Entrie [C]
- Monitored_resource_descriptor [R]
- Link [CRD]
- Saved_querie [CRUD]
- Bucket [CRUD]
- Recent_querie [R]
- Exclusion [CRUD]
- Billing_account [R]
- Entrie [C]
- Sink [CRUD]
- Log [RD]
- Log_scope [CRUD]
- Logging [RU]
- Location [R]
- Folder [RU]
- Project [R]
- Organization [RU]
- Operation [CR]
- View [CRUD]
- Monitored_resource_descriptor [R]
- Metric [CRUD]

📖 [Full logging_api documentation](services/logging_api.md)

### 143. Osconfig_api

**Resources**: 24

- Operation [CR]
- Os_policy_assignment [CRUD]
- Global [RU]
- Report [R]
- Inventorie [R]
- Vulnerability_report [R]
- Patch_job [CR]
- Instance_detail [R]
- Patch_deployment [CRUD]
- Inventorie [R]
- Operation [CR]
- Os_policy_assignment [CRUD]
- Vulnerability_report [R]
- Instance_os_policies_compliance [R]
- Report [R]
- Operation [CRD]
- Policy_orchestrator [CRUD]
- Operation [CRD]
- Policy_orchestrator [CRUD]
- Patch_job [CR]
- Patch_deployment [CRUD]
- Guest_policie [CRUD]
- Instance_detail [R]
- Instance [C]

📖 [Full osconfig_api documentation](services/osconfig_api.md)

### 144. Firebaserules_api

**Resources**: 3

- Release [CRUD]
- Project [C]
- Ruleset [CRD]

📖 [Full firebaserules_api documentation](services/firebaserules_api.md)

### 145. Recommender_api

**Resources**: 9

- Insight [CR]
- Recommendation [CR]
- Recommender [RU]
- Insight_type [RU]
- Location [R]
- Recommendation [CR]
- Insight [CR]
- Insight_type [RU]
- Recommender [RU]

📖 [Full recommender_api documentation](services/recommender_api.md)

### 146. Chromepolicy_api

**Resources**: 6

- Network [C]
- Group [C]
- Orgunit [C]
- Policie [C]
- Policy_schema [R]
- Media [C]

📖 [Full chromepolicy_api documentation](services/chromepolicy_api.md)

### 147. Merchantapi_api

**Resources**: 82

- File_upload [R]
- Data_source [CRUD]
- Regional_inventorie [CRD]
- Local_inventorie [CRD]
- Quota [R]
- Product [R]
- Product_input [CUD]
- Order_tracking_signal [C]
- Data_source [CRUD]
- File_upload [R]
- Lfp_sale [C]
- Lfp_inventorie [C]
- Lfp_store [CRD]
- Lfp_merchant_state [R]
- Conversion_source [CRUD]
- Issueresolution [C]
- Aggregate_product_statuse [R]
- Order_tracking_signal [C]
- Promotion [CR]
- Report [C]
- Issueresolution [C]
- Aggregate_product_statuse [R]
- Promotion [CR]
- Checkout_setting [CRUD]
- Business_info [RU]
- User [CRUD]
- Lfp_provider [CR]
- Service [CR]
- Homepage [CRU]
- Terms_of_service_agreement_state [R]
- Relationship [RU]
- Region [CRUD]
- Gbp_account [CR]
- Account [CRUD]
- Issue [R]
- Automatic_improvement [RU]
- Developer_registration [CR]
- Email_preference [RU]
- Business_identity [RU]
- Terms_of_service [CR]
- Program [CR]
- Shipping_setting [CR]
- Omnichannel_setting [CRU]
- Autofeed_setting [RU]
- Online_return_policie [CRD]
- Product [R]
- Product_input [CUD]
- Notificationsubscription [CRUD]
- Region [CRUD]
- Terms_of_service [CR]
- Lfp_provider [CR]
- Terms_of_service_agreement_state [R]
- Shipping_setting [CR]
- Business_identity [RU]
- Issue [R]
- Program [CR]
- Online_return_policie [CRUD]
- User [CRUD]
- Developer_registration [CR]
- Business_info [RU]
- Gbp_account [CR]
- Service [CR]
- Omnichannel_setting [CRU]
- Account [CRUD]
- Checkout_setting [CRUD]
- Relationship [RU]
- Homepage [CRU]
- Email_preference [RU]
- Autofeed_setting [RU]
- Automatic_improvement [RU]
- Merchant_review [CRD]
- Product_review [CRD]
- Quota [R]
- Conversion_source [CRUD]
- Local_inventorie [CRD]
- Regional_inventorie [CRD]
- Notificationsubscription [CRUD]
- Lfp_sale [C]
- Lfp_merchant_state [R]
- Lfp_inventorie [C]
- Lfp_store [CRD]
- Report [C]

📖 [Full merchantapi_api documentation](services/merchantapi_api.md)

### 148. Networkservices_api

**Resources**: 40

- Edge_cache_keyset [CR]
- Lb_edge_extension [CRUD]
- Location [R]
- Edge_cache_service [CR]
- Lb_route_extension [CRUD]
- Http_route [CRUD]
- Gateway [CRUD]
- Wasm_plugin [CRUD]
- Service_lb_policie [CRUD]
- Service_binding [CRUD]
- Grpc_route [CRUD]
- Endpoint_policie [CRUD]
- Tcp_route [CRUD]
- Tls_route [CRUD]
- Lb_traffic_extension [CRUD]
- Route_view [R]
- Authz_extension [CRUD]
- Edge_cache_origin [CR]
- Meshe [CRUD]
- Version [CRD]
- Operation [CRD]
- Lb_edge_extension [CRUD]
- Lb_tcp_extension [CRUD]
- Authz_extension [CRUD]
- Endpoint_policie [CRUD]
- Gateway [CRUD]
- Wasm_plugin [CRUD]
- Version [CRD]
- Lb_traffic_extension [CRUD]
- Tls_route [CRUD]
- Meshe [CRUD]
- Http_route [CRUD]
- Tcp_route [CRUD]
- Grpc_route [CRUD]
- Location [R]
- Lb_route_extension [CRUD]
- Service_binding [CRUD]
- Route_view [R]
- Operation [CRD]
- Service_lb_policie [CRUD]

📖 [Full networkservices_api documentation](services/networkservices_api.md)

### 149. Cloudkms_api

**Resources**: 12

- Operation [R]
- Crypto_key_version [CRU]
- Ekm_connection [CRU]
- Organization [RU]
- Folder [RU]
- Key_handle [CR]
- Ekm_config [CR]
- Key_ring [CR]
- Project [RU]
- Crypto_key [CRU]
- Location [CRU]
- Import_job [CR]

📖 [Full cloudkms_api documentation](services/cloudkms_api.md)

### 150. Mybusinessbusinesscalls_api

**Resources**: 2

- Location [RU]
- Businesscallsinsight [R]

📖 [Full mybusinessbusinesscalls_api documentation](services/mybusinessbusinesscalls_api.md)

### 151. Apihub_api

**Resources**: 19

- Instance [CRUD]
- Discovered_api_operation [R]
- Style_guide [R]
- Runtime_project_attachment [CRD]
- Operation [CRUD]
- Api_hub_instance [CRD]
- Spec [CRUD]
- External_api [CRUD]
- Definition [R]
- Attribute [CRUD]
- Discovered_api_observation [R]
- Deployment [CRUD]
- Host_project_registration [CR]
- Curation [CRUD]
- Version [CRUD]
- Api [CRUD]
- Plugin [CRUD]
- Dependencie [CRUD]
- Location [CR]

📖 [Full apihub_api documentation](services/apihub_api.md)

### 152. Chromemanagement_api

**Resources**: 14

- Profile [RD]
- Device [R]
- Third_party_profile_user [C]
- Chrome [R]
- Operation [CRD]
- Event [R]
- User [R]
- Certificate_provisioning_processe [CR]
- Command [CR]
- Notification_config [CRD]
- App [R]
- Report [R]
- Android [R]
- Web [R]

📖 [Full chromemanagement_api documentation](services/chromemanagement_api.md)

### 153. Domains_api

**Resources**: 9

- Registration [CRUD]
- Location [R]
- Operation [R]
- Location [R]
- Operation [R]
- Registration [CRUD]
- Registration [CRUD]
- Location [R]
- Operation [R]

📖 [Full domains_api documentation](services/domains_api.md)

### 154. Spectrum_api

**Resources**: 1

- Paw [C]

📖 [Full spectrum_api documentation](services/spectrum_api.md)

### 155. Walletobjects_api

**Resources**: 20

- Genericclas [CRU]
- Eventticketobject [CRU]
- Transitobject [CRU]
- Loyaltyobject [CRU]
- Offerobject [CRU]
- Genericobject [CRU]
- Giftcardobject [CRU]
- Media [CR]
- Flightobject [CRU]
- Smarttap [C]
- Offerclas [CRU]
- Private_content [C]
- Transitclas [CRU]
- Permission [RU]
- Giftcardclas [CRU]
- Loyaltyclas [CRU]
- Issuer [CRU]
- Eventticketclas [CRU]
- Jwt [C]
- Flightclas [CRU]

📖 [Full walletobjects_api documentation](services/walletobjects_api.md)

### 156. Paymentsresellersubscription_api

**Resources**: 5

- Promotion [CR]
- Line_item [U]
- Product [R]
- User_session [C]
- Subscription [CR]

📖 [Full paymentsresellersubscription_api documentation](services/paymentsresellersubscription_api.md)

### 157. Ml_api

**Resources**: 8

- Operation [CR]
- Job [CRU]
- Trial [CRD]
- Location [R]
- Studie [CRD]
- Project [CR]
- Model [CRUD]
- Version [CRUD]

📖 [Full ml_api documentation](services/ml_api.md)

### 158. People_api

**Resources**: 5

- Other_contact [CR]
- Member [C]
- Connection [R]
- People [CRUD]
- Contact_group [CRUD]

📖 [Full people_api documentation](services/people_api.md)

### 159. Parallelstore_api

**Resources**: 6

- Instance [CRUD]
- Location [R]
- Operation [CRD]
- Instance [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full parallelstore_api documentation](services/parallelstore_api.md)

### 160. Firebaseapphosting_api

**Resources**: 14

- Backend [CRUD]
- Domain [CRUD]
- Location [R]
- Build [CRD]
- Rollout [CR]
- Traffic [RU]
- Operation [CRD]
- Backend [CRUD]
- Build [CRD]
- Domain [CRUD]
- Location [R]
- Rollout [CR]
- Operation [CRD]
- Traffic [RU]

📖 [Full firebaseapphosting_api documentation](services/firebaseapphosting_api.md)

### 161. Workloadmanager_api

**Resources**: 9

- Discoveredprofile [R]
- Scanned_resource [R]
- Operation [CRD]
- Evaluation [CRUD]
- Insight [CD]
- Result [R]
- Location [R]
- Execution [CRD]
- Rule [R]

📖 [Full workloadmanager_api documentation](services/workloadmanager_api.md)

### 162. Groupssettings_api

**Resources**: 1

- Group [RU]

📖 [Full groupssettings_api documentation](services/groupssettings_api.md)

### 163. Tagmanager_api

**Resources**: 30

- Tag [CRUD]
- Version [CRUD]
- Permission [CRUD]
- Account [RU]
- Container [CRUD]
- Environment [CRUD]
- Variable [CRUD]
- Reauthorize_environment [U]
- Move_folder [U]
- Trigger [CRUD]
- Entitie [R]
- Folder [CRUD]
- Destination [CR]
- Template [CRUD]
- Variable [CRUD]
- User_permission [CRUD]
- Account [RU]
- Version [CRUD]
- Tag [CRUD]
- Version_header [R]
- Transformation [CRUD]
- Trigger [CRUD]
- Zone [CRUD]
- Folder [CRUD]
- Container [CRUD]
- Gtag_config [CRUD]
- Environment [CRUD]
- Workspace [CRUD]
- Built_in_variable [CRD]
- Client [CRUD]

📖 [Full tagmanager_api documentation](services/tagmanager_api.md)

### 164. Analyticsdata_api

**Resources**: 4

- Propertie [CR]
- Analyticsdata [C]
- Audience_export [CR]
- Propertie [CR]

📖 [Full analyticsdata_api documentation](services/analyticsdata_api.md)

### 165. Assuredworkloads_api

**Resources**: 8

- Workload [CRUD]
- Violation [CR]
- Update [CR]
- Operation [R]
- Update [CR]
- Violation [CR]
- Workload [CRUD]
- Operation [R]

📖 [Full assuredworkloads_api documentation](services/assuredworkloads_api.md)

### 166. Tasks_api

**Resources**: 2

- Tasklist [CRUD]
- Task [CRUD]

📖 [Full tasks_api documentation](services/tasks_api.md)

### 167. Kgsearch_api

**Resources**: 1

- Entitie [R]

📖 [Full kgsearch_api documentation](services/kgsearch_api.md)

### 168. Resource_named_service

**Resources**: 6

- App [CR]
- Service [RUD]
- Instance [CRD]
- Location [R]
- Operation [R]
- Version [CRUD]

📖 [Full resource_named_service documentation](services/resource_named_service.md)

### 169. Abusiveexperiencereport_api

**Resources**: 2

- Violating_site [R]
- Site [R]

📖 [Full abusiveexperiencereport_api documentation](services/abusiveexperiencereport_api.md)

### 170. Pagespeedonline_api

**Resources**: 4

- Pagespeedapi [R]
- Pagespeedapi [R]
- Pagespeedapi [R]
- Pagespeedapi [R]

📖 [Full pagespeedonline_api documentation](services/pagespeedonline_api.md)

### 171. Dataflow_api

**Resources**: 10

- Message [R]
- Stage [R]
- Snapshot [RD]
- Job [CRU]
- Location [C]
- Flex_template [C]
- Template [CR]
- Work_item [C]
- Debug [C]
- Project [CD]

📖 [Full dataflow_api documentation](services/dataflow_api.md)

### 172. Connectors_api

**Resources**: 23

- Custom_connector_version [CRD]
- Location [RU]
- Event_subscription [CRUD]
- Runtime_action_schema [R]
- Runtime_entity_schema [R]
- Connection [CRUD]
- Custom_connector [CRUD]
- Connector [R]
- Provider [CR]
- Global [RU]
- End_user_authentication [CRUD]
- Connection_schema_metadata [CR]
- Endpoint_attachment [CRUD]
- Operation [CRD]
- Version [R]
- Eventtype [R]
- Managed_zone [CRUD]
- Resource [R]
- Entity_type [R]
- Connection [CR]
- Entitie [CRUD]
- Action [CR]
- Tool [CR]

📖 [Full connectors_api documentation](services/connectors_api.md)

### 173. Cloudscheduler_api

**Resources**: 6

- Operation [CRD]
- Location [RU]
- Job [CRUD]
- Job [CRUD]
- Location [R]
- Operation [CRD]

📖 [Full cloudscheduler_api documentation](services/cloudscheduler_api.md)

### 174. Adexchangebuyer2_api

**Resources**: 19

- Filtered_bid [R]
- Detail [R]
- Client [CRU]
- Invitation [CR]
- Bid_metric [R]
- Finalized_proposal [CR]
- Non_billable_winning_bid [R]
- Bid_responses_without_bid [R]
- Impression_metric [R]
- Losing_bid [R]
- Filter_set [CRD]
- User [RU]
- Product [R]
- Creative [CRU]
- Deal_association [CR]
- Publisher_profile [R]
- Proposal [CRU]
- Bid_response_error [R]
- Filtered_bid_request [R]

📖 [Full adexchangebuyer2_api documentation](services/adexchangebuyer2_api.md)

### 175. Datalineage_api

**Resources**: 5

- Run [CRUD]
- Lineage_event [CRD]
- Operation [CRD]
- Processe [CRUD]
- Location [C]

📖 [Full datalineage_api documentation](services/datalineage_api.md)

### 176. Ids_api

**Resources**: 3

- Endpoint [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full ids_api documentation](services/ids_api.md)

### 177. Securesourcemanager_api

**Resources**: 10

- Instance [CRD]
- Issue [CRUD]
- Issue_comment [CRUD]
- Operation [CRD]
- Pull_request [CRU]
- Location [R]
- Pull_request_comment [CRUD]
- Branch_rule [CRUD]
- Repositorie [CRUD]
- Hook [CRUD]

📖 [Full securesourcemanager_api documentation](services/securesourcemanager_api.md)

### 178. Videointelligence_api

**Resources**: 6

- Operation [CRD]
- Video [C]
- Video [C]
- Video [C]
- Video [C]
- Video [C]

📖 [Full videointelligence_api documentation](services/videointelligence_api.md)

### 179. Ideahub_api

**Resources**: 10

- Topic_state [U]
- Idea_activitie [C]
- Idea_state [U]
- Idea [R]
- Locale [R]
- Idea_activitie [C]
- Idea [R]
- Locale [R]
- Idea_state [U]
- Topic_state [U]

📖 [Full ideahub_api documentation](services/ideahub_api.md)

### 180. Cloudfunctions_api

**Resources**: 18

- Location [R]
- Function [CRUD]
- Operation [R]
- Operation [R]
- Function [CRUD]
- Location [R]
- Location [R]
- Runtime [R]
- Function [CRUD]
- Operation [R]
- Runtime [R]
- Function [CRUD]
- Operation [R]
- Location [R]
- Location [R]
- Operation [R]
- Runtime [R]
- Function [CRUD]

📖 [Full cloudfunctions_api documentation](services/cloudfunctions_api.md)

### 181. Privateca_api

**Resources**: 12

- Certificate_authoritie [CRUD]
- Certificate_revocation_list [CRU]
- Operation [CRD]
- Certificate_template [CRUD]
- Certificate [CRU]
- Location [R]
- Ca_pool [CRUD]
- Certificate_authoritie [CR]
- Location [R]
- Reusable_config [CR]
- Operation [CRD]
- Certificate_revocation_list [CR]

📖 [Full privateca_api documentation](services/privateca_api.md)

### 182. Workstations_api

**Resources**: 9

- Workstation_cluster [CRUD]
- Workstation_config [CRUD]
- Location [R]
- Operation [CRD]
- Workstation [CRUD]
- Operation [CRD]
- Workstation_config [CRUD]
- Workstation_cluster [CRUD]
- Workstation [CRUD]

📖 [Full workstations_api documentation](services/workstations_api.md)

### 183. Smartdevicemanagement_api

**Resources**: 3

- Device [CR]
- Room [R]
- Structure [R]

📖 [Full smartdevicemanagement_api documentation](services/smartdevicemanagement_api.md)

### 184. Composer_api

**Resources**: 12

- Environment [CRUD]
- Workload [R]
- User_workloads_secret [CRUD]
- Operation [RD]
- Image_version [R]
- User_workloads_config_map [CRUD]
- Workload [R]
- Image_version [R]
- User_workloads_config_map [CRUD]
- Operation [RD]
- Environment [CRUD]
- User_workloads_secret [CRUD]

📖 [Full composer_api documentation](services/composer_api.md)

### 185. Cloudbuild_api

**Resources**: 22

- Git_lab_config [CRUD]
- Github_enterprise_config [CRUD]
- Build [CR]
- Trigger [CRUD]
- Location [CR]
- Connected_repositorie [C]
- Repo [R]
- Operation [CR]
- Github_dot_com_webhook [C]
- Cloudbuild [C]
- Worker_pool [CRUD]
- Bitbucket_server_config [CRUD]
- Worker_pool [CRUD]
- Operation [CR]
- Operation [CR]
- Repositorie [CRD]
- Connection [CRUD]
- Location [R]
- Operation [CR]
- Worker_pool [CRUD]
- Operation [CR]
- Worker_pool [CRUD]

📖 [Full cloudbuild_api documentation](services/cloudbuild_api.md)

### 186. Surveys_api

**Resources**: 2

- Result [R]
- Survey [CRUD]

📖 [Full surveys_api documentation](services/surveys_api.md)

### 187. Kmsinventory_api

**Resources**: 2

- Protected_resource [R]
- Crypto_key [R]

📖 [Full kmsinventory_api documentation](services/kmsinventory_api.md)

### 188. Aiplatform_api

**Resources**: 133

- Model [CRUD]
- Metadata_schema [CR]
- Hyperparameter_tuning_job [CRD]
- Training_pipeline [CRD]
- Studie [CRD]
- Feature_view [CRUD]
- Notebook_execution_job [CRD]
- Slice [CR]
- Data_labeling_job [CRD]
- Annotation_spec [R]
- Chat [C]
- Notebook_runtime [CRD]
- Evaluation_set [CRUD]
- Nas_trial_detail [R]
- Invoke [C]
- Openapi [C]
- Evaluation [CR]
- Annotation [R]
- Tensorboard [CRUD]
- Custom_job [CRD]
- Rag_file [CRD]
- Feature_view_sync [R]
- Rag_corpora [CRUD]
- Feature [CRUD]
- Operation [CRD]
- Notebook_runtime_template [CRUD]
- Specialist_pool [CRUD]
- Migratable_resource [C]
- Index_endpoint [CRUD]
- Feature_group [CRUD]
- Artifact [CRUD]
- Media [C]
- Entity_type [CRUD]
- Persistent_resource [CRUD]
- Dataset [CRUD]
- Data_item [R]
- Deployment_resource_pool [CRUD]
- Evaluation_run [CRD]
- Trial [CRD]
- Pipeline_job [CRD]
- Nas_job [CRD]
- Endpoint [CRUD]
- Saved_querie [RD]
- Indexe [CRUD]
- Execution [CRUD]
- Schedule [CRUD]
- Experiment [CRUD]
- Dataset_version [CRUD]
- Run [CRUD]
- Featurestore [CRUD]
- Cached_content [CRUD]
- Batch_prediction_job [CRD]
- Feature_online_store [CRUD]
- Reasoning_engine [CRUD]
- Context [CRUD]
- Location [CRU]
- Project [RU]
- Time_serie [CRUD]
- Tuning_job [CR]
- Model_deployment_monitoring_job [CRUD]
- Metadata_store [CRD]
- Evaluation_item [CRD]
- Event [R]
- Nas_job [CRD]
- Cached_content [CRUD]
- Hyperparameter_tuning_job [CRD]
- Metadata_schema [CR]
- Feature_online_store [CRUD]
- Trial [CRD]
- Extension [CRUD]
- Tuning_job [CR]
- Feature_monitor [CRUD]
- Pipeline_job [CRD]
- Feature_view [CRUD]
- Sandbox_environment [CRD]
- Time_serie [CRUD]
- Evaluation_item [CRD]
- Media [C]
- Model_deployment_monitoring_job [CRUD]
- Dataset_version [CRUD]
- Evaluation_set [CRUD]
- Chat [C]
- Reasoning_engine [CRUD]
- Migratable_resource [C]
- Schedule [CRUD]
- Artifact [CRUD]
- Tensorboard [CRUD]
- Feature_monitor_job [CR]
- Notebook_execution_job [CRD]
- Operation [CRD]
- Model [CRUD]
- Training_pipeline [CRD]
- Model_monitor [CRUD]
- Feature_view_sync [R]
- Context [CRUD]
- Location [CRU]
- Session [CRUD]
- Feature_group [CRUD]
- Model_garden_eula [C]
- Deployment_resource_pool [CRUD]
- Memorie [CRUD]
- Featurestore [CRUD]
- Endpoint [CRUD]
- Experiment [CRUD]
- Persistent_resource [CRUD]
- Custom_job [CRD]
- Saved_querie [RD]
- Evaluation_run [CRD]
- Index_endpoint [CRUD]
- Notebook_runtime_template [CRUD]
- Dataset [CRUD]
- Notebook_runtime [CRD]
- Project [CRU]
- Rag_corpora [CRUD]
- Indexe [CRUD]
- Example_store [CRUD]
- Data_item [R]
- Annotation [R]
- Model_monitoring_job [CRD]
- Execution [CRUD]
- Batch_prediction_job [CRD]
- Run [CRUD]
- Slice [CR]
- Nas_trial_detail [R]
- Specialist_pool [CRUD]
- Data_labeling_job [CRD]
- Feature [CRUD]
- Metadata_store [CRD]
- Studie [CRD]
- Entity_type [CRUD]
- Annotation_spec [R]
- Evaluation [CR]
- Rag_file [CRD]

📖 [Full aiplatform_api documentation](services/aiplatform_api.md)

### 189. Clouderrorreporting_api

**Resources**: 5

- Event [CR]
- Group_stat [R]
- Group [RU]
- Location [D]
- Project [D]

📖 [Full clouderrorreporting_api documentation](services/clouderrorreporting_api.md)

### 190. Searchconsole_api

**Resources**: 5

- Mobile_friendly_test [C]
- Sitemap [RUD]
- Index [C]
- Searchanalytic [C]
- Site [RUD]

📖 [Full searchconsole_api documentation](services/searchconsole_api.md)

### 191. Gamesmanagement_api

**Resources**: 5

- Application [R]
- Achievement [C]
- Player [CD]
- Event [C]
- Score [C]

📖 [Full gamesmanagement_api documentation](services/gamesmanagement_api.md)

### 192. Cloudlocationfinder_api

**Resources**: 4

- Cloud_location [R]
- Location [R]
- Location [R]
- Cloud_location [R]

📖 [Full cloudlocationfinder_api documentation](services/cloudlocationfinder_api.md)

### 193. Gameservices_api

**Resources**: 6

- Location [R]
- Operation [CRD]
- Game_server_deployment [CR]
- Location [R]
- Operation [CRD]
- Game_server_deployment [CR]

📖 [Full gameservices_api documentation](services/gameservices_api.md)

### 194. Airquality_api

**Resources**: 4

- Heatmap_tile [R]
- Current_condition [C]
- History [C]
- Forecast [C]

📖 [Full airquality_api documentation](services/airquality_api.md)

### 195. Chat_api

**Resources**: 10

- Space [CRUD]
- Reaction [CRD]
- Member [CRUD]
- Message [CRUD]
- Space_notification_setting [RU]
- Thread [R]
- Attachment [R]
- Custom_emoji [CRD]
- Space_event [R]
- Media [CR]

📖 [Full chat_api documentation](services/chat_api.md)

### 196. Discovery_api

**Resources**: 1

- Api [R]

📖 [Full discovery_api documentation](services/discovery_api.md)

### 197. Language_api

**Resources**: 4

- Document [C]
- Document [C]
- Document [C]
- Document [C]

📖 [Full language_api documentation](services/language_api.md)

### 198. Servicedirectory_api

**Resources**: 9

- Endpoint [CRUD]
- Namespace [CRUD]
- Location [R]
- Service [CRUD]
- Workload [C]
- Endpoint [CRUD]
- Service [CRUD]
- Location [R]
- Namespace [CRUD]

📖 [Full servicedirectory_api documentation](services/servicedirectory_api.md)

### 199. Dataproc_api

**Resources**: 15

- Session_template [CRUD]
- Operation [CRD]
- Batche [CRD]
- Cluster [CRUD]
- Node_group [CR]
- Job [CRUD]
- Spark_application [CR]
- Workflow_template [CRUD]
- Session [CRD]
- Autoscaling_policie [CRUD]
- Cluster [CRUD]
- Autoscaling_policie [CRUD]
- Workflow_template [CRUD]
- Job [CRUD]
- Operation [CRD]

📖 [Full dataproc_api documentation](services/dataproc_api.md)

### 200. Keep_api

**Resources**: 3

- Media [R]
- Permission [C]
- Note [CRD]

📖 [Full keep_api documentation](services/keep_api.md)

### 201. Repeated_any_query_error

**Resources**: 1

- Entrie [C]

📖 [Full repeated_any_query_error documentation](services/repeated_any_query_error.md)

### 202. Content_api

**Resources**: 52

- Promotion [CR]
- Shoppingadsprogram [CR]
- Collectionstatuse [R]
- Recommendation [CR]
- Pubsubnotificationsetting [RU]
- Po [CRD]
- Collection [CRD]
- Ordertrackingsignal [C]
- Regionalinventory [C]
- Returnaddres [CRD]
- Product [CRUD]
- Datafeed [CRUD]
- Conversionsource [CRUD]
- Returncarrier [CRUD]
- Accountstatuse [CR]
- Csse [CR]
- Region [CRUD]
- Productstatuse [CR]
- Label [CRUD]
- Returnpolicy [CRD]
- Datafeedstatuse [CR]
- Report [C]
- Quota [R]
- Account [CRUD]
- Liasetting [CRU]
- Shippingsetting [CRU]
- Productdeliverytime [CRD]
- Merchantsupport [C]
- Credential [C]
- Returnpolicyonline [CRUD]
- Localinventory [C]
- Freelistingsprogram [CR]
- Checkoutsetting [CRD]
- Accounttax [CRU]
- Orderpayment [C]
- Order [CR]
- Orderreturn [R]
- Orderinvoice [C]
- Shippingsetting [CRU]
- Orderreport [R]
- Liasetting [CRU]
- Orderinvoice [C]
- Datafeedstatuse [CR]
- Accountstatuse [CR]
- Datafeed [CRUD]
- Accounttax [CRU]
- Po [CRD]
- Order [CR]
- Orderreturn [R]
- Account [CRUD]
- Productstatuse [CR]
- Product [CRD]

📖 [Full content_api documentation](services/content_api.md)

### 203. Mybusinessbusinessinformation_api

**Resources**: 5

- Attribute [R]
- Location [CRUD]
- Categorie [R]
- Chain [R]
- Google_location [C]

📖 [Full mybusinessbusinessinformation_api documentation](services/mybusinessbusinessinformation_api.md)

### 204. Fcmdata_api

**Resources**: 1

- Delivery_data [R]

📖 [Full fcmdata_api documentation](services/fcmdata_api.md)

### 205. Customsearch_api

**Resources**: 2

- Cse [R]
- Siterestrict [R]

📖 [Full customsearch_api documentation](services/customsearch_api.md)

### 206. Playdeveloperreporting_api

**Resources**: 24

- Issue [R]
- Anrrate [CR]
- Crashrate [CR]
- Count [CR]
- Slowrenderingrate [CR]
- Stuckbackgroundwakelockrate [CR]
- Slowstartrate [CR]
- Lmkrate [CR]
- Anomalie [R]
- Report [R]
- Excessivewakeuprate [CR]
- App [R]
- Slowstartrate [CR]
- Anomalie [R]
- Crashrate [CR]
- App [R]
- Excessivewakeuprate [CR]
- Lmkrate [CR]
- Count [CR]
- Issue [R]
- Slowrenderingrate [CR]
- Anrrate [CR]
- Stuckbackgroundwakelockrate [CR]
- Report [R]

📖 [Full playdeveloperreporting_api documentation](services/playdeveloperreporting_api.md)

### 207. Vectortile_api

**Resources**: 2

- Terraintile [R]
- Featuretile [R]

📖 [Full vectortile_api documentation](services/vectortile_api.md)

### 208. Compute_api

**Resources**: 371

- Instance_template [CRD]
- Https_health_check [CRUD]
- Region_disk [CRUD]
- Target_pool [CRD]
- Snapshot [CRD]
- Region_instant_snapshot [CRD]
- Instance_group_manager_resize_request [CRD]
- Disk_type [R]
- Region_instance_template [CRD]
- Network_firewall_policie [CRUD]
- Region_health_check [CRUD]
- Reservation_sub_block [CR]
- Storage_pool_type [R]
- Target_https_proxie [CRUD]
- Target_grpc_proxie [CRUD]
- Region_network_endpoint_group [CRD]
- Public_advertised_prefixe [CRUD]
- Preview_feature [RU]
- Region_ssl_policie [CRUD]
- Instance [CRUD]
- Global_addresse [CRD]
- Interconnect_remote_location [R]
- Region_autoscaler [CRUD]
- Wire_group [CRUD]
- Forwarding_rule [CRUD]
- Url_map [CRUD]
- Global_network_endpoint_group [CRD]
- Network_endpoint_group [CRD]
- Service_attachment [CRUD]
- Machine_type [R]
- Router [CRUD]
- Instance_group_manager [CRUD]
- Node_type [R]
- Target_tcp_proxie [CRD]
- Interconnect_attachment [CRUD]
- Instant_snapshot [CRD]
- Autoscaler [CRUD]
- Region_backend_service [CRUD]
- Interconnect_location [R]
- Region_target_https_proxie [CRUD]
- Vpn_tunnel [CRD]
- License_code [CR]
- Global_forwarding_rule [CRUD]
- Region_disk_type [R]
- Target_instance [CRD]
- Region_target_http_proxie [CRD]
- Machine_image [CRD]
- Region_operation [CRD]
- Route [CRD]
- Http_health_check [CRUD]
- External_vpn_gateway [CRD]
- Region_instance [C]
- Region_url_map [CRUD]
- Instance_group [CRD]
- Image_family_view [R]
- Firewall [CRUD]
- Global_public_delegated_prefixe [CRUD]
- Region_zone [R]
- Interconnect_attachment_group [CRUD]
- Organization_security_policie [CRUD]
- Reservation [CRUD]
- Region_health_check_service [CRUD]
- Region_target_tcp_proxie [CRD]
- Disk [CRUD]
- Target_ssl_proxie [CRD]
- Packet_mirroring [CRUD]
- Backend_service [CRUD]
- Region_ssl_certificate [CRD]
- Vpn_gateway [CRD]
- Interconnect [CRUD]
- Subnetwork [CRUD]
- Cross_site_network [CRUD]
- Zone [R]
- Region_network_firewall_policie [CRUD]
- Network_attachment [CRUD]
- Instance_setting [RU]
- Security_policie [CRUD]
- Region [R]
- Interconnect_group [CRUD]
- Target_http_proxie [CRUD]
- Future_reservation [CRUD]
- Region_instance_group [CR]
- Region_instance_group_manager [CRUD]
- Reservation_block [CR]
- Snapshot_setting [RU]
- Accelerator_type [R]
- Project [CR]
- Global_operation [CRD]
- Network_profile [R]
- Ssl_policie [CRUD]
- Node_group [CRUD]
- Image [CRUD]
- Region_security_policie [CRUD]
- Region_commitment [CRU]
- Global_organization_operation [RD]
- Storage_pool [CRUD]
- Region_notification_endpoint [CRD]
- License [CRUD]
- Firewall_policie [CRUD]
- Node_template [CRD]
- Ssl_certificate [CRD]
- Network_edge_security_service [CRUD]
- Addresse [CRD]
- Resource_policie [CRUD]
- Target_vpn_gateway [CRD]
- Zone_operation [CRD]
- Backend_bucket [CRUD]
- Network [CRUD]
- Public_delegated_prefixe [CRUD]
- Health_check [CRUD]
- Region_ssl_certificate [CRD]
- Project [CR]
- Region_health_check [CRUD]
- Global_organization_operation [RD]
- Storage_pool [CRUD]
- Snapshot_group [CRD]
- Region_disk_setting [RU]
- Target_http_proxie [CRUD]
- Region_commitment [CRU]
- Network_profile [R]
- Instance_group [CRD]
- Region_instance [C]
- License_code [CR]
- Addresse [CRD]
- Machine_type [R]
- Router [CRUD]
- Network [CRUD]
- Public_advertised_prefixe [CRUD]
- Region_health_aggregation_policie [CRUD]
- Instance_group_manager_resize_request [CRD]
- Backend_service [CRUD]
- Region_network_endpoint_group [CRD]
- Zone_operation [CRD]
- Region_backend_bucket [CRUD]
- Global_folder_operation [R]
- Global_operation [CRD]
- Region_operation [CRD]
- Advice [C]
- Global_public_delegated_prefixe [CRUD]
- Instance_group_manager [CRUD]
- Region_url_map [CRUD]
- Recoverable_snapshot [CRD]
- Subnetwork [CRUD]
- Region_target_http_proxie [CRD]
- Autoscaler [CRUD]
- Interconnect_group [CRUD]
- Wire_group [CRUD]
- Region_snapshot [CRD]
- Ssl_certificate [CRD]
- Region_multi_mig [CRD]
- Region_instance_group [CR]
- Image [CRUD]
- Zone_organization_operation [R]
- Global_vm_extension_policie [CRUD]
- Vpn_tunnel [CRD]
- Cross_site_network [CRUD]
- Snapshot_setting [RU]
- Preview_feature [RU]
- Https_health_check [CRUD]
- Public_delegated_prefixe [CRUD]
- Url_map [CRUD]
- Network_attachment [CRUD]
- Instance_setting [RU]
- Target_pool [CRD]
- Instance_template [CRD]
- Firewall [CRUD]
- Ssl_policie [CRUD]
- Region_notification_endpoint [CRD]
- Machine_image [CRD]
- Region_ssl_policie [CRUD]
- Region_network_policie [CRUD]
- Image_family_view [R]
- Region_target_tcp_proxie [CRD]
- Zone_queued_resource [CRD]
- Zone_vm_extension_policie [CRUD]
- Target_grpc_proxie [CRUD]
- Rollout_plan [CRD]
- Target_https_proxie [CRUD]
- Ha_controller [CRUD]
- Region_snapshot_setting [RU]
- Http_health_check [CRUD]
- Disk_setting [RU]
- Health_check [CRUD]
- Firewall_policie [CRUD]
- Region_instant_snapshot_group [CRD]
- Region_instance_template [CRD]
- Region_disk_type [R]
- Region_instant_snapshot [CRD]
- Node_type [R]
- Reservation_sub_block [CR]
- Target_instance [CRD]
- Instant_snapshot [CRD]
- Snapshot [CRD]
- Zone [R]
- Instant_snapshot_group [CRD]
- External_vpn_gateway [CRD]
- Reservation_block [CR]
- Region_target_https_proxie [CRUD]
- Node_template [CRD]
- Region_security_policie [CRUD]
- Resource_policie [CRUD]
- Region_autoscaler [CRUD]
- Disk_type [R]
- Interconnect_location [R]
- Region [R]
- Network_firewall_policie [CRUD]
- Zone_folder_operation [R]
- Node_group [CRUD]
- Accelerator_type [R]
- Service_attachment [CRUD]
- Storage_pool_type [R]
- Region_multi_mig_member [R]
- Network_edge_security_service [CRUD]
- Interconnect_remote_location [R]
- Packet_mirroring [CRUD]
- Region_health_check_service [CRUD]
- Global_addresse [CRD]
- Future_reservation [CRUD]
- Instance [CRUD]
- Global_network_endpoint_group [CRD]
- Region_instance_group_manager_resize_request [CRD]
- Interconnect_attachment_group [CRUD]
- Target_tcp_proxie [CRD]
- Backend_bucket [CRUD]
- Disk [CRUD]
- Global_forwarding_rule [CRUD]
- Target_ssl_proxie [CRD]
- Network_endpoint_group [CRD]
- Reservation [CRUD]
- Rollout [RUD]
- License [CRUD]
- Region_instance_group_manager [CRUD]
- Interconnect [CRUD]
- Region_health_source [CRUD]
- Region_zone [R]
- Vpn_gateway [CRD]
- Forwarding_rule [CRUD]
- Region_disk [CRUD]
- Security_policie [CRUD]
- Region_backend_service [CRUD]
- Region_network_firewall_policie [CRUD]
- Route [CRD]
- Region_composite_health_check [CRUD]
- Target_vpn_gateway [CRD]
- Reliability_risk [R]
- Organization_security_policie [CRUD]
- Interconnect_attachment [CRUD]
- Region_notification_endpoint [CRD]
- Region_disk_type [R]
- Instance_group [CRD]
- Instant_snapshot [CRD]
- Resource_policie [CRUD]
- Ssl_certificate [CRD]
- License [CRUD]
- Region_url_map [CRUD]
- Region_backend_service [CRUD]
- Region_composite_health_check [CRUD]
- Project [CR]
- Instance_setting [RU]
- Autoscaler [CRUD]
- Region_health_check [CRUD]
- Region_instance_group_manager_resize_request [CRD]
- Region_operation [CRD]
- License_code [CR]
- Region_network_firewall_policie [CRUD]
- Region_disk [CRUD]
- Region_instance_group [CR]
- Snapshot_setting [RU]
- Firewall [CRUD]
- Network_profile [R]
- Region_ssl_certificate [CRD]
- Global_addresse [CRD]
- Interconnect_attachment [CRUD]
- Region_instance [C]
- Target_instance [CRD]
- Security_policie [CRUD]
- Public_delegated_prefixe [CRUD]
- Zone_operation [CRD]
- Disk_setting [RU]
- Region_health_source [CRUD]
- Accelerator_type [R]
- Region_commitment [CRU]
- Backend_service [CRUD]
- Storage_pool [CRUD]
- Ssl_policie [CRUD]
- Https_health_check [CRUD]
- Storage_pool_type [R]
- Network_firewall_policie [CRUD]
- Interconnect_group [CRUD]
- Instance_template [CRD]
- Interconnect [CRUD]
- Node_template [CRD]
- Zone [R]
- Region_snapshot_setting [RU]
- Instance_group_manager [CRUD]
- Forwarding_rule [CRUD]
- Health_check [CRUD]
- Target_https_proxie [CRUD]
- Region_target_http_proxie [CRD]
- Region_instance_template [CRD]
- Region_health_check_service [CRUD]
- Network_endpoint_group [CRD]
- Organization_security_policie [CRUD]
- Preview_feature [RU]
- Global_organization_operation [RD]
- Router [CRUD]
- Snapshot [CRD]
- Node_group [CRUD]
- Network_edge_security_service [CRUD]
- Target_pool [CRD]
- External_vpn_gateway [CRD]
- Region_health_aggregation_policie [CRUD]
- Disk_type [R]
- Image_family_view [R]
- Global_network_endpoint_group [CRD]
- Reservation [CRUD]
- Region_instant_snapshot [CRD]
- Network_attachment [CRUD]
- Interconnect_attachment_group [CRUD]
- Region_security_policie [CRUD]
- Vpn_tunnel [CRD]
- Region_backend_bucket [CRUD]
- Region_snapshot [CRD]
- Reservation_sub_block [CR]
- Region_target_tcp_proxie [CRD]
- Region_zone [R]
- Zone_vm_extension_policie [CRUD]
- Vpn_gateway [CRD]
- Wire_group [CRUD]
- Cross_site_network [CRUD]
- Target_grpc_proxie [CRUD]
- Region_network_policie [CRUD]
- Region_multi_mig [CRD]
- Target_http_proxie [CRUD]
- Service_attachment [CRUD]
- Region_target_https_proxie [CRUD]
- Global_vm_extension_policie [R]
- Addresse [CRD]
- Reservation_block [CR]
- Instance_group_manager_resize_request [CRD]
- Public_advertised_prefixe [CRUD]
- Interconnect_remote_location [R]
- Url_map [CRUD]
- Machine_type [R]
- Target_vpn_gateway [CRD]
- Node_type [R]
- Global_operation [CRD]
- Packet_mirroring [CRUD]
- Machine_image [CRD]
- Target_ssl_proxie [CRD]
- Subnetwork [CRUD]
- Backend_bucket [CRUD]
- Instance [CRUD]
- Global_public_delegated_prefixe [CRUD]
- Disk [CRUD]
- Region_autoscaler [CRUD]
- Region [R]
- Target_tcp_proxie [CRD]
- Global_forwarding_rule [CRUD]
- Route [CRD]
- Advice [C]
- Network [CRUD]
- Region_disk_setting [RU]
- Firewall_policie [CRUD]
- Region_instance_group_manager [CRUD]
- Http_health_check [CRUD]
- Region_ssl_policie [CRUD]
- Region_network_endpoint_group [CRD]
- Future_reservation [CRUD]
- Image [CRUD]
- Interconnect_location [R]

📖 [Full compute_api documentation](services/compute_api.md)

### 209. Monitoring_api

**Resources**: 22

- Metadata [R]
- V1 [C]
- Metrics_scope [R]
- Project [CD]
- Dashboard [CRUD]
- Operation [R]
- Label [R]
- Member [R]
- Alert_policie [CRUD]
- Service_level_objective [CRUD]
- Notification_channel_descriptor [R]
- Uptime_check_config [CRUD]
- Service [CRUD]
- Group [CRUD]
- Time_serie [CR]
- Uptime_check_ip [R]
- Metric_descriptor [CRD]
- Alert [R]
- Collectd_time_serie [C]
- Snooze [CRU]
- Notification_channel [CRUD]
- Monitored_resource_descriptor [R]

📖 [Full monitoring_api documentation](services/monitoring_api.md)

### 210. Sql_api

**Resources**: 8

- Ssl_cert [CRD]
- User [CRUD]
- Database [CRUD]
- Instance [CRUD]
- Tier [R]
- Flag [R]
- Operation [R]
- Backup_run [CRD]

📖 [Full sql_api documentation](services/sql_api.md)

### 211. Deploymentmanager_api

**Resources**: 19

- Type [R]
- Type_provider [CRUD]
- Operation [R]
- Manifest [R]
- Resource [R]
- Deployment [CRUD]
- Composite_type [CRUD]
- Operation [R]
- Manifest [R]
- Resource [R]
- Type [R]
- Deployment [CRUD]
- Composite_type [CRUD]
- Type_provider [CRUD]
- Deployment [CRUD]
- Manifest [R]
- Resource [R]
- Operation [R]
- Type [R]

📖 [Full deploymentmanager_api documentation](services/deploymentmanager_api.md)

### 212. Serviceuser_api

**Resources**: 1

- Service [CR]

📖 [Full serviceuser_api documentation](services/serviceuser_api.md)

### 213. Publicca_api

**Resources**: 3

- External_account_key [C]
- External_account_key [C]
- External_account_key [C]

📖 [Full publicca_api documentation](services/publicca_api.md)

### 214. Orgpolicy_api

**Resources**: 3

- Custom_constraint [CRUD]
- Constraint [R]
- Policie [CRUD]

📖 [Full orgpolicy_api documentation](services/orgpolicy_api.md)

### 215. Dfareporting_api

**Resources**: 506

- Inventory_item [R]
- Targeting_template [CRU]
- Compatible_field [C]
- Dimension_value [C]
- Video_format [R]
- Browser [R]
- Floodlight_activitie [CRUD]
- Order [R]
- User_profile [R]
- Connection_type [R]
- Account_user_profile [CRU]
- Account_permission [R]
- Advertiser_group [CRUD]
- Content_categorie [CRUD]
- Metro [R]
- Creative [CRU]
- Language [R]
- Creative_group [CRU]
- Landing_page [CRUD]
- Report [CRUD]
- User_role_permission_group [R]
- Account [RU]
- Order_document [R]
- Dynamic_targeting_key [CRD]
- Account_permission_group [R]
- Citie [R]
- Directory_site_contact [R]
- Campaign_creative_association [CR]
- Creative_field_value [CRUD]
- File [R]
- Campaign [CRU]
- Size [CR]
- Account_active_ad_summarie [R]
- Operating_system_version [R]
- Site [CRU]
- Directory_site [CR]
- Placement [CRU]
- Change_log [R]
- Operating_system [R]
- Creative_asset [C]
- Creative_field [CRUD]
- Region [R]
- Conversion [C]
- Remarketing_list_share [RU]
- Placement_group [CRU]
- Project [R]
- Floodlight_activity_group [CRU]
- Targetable_remarketing_list [R]
- Countrie [R]
- Floodlight_configuration [RU]
- Event_tag [CRUD]
- Platform_type [R]
- Subaccount [CRU]
- Advertiser [CRU]
- Placement_strategie [CRUD]
- User_role_permission [R]
- Postal_code [R]
- User_role [CRUD]
- Mobile_carrier [R]
- Remarketing_list [CRU]
- Ad [CRU]
- Targetable_remarketing_list [R]
- Account_permission [R]
- Platform_type [R]
- Order [R]
- Creative_asset [C]
- Browser [R]
- Project [R]
- Site [CRU]
- Content_categorie [CRUD]
- Operating_system [R]
- Floodlight_activitie [CRUD]
- Report [CRUD]
- Mobile_carrier [R]
- Subaccount [CRU]
- Dimension_value [C]
- Creative [CRU]
- Advertiser_landing_page [CRU]
- Creative_field_value [CRUD]
- User_role_permission_group [R]
- Inventory_item [R]
- Account_active_ad_summarie [R]
- Campaign_creative_association [CR]
- Account_permission_group [R]
- Placement_strategie [CRUD]
- Remarketing_list [CRU]
- Advertiser [CRU]
- Postal_code [R]
- Placement [CRU]
- Metro [R]
- Connection_type [R]
- User_role [CRUD]
- Ad [CRU]
- Account_user_profile [CRU]
- Advertiser_group [CRUD]
- Conversion [C]
- Account [RU]
- Mobile_app [R]
- Targeting_template [CRU]
- Countrie [R]
- Order_document [R]
- Creative_group [CRU]
- Placement_group [CRU]
- Language [R]
- Region [R]
- User_profile [R]
- Creative_field [CRUD]
- Compatible_field [C]
- Floodlight_activity_group [CRU]
- Citie [R]
- File [R]
- Size [CR]
- Change_log [R]
- Dynamic_targeting_key [CRD]
- Floodlight_configuration [RU]
- Event_tag [CRUD]
- Video_format [R]
- Directory_site [CR]
- Operating_system_version [R]
- User_role_permission [R]
- Campaign [CRU]
- Remarketing_list_share [RU]
- Metro [R]
- Conversion [C]
- Dynamic_targeting_key [CRD]
- Advertiser_group [CRUD]
- Account_permission_group [R]
- Creative_group [CRU]
- Campaign [CRU]
- Order [R]
- Project [R]
- Size [CR]
- Change_log [R]
- Account_permission [R]
- Placement_strategie [CRUD]
- Video_format [R]
- User_profile [R]
- Floodlight_configuration [RU]
- Connection_type [R]
- User_role_permission_group [R]
- Postal_code [R]
- Directory_site [CR]
- Site [CRU]
- Creative_field_value [CRUD]
- Report [CRUD]
- Custom_event [C]
- File [R]
- Event_tag [CRUD]
- Mobile_app [R]
- Targetable_remarketing_list [R]
- Platform_type [R]
- User_role [CRUD]
- Remarketing_list_share [RU]
- Operating_system_version [R]
- Countrie [R]
- Account_user_profile [CRU]
- Content_categorie [CRUD]
- Campaign_creative_association [CR]
- Account [RU]
- Region [R]
- Placement_group [CRU]
- Operating_system [R]
- Creative_field [CRUD]
- Placement [CRU]
- Subaccount [CRU]
- Citie [R]
- Advertiser [CRU]
- Floodlight_activitie [CRUD]
- Ad [CRU]
- Compatible_field [C]
- Browser [R]
- Creative [CRU]
- Language [R]
- Floodlight_activity_group [CRU]
- Inventory_item [R]
- Dimension_value [C]
- Order_document [R]
- Creative_asset [C]
- Advertiser_landing_page [CRU]
- Mobile_carrier [R]
- Account_active_ad_summarie [R]
- User_role_permission [R]
- Targeting_template [CRU]
- Remarketing_list [CRU]
- Media [C]
- Event_tag [CRUD]
- Conversion [C]
- Directory_site_contact [R]
- Placement_strategie [CRUD]
- Dynamic_targeting_key [CRD]
- User_role_permission [R]
- Inventory_item [R]
- Report [CRUD]
- Mobile_carrier [R]
- Content_categorie [CRUD]
- Remarketing_list_share [RU]
- Citie [R]
- Campaign_creative_association [CR]
- Advertiser_group [CRUD]
- Floodlight_activity_group [CRU]
- Campaign [CRU]
- Floodlight_configuration [RU]
- Account_user_profile [CRU]
- Subaccount [CRU]
- Operating_system [R]
- Creative_field [CRUD]
- File [R]
- Size [CR]
- Change_log [R]
- Creative_group [CRU]
- Creative_asset [C]
- Targetable_remarketing_list [R]
- Account_permission_group [R]
- Order_document [R]
- Dimension_value [C]
- Creative [CRU]
- Metro [R]
- Countrie [R]
- Platform_type [R]
- Browser [R]
- Advertiser_landing_page [CRU]
- Connection_type [R]
- Account_permission [R]
- Account_active_ad_summarie [R]
- Advertiser [CRU]
- User_role [CRUD]
- Video_format [R]
- Region [R]
- Site [CRU]
- Order [R]
- Language [R]
- Directory_site [CR]
- Placement [CRU]
- Project [R]
- Operating_system_version [R]
- Account [RU]
- Compatible_field [C]
- Remarketing_list [CRU]
- Postal_code [R]
- User_profile [R]
- Placement_group [CRU]
- Creative_field_value [CRUD]
- User_role_permission_group [R]
- Targeting_template [CRU]
- Ad [CRU]
- Floodlight_activitie [CRUD]
- Mobile_app [R]
- Dimension_value [C]
- Creative_field [CRUD]
- Directory_site [CR]
- Placement_strategie [CRUD]
- Site [CRU]
- Account_permission [R]
- Tv_campaign_detail [R]
- Mobile_carrier [R]
- Content_categorie [CRUD]
- Billing_assignment [CR]
- Change_log [R]
- Floodlight_activity_group [CRU]
- Account_permission_group [R]
- Event_tag [CRUD]
- Creative_field_value [CRUD]
- Advertiser_group [CRUD]
- Region [R]
- Connection_type [R]
- Billing_profile [RU]
- Dynamic_profile [CRU]
- Citie [R]
- Account [RU]
- Report [CRUD]
- Compatible_field [C]
- User_role_permission_group [R]
- Dynamic_targeting_key [CRD]
- Creative_asset [C]
- Targeting_template [CRU]
- User_role_permission [R]
- Size [CR]
- Advertiser_invoice [R]
- Video_format [R]
- Tv_campaign_summarie [R]
- Browser [R]
- Account_user_profile [CRU]
- Postal_code [R]
- Remarketing_list_share [RU]
- Placement_group [CRU]
- Creative_group [CRU]
- Campaign_creative_association [CR]
- Floodlight_activitie [CRUD]
- Studio_creative [CR]
- Remarketing_list [CRU]
- Account_active_ad_summarie [R]
- Language [R]
- User_profile [R]
- Mobile_app [R]
- File [R]
- Operating_system [R]
- Conversion [C]
- Countrie [R]
- Subaccount [CRU]
- Billing_rate [R]
- Advertiser_landing_page [CRU]
- Campaign [CRU]
- Platform_type [R]
- Creative [CRU]
- User_role [CRUD]
- Dynamic_feed [CRU]
- Floodlight_configuration [RU]
- Metro [R]
- Placement [CRU]
- Advertiser [CRU]
- Targetable_remarketing_list [R]
- Studio_creative_asset [C]
- Operating_system_version [R]
- Ad [CRU]
- Site [CRU]
- Operating_system [R]
- Dynamic_feed [CRU]
- Ad [CRU]
- Content_categorie [CRUD]
- Billing_profile [RU]
- Creative [CRU]
- Dynamic_profile [CRU]
- Campaign_creative_association [CR]
- Connection_type [R]
- Placement_group [CRU]
- Directory_site [CR]
- Billing_rate [R]
- Subaccount [CRU]
- Floodlight_configuration [RU]
- Report [CRUD]
- Mobile_carrier [R]
- Targeting_template [CRU]
- Countrie [R]
- Placement_strategie [CRUD]
- Account_active_ad_summarie [R]
- Billing_assignment [CR]
- Remarketing_list_share [RU]
- Inventory_item [R]
- Floodlight_activity_group [CRU]
- Creative_asset [C]
- Citie [R]
- Dynamic_targeting_key [CRD]
- User_role_permission [R]
- Project [R]
- Account_permission_group [R]
- Creative_group [CRU]
- Advertiser [CRU]
- User_role_permission_group [R]
- Advertiser_invoice [R]
- Mobile_app [R]
- Studio_creative_asset [C]
- Placement [CRU]
- Floodlight_activitie [CRUD]
- Region [R]
- Video_format [R]
- Creative_field [CRUD]
- Order [R]
- Tv_campaign_detail [R]
- Change_log [R]
- Account_permission [R]
- Advertiser_landing_page [CRU]
- User_profile [R]
- Browser [R]
- Account [RU]
- Compatible_field [C]
- Operating_system_version [R]
- Campaign [CRU]
- Targetable_remarketing_list [R]
- Language [R]
- Metro [R]
- Conversion [C]
- Platform_type [R]
- Account_user_profile [CRU]
- Postal_code [R]
- Dimension_value [C]
- File [R]
- User_role [CRUD]
- Event_tag [CRUD]
- Tv_campaign_summarie [R]
- Remarketing_list [CRU]
- Advertiser_group [CRUD]
- Creative_field_value [CRUD]
- Size [CR]
- Studio_creative [CR]
- Subaccount [CRU]
- Content_categorie [CRUD]
- Dimension_value [C]
- Size [CR]
- Creative_asset [C]
- User_role [CRUD]
- Platform_type [R]
- Campaign [CRU]
- Advertiser_landing_page [CRU]
- Language [R]
- Region [R]
- Operating_system_version [R]
- Account [RU]
- User_role_permission [R]
- Directory_site_contact [R]
- Inventory_item [R]
- Dynamic_targeting_key [CRD]
- Campaign_creative_association [CR]
- Account_user_profile [CRU]
- Video_format [R]
- Site [CRU]
- Creative_group [CRU]
- Connection_type [R]
- Advertiser [CRU]
- Project [R]
- Account_active_ad_summarie [R]
- Remarketing_list_share [RU]
- Postal_code [R]
- Operating_system [R]
- Ad [CRU]
- Conversion [C]
- Placement_strategie [CRUD]
- File [R]
- Browser [R]
- Account_permission_group [R]
- Targeting_template [CRU]
- Change_log [R]
- Compatible_field [C]
- Targetable_remarketing_list [R]
- Citie [R]
- Order [R]
- Countrie [R]
- Placement_group [CRU]
- Placement [CRU]
- Metro [R]
- Event_tag [CRUD]
- Directory_site [CR]
- User_role_permission_group [R]
- Creative_field [CRUD]
- Advertiser_group [CRUD]
- Floodlight_activity_group [CRU]
- Order_document [R]
- User_profile [R]
- Floodlight_configuration [RU]
- Report [CRUD]
- Creative [CRU]
- Mobile_carrier [R]
- Creative_field_value [CRUD]
- Remarketing_list [CRU]
- Account_permission [R]
- Floodlight_activitie [CRUD]
- Directory_site [CR]
- Language [R]
- Operating_system [R]
- Dynamic_targeting_key [CRD]
- Metro [R]
- Citie [R]
- Remarketing_list_share [RU]
- Video_format [R]
- Creative_field_value [CRUD]
- Remarketing_list [CRU]
- Account_permission [R]
- Compatible_field [C]
- Project [R]
- User_role [CRUD]
- Floodlight_activity_group [CRU]
- Connection_type [R]
- Account_permission_group [R]
- Countrie [R]
- Advertiser_landing_page [CRU]
- Floodlight_configuration [RU]
- Placement [CRU]
- Placement_group [CRU]
- Event_tag [CRUD]
- Report [CRUD]
- Account_active_ad_summarie [R]
- File [R]
- Targetable_remarketing_list [R]
- Operating_system_version [R]
- Creative_field [CRUD]
- User_role_permission [R]
- Order_document [R]
- Platform_type [R]
- Change_log [R]
- Order [R]
- Creative_asset [C]
- User_profile [R]
- Ad [CRU]
- Conversion [C]
- Floodlight_activitie [CRUD]
- Advertiser [CRU]
- User_role_permission_group [R]
- Size [CR]
- Region [R]
- Account [RU]
- Targeting_template [CRU]
- Inventory_item [R]
- Account_user_profile [CRU]
- Postal_code [R]
- Browser [R]
- Advertiser_group [CRUD]
- Campaign [CRU]
- Creative [CRU]
- Content_categorie [CRUD]
- Creative_group [CRU]
- Mobile_carrier [R]
- Dimension_value [C]
- Site [CRU]
- Placement_strategie [CRUD]
- Mobile_app [R]
- Campaign_creative_association [CR]
- Subaccount [CRU]

📖 [Full dfareporting_api documentation](services/dfareporting_api.md)

### 216. Oauth2_api

**Resources**: 6

- Oauth2 [C]
- Userinfo [R]
- Me [R]
- Userinfo [R]
- Me [R]
- Oauth2 [C]

📖 [Full oauth2_api documentation](services/oauth2_api.md)

### 217. Cloudbilling_api

**Resources**: 9

- Sub_account [CR]
- Billing_account [CRU]
- Service [R]
- Sku [R]
- Project [RU]
- Price [R]
- Sku [R]
- Sku_group [R]
- Service [R]

📖 [Full cloudbilling_api documentation](services/cloudbilling_api.md)

### 218. Iamcredentials_api

**Resources**: 1

- Service_account [C]

📖 [Full iamcredentials_api documentation](services/iamcredentials_api.md)

### 219. Apim_api

**Resources**: 6

- Observation_source [CRD]
- Observation_job [CRD]
- Api_observation [CR]
- Location [R]
- Api_operation [R]
- Operation [CRD]

📖 [Full apim_api documentation](services/apim_api.md)

### 220. Replicapoolupdater_api

**Resources**: 2

- Rolling_update [CR]
- Zone_operation [R]

📖 [Full replicapoolupdater_api documentation](services/replicapoolupdater_api.md)

### 221. Chromeuxreport_api

**Resources**: 1

- Record [C]

📖 [Full chromeuxreport_api documentation](services/chromeuxreport_api.md)

### 222. Dataportability_api

**Resources**: 8

- Authorization [C]
- Archive_job [CR]
- Portability_archive [C]
- Access_type [C]
- Archive_job [CR]
- Portability_archive [C]
- Access_type [C]
- Authorization [C]

📖 [Full dataportability_api documentation](services/dataportability_api.md)

### 223. Drivelabels_api

**Resources**: 12

- Revision [U]
- Permission [CRD]
- User [R]
- Limit [R]
- Label [CRUD]
- Lock [R]
- Permission [CRD]
- Revision [U]
- User [R]
- Limit [R]
- Label [CRUD]
- Lock [R]

📖 [Full drivelabels_api documentation](services/drivelabels_api.md)

### 224. Storagebatchoperations_api

**Resources**: 3

- Operation [CRD]
- Job [CRD]
- Location [R]

📖 [Full storagebatchoperations_api documentation](services/storagebatchoperations_api.md)

### 225. Containeranalysis_api

**Resources**: 10

- Resource [C]
- Note [CRUD]
- Occurrence [CRUD]
- Occurrence [CRUD]
- Resource [C]
- Note [CRUD]
- Note [CRUD]
- Scan_config [RU]
- Occurrence [CRUD]
- Operation [CU]

📖 [Full containeranalysis_api documentation](services/containeranalysis_api.md)

### 226. Fitness_api

**Resources**: 4

- Data_source [CRUD]
- Data_point_change [R]
- Dataset [CRUD]
- Session [RUD]

📖 [Full fitness_api documentation](services/fitness_api.md)

### 227. Businessprofileperformance_api

**Resources**: 2

- Monthly [R]
- Location [R]

📖 [Full businessprofileperformance_api documentation](services/businessprofileperformance_api.md)

### 228. Plus_api

**Resources**: 3

- Comment [R]
- People [R]
- Activitie [R]

📖 [Full plus_api documentation](services/plus_api.md)

### 229. Manufacturers_api

**Resources**: 2

- Product [RUD]
- Product_certification [RUD]

📖 [Full manufacturers_api documentation](services/manufacturers_api.md)

### 230. Realtimebidding_api

**Resources**: 8

- Pretargeting_config [CRUD]
- Creative [CRU]
- Bidder [R]
- Publisher_connection [CR]
- Buyer [R]
- User_list [CRU]
- Endpoint [RU]
- Bidding_function [CR]

📖 [Full realtimebidding_api documentation](services/realtimebidding_api.md)

### 231. Datamanager_api

**Resources**: 3

- Request_statu [R]
- Event [C]
- Audience_member [C]

📖 [Full datamanager_api documentation](services/datamanager_api.md)

### 232. Datamigration_api

**Resources**: 12

- Location [R]
- Connection_profile [CRUD]
- Object [CR]
- Migration_job [CRUD]
- Operation [CRD]
- Private_connection [CRD]
- Conversion_workspace [CRUD]
- Mapping_rule [CRD]
- Connection_profile [CRUD]
- Operation [CRD]
- Location [R]
- Migration_job [CRUD]

📖 [Full datamigration_api documentation](services/datamigration_api.md)

### 233. Firebasestorage_api

**Resources**: 3

- Project [RD]
- Bucket [CR]
- Default_bucket [C]

📖 [Full firebasestorage_api documentation](services/firebasestorage_api.md)

### 234. Memcache_api

**Resources**: 6

- Location [R]
- Instance [CRUD]
- Operation [CRD]
- Instance [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full memcache_api documentation](services/memcache_api.md)

### 235. Alertcenter_api

**Resources**: 3

- Feedback [CR]
- Alertcenter [RU]
- Alert [CRD]

📖 [Full alertcenter_api documentation](services/alertcenter_api.md)

### 236. Biglake_api

**Resources**: 3

- Database [CRUD]
- Catalog [CRD]
- Table [CRUD]

📖 [Full biglake_api documentation](services/biglake_api.md)

### 237. Policyanalyzer_api

**Resources**: 2

- Activitie [R]
- Activitie [R]

📖 [Full policyanalyzer_api documentation](services/policyanalyzer_api.md)

### 238. Sts_api

**Resources**: 2

- St [C]
- St [C]

📖 [Full sts_api documentation](services/sts_api.md)

### 239. Authorizedbuyersmarketplace_api

**Resources**: 16

- Deal [CRU]
- User [CRD]
- Client [CRU]
- Proposal [CRU]
- Publisher_profile [R]
- Finalized_deal [CR]
- Auction_package [CR]
- Auction_package [CR]
- Client [CRU]
- Publisher_profile [R]
- Finalized_deal [CR]
- Proposal [CRU]
- Deal [CRU]
- User [CRD]
- Data_segment [CRU]
- Data_segment [CRU]

📖 [Full authorizedbuyersmarketplace_api documentation](services/authorizedbuyersmarketplace_api.md)

### 240. Contentwarehouse_api

**Resources**: 9

- Location [CR]
- Document [CRUD]
- Reference_id [RUD]
- Synonym_set [CRUD]
- Document_link [CD]
- Document_schema [CRUD]
- Rule_set [CRUD]
- Operation [R]
- Project [C]

📖 [Full contentwarehouse_api documentation](services/contentwarehouse_api.md)

### 241. Vault_api

**Resources**: 6

- Hold [CRUD]
- Operation [CRD]
- Saved_querie [CRD]
- Export [CRD]
- Account [CRD]
- Matter [CRUD]

📖 [Full vault_api documentation](services/vault_api.md)

### 242. Sqladmin_api

**Resources**: 20

- Tier [R]
- Backup_run [CRD]
- Flag [R]
- Backup [CRUD]
- Instance [CRUD]
- Ssl_cert [CRD]
- Operation [CR]
- Connect [CR]
- Database [CRUD]
- User [CRUD]
- Instance [CRUD]
- User [CRUD]
- Backup [CRUD]
- Tier [R]
- Database [CRUD]
- Backup_run [CRD]
- Connect [CR]
- Ssl_cert [CRD]
- Flag [R]
- Operation [CR]

📖 [Full sqladmin_api documentation](services/sqladmin_api.md)

### 243. Testing_api

**Resources**: 4

- Test_matrice [CR]
- Device_session [CRU]
- Test_environment_catalog [R]
- Application_detail_service [C]

📖 [Full testing_api documentation](services/testing_api.md)

### 244. Networksecurity_api

**Resources**: 51

- Mirroring_endpoint_group [CRUD]
- Security_profile_group [CRUD]
- Authorization_policie [CRUD]
- Mirroring_deployment_group [CRUD]
- Operation [CRD]
- Mirroring_deployment [CRUD]
- Rule [CRUD]
- Server_tls_policie [CRUD]
- Intercept_deployment [CRUD]
- Location [R]
- Url_list [CRUD]
- Firewall_endpoint_association [CRUD]
- Mirroring_endpoint_group_association [CRUD]
- Backend_authentication_config [CRUD]
- Intercept_endpoint_group_association [CRUD]
- Gateway_security_policie [CRUD]
- Client_tls_policie [CRUD]
- Intercept_deployment_group [CRUD]
- Address_group [CRUD]
- Security_profile [CRUD]
- Firewall_endpoint [CRUD]
- Authz_policie [CRUD]
- Tls_inspection_policie [CRUD]
- Intercept_endpoint_group [CRUD]
- Url_list [CRUD]
- Client_tls_policie [CRUD]
- Backend_authentication_config [CRUD]
- Location [R]
- Mirroring_endpoint_group_association [CRUD]
- Sac_realm [CRD]
- Intercept_endpoint_group [CRUD]
- Mirroring_deployment [CRUD]
- Rule [CRUD]
- Authorization_policie [CRUD]
- Tls_inspection_policie [CRUD]
- Server_tls_policie [CRUD]
- Gateway_security_policie [CRUD]
- Intercept_deployment_group [CRUD]
- Dns_threat_detector [CRUD]
- Firewall_endpoint_association [CRUD]
- Intercept_endpoint_group_association [CRUD]
- Operation [CRD]
- Security_profile_group [CRUD]
- Mirroring_deployment_group [CRUD]
- Intercept_deployment [CRUD]
- Address_group [CRUD]
- Firewall_endpoint [CRUD]
- Security_profile [CRUD]
- Authz_policie [CRUD]
- Sac_attachment [CRD]
- Mirroring_endpoint_group [CRUD]

📖 [Full networksecurity_api documentation](services/networksecurity_api.md)

### 245. Artifactregistry_api

**Resources**: 37

- Python_package [R]
- Tag [CRUD]
- Location [RU]
- Operation [R]
- Repositorie [CRUD]
- Maven_artifact [R]
- Docker_image [R]
- Package [RUD]
- Yum_artifact [C]
- Version [CRUD]
- Generic_artifact [C]
- Project [RU]
- Kfp_artifact [C]
- Googet_artifact [C]
- Rule [CRUD]
- Npm_package [R]
- File [CRUD]
- Attachment [CRD]
- Go_module [C]
- Apt_artifact [C]
- Package [RUD]
- Apt_artifact [C]
- Location [R]
- Version [RD]
- Tag [CRUD]
- Yum_artifact [C]
- Project [RU]
- Operation [R]
- Repositorie [CRUD]
- File [R]
- Location [R]
- Repositorie [CRUD]
- Package [RD]
- Version [RD]
- Tag [CRUD]
- Operation [R]
- File [R]

📖 [Full artifactregistry_api documentation](services/artifactregistry_api.md)

### 246. Workflows_api

**Resources**: 6

- Location [R]
- Workflow [CRUD]
- Operation [RD]
- Location [R]
- Operation [RD]
- Workflow [CRUD]

📖 [Full workflows_api documentation](services/workflows_api.md)

### 247. Cloudprofiler_api

**Resources**: 1

- Profile [CRU]

📖 [Full cloudprofiler_api documentation](services/cloudprofiler_api.md)

### 248. Sasportal_api

**Resources**: 6

- Deployment [CRUD]
- Device [CRUD]
- Customer [CRU]
- Installer [C]
- Policie [CR]
- Node [CRUD]

📖 [Full sasportal_api documentation](services/sasportal_api.md)

### 249. Datacatalog_api

**Resources**: 20

- Tag_template [CRUD]
- Taxonomie [CRUD]
- Field [CUD]
- Entry_group [CRUD]
- Operation [CRD]
- Catalog [C]
- Policy_tag [CRUD]
- Location [CR]
- Entrie [CRUD]
- Tag [CRUD]
- Enum_value [C]
- Entry_group [CRUD]
- Enum_value [C]
- Tag [CRUD]
- Field [CUD]
- Catalog [C]
- Entrie [CRUD]
- Policy_tag [CRUD]
- Tag_template [CRUD]
- Taxonomie [CRUD]

📖 [Full datacatalog_api documentation](services/datacatalog_api.md)

### 250. Firestore_api

**Resources**: 15

- Indexe [CRD]
- User_cred [CRD]
- Operation [CRD]
- Field [RU]
- Location [R]
- Database [CRUD]
- Document [CRUD]
- Backup [RD]
- Backup_schedule [CRUD]
- Database [C]
- Field [RU]
- Indexe [CRD]
- Indexe [CRD]
- Document [CRUD]
- Database [C]

📖 [Full firestore_api documentation](services/firestore_api.md)

### 251. Storage_api

**Resources**: 23

- Service_account [R]
- Object [CRUD]
- Default_object_access_control [CRUD]
- Bucket_access_control [CRUD]
- Object_access_control [CRUD]
- Channel [C]
- Hmac_key [CRUD]
- Notification [CRD]
- Bucket [CRUD]
- Operation [CR]
- Folder [CRD]
- Anywhere_cache [CRU]
- Managed_folder [CRUD]
- Bucket_access_control [CRUD]
- Object [CRUD]
- Object_access_control [CRUD]
- Bucket [CRUD]
- Channel [C]
- Default_object_access_control [CRUD]
- Bucket [CRUD]
- Object_access_control [CRUD]
- Object [CRUD]
- Bucket_access_control [CRUD]

📖 [Full storage_api documentation](services/storage_api.md)

### 252. Appengine_api

**Resources**: 39

- Instance [CRD]
- Module [RUD]
- Location [R]
- App [CRU]
- Version [CRUD]
- Operation [R]
- Authorized_domain [R]
- Location [R]
- Operation [R]
- Service [RUD]
- Application [U]
- Instance [CRD]
- Version [CRUD]
- Ingress_rule [CRUD]
- Domain_mapping [CRUD]
- Authorized_certificate [CRUD]
- App [CRU]
- Location [R]
- Service [RUD]
- Version [CRUD]
- App [CRU]
- Operation [R]
- Instance [CRD]
- Authorized_certificate [CRUD]
- Domain_mapping [CRUD]
- Location [R]
- Operation [R]
- Authorized_domain [R]
- Authorized_domain [R]
- Operation [R]
- Domain_mapping [CRUD]
- Version [CRUD]
- Ingress_rule [CRUD]
- Instance [CRD]
- App [CRU]
- Location [R]
- Authorized_certificate [CRUD]
- Service [RUD]
- Application [U]

📖 [Full appengine_api documentation](services/appengine_api.md)

### 253. Adexperiencereport_api

**Resources**: 2

- Site [R]
- Violating_site [R]

📖 [Full adexperiencereport_api documentation](services/adexperiencereport_api.md)

### 254. Firebaseremoteconfig_api

**Resources**: 1

- Project [RU]

📖 [Full firebaseremoteconfig_api documentation](services/firebaseremoteconfig_api.md)

### 255. Rapidmigrationassessment_api

**Resources**: 4

- Collector [CRUD]
- Location [R]
- Operation [CRD]
- Annotation [CR]

📖 [Full rapidmigrationassessment_api documentation](services/rapidmigrationassessment_api.md)

### 256. Cloudchannel_api

**Resources**: 15

- Entitlement [CR]
- Report_job [C]
- Operation [CRD]
- Customer [CRUD]
- Offer [R]
- Channel_partner_link [CRU]
- Integrator [CR]
- Account [CR]
- Report [CR]
- Channel_partner_repricing_config [CRUD]
- Sku_group [R]
- Sku [R]
- Product [R]
- Customer_repricing_config [CRUD]
- Billable_sku [R]

📖 [Full cloudchannel_api documentation](services/cloudchannel_api.md)

### 257. Managedidentities_api

**Resources**: 18

- Backup [CRUD]
- Domain [CRUD]
- Operation [CRD]
- Sql_integration [R]
- Location [R]
- Peering [CRUD]
- Sql_integration [R]
- Domain [CRUD]
- Operation [CRD]
- Backup [CRUD]
- Location [R]
- Peering [CRUD]
- Domain [CRUD]
- Sql_integration [R]
- Backup [CRUD]
- Peering [CRUD]
- Operation [CRD]
- Location [R]

📖 [Full managedidentities_api documentation](services/managedidentities_api.md)

### 258. Storagetransfer_api

**Resources**: 4

- Agent_pool [CRUD]
- Google_service_account [R]
- Transfer_operation [CR]
- Transfer_job [CRUD]

📖 [Full storagetransfer_api documentation](services/storagetransfer_api.md)

### 259. Advisorynotifications_api

**Resources**: 2

- Location [RU]
- Notification [R]

📖 [Full advisorynotifications_api documentation](services/advisorynotifications_api.md)

### 260. Gkebackup_api

**Resources**: 12

- Operation [CRD]
- Backup_plan_binding [R]
- Backup_plan [CRUD]
- Restore_channel [CRUD]
- Restore_plan_binding [R]
- Backup [CRUD]
- Volume_backup [CR]
- Restore_plan [CRUD]
- Restore [CRUD]
- Location [R]
- Volume_restore [CR]
- Backup_channel [CRUD]

📖 [Full gkebackup_api documentation](services/gkebackup_api.md)

### 261. Adsenseplatform_api

**Resources**: 8

- Site [CRD]
- Event [C]
- Account [CR]
- Site [CRUD]
- Platform [R]
- Group [RU]
- Event [C]
- Account [CR]

📖 [Full adsenseplatform_api documentation](services/adsenseplatform_api.md)

### 262. Addressvalidation_api

**Resources**: 1

- Addressvalidation [C]

📖 [Full addressvalidation_api documentation](services/addressvalidation_api.md)

### 263. Pollen_api

**Resources**: 2

- Heatmap_tile [R]
- Forecast [R]

📖 [Full pollen_api documentation](services/pollen_api.md)

### 264. Blogger_3

**Resources**: 8

- Comment [CRD]
- Post_user_info [R]
- Blog_user_info [R]
- Blog [R]
- Post [CRUD]
- User [R]
- Page [CRUD]
- Page_view [R]

📖 [Full blogger_3 documentation](services/blogger_3.md)

### 265. Firebasedatabase_api

**Resources**: 1

- Instance [CRD]

📖 [Full firebasedatabase_api documentation](services/firebasedatabase_api.md)

### 266. Networkconnectivity_api

**Resources**: 22

- Operation [CRD]
- Multicloud_data_transfer_config [CRUD]
- Internal_range [CRUD]
- Regional_endpoint [CRD]
- Service_connection_token [CRD]
- Service_connection_policie [CRUD]
- Route_table [R]
- Service_connection_map [CRUD]
- Hub [CRUD]
- Policy_based_route [CRD]
- Group [CRU]
- Location [CR]
- Route [R]
- Service_classe [CRUD]
- Destination [CRUD]
- Multicloud_data_transfer_supported_service [R]
- Spoke [CRUD]
- Internal_range [CRUD]
- Operation [CRD]
- Location [R]
- Hub [CRUD]
- Spoke [CRUD]

📖 [Full networkconnectivity_api documentation](services/networkconnectivity_api.md)

### 267. Texttospeech_api

**Resources**: 8

- Voice [R]
- Location [C]
- Text [C]
- Operation [CRD]
- Text [C]
- Operation [R]
- Location [C]
- Voice [R]

📖 [Full texttospeech_api documentation](services/texttospeech_api.md)

### 268. Lifesciences_api

**Resources**: 3

- Pipeline [C]
- Operation [CR]
- Location [R]

📖 [Full lifesciences_api documentation](services/lifesciences_api.md)

### 269. Cloudcontrolspartner_api

**Resources**: 10

- Location [R]
- Workload [R]
- Access_approval_request [R]
- Violation [R]
- Customer [CRUD]
- Violation [R]
- Access_approval_request [R]
- Customer [CRUD]
- Workload [R]
- Location [R]

📖 [Full cloudcontrolspartner_api documentation](services/cloudcontrolspartner_api.md)

### 270. Acceleratedmobilepageurl_api

**Resources**: 1

- Amp_url [C]

📖 [Full acceleratedmobilepageurl_api documentation](services/acceleratedmobilepageurl_api.md)

### 271. Mybusinessplaceactions_api

**Resources**: 2

- Place_action_link [CRUD]
- Place_action_type_metadata [R]

📖 [Full mybusinessplaceactions_api documentation](services/mybusinessplaceactions_api.md)

### 272. Mybusinessaccountmanagement_api

**Resources**: 4

- Admin [CRUD]
- Account [CRU]
- Invitation [CR]
- Location [C]

📖 [Full mybusinessaccountmanagement_api documentation](services/mybusinessaccountmanagement_api.md)

### 273. Qpxexpress_api

**Resources**: 1

- Trip [C]

📖 [Full qpxexpress_api documentation](services/qpxexpress_api.md)

### 274. Homegraph_api

**Resources**: 2

- Device [C]
- Agent_user [D]

📖 [Full homegraph_api documentation](services/homegraph_api.md)

### 275. Any

**Resources**: 5

- Log [RD]
- Entrie [C]
- Log_service [R]
- Indexe [R]
- Sink [CRUD]

📖 [Full any documentation](services/any.md)

### 276. Youtubereporting_api

**Resources**: 4

- Job [CRD]
- Report [R]
- Media [R]
- Report_type [R]

📖 [Full youtubereporting_api documentation](services/youtubereporting_api.md)

### 277. Bigquerydatapolicy_api

**Resources**: 2

- Data_policie [CRUD]
- Data_policie [CRUD]

📖 [Full bigquerydatapolicy_api documentation](services/bigquerydatapolicy_api.md)

### 278. Gkehub_api

**Resources**: 42

- Operation [CRD]
- Rbacrolebinding [CRUD]
- Feature [CRUD]
- Membership [CRUD]
- Fleet [CRUD]
- Location [R]
- Binding [CRUD]
- Namespace [CRUD]
- Scope [CRUD]
- Feature [CRUD]
- Membership [CRUD]
- Fleet [CRUD]
- Rbacrolebinding [CRUD]
- Namespace [CRUD]
- Scope [CRUD]
- Operation [CRD]
- Location [R]
- Binding [CRUD]
- Location [R]
- Membership [CRUD]
- Operation [CRD]
- Location [R]
- Operation [CR]
- Feature [CRUD]
- Location [R]
- Operation [CR]
- Feature [CRUD]
- Scope [CRUD]
- Fleet [CRUD]
- Location [R]
- Namespace [CRUD]
- Membership [CRUD]
- Operation [CRD]
- Rbacrolebinding [CRUD]
- Binding [CRUD]
- Feature [CRUD]
- Membership [CRUD]
- Operation [CRD]
- Location [R]
- Location [R]
- Operation [CR]
- Feature [CRUD]

📖 [Full gkehub_api documentation](services/gkehub_api.md)

### 279. Classroom_api

**Resources**: 17

- Course_work [CRUD]
- Announcement [CRUD]
- Add_on_attachment [CRUD]
- Teacher [CRD]
- Guardian_invitation [CRU]
- Student [CRD]
- Course [CRUD]
- Registration [CD]
- Aliase [CRD]
- User_profile [R]
- Topic [CRUD]
- Rubric [CRUD]
- Course_work_material [CRUD]
- Invitation [CRD]
- Post [R]
- Guardian [RD]
- Student_submission [CRU]

📖 [Full classroom_api documentation](services/classroom_api.md)

### 280. Verifiedaccess_api

**Resources**: 2

- Challenge [C]
- Challenge [C]

📖 [Full verifiedaccess_api documentation](services/verifiedaccess_api.md)

### 281. Datastore_api

**Resources**: 5

- Project [C]
- Project [C]
- Operation [CRD]
- Indexe [CRD]
- Project [C]

📖 [Full datastore_api documentation](services/datastore_api.md)

### 282. Essentialcontacts_api

**Resources**: 1

- Contact [CRUD]

📖 [Full essentialcontacts_api documentation](services/essentialcontacts_api.md)

### 283. Beyondcorp_api

**Resources**: 20

- App_connector [CRUD]
- App_gateway [CRD]
- App_connection [CRUD]
- Location [R]
- Application [CRUD]
- Operation [CRD]
- Security_gateway [CRUD]
- App_connection [CRUD]
- Application_domain [CR]
- Partner_tenant [RD]
- Location [R]
- Application [CRUD]
- App_connector [CRUD]
- Security_gateway [CRUD]
- App_gateway [CRD]
- Subscription [CRU]
- Operation [CRD]
- Insight [R]
- Connection [CRUD]
- Connector [CRUD]

📖 [Full beyondcorp_api documentation](services/beyondcorp_api.md)

### 284. Tpu_api

**Resources**: 23

- Tensorflow_version [R]
- Node [CRD]
- Operation [CRD]
- Accelerator_type [R]
- Location [R]
- Accelerator_type [R]
- Node [CRUD]
- Operation [CRD]
- Queued_resource [CRD]
- Location [CR]
- Runtime_version [R]
- Node [CRD]
- Location [R]
- Accelerator_type [R]
- Tensorflow_version [R]
- Operation [CRD]
- Node [CRUD]
- Reservation [R]
- Operation [CRD]
- Location [CR]
- Accelerator_type [R]
- Queued_resource [CRD]
- Runtime_version [R]

📖 [Full tpu_api documentation](services/tpu_api.md)

### 285. Admin_api

**Resources**: 34

- Photo [RUD]
- Token [RD]
- Mobiledevice [CRD]
- Asp [RD]
- Schema [CRUD]
- Two_step_verification [C]
- Role [CRUD]
- Print_server [CRUD]
- Building [CRUD]
- Command [R]
- Aliase [CRD]
- Orgunit [CRUD]
- Customer [RU]
- Privilege [R]
- Channel [C]
- Role_assignment [CRD]
- Chromeosdevice [CRU]
- Printer [CRUD]
- Domain_aliase [CRD]
- Group [CRUD]
- User [CRUD]
- Domain [CRD]
- Feature [CRUD]
- Chromeo [C]
- Calendar [CRUD]
- Member [CRUD]
- Verification_code [CR]
- Transfer [CR]
- Application [R]
- Channel [C]
- Activitie [CR]
- User_usage_report [R]
- Customer_usage_report [R]
- Entity_usage_report [R]

📖 [Full admin_api documentation](services/admin_api.md)

### 286. Documentai_api

**Resources**: 21

- Processor_type [R]
- Location [R]
- Evaluation [R]
- Schema [CRUD]
- Processor_version [CRD]
- Operation [CR]
- Dataset [CRU]
- Schema_version [CRUD]
- Human_review_config [C]
- Processor [CRUD]
- Schema_version [CRUD]
- Processor_type [R]
- Evaluation [R]
- Processor [CRD]
- Processor_version [CRD]
- Operation [CRD]
- Location [R]
- Human_review_config [C]
- Schema [CRUD]
- Document [C]
- Operation [R]

📖 [Full documentai_api documentation](services/documentai_api.md)

### 287. Retail_api

**Resources**: 38

- Placement [C]
- Attributes_config [C]
- Catalog [CRU]
- Project [RU]
- Serving_config [CRUD]
- Control [CRUD]
- Completion_data [C]
- Generative_question [CR]
- User_event [C]
- Model [CRUD]
- Operation [R]
- Product [CRUD]
- Completion_data [C]
- Product [CRUD]
- Serving_config [CRUD]
- Control [CRUD]
- Operation [R]
- Model [CRUD]
- Catalog [CRU]
- Generative_question [CR]
- User_event [C]
- Attributes_config [C]
- Placement [C]
- Placement [C]
- Product [CRUD]
- Serving_config [CRUD]
- Branche [R]
- Operation [R]
- Model [CRUD]
- Catalog [CRU]
- Merchant_center_account_link [CRD]
- Control [CRUD]
- Retail_project [C]
- Project [CRU]
- User_event [C]
- Generative_question [CR]
- Completion_data [C]
- Attributes_config [C]

📖 [Full retail_api documentation](services/retail_api.md)

### 288. Cloudidentity_api

**Resources**: 22

- Membership [CRD]
- Inbound_sso_assignment [CRUD]
- Userinvitation [CR]
- Idp_credential [CRD]
- Inbound_oidc_sso_profile [CRUD]
- Inbound_saml_sso_profile [CRUD]
- Device [CRD]
- Client_state [RU]
- Policie [R]
- Group [CRUD]
- Device_user [CRD]
- Policie [CRUD]
- Userinvitation [CR]
- Group [CRUD]
- Idp_credential [CRD]
- Device [CRD]
- Client_state [RU]
- Device_user [CRD]
- Inbound_oidc_sso_profile [CRUD]
- Inbound_saml_sso_profile [CRUD]
- Inbound_sso_assignment [CRUD]
- Membership [CRD]

📖 [Full cloudidentity_api documentation](services/cloudidentity_api.md)

### 289. Doubleclicksearch_api

**Resources**: 3

- Conversion [CRU]
- Saved_column [R]
- Report [CR]

📖 [Full doubleclicksearch_api documentation](services/doubleclicksearch_api.md)

### 290. Forms_api

**Resources**: 3

- Form [CR]
- Watche [CRD]
- Response [R]

📖 [Full forms_api documentation](services/forms_api.md)

### 291. Batch_api

**Resources**: 5

- Task [R]
- Operation [CRD]
- Location [R]
- State [C]
- Job [CRD]

📖 [Full batch_api documentation](services/batch_api.md)

### 292. Migrationcenter_api

**Resources**: 27

- Group [CRUD]
- Relation [R]
- Import_data_file [CRD]
- Operation [CRD]
- Import_job [CRUD]
- Location [RU]
- Preference_set [CRUD]
- Asset [CRUD]
- Discovery_client [CRUD]
- Source [CRUD]
- Error_frame [R]
- Report [CRD]
- Report_config [CRD]
- Report [CRD]
- Import_data_file [CRD]
- Error_frame [R]
- Import_job [CRUD]
- Source [CRUD]
- Preference_set [CRUD]
- Relation [R]
- Report_config [CRD]
- Discovery_client [CRUD]
- Asset [CRUD]
- Group [CRUD]
- Operation [CRD]
- Assets_export_job [CRD]
- Location [RU]

📖 [Full migrationcenter_api documentation](services/migrationcenter_api.md)

### 293. Androidenterprise_api

**Resources**: 17

- Permission [R]
- User [CRUD]
- Grouplicense [R]
- Install [RUD]
- Managedconfigurationsfordevice [RUD]
- Device [CRU]
- Product [CR]
- Managedconfigurationsforuser [RUD]
- Grouplicenseuser [R]
- Storelayoutpage [CRUD]
- Serviceaccountkey [CRD]
- Entitlement [RUD]
- Enterprise [CRU]
- Managedconfigurationssetting [R]
- Enrollment_token [C]
- Storelayoutcluster [CRUD]
- Webapp [CRUD]

📖 [Full androidenterprise_api documentation](services/androidenterprise_api.md)

### 294. Cloudshell_api

**Resources**: 4

- Environment [CR]
- Operation [CRD]
- Environment [CRU]
- Public_key [CD]

📖 [Full cloudshell_api documentation](services/cloudshell_api.md)

### 295. Redis_api

**Resources**: 12

- Backup [CRD]
- Cluster [CRUD]
- Instance [CRUD]
- Location [R]
- Operation [CRD]
- Backup_collection [R]
- Cluster [CRUD]
- Operation [CRD]
- Location [R]
- Backup_collection [R]
- Instance [CRUD]
- Backup [CRD]

📖 [Full redis_api documentation](services/redis_api.md)

### 296. Websecurityscanner_api

**Resources**: 15

- Scan_config [CRUD]
- Crawled_url [R]
- Finding [R]
- Scan_run [CR]
- Finding_type_stat [R]
- Scan_run [CR]
- Finding_type_stat [R]
- Finding [R]
- Crawled_url [R]
- Scan_config [CRUD]
- Scan_run [CR]
- Crawled_url [R]
- Finding_type_stat [R]
- Scan_config [CRUD]
- Finding [R]

📖 [Full websecurityscanner_api documentation](services/websecurityscanner_api.md)

### 297. Appsactivity_api

**Resources**: 1

- Activitie [R]

📖 [Full appsactivity_api documentation](services/appsactivity_api.md)

### 298. Youtubeanalytics_api

**Resources**: 6

- Report [R]
- Group_item [CRD]
- Group [CRUD]
- Group [CRUD]
- Group_item [CRD]
- Report [R]

📖 [Full youtubeanalytics_api documentation](services/youtubeanalytics_api.md)

### 299. Speech_api

**Resources**: 12

- Speech [C]
- Phrase_set [CRUD]
- Custom_classe [CRUD]
- Operation [R]
- Operation [R]
- Operation [R]
- Operation [R]
- Speech [C]
- Operation [R]
- Speech [C]
- Phrase_set [CRUD]
- Custom_classe [CRUD]

📖 [Full speech_api documentation](services/speech_api.md)

### 300. Vmmigration_api

**Resources**: 28

- Image_import_job [CR]
- Operation [CRD]
- Group [CRUD]
- Cutover_job [CR]
- Source [CRUD]
- Disk_migration_job [CRUD]
- Location [R]
- Clone_job [CR]
- Migrating_vm [CRUD]
- Utilization_report [CRD]
- Replication_cycle [R]
- Datacenter_connector [CRD]
- Image_import [CRD]
- Target_project [CRUD]
- Image_import [CRD]
- Clone_job [CR]
- Replication_cycle [R]
- Location [R]
- Source [CRUD]
- Image_import_job [CR]
- Operation [CRD]
- Target_project [CRUD]
- Migrating_vm [CRUD]
- Group [CRUD]
- Disk_migration_job [CRUD]
- Cutover_job [CR]
- Utilization_report [CRD]
- Datacenter_connector [CRD]

📖 [Full vmmigration_api documentation](services/vmmigration_api.md)

### 301. Webfonts_api

**Resources**: 1

- Webfont [R]

📖 [Full webfonts_api documentation](services/webfonts_api.md)

### 302. Safebrowsing_api

**Resources**: 9

- Hash_list [R]
- Hashe [R]
- Threat_list_update [C]
- Threat_matche [C]
- Threat_list [R]
- Encoded_full_hashe [R]
- Encoded_update [R]
- Threat_hit [C]
- Full_hashe [C]

📖 [Full safebrowsing_api documentation](services/safebrowsing_api.md)

### 303. Admob_api

**Resources**: 16

- App [R]
- Network_report [C]
- Account [R]
- Ad_unit [R]
- Mediation_report [C]
- Ad_source [R]
- Account [R]
- Network_report [C]
- Mediation_report [C]
- Mediation_ab_experiment [C]
- Campaign_report [C]
- Ad_unit_mapping [CR]
- Adapter [R]
- Mediation_group [CRU]
- App [CR]
- Ad_unit [CR]

📖 [Full admob_api documentation](services/admob_api.md)

### 304. Places_api

**Resources**: 2

- Place [CR]
- Photo [R]

📖 [Full places_api documentation](services/places_api.md)

### 305. Webrisk_api

**Resources**: 5

- Submission [C]
- Uri [R]
- Operation [CRD]
- Hashe [R]
- Threat_list [R]

📖 [Full webrisk_api documentation](services/webrisk_api.md)

### 306. Readerrevenuesubscriptionlinking_api

**Resources**: 1

- Reader [RUD]

📖 [Full readerrevenuesubscriptionlinking_api documentation](services/readerrevenuesubscriptionlinking_api.md)

### 307. Workflowexecutions_api

**Resources**: 5

- Workflow [C]
- Step_entrie [R]
- Execution [CR]
- Callback [R]
- Execution [CR]

📖 [Full workflowexecutions_api documentation](services/workflowexecutions_api.md)

### 308. Analyticshub_api

**Resources**: 6

- Subscription [CRD]
- Listing [CRUD]
- Data_exchange [CRUD]
- Query_template [CRUD]
- Listing [CRUD]
- Data_exchange [CRUD]

📖 [Full analyticshub_api documentation](services/analyticshub_api.md)

### 309. Analyticsadmin_api

**Resources**: 39

- Key_event [CRUD]
- Reporting_data_annotation [CRUD]
- Ad_sense_link [CRD]
- Expanded_data_set [CRUD]
- Google_ads_link [CRUD]
- Event_edit_rule [CRUD]
- Propertie [CRUD]
- Custom_metric [CRU]
- Data_stream [CRUD]
- Event_create_rule [CRUD]
- Big_query_link [CRUD]
- Channel_group [CRUD]
- Subproperty_sync_config [RU]
- Access_binding [CRUD]
- Calculated_metric [CRUD]
- Conversion_event [CRUD]
- Account_summarie [R]
- Account [CRUD]
- Sk_ad_network_conversion_value_schema [CRUD]
- Search_ads360_link [CRUD]
- Display_video360_advertiser_link [CRUD]
- Custom_dimension [CRU]
- Firebase_link [CRD]
- Rollup_property_source_link [CRD]
- Subproperty_event_filter [CRUD]
- Display_video360_advertiser_link_proposal [CRD]
- Audience [CRU]
- Measurement_protocol_secret [CRUD]
- Account [CRUD]
- Custom_dimension [CRU]
- Key_event [CRUD]
- Conversion_event [CRUD]
- Firebase_link [CRD]
- Propertie [CRUD]
- Data_stream [CRUD]
- Custom_metric [CRU]
- Account_summarie [R]
- Measurement_protocol_secret [CRUD]
- Google_ads_link [CRUD]

📖 [Full analyticsadmin_api documentation](services/analyticsadmin_api.md)

### 310. Playmoviespartner_api

**Resources**: 4

- Country [R]
- Store_info [R]
- Avail [R]
- Order [R]

📖 [Full playmoviespartner_api documentation](services/playmoviespartner_api.md)

### 311. Jobs_api

**Resources**: 16

- Companie [CRUD]
- Job [CRUD]
- Job [CRUD]
- Companie [CRUD]
- Tenant [CRUD]
- Operation [R]
- Client_event [C]
- Project [R]
- Job [CRUD]
- Companie [CRUD]
- Client_event [C]
- Operation [R]
- Project [R]
- Companie [CRUD]
- Job [CRUD]
- Client_event [C]

📖 [Full jobs_api documentation](services/jobs_api.md)

### 312. Firebasedynamiclinks_api

**Resources**: 3

- Firebasedynamiclink [CR]
- Managed_short_link [C]
- Short_link [C]

📖 [Full firebasedynamiclinks_api documentation](services/firebasedynamiclinks_api.md)

### 313. Translate_api

**Resources**: 18

- Location [CR]
- Project [CR]
- Glossarie [CRD]
- Operation [CRD]
- Detection [CR]
- Translation [CR]
- Language [R]
- Operation [CRD]
- Glossarie [CRUD]
- Example [R]
- Project [CR]
- Glossary_entrie [CRUD]
- Location [CR]
- Adaptive_mt_sentence [R]
- Dataset [CRD]
- Model [CRD]
- Adaptive_mt_dataset [CRD]
- Adaptive_mt_file [RD]

📖 [Full translate_api documentation](services/translate_api.md)

### 314. Remotebuildexecution_api

**Resources**: 10

- Media [CR]
- Operation [CRD]
- Instance [CRUD]
- Workerpool [CRUD]
- Operation [R]
- Remotebuildexecution [R]
- Operation [C]
- Action [C]
- Action_result [RU]
- Blob [CR]

📖 [Full remotebuildexecution_api documentation](services/remotebuildexecution_api.md)

### 315. Developerconnect_api

**Resources**: 7

- Connection [CRUD]
- Account_connector [CRUD]
- Operation [CRD]
- Insights_config [CRUD]
- Git_repository_link [CRD]
- User [CRD]
- Location [R]

📖 [Full developerconnect_api documentation](services/developerconnect_api.md)

### 316. Firebaseappdistribution_api

**Resources**: 16

- Group [CRUD]
- Feedback_report [RD]
- Release [CRU]
- Tester [CRU]
- App [R]
- Operation [CRD]
- Media [C]
- App [RU]
- Release [C]
- Tester [R]
- Test [CR]
- Note [C]
- Release_by_hash [R]
- Project [R]
- Test_case [CRUD]
- Upload_statu [R]

📖 [Full firebaseappdistribution_api documentation](services/firebaseappdistribution_api.md)

### 317. Playcustomapp_api

**Resources**: 1

- Custom_app [C]

📖 [Full playcustomapp_api documentation](services/playcustomapp_api.md)

### 318. Datapipelines_api

**Resources**: 2

- Pipeline [CRUD]
- Job [R]

📖 [Full datapipelines_api documentation](services/datapipelines_api.md)

### 319. Dns_api

**Resources**: 34

- Response_policy_rule [CRUD]
- Managed_zone [CRUD]
- Project [R]
- Dns_key [R]
- Response_policie [CRUD]
- Resource_record_set [CRUD]
- Policie [CRUD]
- Managed_zone_operation [R]
- Change [CR]
- Response_policy_rule [CRUD]
- Response_policie [CRUD]
- Policie [CRUD]
- Managed_zone [CRUD]
- Project [R]
- Change [CR]
- Managed_zone_operation [R]
- Resource_record_set [CRUD]
- Dns_key [R]
- Resource_record_set [R]
- Policie [CRUD]
- Managed_zone_operation [R]
- Dns_key [R]
- Project [R]
- Managed_zone [CRUD]
- Change [CR]
- Response_policy_rule [CRUD]
- Project [R]
- Policie [CRUD]
- Change [CR]
- Managed_zone [CRUD]
- Response_policie [CRUD]
- Managed_zone_operation [R]
- Resource_record_set [CRUD]
- Dns_key [R]

📖 [Full dns_api documentation](services/dns_api.md)

### 320. Solar_api

**Resources**: 3

- Building_insight [R]
- Geo_tiff [R]
- Data_layer [R]

📖 [Full solar_api documentation](services/solar_api.md)

### 321. Accesscontextmanager_api

**Resources**: 11

- Operation [CRD]
- Authorized_orgs_desc [CRUD]
- Service_perimeter [CRUD]
- Gcp_user_access_binding [CRUD]
- Service [R]
- Access_level [CRUD]
- Access_policie [CRUD]
- Access_policie [CRUD]
- Service_perimeter [CRUD]
- Operation [R]
- Access_level [CRUD]

📖 [Full accesscontextmanager_api documentation](services/accesscontextmanager_api.md)

### 322. Versionhistory_api

**Resources**: 4

- Release [R]
- Platform [R]
- Channel [R]
- Version [R]

📖 [Full versionhistory_api documentation](services/versionhistory_api.md)

### 323. Recommendationengine_api

**Resources**: 6

- Placement [C]
- Prediction_api_key_registration [CRD]
- Catalog [RU]
- Operation [R]
- Catalog_item [CRUD]
- User_event [CR]

📖 [Full recommendationengine_api documentation](services/recommendationengine_api.md)

### 324. Identitytoolkit_api

**Resources**: 12

- Identity_platform [C]
- Oauth_idp_config [CRUD]
- Project [RU]
- Identitytoolkit [R]
- Mfa_enrollment [C]
- Mfa_sign_in [C]
- Default_supported_idp_config [CRUD]
- Default_supported_idp [R]
- Tenant [CRUD]
- Account [C]
- Inbound_saml_config [CRUD]
- Relyingparty [CR]

📖 [Full identitytoolkit_api documentation](services/identitytoolkit_api.md)

### 325. Bigqueryconnection_api

**Resources**: 2

- Connection [CRUD]
- Connection [CRUD]

📖 [Full bigqueryconnection_api documentation](services/bigqueryconnection_api.md)

### 326. Digitalassetlinks_api

**Resources**: 2

- Assetlink [CR]
- Statement [R]

📖 [Full digitalassetlinks_api documentation](services/digitalassetlinks_api.md)

### 327. Fcm_api

**Resources**: 1

- Message [C]

📖 [Full fcm_api documentation](services/fcm_api.md)

### 328. Analyticsreporting_api

**Resources**: 2

- Report [C]
- User_activity [C]

📖 [Full analyticsreporting_api documentation](services/analyticsreporting_api.md)

### 329. Pubsublite_api

**Resources**: 5

- Subscription [CRUD]
- Reservation [CRUD]
- Operation [CRD]
- Cursor [R]
- Topic [CRUD]

📖 [Full pubsublite_api documentation](services/pubsublite_api.md)

### 330. Test_api

**Resources**: 2

- Bucket [R]
- Oauth2 [R]

📖 [Full test_api documentation](services/test_api.md)

### 331. Gmailpostmastertools_api

**Resources**: 4

- Traffic_stat [R]
- Domain [R]
- Domain [R]
- Traffic_stat [R]

📖 [Full gmailpostmastertools_api documentation](services/gmailpostmastertools_api.md)

### 332. Workspaceevents_api

**Resources**: 2

- Subscription [CRUD]
- Operation [R]

📖 [Full workspaceevents_api documentation](services/workspaceevents_api.md)

### 333. Analytics_api

**Resources**: 32

- Experiment [CRUD]
- Mcf [R]
- Provisioning [C]
- Account_summarie [R]
- Unsampled_report [CRD]
- Account [R]
- Realtime [R]
- Goal [CRU]
- Profile_user_link [CRUD]
- Segment [R]
- Profile_filter_link [CRUD]
- Upload [CR]
- Filter [CRUD]
- Webpropertie [CRU]
- Custom_dimension [CRU]
- Custom_data_source [R]
- Column [R]
- Client_id [C]
- Remarketing_audience [CRUD]
- Webproperty_user_link [CRUD]
- Custom_metric [CRU]
- User_deletion_request [C]
- Web_property_ad_words_link [CRUD]
- Ga [R]
- Account_user_link [CRUD]
- Profile [CRUD]
- Profile [R]
- Data [R]
- Goal [R]
- Account [R]
- Segment [R]
- Webpropertie [R]

📖 [Full analytics_api documentation](services/analytics_api.md)

### 334. Dialogflow_api

**Resources**: 101

- Operation [CR]
- Result [R]
- Playbook [CRUD]
- Example [CRUD]
- Transition_route_group [CRUD]
- Entity_type [CRUD]
- Deployment [R]
- Location [R]
- Environment [CRUD]
- Version [CRUD]
- Conversation [RD]
- Webhook [CRUD]
- Generator [CRUD]
- Changelog [R]
- Intent [CRUD]
- Agent [CRUD]
- Flow [CRUD]
- Page [CRUD]
- Tool [CRUD]
- Session [C]
- Security_setting [CRUD]
- Test_case [CRU]
- Experiment [CRUD]
- Continuous_test_result [R]
- Answer_record [RU]
- Document [CRUD]
- Entity_type [CRUD]
- Evaluation [CRD]
- Suggestion [CR]
- Generator [CRUD]
- Conversation_profile [CRUD]
- Project [CRD]
- Session [CD]
- Version [CRUD]
- Phone_number [CRUD]
- Participant [CRU]
- Encryption_spec [C]
- Sip_trunk [CRUD]
- Agent [CRU]
- Intent [CRUD]
- Entitie [C]
- Operation [CR]
- Tool [CRUD]
- Conversation [CR]
- Message [CR]
- Stateless_suggestion [C]
- Location [CRD]
- Knowledge_base [CRUD]
- Environment [CRUD]
- Context [CRUD]
- Location [CRD]
- Conversation_dataset [CRD]
- Version [CRUD]
- Conversation_model [CRD]
- Sip_trunk [CRUD]
- Knowledge_base [CRUD]
- Evaluation [CRD]
- Document [CRUD]
- Suggestion [C]
- Entity_type [CRUD]
- Participant [CRU]
- Stateless_suggestion [C]
- Encryption_spec [C]
- Operation [CR]
- Context [CRUD]
- Conversation_profile [CRUD]
- Conversation [CR]
- Message [R]
- Environment [CRUD]
- Generator [CRUD]
- Session [CD]
- Intent [CRUD]
- Project [CRD]
- Agent [CRU]
- Tool [CRUD]
- Answer_record [RU]
- Entitie [C]
- Deployment [R]
- Tool [CRUD]
- Page [CRUD]
- Security_setting [CRUD]
- Agent [CRUD]
- Example [CRUD]
- Location [R]
- Experiment [CRUD]
- Test_case [CRU]
- Generator [CRUD]
- Operation [CR]
- Version [CRUD]
- Continuous_test_result [R]
- Session [C]
- Flow [CRUD]
- Playbook [CRUD]
- Environment [CRUD]
- Webhook [CRUD]
- Entity_type [CRUD]
- Transition_route_group [CRUD]
- Intent [CRUD]
- Result [R]
- Changelog [R]
- Operation [CR]

📖 [Full dialogflow_api documentation](services/dialogflow_api.md)

### 335. Iap_api

**Resources**: 5

- Identity_aware_proxy_client [CRD]
- Brand [CR]
- Dest_group [CRUD]
- Iap [CRU]
- Iap [C]

📖 [Full iap_api documentation](services/iap_api.md)

### 336. Healthcare_api

**Resources**: 60

- Operation [CRD]
- Consent_store [CRUD]
- Consent_artifact [CRD]
- Attribute_definition [CRUD]
- Dicom_store [CRUD]
- Dataset [CRUD]
- Studie [CRD]
- Consent [CRUD]
- Instance [RD]
- Serie [RD]
- Message [CRUD]
- Hl7_v2_store [CRUD]
- Frame [R]
- Data_mapper_workspace [CR]
- Bulkdata [R]
- Fhir_store [CRUD]
- Location [R]
- User_data_mapping [CRUD]
- Fhir [CRUD]
- Nlp [C]
- Operation [R]
- Hl7_v2_store [CR]
- Location [R]
- Dataset [CR]
- Dicom_store [CR]
- Message [CRUD]
- Fhir_store [CRUD]
- Annotation [CRUD]
- Annotation_store [CRUD]
- Dicom_web [CR]
- Operation [R]
- Studie [CRD]
- Location [R]
- Frame [R]
- Serie [RD]
- Fhir [CRUD]
- Dataset [CRUD]
- Hl7_v2_store [CRUD]
- Dicom_store [CRUD]
- Instance [RD]
- Consent_artifact [CRD]
- Data_mapper_workspace [CR]
- Location [R]
- Consent [CRUD]
- Nlp [C]
- Consent_store [CRUD]
- Serie [RUD]
- Attribute_definition [CRUD]
- Message [CRUD]
- Bulkdata [R]
- User_data_mapping [CRUD]
- Dicom_store [CRUD]
- Instance [RUD]
- Operation [CRD]
- Frame [R]
- Hl7_v2_store [CRUD]
- Fhir [CRUD]
- Fhir_store [CRUD]
- Studie [CRUD]
- Dataset [CRUD]

📖 [Full healthcare_api documentation](services/healthcare_api.md)

### 337. Cloudprivatecatalogproducer_api

**Resources**: 6

- Version [CRUD]
- Icon [C]
- Catalog [CRUD]
- Operation [CRD]
- Association [CRD]
- Product [CRUD]

📖 [Full cloudprivatecatalogproducer_api documentation](services/cloudprivatecatalogproducer_api.md)

### 338. Groupsmigration_api

**Resources**: 1

- Archive [C]

📖 [Full groupsmigration_api documentation](services/groupsmigration_api.md)

### 339. Cloudresourcemanager_api

**Resources**: 24

- Organization [CR]
- Project [CRUD]
- Folder [C]
- Operation [R]
- Lien [CRD]
- Operation [R]
- Folder [CRUD]
- Folder [CRUD]
- Operation [R]
- Organization [CRU]
- Project [CRUD]
- Effective_tag [R]
- Operation [R]
- Tag_binding_collection [RU]
- Tag_value [CRUD]
- Tag_hold [CRD]
- Lien [CRD]
- Effective_tag_binding_collection [R]
- Tag_key [CRUD]
- Folder [CRUD]
- Capabilitie [RU]
- Tag_binding [CRD]
- Organization [CR]
- Project [CRUD]

📖 [Full cloudresourcemanager_api documentation](services/cloudresourcemanager_api.md)

### 340. Json_body

**Resources**: 6

- Job [CRU]
- Operation [CRD]
- Project [CR]
- Version [CRUD]
- Location [R]
- Model [CRUD]

📖 [Full json_body documentation](services/json_body.md)

### 341. Playgrouping_api

**Resources**: 2

- Tag [C]
- Token [C]

📖 [Full playgrouping_api documentation](services/playgrouping_api.md)

### 342. Docs_api

**Resources**: 1

- Document [CR]

📖 [Full docs_api documentation](services/docs_api.md)

### 343. Alloydb_api

**Resources**: 21

- Location [R]
- Cluster [CRUD]
- Backup [CRUD]
- Operation [CRD]
- User [CRUD]
- Instance [CRUD]
- Supported_database_flag [R]
- Location [R]
- Operation [CRD]
- Backup [CRUD]
- Cluster [CRUD]
- Supported_database_flag [R]
- Instance [CRUD]
- User [CRUD]
- Instance [CRUD]
- Backup [CRUD]
- Location [R]
- Operation [CRD]
- User [CRUD]
- Supported_database_flag [R]
- Cluster [CRUD]

📖 [Full alloydb_api documentation](services/alloydb_api.md)

### 344. Domainsrdap_api

**Resources**: 6

- Domainsrdap [R]
- Ip [R]
- Autnum [R]
- Nameserver [R]
- Domain [R]
- Entity [R]

📖 [Full domainsrdap_api documentation](services/domainsrdap_api.md)

### 345. Tracing_api

**Resources**: 2

- Span [C]
- Trace [CR]

📖 [Full tracing_api documentation](services/tracing_api.md)

### 346. Certificatemanager_api

**Resources**: 8

- Certificate [CRUD]
- Certificate_issuance_config [CRUD]
- Trust_config [CRUD]
- Operation [CRD]
- Certificate_map [CRUD]
- Certificate_map_entrie [CRUD]
- Dns_authorization [CRUD]
- Location [R]

📖 [Full certificatemanager_api documentation](services/certificatemanager_api.md)

### 347. Firebaseml_api

**Resources**: 4

- Operation [CRD]
- Operation [R]
- Model [CRUD]
- Model [C]

📖 [Full firebaseml_api documentation](services/firebaseml_api.md)

### 348. Cloudsearch_api

**Resources**: 12

- Operation [R]
- Stat [R]
- Unmappedid [R]
- Datasource [CRUD]
- Media [C]
- Source [R]
- Setting [RU]
- Searchapplication [CRUD]
- Lro [R]
- Item [CRD]
- Cloudsearch [C]
- Query [C]

📖 [Full cloudsearch_api documentation](services/cloudsearch_api.md)


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

# Create caption
caption = provider.mediaupload.Caption {
}

# Use resource outputs
caption_id = caption.id

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
