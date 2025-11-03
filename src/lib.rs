//! Gcp Provider for Hemmer
//!
//! Auto-generated unified provider from gcp SDK version v1
//!
//! This provider includes multiple services:
//! - translate_api
//! - area120tables_api
//! - abusiveexperiencereport_api
//! - securesourcemanager_api
//! - file_api
//! - oauth2_api
//! - serviceuser_api
//! - bigquerydatatransfer_api
//! - cloudfunctions_api
//! - networkconnectivity_api
//! - mybusinessverifications_api
//! - qpxexpress_api
//! - dataproc_api
//! - playgrouping_api
//! - mirror_api
//! - adexchangeseller_api
//! - binaryauthorization_api
//! - marketingplatformadmin_api
//! - cloudprivatecatalogproducer_api
//! - pubsublite_api
//! - slides_api
//! - billingbudgets_api
//! - workstations_api
//! - civicinfo_api
//! - spectrum_api
//! - surveys_api
//! - dataplex_api
//! - redis_api
//! - addressvalidation_api
//! - sourcerepo_api
//! - osconfig_api
//! - domainsrdap_api
//! - speech_api
//! - trafficdirector_api
//! - indexing_api
//! - ondemandscanning_api
//! - drive_api
//! - mybusinessnotifications_api
//! - workflowexecutions_api
//! - developerconnect_api
//! - analyticsadmin_api
//! - playdeveloperreporting_api
//! - prod_tt_sasportal_api
//! - searchconsole_api
//! - firebaseremoteconfig_api
//! - publicca_api
//! - discovery_api
//! - firebasehosting_api
//! - memcache_api
//! - playablelocations_api
//! - jobs_api
//! - iam_api
//! - reseller_api
//! - realtimebidding_api
//! - datamigration_api
//! - safebrowsing_api
//! - libraryagent_api
//! - css_api
//! - androidpublisher_api
//! - datacatalog_api
//! - cloudlocationfinder_api
//! - storagetransfer_api
//! - resourcesettings_api
//! - firebasedatabase_api
//! - analyticshub_api
//! - http_body
//! - managedkafka_api
//! - poly_api
//! - orgpolicy_api
//! - certificatemanager_api
//! - apigateway_api
//! - datastore_api
//! - cloudtasks_api
//! - firebaseappdistribution_api
//! - domains_api
//! - policysimulator_api
//! - apihub_api
//! - mybusinessaccountmanagement_api
//! - smartdevicemanagement_api
//! - tracing_api
//! - gamesmanagement_api
//! - firestore_api
//! - language_api
//! - json_body
//! - replicapoolupdater_api
//! - androidmanagement_api
//! - licensing_api
//! - sasportal_api
//! - vmwareengine_api
//! - pollen_api
//! - cloudsupport_api
//! - servicemanagement_api
//! - cloudiot_api
//! - androiddeviceprovisioning_api
//! - gamesconfiguration_api
//! - areainsights_api
//! - meet_api
//! - adsense_api
//! - toolresults_api
//! - artifactregistry_api
//! - cloudcommerceprocurement_api
//! - alertcenter_api
//! - mybusinessbusinessinformation_api
//! - bigquerydatapolicy_api
//! - androidenterprise_api
//! - playmoviespartner_api
//! - oracledatabase_api
//! - composer_api
//! - dataform_api
//! - adsensehost_api
//! - deploymentmanager_api
//! - displayvideo_api
//! - chromewebstore_api
//! - cloudkms_api
//! - bigtableadmin_api
//! - drivelabels_api
//! - vision_api
//! - streetviewpublish_api
//! - clouddeploy_api
//! - biglake_api
//! - eventarc_api
//! - apigeeregistry_api
//! - cloudbuild_api
//! - secretmanager_api
//! - sheets_api
//! - recommendationengine_api
//! - appengine_api
//! - dataportability_api
//! - remotebuildexecution_api
//! - containeranalysis_api
//! - workloadmanager_api
//! - cloudcontrolspartner_api
//! - monitoring_api
//! - privateca_api
//! - container_api
//! - merchantapi_api
//! - datapipelines_api
//! - sts_api
//! - observability_api
//! - dataflow_api
//! - cloudresourcemanager_api
//! - firebaseapphosting_api
//! - verifiedaccess_api
//! - accessapproval_api
//! - firebasedataconnect_api
//! - assuredworkloads_api
//! - datafusion_api
//! - doubleclicksearch_api
//! - ideahub_api
//! - alloydb_api
//! - doubleclickbidmanager_api
//! - bigqueryreservation_api
//! - notebooks_api
//! - readerrevenuesubscriptionlinking_api
//! - authorizedbuyersmarketplace_api
//! - adexchangebuyer2_api
//! - adsenseplatform_api
//! - mybusinessplaceactions_api
//! - firebasestorage_api
//! - calendar_api
//! - networkmanagement_api
//! - admin_api
//! - acceleratedmobilepageurl_api
//! - fusiontables_api
//! - chromepolicy_api
//! - securitycenter_api
//! - analyticsreporting_api
//! - iap_api
//! - consumersurveys_api
//! - siteverification_api
//! - versionhistory_api
//! - storagebatchoperations_api
//! - acmedns_api
//! - storage_api
//! - networkservices_api
//! - dfareporting_api
//! - webfonts_api
//! - apim_api
//! - chromeuxreport_api
//! - aiplatform_api
//! - script_api
//! - contentwarehouse_api
//! - batch_api
//! - kmsinventory_api
//! - run_api
//! - servicenetworking_api
//! - cloudscheduler_api
//! - cloudsearch_api
//! - admob_api
//! - workspaceevents_api
//! - metastore_api
//! - policytroubleshooter_api
//! - analyticsdata_api
//! - contactcenteraiplatform_api
//! - forms_api
//! - clouderrorreporting_api
//! - pagespeedonline_api
//! - parallelstore_api
//! - cloudbilling_api
//! - cloudidentity_api
//! - appsactivity_api
//! - groupssettings_api
//! - adexchangebuyer_api
//! - youtube_api
//! - looker_api
//! - gkebackup_api
//! - cloudasset_api
//! - people_api
//! - mybusinessqanda_api
//! - beyondcorp_api
//! - pubsub_api
//! - servicebroker_api
//! - contactcenterinsights_api
//! - accesscontextmanager_api
//! - searchads360_api
//! - oslogin_api
//! - playcustomapp_api
//! - webmasters_api
//! - fitness_api
//! - parametermanager_api
//! - businessprofileperformance_api
//! - videointelligence_api
//! - replicapool_api
//! - dlp_api
//! - migrationcenter_api
//! - kgsearch_api
//! - cloudprofiler_api
//! - healthcare_api
//! - firebaseappcheck_api
//! - gkeonprem_api
//! - repeated_any_query_error
//! - transcoder_api
//! - firebaserules_api
//! - plusdomains_api
//! - proximitybeacon_api
//! - firebase_api
//! - resource_named_service
//! - config_api
//! - games_api
//! - appstate_api
//! - saasservicemgmt_api
//! - test_api
//! - gkehub_api
//! - analytics_api
//! - discoveryengine_api
//! - playintegrity_api
//! - homegraph_api
//! - apikeys_api
//! - serviceconsumermanagement_api
//! - datalineage_api
//! - manufacturers_api
//! - vectortile_api
//! - chat_api
//! - bigquery_api
//! - plus_api
//! - bigqueryconnection_api
//! - places_api
//! - logging_api
//! - webrisk_api
//! - vpcaccess_api
//! - urlshortener_api
//! - sqladmin_api
//! - keep_api
//! - walletobjects_api
//! - apphub_api
//! - vmmigration_api
//! - airquality_api
//! - chromemanagement_api
//! - adexperiencereport_api
//! - recommender_api
//! - driveactivity_api
//! - dns_api
//! - cloudshell_api
//! - content_api
//! - commentanalyzer_api
//! - backupdr_api
//! - groupsmigration_api
//! - connectors_api
//! - solar_api
//! - fcm_api
//! - networksecurity_api
//! - managedidentities_api
//! - serviceusage_api
//! - digitalassetlinks_api
//! - clouddebugger_api
//! - datamanager_api
//! - ids_api
//! - workflows_api
//! - cloudtrace_api
//! - advisorynotifications_api
//! - fcmdata_api
//! - checks_api
//! - blogger_3
//! - mediaupload
//! - paymentsresellersubscription_api
//! - spanner_api
//! - cloudchannel_api
//! - gameservices_api
//! - cloudprivatecatalog_api
//! - gmailpostmastertools_api
//! - partners_api
//! - any
//! - runtimeconfig_api
//! - retail_api
//! - books_api
//! - websecurityscanner_api
//! - localservices_api
//! - servicedirectory_api
//! - customsearch_api
//! - policyanalyzer_api
//! - mybusinessbusinesscalls_api
//! - ml_api
//! - firebaseml_api
//! - blogger_api
//! - vault_api
//! - baremetalsolution_api
//! - testing_api
//! - recaptchaenterprise_api
//! - servicecontrol_api
//! - documentai_api
//! - firebasedynamiclinks_api
//! - apigee_api
//! - netapp_api
//! - identitytoolkit_api
//! - datastream_api
//! - iamcredentials_api
//! - datalabeling_api
//! - youtubereporting_api
//! - genomics_api
//! - lifesciences_api
//! - compute_api
//! - youtubeanalytics_api
//! - factchecktools_api
//! - dialogflow_api
//! - docs_api
//! - tagmanager_api
//! - texttospeech_api
//! - gmail_api
//! - tasks_api
//! - securityposture_api
//! - sql_api
//! - integrations_api
//! - essentialcontacts_api
//! - tpu_api
//! - classroom_api
//! - mybusinesslodging_api
//! - rapidmigrationassessment_api
//! - blockchainnodeengine_api


pub mod translate_api;
pub mod area120tables_api;
pub mod abusiveexperiencereport_api;
pub mod securesourcemanager_api;
pub mod file_api;
pub mod oauth2_api;
pub mod serviceuser_api;
pub mod bigquerydatatransfer_api;
pub mod cloudfunctions_api;
pub mod networkconnectivity_api;
pub mod mybusinessverifications_api;
pub mod qpxexpress_api;
pub mod dataproc_api;
pub mod playgrouping_api;
pub mod mirror_api;
pub mod adexchangeseller_api;
pub mod binaryauthorization_api;
pub mod marketingplatformadmin_api;
pub mod cloudprivatecatalogproducer_api;
pub mod pubsublite_api;
pub mod slides_api;
pub mod billingbudgets_api;
pub mod workstations_api;
pub mod civicinfo_api;
pub mod spectrum_api;
pub mod surveys_api;
pub mod dataplex_api;
pub mod redis_api;
pub mod addressvalidation_api;
pub mod sourcerepo_api;
pub mod osconfig_api;
pub mod domainsrdap_api;
pub mod speech_api;
pub mod trafficdirector_api;
pub mod indexing_api;
pub mod ondemandscanning_api;
pub mod drive_api;
pub mod mybusinessnotifications_api;
pub mod workflowexecutions_api;
pub mod developerconnect_api;
pub mod analyticsadmin_api;
pub mod playdeveloperreporting_api;
pub mod prod_tt_sasportal_api;
pub mod searchconsole_api;
pub mod firebaseremoteconfig_api;
pub mod publicca_api;
pub mod discovery_api;
pub mod firebasehosting_api;
pub mod memcache_api;
pub mod playablelocations_api;
pub mod jobs_api;
pub mod iam_api;
pub mod reseller_api;
pub mod realtimebidding_api;
pub mod datamigration_api;
pub mod safebrowsing_api;
pub mod libraryagent_api;
pub mod css_api;
pub mod androidpublisher_api;
pub mod datacatalog_api;
pub mod cloudlocationfinder_api;
pub mod storagetransfer_api;
pub mod resourcesettings_api;
pub mod firebasedatabase_api;
pub mod analyticshub_api;
pub mod http_body;
pub mod managedkafka_api;
pub mod poly_api;
pub mod orgpolicy_api;
pub mod certificatemanager_api;
pub mod apigateway_api;
pub mod datastore_api;
pub mod cloudtasks_api;
pub mod firebaseappdistribution_api;
pub mod domains_api;
pub mod policysimulator_api;
pub mod apihub_api;
pub mod mybusinessaccountmanagement_api;
pub mod smartdevicemanagement_api;
pub mod tracing_api;
pub mod gamesmanagement_api;
pub mod firestore_api;
pub mod language_api;
pub mod json_body;
pub mod replicapoolupdater_api;
pub mod androidmanagement_api;
pub mod licensing_api;
pub mod sasportal_api;
pub mod vmwareengine_api;
pub mod pollen_api;
pub mod cloudsupport_api;
pub mod servicemanagement_api;
pub mod cloudiot_api;
pub mod androiddeviceprovisioning_api;
pub mod gamesconfiguration_api;
pub mod areainsights_api;
pub mod meet_api;
pub mod adsense_api;
pub mod toolresults_api;
pub mod artifactregistry_api;
pub mod cloudcommerceprocurement_api;
pub mod alertcenter_api;
pub mod mybusinessbusinessinformation_api;
pub mod bigquerydatapolicy_api;
pub mod androidenterprise_api;
pub mod playmoviespartner_api;
pub mod oracledatabase_api;
pub mod composer_api;
pub mod dataform_api;
pub mod adsensehost_api;
pub mod deploymentmanager_api;
pub mod displayvideo_api;
pub mod chromewebstore_api;
pub mod cloudkms_api;
pub mod bigtableadmin_api;
pub mod drivelabels_api;
pub mod vision_api;
pub mod streetviewpublish_api;
pub mod clouddeploy_api;
pub mod biglake_api;
pub mod eventarc_api;
pub mod apigeeregistry_api;
pub mod cloudbuild_api;
pub mod secretmanager_api;
pub mod sheets_api;
pub mod recommendationengine_api;
pub mod appengine_api;
pub mod dataportability_api;
pub mod remotebuildexecution_api;
pub mod containeranalysis_api;
pub mod workloadmanager_api;
pub mod cloudcontrolspartner_api;
pub mod monitoring_api;
pub mod privateca_api;
pub mod container_api;
pub mod merchantapi_api;
pub mod datapipelines_api;
pub mod sts_api;
pub mod observability_api;
pub mod dataflow_api;
pub mod cloudresourcemanager_api;
pub mod firebaseapphosting_api;
pub mod verifiedaccess_api;
pub mod accessapproval_api;
pub mod firebasedataconnect_api;
pub mod assuredworkloads_api;
pub mod datafusion_api;
pub mod doubleclicksearch_api;
pub mod ideahub_api;
pub mod alloydb_api;
pub mod doubleclickbidmanager_api;
pub mod bigqueryreservation_api;
pub mod notebooks_api;
pub mod readerrevenuesubscriptionlinking_api;
pub mod authorizedbuyersmarketplace_api;
pub mod adexchangebuyer2_api;
pub mod adsenseplatform_api;
pub mod mybusinessplaceactions_api;
pub mod firebasestorage_api;
pub mod calendar_api;
pub mod networkmanagement_api;
pub mod admin_api;
pub mod acceleratedmobilepageurl_api;
pub mod fusiontables_api;
pub mod chromepolicy_api;
pub mod securitycenter_api;
pub mod analyticsreporting_api;
pub mod iap_api;
pub mod consumersurveys_api;
pub mod siteverification_api;
pub mod versionhistory_api;
pub mod storagebatchoperations_api;
pub mod acmedns_api;
pub mod storage_api;
pub mod networkservices_api;
pub mod dfareporting_api;
pub mod webfonts_api;
pub mod apim_api;
pub mod chromeuxreport_api;
pub mod aiplatform_api;
pub mod script_api;
pub mod contentwarehouse_api;
pub mod batch_api;
pub mod kmsinventory_api;
pub mod run_api;
pub mod servicenetworking_api;
pub mod cloudscheduler_api;
pub mod cloudsearch_api;
pub mod admob_api;
pub mod workspaceevents_api;
pub mod metastore_api;
pub mod policytroubleshooter_api;
pub mod analyticsdata_api;
pub mod contactcenteraiplatform_api;
pub mod forms_api;
pub mod clouderrorreporting_api;
pub mod pagespeedonline_api;
pub mod parallelstore_api;
pub mod cloudbilling_api;
pub mod cloudidentity_api;
pub mod appsactivity_api;
pub mod groupssettings_api;
pub mod adexchangebuyer_api;
pub mod youtube_api;
pub mod looker_api;
pub mod gkebackup_api;
pub mod cloudasset_api;
pub mod people_api;
pub mod mybusinessqanda_api;
pub mod beyondcorp_api;
pub mod pubsub_api;
pub mod servicebroker_api;
pub mod contactcenterinsights_api;
pub mod accesscontextmanager_api;
pub mod searchads360_api;
pub mod oslogin_api;
pub mod playcustomapp_api;
pub mod webmasters_api;
pub mod fitness_api;
pub mod parametermanager_api;
pub mod businessprofileperformance_api;
pub mod videointelligence_api;
pub mod replicapool_api;
pub mod dlp_api;
pub mod migrationcenter_api;
pub mod kgsearch_api;
pub mod cloudprofiler_api;
pub mod healthcare_api;
pub mod firebaseappcheck_api;
pub mod gkeonprem_api;
pub mod repeated_any_query_error;
pub mod transcoder_api;
pub mod firebaserules_api;
pub mod plusdomains_api;
pub mod proximitybeacon_api;
pub mod firebase_api;
pub mod resource_named_service;
pub mod config_api;
pub mod games_api;
pub mod appstate_api;
pub mod saasservicemgmt_api;
pub mod test_api;
pub mod gkehub_api;
pub mod analytics_api;
pub mod discoveryengine_api;
pub mod playintegrity_api;
pub mod homegraph_api;
pub mod apikeys_api;
pub mod serviceconsumermanagement_api;
pub mod datalineage_api;
pub mod manufacturers_api;
pub mod vectortile_api;
pub mod chat_api;
pub mod bigquery_api;
pub mod plus_api;
pub mod bigqueryconnection_api;
pub mod places_api;
pub mod logging_api;
pub mod webrisk_api;
pub mod vpcaccess_api;
pub mod urlshortener_api;
pub mod sqladmin_api;
pub mod keep_api;
pub mod walletobjects_api;
pub mod apphub_api;
pub mod vmmigration_api;
pub mod airquality_api;
pub mod chromemanagement_api;
pub mod adexperiencereport_api;
pub mod recommender_api;
pub mod driveactivity_api;
pub mod dns_api;
pub mod cloudshell_api;
pub mod content_api;
pub mod commentanalyzer_api;
pub mod backupdr_api;
pub mod groupsmigration_api;
pub mod connectors_api;
pub mod solar_api;
pub mod fcm_api;
pub mod networksecurity_api;
pub mod managedidentities_api;
pub mod serviceusage_api;
pub mod digitalassetlinks_api;
pub mod clouddebugger_api;
pub mod datamanager_api;
pub mod ids_api;
pub mod workflows_api;
pub mod cloudtrace_api;
pub mod advisorynotifications_api;
pub mod fcmdata_api;
pub mod checks_api;
pub mod blogger_3;
pub mod mediaupload;
pub mod paymentsresellersubscription_api;
pub mod spanner_api;
pub mod cloudchannel_api;
pub mod gameservices_api;
pub mod cloudprivatecatalog_api;
pub mod gmailpostmastertools_api;
pub mod partners_api;
pub mod any;
pub mod runtimeconfig_api;
pub mod retail_api;
pub mod books_api;
pub mod websecurityscanner_api;
pub mod localservices_api;
pub mod servicedirectory_api;
pub mod customsearch_api;
pub mod policyanalyzer_api;
pub mod mybusinessbusinesscalls_api;
pub mod ml_api;
pub mod firebaseml_api;
pub mod blogger_api;
pub mod vault_api;
pub mod baremetalsolution_api;
pub mod testing_api;
pub mod recaptchaenterprise_api;
pub mod servicecontrol_api;
pub mod documentai_api;
pub mod firebasedynamiclinks_api;
pub mod apigee_api;
pub mod netapp_api;
pub mod identitytoolkit_api;
pub mod datastream_api;
pub mod iamcredentials_api;
pub mod datalabeling_api;
pub mod youtubereporting_api;
pub mod genomics_api;
pub mod lifesciences_api;
pub mod compute_api;
pub mod youtubeanalytics_api;
pub mod factchecktools_api;
pub mod dialogflow_api;
pub mod docs_api;
pub mod tagmanager_api;
pub mod texttospeech_api;
pub mod gmail_api;
pub mod tasks_api;
pub mod securityposture_api;
pub mod sql_api;
pub mod integrations_api;
pub mod essentialcontacts_api;
pub mod tpu_api;
pub mod classroom_api;
pub mod mybusinesslodging_api;
pub mod rapidmigrationassessment_api;
pub mod blockchainnodeengine_api;


use async_trait::async_trait;
use hemmer_core::Result;
use hemmer_provider::{ProviderConfig, ProviderExecutor, ResourceInput, ResourceOutput, ResourcePlan};
use thiserror::Error;

/// Provider error types
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("SDK error: {0}")]
    SdkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;

/// Unified provider client for Gcp
pub struct GcpProvider {
    // GCP clients
    // translate_api_client: google_translate_api::Client,
    // area120tables_api_client: google_area120tables_api::Client,
    // abusiveexperiencereport_api_client: google_abusiveexperiencereport_api::Client,
    // securesourcemanager_api_client: google_securesourcemanager_api::Client,
    // file_api_client: google_file_api::Client,
    // oauth2_api_client: google_oauth2_api::Client,
    // serviceuser_api_client: google_serviceuser_api::Client,
    // bigquerydatatransfer_api_client: google_bigquerydatatransfer_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // networkconnectivity_api_client: google_networkconnectivity_api::Client,
    // mybusinessverifications_api_client: google_mybusinessverifications_api::Client,
    // qpxexpress_api_client: google_qpxexpress_api::Client,
    // dataproc_api_client: google_dataproc_api::Client,
    // playgrouping_api_client: google_playgrouping_api::Client,
    // mirror_api_client: google_mirror_api::Client,
    // adexchangeseller_api_client: google_adexchangeseller_api::Client,
    // binaryauthorization_api_client: google_binaryauthorization_api::Client,
    // marketingplatformadmin_api_client: google_marketingplatformadmin_api::Client,
    // cloudprivatecatalogproducer_api_client: google_cloudprivatecatalogproducer_api::Client,
    // pubsublite_api_client: google_pubsublite_api::Client,
    // slides_api_client: google_slides_api::Client,
    // billingbudgets_api_client: google_billingbudgets_api::Client,
    // workstations_api_client: google_workstations_api::Client,
    // civicinfo_api_client: google_civicinfo_api::Client,
    // spectrum_api_client: google_spectrum_api::Client,
    // surveys_api_client: google_surveys_api::Client,
    // dataplex_api_client: google_dataplex_api::Client,
    // redis_api_client: google_redis_api::Client,
    // addressvalidation_api_client: google_addressvalidation_api::Client,
    // sourcerepo_api_client: google_sourcerepo_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // domainsrdap_api_client: google_domainsrdap_api::Client,
    // speech_api_client: google_speech_api::Client,
    // trafficdirector_api_client: google_trafficdirector_api::Client,
    // indexing_api_client: google_indexing_api::Client,
    // ondemandscanning_api_client: google_ondemandscanning_api::Client,
    // drive_api_client: google_drive_api::Client,
    // mybusinessnotifications_api_client: google_mybusinessnotifications_api::Client,
    // workflowexecutions_api_client: google_workflowexecutions_api::Client,
    // developerconnect_api_client: google_developerconnect_api::Client,
    // analyticsadmin_api_client: google_analyticsadmin_api::Client,
    // playdeveloperreporting_api_client: google_playdeveloperreporting_api::Client,
    // prod_tt_sasportal_api_client: google_prod_tt_sasportal_api::Client,
    // searchconsole_api_client: google_searchconsole_api::Client,
    // firebaseremoteconfig_api_client: google_firebaseremoteconfig_api::Client,
    // publicca_api_client: google_publicca_api::Client,
    // discovery_api_client: google_discovery_api::Client,
    // firebasehosting_api_client: google_firebasehosting_api::Client,
    // memcache_api_client: google_memcache_api::Client,
    // playablelocations_api_client: google_playablelocations_api::Client,
    // jobs_api_client: google_jobs_api::Client,
    // iam_api_client: google_iam_api::Client,
    // reseller_api_client: google_reseller_api::Client,
    // realtimebidding_api_client: google_realtimebidding_api::Client,
    // datamigration_api_client: google_datamigration_api::Client,
    // safebrowsing_api_client: google_safebrowsing_api::Client,
    // libraryagent_api_client: google_libraryagent_api::Client,
    // css_api_client: google_css_api::Client,
    // androidpublisher_api_client: google_androidpublisher_api::Client,
    // datacatalog_api_client: google_datacatalog_api::Client,
    // cloudlocationfinder_api_client: google_cloudlocationfinder_api::Client,
    // storagetransfer_api_client: google_storagetransfer_api::Client,
    // resourcesettings_api_client: google_resourcesettings_api::Client,
    // firebasedatabase_api_client: google_firebasedatabase_api::Client,
    // analyticshub_api_client: google_analyticshub_api::Client,
    // http_body_client: google_http_body::Client,
    // managedkafka_api_client: google_managedkafka_api::Client,
    // poly_api_client: google_poly_api::Client,
    // orgpolicy_api_client: google_orgpolicy_api::Client,
    // certificatemanager_api_client: google_certificatemanager_api::Client,
    // apigateway_api_client: google_apigateway_api::Client,
    // datastore_api_client: google_datastore_api::Client,
    // cloudtasks_api_client: google_cloudtasks_api::Client,
    // firebaseappdistribution_api_client: google_firebaseappdistribution_api::Client,
    // domains_api_client: google_domains_api::Client,
    // policysimulator_api_client: google_policysimulator_api::Client,
    // apihub_api_client: google_apihub_api::Client,
    // mybusinessaccountmanagement_api_client: google_mybusinessaccountmanagement_api::Client,
    // smartdevicemanagement_api_client: google_smartdevicemanagement_api::Client,
    // tracing_api_client: google_tracing_api::Client,
    // gamesmanagement_api_client: google_gamesmanagement_api::Client,
    // firestore_api_client: google_firestore_api::Client,
    // language_api_client: google_language_api::Client,
    // json_body_client: google_json_body::Client,
    // replicapoolupdater_api_client: google_replicapoolupdater_api::Client,
    // androidmanagement_api_client: google_androidmanagement_api::Client,
    // licensing_api_client: google_licensing_api::Client,
    // sasportal_api_client: google_sasportal_api::Client,
    // vmwareengine_api_client: google_vmwareengine_api::Client,
    // pollen_api_client: google_pollen_api::Client,
    // cloudsupport_api_client: google_cloudsupport_api::Client,
    // servicemanagement_api_client: google_servicemanagement_api::Client,
    // cloudiot_api_client: google_cloudiot_api::Client,
    // androiddeviceprovisioning_api_client: google_androiddeviceprovisioning_api::Client,
    // gamesconfiguration_api_client: google_gamesconfiguration_api::Client,
    // areainsights_api_client: google_areainsights_api::Client,
    // meet_api_client: google_meet_api::Client,
    // adsense_api_client: google_adsense_api::Client,
    // toolresults_api_client: google_toolresults_api::Client,
    // artifactregistry_api_client: google_artifactregistry_api::Client,
    // cloudcommerceprocurement_api_client: google_cloudcommerceprocurement_api::Client,
    // alertcenter_api_client: google_alertcenter_api::Client,
    // mybusinessbusinessinformation_api_client: google_mybusinessbusinessinformation_api::Client,
    // bigquerydatapolicy_api_client: google_bigquerydatapolicy_api::Client,
    // androidenterprise_api_client: google_androidenterprise_api::Client,
    // playmoviespartner_api_client: google_playmoviespartner_api::Client,
    // oracledatabase_api_client: google_oracledatabase_api::Client,
    // composer_api_client: google_composer_api::Client,
    // dataform_api_client: google_dataform_api::Client,
    // adsensehost_api_client: google_adsensehost_api::Client,
    // deploymentmanager_api_client: google_deploymentmanager_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // chromewebstore_api_client: google_chromewebstore_api::Client,
    // cloudkms_api_client: google_cloudkms_api::Client,
    // bigtableadmin_api_client: google_bigtableadmin_api::Client,
    // drivelabels_api_client: google_drivelabels_api::Client,
    // vision_api_client: google_vision_api::Client,
    // streetviewpublish_api_client: google_streetviewpublish_api::Client,
    // clouddeploy_api_client: google_clouddeploy_api::Client,
    // biglake_api_client: google_biglake_api::Client,
    // eventarc_api_client: google_eventarc_api::Client,
    // apigeeregistry_api_client: google_apigeeregistry_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // secretmanager_api_client: google_secretmanager_api::Client,
    // sheets_api_client: google_sheets_api::Client,
    // recommendationengine_api_client: google_recommendationengine_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // dataportability_api_client: google_dataportability_api::Client,
    // remotebuildexecution_api_client: google_remotebuildexecution_api::Client,
    // containeranalysis_api_client: google_containeranalysis_api::Client,
    // workloadmanager_api_client: google_workloadmanager_api::Client,
    // cloudcontrolspartner_api_client: google_cloudcontrolspartner_api::Client,
    // monitoring_api_client: google_monitoring_api::Client,
    // privateca_api_client: google_privateca_api::Client,
    // container_api_client: google_container_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // datapipelines_api_client: google_datapipelines_api::Client,
    // sts_api_client: google_sts_api::Client,
    // observability_api_client: google_observability_api::Client,
    // dataflow_api_client: google_dataflow_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // firebaseapphosting_api_client: google_firebaseapphosting_api::Client,
    // verifiedaccess_api_client: google_verifiedaccess_api::Client,
    // accessapproval_api_client: google_accessapproval_api::Client,
    // firebasedataconnect_api_client: google_firebasedataconnect_api::Client,
    // assuredworkloads_api_client: google_assuredworkloads_api::Client,
    // datafusion_api_client: google_datafusion_api::Client,
    // doubleclicksearch_api_client: google_doubleclicksearch_api::Client,
    // ideahub_api_client: google_ideahub_api::Client,
    // alloydb_api_client: google_alloydb_api::Client,
    // doubleclickbidmanager_api_client: google_doubleclickbidmanager_api::Client,
    // bigqueryreservation_api_client: google_bigqueryreservation_api::Client,
    // notebooks_api_client: google_notebooks_api::Client,
    // readerrevenuesubscriptionlinking_api_client: google_readerrevenuesubscriptionlinking_api::Client,
    // authorizedbuyersmarketplace_api_client: google_authorizedbuyersmarketplace_api::Client,
    // adexchangebuyer2_api_client: google_adexchangebuyer2_api::Client,
    // adsenseplatform_api_client: google_adsenseplatform_api::Client,
    // mybusinessplaceactions_api_client: google_mybusinessplaceactions_api::Client,
    // firebasestorage_api_client: google_firebasestorage_api::Client,
    // calendar_api_client: google_calendar_api::Client,
    // networkmanagement_api_client: google_networkmanagement_api::Client,
    // admin_api_client: google_admin_api::Client,
    // acceleratedmobilepageurl_api_client: google_acceleratedmobilepageurl_api::Client,
    // fusiontables_api_client: google_fusiontables_api::Client,
    // chromepolicy_api_client: google_chromepolicy_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // analyticsreporting_api_client: google_analyticsreporting_api::Client,
    // iap_api_client: google_iap_api::Client,
    // consumersurveys_api_client: google_consumersurveys_api::Client,
    // siteverification_api_client: google_siteverification_api::Client,
    // versionhistory_api_client: google_versionhistory_api::Client,
    // storagebatchoperations_api_client: google_storagebatchoperations_api::Client,
    // acmedns_api_client: google_acmedns_api::Client,
    // storage_api_client: google_storage_api::Client,
    // networkservices_api_client: google_networkservices_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // webfonts_api_client: google_webfonts_api::Client,
    // apim_api_client: google_apim_api::Client,
    // chromeuxreport_api_client: google_chromeuxreport_api::Client,
    // aiplatform_api_client: google_aiplatform_api::Client,
    // script_api_client: google_script_api::Client,
    // contentwarehouse_api_client: google_contentwarehouse_api::Client,
    // batch_api_client: google_batch_api::Client,
    // kmsinventory_api_client: google_kmsinventory_api::Client,
    // run_api_client: google_run_api::Client,
    // servicenetworking_api_client: google_servicenetworking_api::Client,
    // cloudscheduler_api_client: google_cloudscheduler_api::Client,
    // cloudsearch_api_client: google_cloudsearch_api::Client,
    // admob_api_client: google_admob_api::Client,
    // workspaceevents_api_client: google_workspaceevents_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // policytroubleshooter_api_client: google_policytroubleshooter_api::Client,
    // analyticsdata_api_client: google_analyticsdata_api::Client,
    // contactcenteraiplatform_api_client: google_contactcenteraiplatform_api::Client,
    // forms_api_client: google_forms_api::Client,
    // clouderrorreporting_api_client: google_clouderrorreporting_api::Client,
    // pagespeedonline_api_client: google_pagespeedonline_api::Client,
    // parallelstore_api_client: google_parallelstore_api::Client,
    // cloudbilling_api_client: google_cloudbilling_api::Client,
    // cloudidentity_api_client: google_cloudidentity_api::Client,
    // appsactivity_api_client: google_appsactivity_api::Client,
    // groupssettings_api_client: google_groupssettings_api::Client,
    // adexchangebuyer_api_client: google_adexchangebuyer_api::Client,
    // youtube_api_client: google_youtube_api::Client,
    // looker_api_client: google_looker_api::Client,
    // gkebackup_api_client: google_gkebackup_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // people_api_client: google_people_api::Client,
    // mybusinessqanda_api_client: google_mybusinessqanda_api::Client,
    // beyondcorp_api_client: google_beyondcorp_api::Client,
    // pubsub_api_client: google_pubsub_api::Client,
    // servicebroker_api_client: google_servicebroker_api::Client,
    // contactcenterinsights_api_client: google_contactcenterinsights_api::Client,
    // accesscontextmanager_api_client: google_accesscontextmanager_api::Client,
    // searchads360_api_client: google_searchads360_api::Client,
    // oslogin_api_client: google_oslogin_api::Client,
    // playcustomapp_api_client: google_playcustomapp_api::Client,
    // webmasters_api_client: google_webmasters_api::Client,
    // fitness_api_client: google_fitness_api::Client,
    // parametermanager_api_client: google_parametermanager_api::Client,
    // businessprofileperformance_api_client: google_businessprofileperformance_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // replicapool_api_client: google_replicapool_api::Client,
    // dlp_api_client: google_dlp_api::Client,
    // migrationcenter_api_client: google_migrationcenter_api::Client,
    // kgsearch_api_client: google_kgsearch_api::Client,
    // cloudprofiler_api_client: google_cloudprofiler_api::Client,
    // healthcare_api_client: google_healthcare_api::Client,
    // firebaseappcheck_api_client: google_firebaseappcheck_api::Client,
    // gkeonprem_api_client: google_gkeonprem_api::Client,
    // repeated_any_query_error_client: google_repeated_any_query_error::Client,
    // transcoder_api_client: google_transcoder_api::Client,
    // firebaserules_api_client: google_firebaserules_api::Client,
    // plusdomains_api_client: google_plusdomains_api::Client,
    // proximitybeacon_api_client: google_proximitybeacon_api::Client,
    // firebase_api_client: google_firebase_api::Client,
    // resource_named_service_client: google_resource_named_service::Client,
    // config_api_client: google_config_api::Client,
    // games_api_client: google_games_api::Client,
    // appstate_api_client: google_appstate_api::Client,
    // saasservicemgmt_api_client: google_saasservicemgmt_api::Client,
    // test_api_client: google_test_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // analytics_api_client: google_analytics_api::Client,
    // discoveryengine_api_client: google_discoveryengine_api::Client,
    // playintegrity_api_client: google_playintegrity_api::Client,
    // homegraph_api_client: google_homegraph_api::Client,
    // apikeys_api_client: google_apikeys_api::Client,
    // serviceconsumermanagement_api_client: google_serviceconsumermanagement_api::Client,
    // datalineage_api_client: google_datalineage_api::Client,
    // manufacturers_api_client: google_manufacturers_api::Client,
    // vectortile_api_client: google_vectortile_api::Client,
    // chat_api_client: google_chat_api::Client,
    // bigquery_api_client: google_bigquery_api::Client,
    // plus_api_client: google_plus_api::Client,
    // bigqueryconnection_api_client: google_bigqueryconnection_api::Client,
    // places_api_client: google_places_api::Client,
    // logging_api_client: google_logging_api::Client,
    // webrisk_api_client: google_webrisk_api::Client,
    // vpcaccess_api_client: google_vpcaccess_api::Client,
    // urlshortener_api_client: google_urlshortener_api::Client,
    // sqladmin_api_client: google_sqladmin_api::Client,
    // keep_api_client: google_keep_api::Client,
    // walletobjects_api_client: google_walletobjects_api::Client,
    // apphub_api_client: google_apphub_api::Client,
    // vmmigration_api_client: google_vmmigration_api::Client,
    // airquality_api_client: google_airquality_api::Client,
    // chromemanagement_api_client: google_chromemanagement_api::Client,
    // adexperiencereport_api_client: google_adexperiencereport_api::Client,
    // recommender_api_client: google_recommender_api::Client,
    // driveactivity_api_client: google_driveactivity_api::Client,
    // dns_api_client: google_dns_api::Client,
    // cloudshell_api_client: google_cloudshell_api::Client,
    // content_api_client: google_content_api::Client,
    // commentanalyzer_api_client: google_commentanalyzer_api::Client,
    // backupdr_api_client: google_backupdr_api::Client,
    // groupsmigration_api_client: google_groupsmigration_api::Client,
    // connectors_api_client: google_connectors_api::Client,
    // solar_api_client: google_solar_api::Client,
    // fcm_api_client: google_fcm_api::Client,
    // networksecurity_api_client: google_networksecurity_api::Client,
    // managedidentities_api_client: google_managedidentities_api::Client,
    // serviceusage_api_client: google_serviceusage_api::Client,
    // digitalassetlinks_api_client: google_digitalassetlinks_api::Client,
    // clouddebugger_api_client: google_clouddebugger_api::Client,
    // datamanager_api_client: google_datamanager_api::Client,
    // ids_api_client: google_ids_api::Client,
    // workflows_api_client: google_workflows_api::Client,
    // cloudtrace_api_client: google_cloudtrace_api::Client,
    // advisorynotifications_api_client: google_advisorynotifications_api::Client,
    // fcmdata_api_client: google_fcmdata_api::Client,
    // checks_api_client: google_checks_api::Client,
    // blogger_3_client: google_blogger_3::Client,
    // mediaupload_client: google_mediaupload::Client,
    // paymentsresellersubscription_api_client: google_paymentsresellersubscription_api::Client,
    // spanner_api_client: google_spanner_api::Client,
    // cloudchannel_api_client: google_cloudchannel_api::Client,
    // gameservices_api_client: google_gameservices_api::Client,
    // cloudprivatecatalog_api_client: google_cloudprivatecatalog_api::Client,
    // gmailpostmastertools_api_client: google_gmailpostmastertools_api::Client,
    // partners_api_client: google_partners_api::Client,
    // any_client: google_any::Client,
    // runtimeconfig_api_client: google_runtimeconfig_api::Client,
    // retail_api_client: google_retail_api::Client,
    // books_api_client: google_books_api::Client,
    // websecurityscanner_api_client: google_websecurityscanner_api::Client,
    // localservices_api_client: google_localservices_api::Client,
    // servicedirectory_api_client: google_servicedirectory_api::Client,
    // customsearch_api_client: google_customsearch_api::Client,
    // policyanalyzer_api_client: google_policyanalyzer_api::Client,
    // mybusinessbusinesscalls_api_client: google_mybusinessbusinesscalls_api::Client,
    // ml_api_client: google_ml_api::Client,
    // firebaseml_api_client: google_firebaseml_api::Client,
    // blogger_api_client: google_blogger_api::Client,
    // vault_api_client: google_vault_api::Client,
    // baremetalsolution_api_client: google_baremetalsolution_api::Client,
    // testing_api_client: google_testing_api::Client,
    // recaptchaenterprise_api_client: google_recaptchaenterprise_api::Client,
    // servicecontrol_api_client: google_servicecontrol_api::Client,
    // documentai_api_client: google_documentai_api::Client,
    // firebasedynamiclinks_api_client: google_firebasedynamiclinks_api::Client,
    // apigee_api_client: google_apigee_api::Client,
    // netapp_api_client: google_netapp_api::Client,
    // identitytoolkit_api_client: google_identitytoolkit_api::Client,
    // datastream_api_client: google_datastream_api::Client,
    // iamcredentials_api_client: google_iamcredentials_api::Client,
    // datalabeling_api_client: google_datalabeling_api::Client,
    // youtubereporting_api_client: google_youtubereporting_api::Client,
    // genomics_api_client: google_genomics_api::Client,
    // lifesciences_api_client: google_lifesciences_api::Client,
    // compute_api_client: google_compute_api::Client,
    // youtubeanalytics_api_client: google_youtubeanalytics_api::Client,
    // factchecktools_api_client: google_factchecktools_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // docs_api_client: google_docs_api::Client,
    // tagmanager_api_client: google_tagmanager_api::Client,
    // texttospeech_api_client: google_texttospeech_api::Client,
    // gmail_api_client: google_gmail_api::Client,
    // tasks_api_client: google_tasks_api::Client,
    // securityposture_api_client: google_securityposture_api::Client,
    // sql_api_client: google_sql_api::Client,
    // integrations_api_client: google_integrations_api::Client,
    // essentialcontacts_api_client: google_essentialcontacts_api::Client,
    // tpu_api_client: google_tpu_api::Client,
    // classroom_api_client: google_classroom_api::Client,
    // mybusinesslodging_api_client: google_mybusinesslodging_api::Client,
    // rapidmigrationassessment_api_client: google_rapidmigrationassessment_api::Client,
    // blockchainnodeengine_api_client: google_blockchainnodeengine_api::Client,
    runtime: tokio::runtime::Runtime,

}

impl GcpProvider {
    /// Create a new unified provider instance
    pub fn new() -> Result<Self> {
        // Create Tokio runtime for async operations
        // This ensures async AWS SDK calls work when the provider is loaded as a dynamic library
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ProviderError::SdkError(format!("Failed to create Tokio runtime: {}", e)))?;

        Ok(Self {

        })
    }

    /// Get translate_api service handler
    pub fn translate_api(&self) -> translate_api::Translate_apiService<'_> {
        translate_api::Translate_apiService::new(self)
    }
    /// Get area120tables_api service handler
    pub fn area120tables_api(&self) -> area120tables_api::Area120tables_apiService<'_> {
        area120tables_api::Area120tables_apiService::new(self)
    }
    /// Get abusiveexperiencereport_api service handler
    pub fn abusiveexperiencereport_api(&self) -> abusiveexperiencereport_api::Abusiveexperiencereport_apiService<'_> {
        abusiveexperiencereport_api::Abusiveexperiencereport_apiService::new(self)
    }
    /// Get securesourcemanager_api service handler
    pub fn securesourcemanager_api(&self) -> securesourcemanager_api::Securesourcemanager_apiService<'_> {
        securesourcemanager_api::Securesourcemanager_apiService::new(self)
    }
    /// Get file_api service handler
    pub fn file_api(&self) -> file_api::File_apiService<'_> {
        file_api::File_apiService::new(self)
    }
    /// Get oauth2_api service handler
    pub fn oauth2_api(&self) -> oauth2_api::Oauth2_apiService<'_> {
        oauth2_api::Oauth2_apiService::new(self)
    }
    /// Get serviceuser_api service handler
    pub fn serviceuser_api(&self) -> serviceuser_api::Serviceuser_apiService<'_> {
        serviceuser_api::Serviceuser_apiService::new(self)
    }
    /// Get bigquerydatatransfer_api service handler
    pub fn bigquerydatatransfer_api(&self) -> bigquerydatatransfer_api::Bigquerydatatransfer_apiService<'_> {
        bigquerydatatransfer_api::Bigquerydatatransfer_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get networkconnectivity_api service handler
    pub fn networkconnectivity_api(&self) -> networkconnectivity_api::Networkconnectivity_apiService<'_> {
        networkconnectivity_api::Networkconnectivity_apiService::new(self)
    }
    /// Get mybusinessverifications_api service handler
    pub fn mybusinessverifications_api(&self) -> mybusinessverifications_api::Mybusinessverifications_apiService<'_> {
        mybusinessverifications_api::Mybusinessverifications_apiService::new(self)
    }
    /// Get qpxexpress_api service handler
    pub fn qpxexpress_api(&self) -> qpxexpress_api::Qpxexpress_apiService<'_> {
        qpxexpress_api::Qpxexpress_apiService::new(self)
    }
    /// Get dataproc_api service handler
    pub fn dataproc_api(&self) -> dataproc_api::Dataproc_apiService<'_> {
        dataproc_api::Dataproc_apiService::new(self)
    }
    /// Get playgrouping_api service handler
    pub fn playgrouping_api(&self) -> playgrouping_api::Playgrouping_apiService<'_> {
        playgrouping_api::Playgrouping_apiService::new(self)
    }
    /// Get mirror_api service handler
    pub fn mirror_api(&self) -> mirror_api::Mirror_apiService<'_> {
        mirror_api::Mirror_apiService::new(self)
    }
    /// Get adexchangeseller_api service handler
    pub fn adexchangeseller_api(&self) -> adexchangeseller_api::Adexchangeseller_apiService<'_> {
        adexchangeseller_api::Adexchangeseller_apiService::new(self)
    }
    /// Get binaryauthorization_api service handler
    pub fn binaryauthorization_api(&self) -> binaryauthorization_api::Binaryauthorization_apiService<'_> {
        binaryauthorization_api::Binaryauthorization_apiService::new(self)
    }
    /// Get marketingplatformadmin_api service handler
    pub fn marketingplatformadmin_api(&self) -> marketingplatformadmin_api::Marketingplatformadmin_apiService<'_> {
        marketingplatformadmin_api::Marketingplatformadmin_apiService::new(self)
    }
    /// Get cloudprivatecatalogproducer_api service handler
    pub fn cloudprivatecatalogproducer_api(&self) -> cloudprivatecatalogproducer_api::Cloudprivatecatalogproducer_apiService<'_> {
        cloudprivatecatalogproducer_api::Cloudprivatecatalogproducer_apiService::new(self)
    }
    /// Get pubsublite_api service handler
    pub fn pubsublite_api(&self) -> pubsublite_api::Pubsublite_apiService<'_> {
        pubsublite_api::Pubsublite_apiService::new(self)
    }
    /// Get slides_api service handler
    pub fn slides_api(&self) -> slides_api::Slides_apiService<'_> {
        slides_api::Slides_apiService::new(self)
    }
    /// Get billingbudgets_api service handler
    pub fn billingbudgets_api(&self) -> billingbudgets_api::Billingbudgets_apiService<'_> {
        billingbudgets_api::Billingbudgets_apiService::new(self)
    }
    /// Get workstations_api service handler
    pub fn workstations_api(&self) -> workstations_api::Workstations_apiService<'_> {
        workstations_api::Workstations_apiService::new(self)
    }
    /// Get civicinfo_api service handler
    pub fn civicinfo_api(&self) -> civicinfo_api::Civicinfo_apiService<'_> {
        civicinfo_api::Civicinfo_apiService::new(self)
    }
    /// Get spectrum_api service handler
    pub fn spectrum_api(&self) -> spectrum_api::Spectrum_apiService<'_> {
        spectrum_api::Spectrum_apiService::new(self)
    }
    /// Get surveys_api service handler
    pub fn surveys_api(&self) -> surveys_api::Surveys_apiService<'_> {
        surveys_api::Surveys_apiService::new(self)
    }
    /// Get dataplex_api service handler
    pub fn dataplex_api(&self) -> dataplex_api::Dataplex_apiService<'_> {
        dataplex_api::Dataplex_apiService::new(self)
    }
    /// Get redis_api service handler
    pub fn redis_api(&self) -> redis_api::Redis_apiService<'_> {
        redis_api::Redis_apiService::new(self)
    }
    /// Get addressvalidation_api service handler
    pub fn addressvalidation_api(&self) -> addressvalidation_api::Addressvalidation_apiService<'_> {
        addressvalidation_api::Addressvalidation_apiService::new(self)
    }
    /// Get sourcerepo_api service handler
    pub fn sourcerepo_api(&self) -> sourcerepo_api::Sourcerepo_apiService<'_> {
        sourcerepo_api::Sourcerepo_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get domainsrdap_api service handler
    pub fn domainsrdap_api(&self) -> domainsrdap_api::Domainsrdap_apiService<'_> {
        domainsrdap_api::Domainsrdap_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get trafficdirector_api service handler
    pub fn trafficdirector_api(&self) -> trafficdirector_api::Trafficdirector_apiService<'_> {
        trafficdirector_api::Trafficdirector_apiService::new(self)
    }
    /// Get indexing_api service handler
    pub fn indexing_api(&self) -> indexing_api::Indexing_apiService<'_> {
        indexing_api::Indexing_apiService::new(self)
    }
    /// Get ondemandscanning_api service handler
    pub fn ondemandscanning_api(&self) -> ondemandscanning_api::Ondemandscanning_apiService<'_> {
        ondemandscanning_api::Ondemandscanning_apiService::new(self)
    }
    /// Get drive_api service handler
    pub fn drive_api(&self) -> drive_api::Drive_apiService<'_> {
        drive_api::Drive_apiService::new(self)
    }
    /// Get mybusinessnotifications_api service handler
    pub fn mybusinessnotifications_api(&self) -> mybusinessnotifications_api::Mybusinessnotifications_apiService<'_> {
        mybusinessnotifications_api::Mybusinessnotifications_apiService::new(self)
    }
    /// Get workflowexecutions_api service handler
    pub fn workflowexecutions_api(&self) -> workflowexecutions_api::Workflowexecutions_apiService<'_> {
        workflowexecutions_api::Workflowexecutions_apiService::new(self)
    }
    /// Get developerconnect_api service handler
    pub fn developerconnect_api(&self) -> developerconnect_api::Developerconnect_apiService<'_> {
        developerconnect_api::Developerconnect_apiService::new(self)
    }
    /// Get analyticsadmin_api service handler
    pub fn analyticsadmin_api(&self) -> analyticsadmin_api::Analyticsadmin_apiService<'_> {
        analyticsadmin_api::Analyticsadmin_apiService::new(self)
    }
    /// Get playdeveloperreporting_api service handler
    pub fn playdeveloperreporting_api(&self) -> playdeveloperreporting_api::Playdeveloperreporting_apiService<'_> {
        playdeveloperreporting_api::Playdeveloperreporting_apiService::new(self)
    }
    /// Get prod_tt_sasportal_api service handler
    pub fn prod_tt_sasportal_api(&self) -> prod_tt_sasportal_api::Prod_tt_sasportal_apiService<'_> {
        prod_tt_sasportal_api::Prod_tt_sasportal_apiService::new(self)
    }
    /// Get searchconsole_api service handler
    pub fn searchconsole_api(&self) -> searchconsole_api::Searchconsole_apiService<'_> {
        searchconsole_api::Searchconsole_apiService::new(self)
    }
    /// Get firebaseremoteconfig_api service handler
    pub fn firebaseremoteconfig_api(&self) -> firebaseremoteconfig_api::Firebaseremoteconfig_apiService<'_> {
        firebaseremoteconfig_api::Firebaseremoteconfig_apiService::new(self)
    }
    /// Get publicca_api service handler
    pub fn publicca_api(&self) -> publicca_api::Publicca_apiService<'_> {
        publicca_api::Publicca_apiService::new(self)
    }
    /// Get discovery_api service handler
    pub fn discovery_api(&self) -> discovery_api::Discovery_apiService<'_> {
        discovery_api::Discovery_apiService::new(self)
    }
    /// Get firebasehosting_api service handler
    pub fn firebasehosting_api(&self) -> firebasehosting_api::Firebasehosting_apiService<'_> {
        firebasehosting_api::Firebasehosting_apiService::new(self)
    }
    /// Get memcache_api service handler
    pub fn memcache_api(&self) -> memcache_api::Memcache_apiService<'_> {
        memcache_api::Memcache_apiService::new(self)
    }
    /// Get playablelocations_api service handler
    pub fn playablelocations_api(&self) -> playablelocations_api::Playablelocations_apiService<'_> {
        playablelocations_api::Playablelocations_apiService::new(self)
    }
    /// Get jobs_api service handler
    pub fn jobs_api(&self) -> jobs_api::Jobs_apiService<'_> {
        jobs_api::Jobs_apiService::new(self)
    }
    /// Get iam_api service handler
    pub fn iam_api(&self) -> iam_api::Iam_apiService<'_> {
        iam_api::Iam_apiService::new(self)
    }
    /// Get reseller_api service handler
    pub fn reseller_api(&self) -> reseller_api::Reseller_apiService<'_> {
        reseller_api::Reseller_apiService::new(self)
    }
    /// Get realtimebidding_api service handler
    pub fn realtimebidding_api(&self) -> realtimebidding_api::Realtimebidding_apiService<'_> {
        realtimebidding_api::Realtimebidding_apiService::new(self)
    }
    /// Get datamigration_api service handler
    pub fn datamigration_api(&self) -> datamigration_api::Datamigration_apiService<'_> {
        datamigration_api::Datamigration_apiService::new(self)
    }
    /// Get safebrowsing_api service handler
    pub fn safebrowsing_api(&self) -> safebrowsing_api::Safebrowsing_apiService<'_> {
        safebrowsing_api::Safebrowsing_apiService::new(self)
    }
    /// Get libraryagent_api service handler
    pub fn libraryagent_api(&self) -> libraryagent_api::Libraryagent_apiService<'_> {
        libraryagent_api::Libraryagent_apiService::new(self)
    }
    /// Get css_api service handler
    pub fn css_api(&self) -> css_api::Css_apiService<'_> {
        css_api::Css_apiService::new(self)
    }
    /// Get androidpublisher_api service handler
    pub fn androidpublisher_api(&self) -> androidpublisher_api::Androidpublisher_apiService<'_> {
        androidpublisher_api::Androidpublisher_apiService::new(self)
    }
    /// Get datacatalog_api service handler
    pub fn datacatalog_api(&self) -> datacatalog_api::Datacatalog_apiService<'_> {
        datacatalog_api::Datacatalog_apiService::new(self)
    }
    /// Get cloudlocationfinder_api service handler
    pub fn cloudlocationfinder_api(&self) -> cloudlocationfinder_api::Cloudlocationfinder_apiService<'_> {
        cloudlocationfinder_api::Cloudlocationfinder_apiService::new(self)
    }
    /// Get storagetransfer_api service handler
    pub fn storagetransfer_api(&self) -> storagetransfer_api::Storagetransfer_apiService<'_> {
        storagetransfer_api::Storagetransfer_apiService::new(self)
    }
    /// Get resourcesettings_api service handler
    pub fn resourcesettings_api(&self) -> resourcesettings_api::Resourcesettings_apiService<'_> {
        resourcesettings_api::Resourcesettings_apiService::new(self)
    }
    /// Get firebasedatabase_api service handler
    pub fn firebasedatabase_api(&self) -> firebasedatabase_api::Firebasedatabase_apiService<'_> {
        firebasedatabase_api::Firebasedatabase_apiService::new(self)
    }
    /// Get analyticshub_api service handler
    pub fn analyticshub_api(&self) -> analyticshub_api::Analyticshub_apiService<'_> {
        analyticshub_api::Analyticshub_apiService::new(self)
    }
    /// Get http_body service handler
    pub fn http_body(&self) -> http_body::Http_bodyService<'_> {
        http_body::Http_bodyService::new(self)
    }
    /// Get managedkafka_api service handler
    pub fn managedkafka_api(&self) -> managedkafka_api::Managedkafka_apiService<'_> {
        managedkafka_api::Managedkafka_apiService::new(self)
    }
    /// Get poly_api service handler
    pub fn poly_api(&self) -> poly_api::Poly_apiService<'_> {
        poly_api::Poly_apiService::new(self)
    }
    /// Get orgpolicy_api service handler
    pub fn orgpolicy_api(&self) -> orgpolicy_api::Orgpolicy_apiService<'_> {
        orgpolicy_api::Orgpolicy_apiService::new(self)
    }
    /// Get certificatemanager_api service handler
    pub fn certificatemanager_api(&self) -> certificatemanager_api::Certificatemanager_apiService<'_> {
        certificatemanager_api::Certificatemanager_apiService::new(self)
    }
    /// Get apigateway_api service handler
    pub fn apigateway_api(&self) -> apigateway_api::Apigateway_apiService<'_> {
        apigateway_api::Apigateway_apiService::new(self)
    }
    /// Get datastore_api service handler
    pub fn datastore_api(&self) -> datastore_api::Datastore_apiService<'_> {
        datastore_api::Datastore_apiService::new(self)
    }
    /// Get cloudtasks_api service handler
    pub fn cloudtasks_api(&self) -> cloudtasks_api::Cloudtasks_apiService<'_> {
        cloudtasks_api::Cloudtasks_apiService::new(self)
    }
    /// Get firebaseappdistribution_api service handler
    pub fn firebaseappdistribution_api(&self) -> firebaseappdistribution_api::Firebaseappdistribution_apiService<'_> {
        firebaseappdistribution_api::Firebaseappdistribution_apiService::new(self)
    }
    /// Get domains_api service handler
    pub fn domains_api(&self) -> domains_api::Domains_apiService<'_> {
        domains_api::Domains_apiService::new(self)
    }
    /// Get policysimulator_api service handler
    pub fn policysimulator_api(&self) -> policysimulator_api::Policysimulator_apiService<'_> {
        policysimulator_api::Policysimulator_apiService::new(self)
    }
    /// Get apihub_api service handler
    pub fn apihub_api(&self) -> apihub_api::Apihub_apiService<'_> {
        apihub_api::Apihub_apiService::new(self)
    }
    /// Get mybusinessaccountmanagement_api service handler
    pub fn mybusinessaccountmanagement_api(&self) -> mybusinessaccountmanagement_api::Mybusinessaccountmanagement_apiService<'_> {
        mybusinessaccountmanagement_api::Mybusinessaccountmanagement_apiService::new(self)
    }
    /// Get smartdevicemanagement_api service handler
    pub fn smartdevicemanagement_api(&self) -> smartdevicemanagement_api::Smartdevicemanagement_apiService<'_> {
        smartdevicemanagement_api::Smartdevicemanagement_apiService::new(self)
    }
    /// Get tracing_api service handler
    pub fn tracing_api(&self) -> tracing_api::Tracing_apiService<'_> {
        tracing_api::Tracing_apiService::new(self)
    }
    /// Get gamesmanagement_api service handler
    pub fn gamesmanagement_api(&self) -> gamesmanagement_api::Gamesmanagement_apiService<'_> {
        gamesmanagement_api::Gamesmanagement_apiService::new(self)
    }
    /// Get firestore_api service handler
    pub fn firestore_api(&self) -> firestore_api::Firestore_apiService<'_> {
        firestore_api::Firestore_apiService::new(self)
    }
    /// Get language_api service handler
    pub fn language_api(&self) -> language_api::Language_apiService<'_> {
        language_api::Language_apiService::new(self)
    }
    /// Get json_body service handler
    pub fn json_body(&self) -> json_body::Json_bodyService<'_> {
        json_body::Json_bodyService::new(self)
    }
    /// Get replicapoolupdater_api service handler
    pub fn replicapoolupdater_api(&self) -> replicapoolupdater_api::Replicapoolupdater_apiService<'_> {
        replicapoolupdater_api::Replicapoolupdater_apiService::new(self)
    }
    /// Get androidmanagement_api service handler
    pub fn androidmanagement_api(&self) -> androidmanagement_api::Androidmanagement_apiService<'_> {
        androidmanagement_api::Androidmanagement_apiService::new(self)
    }
    /// Get licensing_api service handler
    pub fn licensing_api(&self) -> licensing_api::Licensing_apiService<'_> {
        licensing_api::Licensing_apiService::new(self)
    }
    /// Get sasportal_api service handler
    pub fn sasportal_api(&self) -> sasportal_api::Sasportal_apiService<'_> {
        sasportal_api::Sasportal_apiService::new(self)
    }
    /// Get vmwareengine_api service handler
    pub fn vmwareengine_api(&self) -> vmwareengine_api::Vmwareengine_apiService<'_> {
        vmwareengine_api::Vmwareengine_apiService::new(self)
    }
    /// Get pollen_api service handler
    pub fn pollen_api(&self) -> pollen_api::Pollen_apiService<'_> {
        pollen_api::Pollen_apiService::new(self)
    }
    /// Get cloudsupport_api service handler
    pub fn cloudsupport_api(&self) -> cloudsupport_api::Cloudsupport_apiService<'_> {
        cloudsupport_api::Cloudsupport_apiService::new(self)
    }
    /// Get servicemanagement_api service handler
    pub fn servicemanagement_api(&self) -> servicemanagement_api::Servicemanagement_apiService<'_> {
        servicemanagement_api::Servicemanagement_apiService::new(self)
    }
    /// Get cloudiot_api service handler
    pub fn cloudiot_api(&self) -> cloudiot_api::Cloudiot_apiService<'_> {
        cloudiot_api::Cloudiot_apiService::new(self)
    }
    /// Get androiddeviceprovisioning_api service handler
    pub fn androiddeviceprovisioning_api(&self) -> androiddeviceprovisioning_api::Androiddeviceprovisioning_apiService<'_> {
        androiddeviceprovisioning_api::Androiddeviceprovisioning_apiService::new(self)
    }
    /// Get gamesconfiguration_api service handler
    pub fn gamesconfiguration_api(&self) -> gamesconfiguration_api::Gamesconfiguration_apiService<'_> {
        gamesconfiguration_api::Gamesconfiguration_apiService::new(self)
    }
    /// Get areainsights_api service handler
    pub fn areainsights_api(&self) -> areainsights_api::Areainsights_apiService<'_> {
        areainsights_api::Areainsights_apiService::new(self)
    }
    /// Get meet_api service handler
    pub fn meet_api(&self) -> meet_api::Meet_apiService<'_> {
        meet_api::Meet_apiService::new(self)
    }
    /// Get adsense_api service handler
    pub fn adsense_api(&self) -> adsense_api::Adsense_apiService<'_> {
        adsense_api::Adsense_apiService::new(self)
    }
    /// Get toolresults_api service handler
    pub fn toolresults_api(&self) -> toolresults_api::Toolresults_apiService<'_> {
        toolresults_api::Toolresults_apiService::new(self)
    }
    /// Get artifactregistry_api service handler
    pub fn artifactregistry_api(&self) -> artifactregistry_api::Artifactregistry_apiService<'_> {
        artifactregistry_api::Artifactregistry_apiService::new(self)
    }
    /// Get cloudcommerceprocurement_api service handler
    pub fn cloudcommerceprocurement_api(&self) -> cloudcommerceprocurement_api::Cloudcommerceprocurement_apiService<'_> {
        cloudcommerceprocurement_api::Cloudcommerceprocurement_apiService::new(self)
    }
    /// Get alertcenter_api service handler
    pub fn alertcenter_api(&self) -> alertcenter_api::Alertcenter_apiService<'_> {
        alertcenter_api::Alertcenter_apiService::new(self)
    }
    /// Get mybusinessbusinessinformation_api service handler
    pub fn mybusinessbusinessinformation_api(&self) -> mybusinessbusinessinformation_api::Mybusinessbusinessinformation_apiService<'_> {
        mybusinessbusinessinformation_api::Mybusinessbusinessinformation_apiService::new(self)
    }
    /// Get bigquerydatapolicy_api service handler
    pub fn bigquerydatapolicy_api(&self) -> bigquerydatapolicy_api::Bigquerydatapolicy_apiService<'_> {
        bigquerydatapolicy_api::Bigquerydatapolicy_apiService::new(self)
    }
    /// Get androidenterprise_api service handler
    pub fn androidenterprise_api(&self) -> androidenterprise_api::Androidenterprise_apiService<'_> {
        androidenterprise_api::Androidenterprise_apiService::new(self)
    }
    /// Get playmoviespartner_api service handler
    pub fn playmoviespartner_api(&self) -> playmoviespartner_api::Playmoviespartner_apiService<'_> {
        playmoviespartner_api::Playmoviespartner_apiService::new(self)
    }
    /// Get oracledatabase_api service handler
    pub fn oracledatabase_api(&self) -> oracledatabase_api::Oracledatabase_apiService<'_> {
        oracledatabase_api::Oracledatabase_apiService::new(self)
    }
    /// Get composer_api service handler
    pub fn composer_api(&self) -> composer_api::Composer_apiService<'_> {
        composer_api::Composer_apiService::new(self)
    }
    /// Get dataform_api service handler
    pub fn dataform_api(&self) -> dataform_api::Dataform_apiService<'_> {
        dataform_api::Dataform_apiService::new(self)
    }
    /// Get adsensehost_api service handler
    pub fn adsensehost_api(&self) -> adsensehost_api::Adsensehost_apiService<'_> {
        adsensehost_api::Adsensehost_apiService::new(self)
    }
    /// Get deploymentmanager_api service handler
    pub fn deploymentmanager_api(&self) -> deploymentmanager_api::Deploymentmanager_apiService<'_> {
        deploymentmanager_api::Deploymentmanager_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get chromewebstore_api service handler
    pub fn chromewebstore_api(&self) -> chromewebstore_api::Chromewebstore_apiService<'_> {
        chromewebstore_api::Chromewebstore_apiService::new(self)
    }
    /// Get cloudkms_api service handler
    pub fn cloudkms_api(&self) -> cloudkms_api::Cloudkms_apiService<'_> {
        cloudkms_api::Cloudkms_apiService::new(self)
    }
    /// Get bigtableadmin_api service handler
    pub fn bigtableadmin_api(&self) -> bigtableadmin_api::Bigtableadmin_apiService<'_> {
        bigtableadmin_api::Bigtableadmin_apiService::new(self)
    }
    /// Get drivelabels_api service handler
    pub fn drivelabels_api(&self) -> drivelabels_api::Drivelabels_apiService<'_> {
        drivelabels_api::Drivelabels_apiService::new(self)
    }
    /// Get vision_api service handler
    pub fn vision_api(&self) -> vision_api::Vision_apiService<'_> {
        vision_api::Vision_apiService::new(self)
    }
    /// Get streetviewpublish_api service handler
    pub fn streetviewpublish_api(&self) -> streetviewpublish_api::Streetviewpublish_apiService<'_> {
        streetviewpublish_api::Streetviewpublish_apiService::new(self)
    }
    /// Get clouddeploy_api service handler
    pub fn clouddeploy_api(&self) -> clouddeploy_api::Clouddeploy_apiService<'_> {
        clouddeploy_api::Clouddeploy_apiService::new(self)
    }
    /// Get biglake_api service handler
    pub fn biglake_api(&self) -> biglake_api::Biglake_apiService<'_> {
        biglake_api::Biglake_apiService::new(self)
    }
    /// Get eventarc_api service handler
    pub fn eventarc_api(&self) -> eventarc_api::Eventarc_apiService<'_> {
        eventarc_api::Eventarc_apiService::new(self)
    }
    /// Get apigeeregistry_api service handler
    pub fn apigeeregistry_api(&self) -> apigeeregistry_api::Apigeeregistry_apiService<'_> {
        apigeeregistry_api::Apigeeregistry_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get secretmanager_api service handler
    pub fn secretmanager_api(&self) -> secretmanager_api::Secretmanager_apiService<'_> {
        secretmanager_api::Secretmanager_apiService::new(self)
    }
    /// Get sheets_api service handler
    pub fn sheets_api(&self) -> sheets_api::Sheets_apiService<'_> {
        sheets_api::Sheets_apiService::new(self)
    }
    /// Get recommendationengine_api service handler
    pub fn recommendationengine_api(&self) -> recommendationengine_api::Recommendationengine_apiService<'_> {
        recommendationengine_api::Recommendationengine_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get dataportability_api service handler
    pub fn dataportability_api(&self) -> dataportability_api::Dataportability_apiService<'_> {
        dataportability_api::Dataportability_apiService::new(self)
    }
    /// Get remotebuildexecution_api service handler
    pub fn remotebuildexecution_api(&self) -> remotebuildexecution_api::Remotebuildexecution_apiService<'_> {
        remotebuildexecution_api::Remotebuildexecution_apiService::new(self)
    }
    /// Get containeranalysis_api service handler
    pub fn containeranalysis_api(&self) -> containeranalysis_api::Containeranalysis_apiService<'_> {
        containeranalysis_api::Containeranalysis_apiService::new(self)
    }
    /// Get workloadmanager_api service handler
    pub fn workloadmanager_api(&self) -> workloadmanager_api::Workloadmanager_apiService<'_> {
        workloadmanager_api::Workloadmanager_apiService::new(self)
    }
    /// Get cloudcontrolspartner_api service handler
    pub fn cloudcontrolspartner_api(&self) -> cloudcontrolspartner_api::Cloudcontrolspartner_apiService<'_> {
        cloudcontrolspartner_api::Cloudcontrolspartner_apiService::new(self)
    }
    /// Get monitoring_api service handler
    pub fn monitoring_api(&self) -> monitoring_api::Monitoring_apiService<'_> {
        monitoring_api::Monitoring_apiService::new(self)
    }
    /// Get privateca_api service handler
    pub fn privateca_api(&self) -> privateca_api::Privateca_apiService<'_> {
        privateca_api::Privateca_apiService::new(self)
    }
    /// Get container_api service handler
    pub fn container_api(&self) -> container_api::Container_apiService<'_> {
        container_api::Container_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get datapipelines_api service handler
    pub fn datapipelines_api(&self) -> datapipelines_api::Datapipelines_apiService<'_> {
        datapipelines_api::Datapipelines_apiService::new(self)
    }
    /// Get sts_api service handler
    pub fn sts_api(&self) -> sts_api::Sts_apiService<'_> {
        sts_api::Sts_apiService::new(self)
    }
    /// Get observability_api service handler
    pub fn observability_api(&self) -> observability_api::Observability_apiService<'_> {
        observability_api::Observability_apiService::new(self)
    }
    /// Get dataflow_api service handler
    pub fn dataflow_api(&self) -> dataflow_api::Dataflow_apiService<'_> {
        dataflow_api::Dataflow_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get firebaseapphosting_api service handler
    pub fn firebaseapphosting_api(&self) -> firebaseapphosting_api::Firebaseapphosting_apiService<'_> {
        firebaseapphosting_api::Firebaseapphosting_apiService::new(self)
    }
    /// Get verifiedaccess_api service handler
    pub fn verifiedaccess_api(&self) -> verifiedaccess_api::Verifiedaccess_apiService<'_> {
        verifiedaccess_api::Verifiedaccess_apiService::new(self)
    }
    /// Get accessapproval_api service handler
    pub fn accessapproval_api(&self) -> accessapproval_api::Accessapproval_apiService<'_> {
        accessapproval_api::Accessapproval_apiService::new(self)
    }
    /// Get firebasedataconnect_api service handler
    pub fn firebasedataconnect_api(&self) -> firebasedataconnect_api::Firebasedataconnect_apiService<'_> {
        firebasedataconnect_api::Firebasedataconnect_apiService::new(self)
    }
    /// Get assuredworkloads_api service handler
    pub fn assuredworkloads_api(&self) -> assuredworkloads_api::Assuredworkloads_apiService<'_> {
        assuredworkloads_api::Assuredworkloads_apiService::new(self)
    }
    /// Get datafusion_api service handler
    pub fn datafusion_api(&self) -> datafusion_api::Datafusion_apiService<'_> {
        datafusion_api::Datafusion_apiService::new(self)
    }
    /// Get doubleclicksearch_api service handler
    pub fn doubleclicksearch_api(&self) -> doubleclicksearch_api::Doubleclicksearch_apiService<'_> {
        doubleclicksearch_api::Doubleclicksearch_apiService::new(self)
    }
    /// Get ideahub_api service handler
    pub fn ideahub_api(&self) -> ideahub_api::Ideahub_apiService<'_> {
        ideahub_api::Ideahub_apiService::new(self)
    }
    /// Get alloydb_api service handler
    pub fn alloydb_api(&self) -> alloydb_api::Alloydb_apiService<'_> {
        alloydb_api::Alloydb_apiService::new(self)
    }
    /// Get doubleclickbidmanager_api service handler
    pub fn doubleclickbidmanager_api(&self) -> doubleclickbidmanager_api::Doubleclickbidmanager_apiService<'_> {
        doubleclickbidmanager_api::Doubleclickbidmanager_apiService::new(self)
    }
    /// Get bigqueryreservation_api service handler
    pub fn bigqueryreservation_api(&self) -> bigqueryreservation_api::Bigqueryreservation_apiService<'_> {
        bigqueryreservation_api::Bigqueryreservation_apiService::new(self)
    }
    /// Get notebooks_api service handler
    pub fn notebooks_api(&self) -> notebooks_api::Notebooks_apiService<'_> {
        notebooks_api::Notebooks_apiService::new(self)
    }
    /// Get readerrevenuesubscriptionlinking_api service handler
    pub fn readerrevenuesubscriptionlinking_api(&self) -> readerrevenuesubscriptionlinking_api::Readerrevenuesubscriptionlinking_apiService<'_> {
        readerrevenuesubscriptionlinking_api::Readerrevenuesubscriptionlinking_apiService::new(self)
    }
    /// Get authorizedbuyersmarketplace_api service handler
    pub fn authorizedbuyersmarketplace_api(&self) -> authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService<'_> {
        authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService::new(self)
    }
    /// Get adexchangebuyer2_api service handler
    pub fn adexchangebuyer2_api(&self) -> adexchangebuyer2_api::Adexchangebuyer2_apiService<'_> {
        adexchangebuyer2_api::Adexchangebuyer2_apiService::new(self)
    }
    /// Get adsenseplatform_api service handler
    pub fn adsenseplatform_api(&self) -> adsenseplatform_api::Adsenseplatform_apiService<'_> {
        adsenseplatform_api::Adsenseplatform_apiService::new(self)
    }
    /// Get mybusinessplaceactions_api service handler
    pub fn mybusinessplaceactions_api(&self) -> mybusinessplaceactions_api::Mybusinessplaceactions_apiService<'_> {
        mybusinessplaceactions_api::Mybusinessplaceactions_apiService::new(self)
    }
    /// Get firebasestorage_api service handler
    pub fn firebasestorage_api(&self) -> firebasestorage_api::Firebasestorage_apiService<'_> {
        firebasestorage_api::Firebasestorage_apiService::new(self)
    }
    /// Get calendar_api service handler
    pub fn calendar_api(&self) -> calendar_api::Calendar_apiService<'_> {
        calendar_api::Calendar_apiService::new(self)
    }
    /// Get networkmanagement_api service handler
    pub fn networkmanagement_api(&self) -> networkmanagement_api::Networkmanagement_apiService<'_> {
        networkmanagement_api::Networkmanagement_apiService::new(self)
    }
    /// Get admin_api service handler
    pub fn admin_api(&self) -> admin_api::Admin_apiService<'_> {
        admin_api::Admin_apiService::new(self)
    }
    /// Get acceleratedmobilepageurl_api service handler
    pub fn acceleratedmobilepageurl_api(&self) -> acceleratedmobilepageurl_api::Acceleratedmobilepageurl_apiService<'_> {
        acceleratedmobilepageurl_api::Acceleratedmobilepageurl_apiService::new(self)
    }
    /// Get fusiontables_api service handler
    pub fn fusiontables_api(&self) -> fusiontables_api::Fusiontables_apiService<'_> {
        fusiontables_api::Fusiontables_apiService::new(self)
    }
    /// Get chromepolicy_api service handler
    pub fn chromepolicy_api(&self) -> chromepolicy_api::Chromepolicy_apiService<'_> {
        chromepolicy_api::Chromepolicy_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get analyticsreporting_api service handler
    pub fn analyticsreporting_api(&self) -> analyticsreporting_api::Analyticsreporting_apiService<'_> {
        analyticsreporting_api::Analyticsreporting_apiService::new(self)
    }
    /// Get iap_api service handler
    pub fn iap_api(&self) -> iap_api::Iap_apiService<'_> {
        iap_api::Iap_apiService::new(self)
    }
    /// Get consumersurveys_api service handler
    pub fn consumersurveys_api(&self) -> consumersurveys_api::Consumersurveys_apiService<'_> {
        consumersurveys_api::Consumersurveys_apiService::new(self)
    }
    /// Get siteverification_api service handler
    pub fn siteverification_api(&self) -> siteverification_api::Siteverification_apiService<'_> {
        siteverification_api::Siteverification_apiService::new(self)
    }
    /// Get versionhistory_api service handler
    pub fn versionhistory_api(&self) -> versionhistory_api::Versionhistory_apiService<'_> {
        versionhistory_api::Versionhistory_apiService::new(self)
    }
    /// Get storagebatchoperations_api service handler
    pub fn storagebatchoperations_api(&self) -> storagebatchoperations_api::Storagebatchoperations_apiService<'_> {
        storagebatchoperations_api::Storagebatchoperations_apiService::new(self)
    }
    /// Get acmedns_api service handler
    pub fn acmedns_api(&self) -> acmedns_api::Acmedns_apiService<'_> {
        acmedns_api::Acmedns_apiService::new(self)
    }
    /// Get storage_api service handler
    pub fn storage_api(&self) -> storage_api::Storage_apiService<'_> {
        storage_api::Storage_apiService::new(self)
    }
    /// Get networkservices_api service handler
    pub fn networkservices_api(&self) -> networkservices_api::Networkservices_apiService<'_> {
        networkservices_api::Networkservices_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get webfonts_api service handler
    pub fn webfonts_api(&self) -> webfonts_api::Webfonts_apiService<'_> {
        webfonts_api::Webfonts_apiService::new(self)
    }
    /// Get apim_api service handler
    pub fn apim_api(&self) -> apim_api::Apim_apiService<'_> {
        apim_api::Apim_apiService::new(self)
    }
    /// Get chromeuxreport_api service handler
    pub fn chromeuxreport_api(&self) -> chromeuxreport_api::Chromeuxreport_apiService<'_> {
        chromeuxreport_api::Chromeuxreport_apiService::new(self)
    }
    /// Get aiplatform_api service handler
    pub fn aiplatform_api(&self) -> aiplatform_api::Aiplatform_apiService<'_> {
        aiplatform_api::Aiplatform_apiService::new(self)
    }
    /// Get script_api service handler
    pub fn script_api(&self) -> script_api::Script_apiService<'_> {
        script_api::Script_apiService::new(self)
    }
    /// Get contentwarehouse_api service handler
    pub fn contentwarehouse_api(&self) -> contentwarehouse_api::Contentwarehouse_apiService<'_> {
        contentwarehouse_api::Contentwarehouse_apiService::new(self)
    }
    /// Get batch_api service handler
    pub fn batch_api(&self) -> batch_api::Batch_apiService<'_> {
        batch_api::Batch_apiService::new(self)
    }
    /// Get kmsinventory_api service handler
    pub fn kmsinventory_api(&self) -> kmsinventory_api::Kmsinventory_apiService<'_> {
        kmsinventory_api::Kmsinventory_apiService::new(self)
    }
    /// Get run_api service handler
    pub fn run_api(&self) -> run_api::Run_apiService<'_> {
        run_api::Run_apiService::new(self)
    }
    /// Get servicenetworking_api service handler
    pub fn servicenetworking_api(&self) -> servicenetworking_api::Servicenetworking_apiService<'_> {
        servicenetworking_api::Servicenetworking_apiService::new(self)
    }
    /// Get cloudscheduler_api service handler
    pub fn cloudscheduler_api(&self) -> cloudscheduler_api::Cloudscheduler_apiService<'_> {
        cloudscheduler_api::Cloudscheduler_apiService::new(self)
    }
    /// Get cloudsearch_api service handler
    pub fn cloudsearch_api(&self) -> cloudsearch_api::Cloudsearch_apiService<'_> {
        cloudsearch_api::Cloudsearch_apiService::new(self)
    }
    /// Get admob_api service handler
    pub fn admob_api(&self) -> admob_api::Admob_apiService<'_> {
        admob_api::Admob_apiService::new(self)
    }
    /// Get workspaceevents_api service handler
    pub fn workspaceevents_api(&self) -> workspaceevents_api::Workspaceevents_apiService<'_> {
        workspaceevents_api::Workspaceevents_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get policytroubleshooter_api service handler
    pub fn policytroubleshooter_api(&self) -> policytroubleshooter_api::Policytroubleshooter_apiService<'_> {
        policytroubleshooter_api::Policytroubleshooter_apiService::new(self)
    }
    /// Get analyticsdata_api service handler
    pub fn analyticsdata_api(&self) -> analyticsdata_api::Analyticsdata_apiService<'_> {
        analyticsdata_api::Analyticsdata_apiService::new(self)
    }
    /// Get contactcenteraiplatform_api service handler
    pub fn contactcenteraiplatform_api(&self) -> contactcenteraiplatform_api::Contactcenteraiplatform_apiService<'_> {
        contactcenteraiplatform_api::Contactcenteraiplatform_apiService::new(self)
    }
    /// Get forms_api service handler
    pub fn forms_api(&self) -> forms_api::Forms_apiService<'_> {
        forms_api::Forms_apiService::new(self)
    }
    /// Get clouderrorreporting_api service handler
    pub fn clouderrorreporting_api(&self) -> clouderrorreporting_api::Clouderrorreporting_apiService<'_> {
        clouderrorreporting_api::Clouderrorreporting_apiService::new(self)
    }
    /// Get pagespeedonline_api service handler
    pub fn pagespeedonline_api(&self) -> pagespeedonline_api::Pagespeedonline_apiService<'_> {
        pagespeedonline_api::Pagespeedonline_apiService::new(self)
    }
    /// Get parallelstore_api service handler
    pub fn parallelstore_api(&self) -> parallelstore_api::Parallelstore_apiService<'_> {
        parallelstore_api::Parallelstore_apiService::new(self)
    }
    /// Get cloudbilling_api service handler
    pub fn cloudbilling_api(&self) -> cloudbilling_api::Cloudbilling_apiService<'_> {
        cloudbilling_api::Cloudbilling_apiService::new(self)
    }
    /// Get cloudidentity_api service handler
    pub fn cloudidentity_api(&self) -> cloudidentity_api::Cloudidentity_apiService<'_> {
        cloudidentity_api::Cloudidentity_apiService::new(self)
    }
    /// Get appsactivity_api service handler
    pub fn appsactivity_api(&self) -> appsactivity_api::Appsactivity_apiService<'_> {
        appsactivity_api::Appsactivity_apiService::new(self)
    }
    /// Get groupssettings_api service handler
    pub fn groupssettings_api(&self) -> groupssettings_api::Groupssettings_apiService<'_> {
        groupssettings_api::Groupssettings_apiService::new(self)
    }
    /// Get adexchangebuyer_api service handler
    pub fn adexchangebuyer_api(&self) -> adexchangebuyer_api::Adexchangebuyer_apiService<'_> {
        adexchangebuyer_api::Adexchangebuyer_apiService::new(self)
    }
    /// Get youtube_api service handler
    pub fn youtube_api(&self) -> youtube_api::Youtube_apiService<'_> {
        youtube_api::Youtube_apiService::new(self)
    }
    /// Get looker_api service handler
    pub fn looker_api(&self) -> looker_api::Looker_apiService<'_> {
        looker_api::Looker_apiService::new(self)
    }
    /// Get gkebackup_api service handler
    pub fn gkebackup_api(&self) -> gkebackup_api::Gkebackup_apiService<'_> {
        gkebackup_api::Gkebackup_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get people_api service handler
    pub fn people_api(&self) -> people_api::People_apiService<'_> {
        people_api::People_apiService::new(self)
    }
    /// Get mybusinessqanda_api service handler
    pub fn mybusinessqanda_api(&self) -> mybusinessqanda_api::Mybusinessqanda_apiService<'_> {
        mybusinessqanda_api::Mybusinessqanda_apiService::new(self)
    }
    /// Get beyondcorp_api service handler
    pub fn beyondcorp_api(&self) -> beyondcorp_api::Beyondcorp_apiService<'_> {
        beyondcorp_api::Beyondcorp_apiService::new(self)
    }
    /// Get pubsub_api service handler
    pub fn pubsub_api(&self) -> pubsub_api::Pubsub_apiService<'_> {
        pubsub_api::Pubsub_apiService::new(self)
    }
    /// Get servicebroker_api service handler
    pub fn servicebroker_api(&self) -> servicebroker_api::Servicebroker_apiService<'_> {
        servicebroker_api::Servicebroker_apiService::new(self)
    }
    /// Get contactcenterinsights_api service handler
    pub fn contactcenterinsights_api(&self) -> contactcenterinsights_api::Contactcenterinsights_apiService<'_> {
        contactcenterinsights_api::Contactcenterinsights_apiService::new(self)
    }
    /// Get accesscontextmanager_api service handler
    pub fn accesscontextmanager_api(&self) -> accesscontextmanager_api::Accesscontextmanager_apiService<'_> {
        accesscontextmanager_api::Accesscontextmanager_apiService::new(self)
    }
    /// Get searchads360_api service handler
    pub fn searchads360_api(&self) -> searchads360_api::Searchads360_apiService<'_> {
        searchads360_api::Searchads360_apiService::new(self)
    }
    /// Get oslogin_api service handler
    pub fn oslogin_api(&self) -> oslogin_api::Oslogin_apiService<'_> {
        oslogin_api::Oslogin_apiService::new(self)
    }
    /// Get playcustomapp_api service handler
    pub fn playcustomapp_api(&self) -> playcustomapp_api::Playcustomapp_apiService<'_> {
        playcustomapp_api::Playcustomapp_apiService::new(self)
    }
    /// Get webmasters_api service handler
    pub fn webmasters_api(&self) -> webmasters_api::Webmasters_apiService<'_> {
        webmasters_api::Webmasters_apiService::new(self)
    }
    /// Get fitness_api service handler
    pub fn fitness_api(&self) -> fitness_api::Fitness_apiService<'_> {
        fitness_api::Fitness_apiService::new(self)
    }
    /// Get parametermanager_api service handler
    pub fn parametermanager_api(&self) -> parametermanager_api::Parametermanager_apiService<'_> {
        parametermanager_api::Parametermanager_apiService::new(self)
    }
    /// Get businessprofileperformance_api service handler
    pub fn businessprofileperformance_api(&self) -> businessprofileperformance_api::Businessprofileperformance_apiService<'_> {
        businessprofileperformance_api::Businessprofileperformance_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get replicapool_api service handler
    pub fn replicapool_api(&self) -> replicapool_api::Replicapool_apiService<'_> {
        replicapool_api::Replicapool_apiService::new(self)
    }
    /// Get dlp_api service handler
    pub fn dlp_api(&self) -> dlp_api::Dlp_apiService<'_> {
        dlp_api::Dlp_apiService::new(self)
    }
    /// Get migrationcenter_api service handler
    pub fn migrationcenter_api(&self) -> migrationcenter_api::Migrationcenter_apiService<'_> {
        migrationcenter_api::Migrationcenter_apiService::new(self)
    }
    /// Get kgsearch_api service handler
    pub fn kgsearch_api(&self) -> kgsearch_api::Kgsearch_apiService<'_> {
        kgsearch_api::Kgsearch_apiService::new(self)
    }
    /// Get cloudprofiler_api service handler
    pub fn cloudprofiler_api(&self) -> cloudprofiler_api::Cloudprofiler_apiService<'_> {
        cloudprofiler_api::Cloudprofiler_apiService::new(self)
    }
    /// Get healthcare_api service handler
    pub fn healthcare_api(&self) -> healthcare_api::Healthcare_apiService<'_> {
        healthcare_api::Healthcare_apiService::new(self)
    }
    /// Get firebaseappcheck_api service handler
    pub fn firebaseappcheck_api(&self) -> firebaseappcheck_api::Firebaseappcheck_apiService<'_> {
        firebaseappcheck_api::Firebaseappcheck_apiService::new(self)
    }
    /// Get gkeonprem_api service handler
    pub fn gkeonprem_api(&self) -> gkeonprem_api::Gkeonprem_apiService<'_> {
        gkeonprem_api::Gkeonprem_apiService::new(self)
    }
    /// Get repeated_any_query_error service handler
    pub fn repeated_any_query_error(&self) -> repeated_any_query_error::Repeated_any_query_errorService<'_> {
        repeated_any_query_error::Repeated_any_query_errorService::new(self)
    }
    /// Get transcoder_api service handler
    pub fn transcoder_api(&self) -> transcoder_api::Transcoder_apiService<'_> {
        transcoder_api::Transcoder_apiService::new(self)
    }
    /// Get firebaserules_api service handler
    pub fn firebaserules_api(&self) -> firebaserules_api::Firebaserules_apiService<'_> {
        firebaserules_api::Firebaserules_apiService::new(self)
    }
    /// Get plusdomains_api service handler
    pub fn plusdomains_api(&self) -> plusdomains_api::Plusdomains_apiService<'_> {
        plusdomains_api::Plusdomains_apiService::new(self)
    }
    /// Get proximitybeacon_api service handler
    pub fn proximitybeacon_api(&self) -> proximitybeacon_api::Proximitybeacon_apiService<'_> {
        proximitybeacon_api::Proximitybeacon_apiService::new(self)
    }
    /// Get firebase_api service handler
    pub fn firebase_api(&self) -> firebase_api::Firebase_apiService<'_> {
        firebase_api::Firebase_apiService::new(self)
    }
    /// Get resource_named_service service handler
    pub fn resource_named_service(&self) -> resource_named_service::Resource_named_serviceService<'_> {
        resource_named_service::Resource_named_serviceService::new(self)
    }
    /// Get config_api service handler
    pub fn config_api(&self) -> config_api::Config_apiService<'_> {
        config_api::Config_apiService::new(self)
    }
    /// Get games_api service handler
    pub fn games_api(&self) -> games_api::Games_apiService<'_> {
        games_api::Games_apiService::new(self)
    }
    /// Get appstate_api service handler
    pub fn appstate_api(&self) -> appstate_api::Appstate_apiService<'_> {
        appstate_api::Appstate_apiService::new(self)
    }
    /// Get saasservicemgmt_api service handler
    pub fn saasservicemgmt_api(&self) -> saasservicemgmt_api::Saasservicemgmt_apiService<'_> {
        saasservicemgmt_api::Saasservicemgmt_apiService::new(self)
    }
    /// Get test_api service handler
    pub fn test_api(&self) -> test_api::Test_apiService<'_> {
        test_api::Test_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get analytics_api service handler
    pub fn analytics_api(&self) -> analytics_api::Analytics_apiService<'_> {
        analytics_api::Analytics_apiService::new(self)
    }
    /// Get discoveryengine_api service handler
    pub fn discoveryengine_api(&self) -> discoveryengine_api::Discoveryengine_apiService<'_> {
        discoveryengine_api::Discoveryengine_apiService::new(self)
    }
    /// Get playintegrity_api service handler
    pub fn playintegrity_api(&self) -> playintegrity_api::Playintegrity_apiService<'_> {
        playintegrity_api::Playintegrity_apiService::new(self)
    }
    /// Get homegraph_api service handler
    pub fn homegraph_api(&self) -> homegraph_api::Homegraph_apiService<'_> {
        homegraph_api::Homegraph_apiService::new(self)
    }
    /// Get apikeys_api service handler
    pub fn apikeys_api(&self) -> apikeys_api::Apikeys_apiService<'_> {
        apikeys_api::Apikeys_apiService::new(self)
    }
    /// Get serviceconsumermanagement_api service handler
    pub fn serviceconsumermanagement_api(&self) -> serviceconsumermanagement_api::Serviceconsumermanagement_apiService<'_> {
        serviceconsumermanagement_api::Serviceconsumermanagement_apiService::new(self)
    }
    /// Get datalineage_api service handler
    pub fn datalineage_api(&self) -> datalineage_api::Datalineage_apiService<'_> {
        datalineage_api::Datalineage_apiService::new(self)
    }
    /// Get manufacturers_api service handler
    pub fn manufacturers_api(&self) -> manufacturers_api::Manufacturers_apiService<'_> {
        manufacturers_api::Manufacturers_apiService::new(self)
    }
    /// Get vectortile_api service handler
    pub fn vectortile_api(&self) -> vectortile_api::Vectortile_apiService<'_> {
        vectortile_api::Vectortile_apiService::new(self)
    }
    /// Get chat_api service handler
    pub fn chat_api(&self) -> chat_api::Chat_apiService<'_> {
        chat_api::Chat_apiService::new(self)
    }
    /// Get bigquery_api service handler
    pub fn bigquery_api(&self) -> bigquery_api::Bigquery_apiService<'_> {
        bigquery_api::Bigquery_apiService::new(self)
    }
    /// Get plus_api service handler
    pub fn plus_api(&self) -> plus_api::Plus_apiService<'_> {
        plus_api::Plus_apiService::new(self)
    }
    /// Get bigqueryconnection_api service handler
    pub fn bigqueryconnection_api(&self) -> bigqueryconnection_api::Bigqueryconnection_apiService<'_> {
        bigqueryconnection_api::Bigqueryconnection_apiService::new(self)
    }
    /// Get places_api service handler
    pub fn places_api(&self) -> places_api::Places_apiService<'_> {
        places_api::Places_apiService::new(self)
    }
    /// Get logging_api service handler
    pub fn logging_api(&self) -> logging_api::Logging_apiService<'_> {
        logging_api::Logging_apiService::new(self)
    }
    /// Get webrisk_api service handler
    pub fn webrisk_api(&self) -> webrisk_api::Webrisk_apiService<'_> {
        webrisk_api::Webrisk_apiService::new(self)
    }
    /// Get vpcaccess_api service handler
    pub fn vpcaccess_api(&self) -> vpcaccess_api::Vpcaccess_apiService<'_> {
        vpcaccess_api::Vpcaccess_apiService::new(self)
    }
    /// Get urlshortener_api service handler
    pub fn urlshortener_api(&self) -> urlshortener_api::Urlshortener_apiService<'_> {
        urlshortener_api::Urlshortener_apiService::new(self)
    }
    /// Get sqladmin_api service handler
    pub fn sqladmin_api(&self) -> sqladmin_api::Sqladmin_apiService<'_> {
        sqladmin_api::Sqladmin_apiService::new(self)
    }
    /// Get keep_api service handler
    pub fn keep_api(&self) -> keep_api::Keep_apiService<'_> {
        keep_api::Keep_apiService::new(self)
    }
    /// Get walletobjects_api service handler
    pub fn walletobjects_api(&self) -> walletobjects_api::Walletobjects_apiService<'_> {
        walletobjects_api::Walletobjects_apiService::new(self)
    }
    /// Get apphub_api service handler
    pub fn apphub_api(&self) -> apphub_api::Apphub_apiService<'_> {
        apphub_api::Apphub_apiService::new(self)
    }
    /// Get vmmigration_api service handler
    pub fn vmmigration_api(&self) -> vmmigration_api::Vmmigration_apiService<'_> {
        vmmigration_api::Vmmigration_apiService::new(self)
    }
    /// Get airquality_api service handler
    pub fn airquality_api(&self) -> airquality_api::Airquality_apiService<'_> {
        airquality_api::Airquality_apiService::new(self)
    }
    /// Get chromemanagement_api service handler
    pub fn chromemanagement_api(&self) -> chromemanagement_api::Chromemanagement_apiService<'_> {
        chromemanagement_api::Chromemanagement_apiService::new(self)
    }
    /// Get adexperiencereport_api service handler
    pub fn adexperiencereport_api(&self) -> adexperiencereport_api::Adexperiencereport_apiService<'_> {
        adexperiencereport_api::Adexperiencereport_apiService::new(self)
    }
    /// Get recommender_api service handler
    pub fn recommender_api(&self) -> recommender_api::Recommender_apiService<'_> {
        recommender_api::Recommender_apiService::new(self)
    }
    /// Get driveactivity_api service handler
    pub fn driveactivity_api(&self) -> driveactivity_api::Driveactivity_apiService<'_> {
        driveactivity_api::Driveactivity_apiService::new(self)
    }
    /// Get dns_api service handler
    pub fn dns_api(&self) -> dns_api::Dns_apiService<'_> {
        dns_api::Dns_apiService::new(self)
    }
    /// Get cloudshell_api service handler
    pub fn cloudshell_api(&self) -> cloudshell_api::Cloudshell_apiService<'_> {
        cloudshell_api::Cloudshell_apiService::new(self)
    }
    /// Get content_api service handler
    pub fn content_api(&self) -> content_api::Content_apiService<'_> {
        content_api::Content_apiService::new(self)
    }
    /// Get commentanalyzer_api service handler
    pub fn commentanalyzer_api(&self) -> commentanalyzer_api::Commentanalyzer_apiService<'_> {
        commentanalyzer_api::Commentanalyzer_apiService::new(self)
    }
    /// Get backupdr_api service handler
    pub fn backupdr_api(&self) -> backupdr_api::Backupdr_apiService<'_> {
        backupdr_api::Backupdr_apiService::new(self)
    }
    /// Get groupsmigration_api service handler
    pub fn groupsmigration_api(&self) -> groupsmigration_api::Groupsmigration_apiService<'_> {
        groupsmigration_api::Groupsmigration_apiService::new(self)
    }
    /// Get connectors_api service handler
    pub fn connectors_api(&self) -> connectors_api::Connectors_apiService<'_> {
        connectors_api::Connectors_apiService::new(self)
    }
    /// Get solar_api service handler
    pub fn solar_api(&self) -> solar_api::Solar_apiService<'_> {
        solar_api::Solar_apiService::new(self)
    }
    /// Get fcm_api service handler
    pub fn fcm_api(&self) -> fcm_api::Fcm_apiService<'_> {
        fcm_api::Fcm_apiService::new(self)
    }
    /// Get networksecurity_api service handler
    pub fn networksecurity_api(&self) -> networksecurity_api::Networksecurity_apiService<'_> {
        networksecurity_api::Networksecurity_apiService::new(self)
    }
    /// Get managedidentities_api service handler
    pub fn managedidentities_api(&self) -> managedidentities_api::Managedidentities_apiService<'_> {
        managedidentities_api::Managedidentities_apiService::new(self)
    }
    /// Get serviceusage_api service handler
    pub fn serviceusage_api(&self) -> serviceusage_api::Serviceusage_apiService<'_> {
        serviceusage_api::Serviceusage_apiService::new(self)
    }
    /// Get digitalassetlinks_api service handler
    pub fn digitalassetlinks_api(&self) -> digitalassetlinks_api::Digitalassetlinks_apiService<'_> {
        digitalassetlinks_api::Digitalassetlinks_apiService::new(self)
    }
    /// Get clouddebugger_api service handler
    pub fn clouddebugger_api(&self) -> clouddebugger_api::Clouddebugger_apiService<'_> {
        clouddebugger_api::Clouddebugger_apiService::new(self)
    }
    /// Get datamanager_api service handler
    pub fn datamanager_api(&self) -> datamanager_api::Datamanager_apiService<'_> {
        datamanager_api::Datamanager_apiService::new(self)
    }
    /// Get ids_api service handler
    pub fn ids_api(&self) -> ids_api::Ids_apiService<'_> {
        ids_api::Ids_apiService::new(self)
    }
    /// Get workflows_api service handler
    pub fn workflows_api(&self) -> workflows_api::Workflows_apiService<'_> {
        workflows_api::Workflows_apiService::new(self)
    }
    /// Get cloudtrace_api service handler
    pub fn cloudtrace_api(&self) -> cloudtrace_api::Cloudtrace_apiService<'_> {
        cloudtrace_api::Cloudtrace_apiService::new(self)
    }
    /// Get advisorynotifications_api service handler
    pub fn advisorynotifications_api(&self) -> advisorynotifications_api::Advisorynotifications_apiService<'_> {
        advisorynotifications_api::Advisorynotifications_apiService::new(self)
    }
    /// Get fcmdata_api service handler
    pub fn fcmdata_api(&self) -> fcmdata_api::Fcmdata_apiService<'_> {
        fcmdata_api::Fcmdata_apiService::new(self)
    }
    /// Get checks_api service handler
    pub fn checks_api(&self) -> checks_api::Checks_apiService<'_> {
        checks_api::Checks_apiService::new(self)
    }
    /// Get blogger_3 service handler
    pub fn blogger_3(&self) -> blogger_3::Blogger_3Service<'_> {
        blogger_3::Blogger_3Service::new(self)
    }
    /// Get mediaupload service handler
    pub fn mediaupload(&self) -> mediaupload::MediauploadService<'_> {
        mediaupload::MediauploadService::new(self)
    }
    /// Get paymentsresellersubscription_api service handler
    pub fn paymentsresellersubscription_api(&self) -> paymentsresellersubscription_api::Paymentsresellersubscription_apiService<'_> {
        paymentsresellersubscription_api::Paymentsresellersubscription_apiService::new(self)
    }
    /// Get spanner_api service handler
    pub fn spanner_api(&self) -> spanner_api::Spanner_apiService<'_> {
        spanner_api::Spanner_apiService::new(self)
    }
    /// Get cloudchannel_api service handler
    pub fn cloudchannel_api(&self) -> cloudchannel_api::Cloudchannel_apiService<'_> {
        cloudchannel_api::Cloudchannel_apiService::new(self)
    }
    /// Get gameservices_api service handler
    pub fn gameservices_api(&self) -> gameservices_api::Gameservices_apiService<'_> {
        gameservices_api::Gameservices_apiService::new(self)
    }
    /// Get cloudprivatecatalog_api service handler
    pub fn cloudprivatecatalog_api(&self) -> cloudprivatecatalog_api::Cloudprivatecatalog_apiService<'_> {
        cloudprivatecatalog_api::Cloudprivatecatalog_apiService::new(self)
    }
    /// Get gmailpostmastertools_api service handler
    pub fn gmailpostmastertools_api(&self) -> gmailpostmastertools_api::Gmailpostmastertools_apiService<'_> {
        gmailpostmastertools_api::Gmailpostmastertools_apiService::new(self)
    }
    /// Get partners_api service handler
    pub fn partners_api(&self) -> partners_api::Partners_apiService<'_> {
        partners_api::Partners_apiService::new(self)
    }
    /// Get any service handler
    pub fn any(&self) -> any::AnyService<'_> {
        any::AnyService::new(self)
    }
    /// Get runtimeconfig_api service handler
    pub fn runtimeconfig_api(&self) -> runtimeconfig_api::Runtimeconfig_apiService<'_> {
        runtimeconfig_api::Runtimeconfig_apiService::new(self)
    }
    /// Get retail_api service handler
    pub fn retail_api(&self) -> retail_api::Retail_apiService<'_> {
        retail_api::Retail_apiService::new(self)
    }
    /// Get books_api service handler
    pub fn books_api(&self) -> books_api::Books_apiService<'_> {
        books_api::Books_apiService::new(self)
    }
    /// Get websecurityscanner_api service handler
    pub fn websecurityscanner_api(&self) -> websecurityscanner_api::Websecurityscanner_apiService<'_> {
        websecurityscanner_api::Websecurityscanner_apiService::new(self)
    }
    /// Get localservices_api service handler
    pub fn localservices_api(&self) -> localservices_api::Localservices_apiService<'_> {
        localservices_api::Localservices_apiService::new(self)
    }
    /// Get servicedirectory_api service handler
    pub fn servicedirectory_api(&self) -> servicedirectory_api::Servicedirectory_apiService<'_> {
        servicedirectory_api::Servicedirectory_apiService::new(self)
    }
    /// Get customsearch_api service handler
    pub fn customsearch_api(&self) -> customsearch_api::Customsearch_apiService<'_> {
        customsearch_api::Customsearch_apiService::new(self)
    }
    /// Get policyanalyzer_api service handler
    pub fn policyanalyzer_api(&self) -> policyanalyzer_api::Policyanalyzer_apiService<'_> {
        policyanalyzer_api::Policyanalyzer_apiService::new(self)
    }
    /// Get mybusinessbusinesscalls_api service handler
    pub fn mybusinessbusinesscalls_api(&self) -> mybusinessbusinesscalls_api::Mybusinessbusinesscalls_apiService<'_> {
        mybusinessbusinesscalls_api::Mybusinessbusinesscalls_apiService::new(self)
    }
    /// Get ml_api service handler
    pub fn ml_api(&self) -> ml_api::Ml_apiService<'_> {
        ml_api::Ml_apiService::new(self)
    }
    /// Get firebaseml_api service handler
    pub fn firebaseml_api(&self) -> firebaseml_api::Firebaseml_apiService<'_> {
        firebaseml_api::Firebaseml_apiService::new(self)
    }
    /// Get blogger_api service handler
    pub fn blogger_api(&self) -> blogger_api::Blogger_apiService<'_> {
        blogger_api::Blogger_apiService::new(self)
    }
    /// Get vault_api service handler
    pub fn vault_api(&self) -> vault_api::Vault_apiService<'_> {
        vault_api::Vault_apiService::new(self)
    }
    /// Get baremetalsolution_api service handler
    pub fn baremetalsolution_api(&self) -> baremetalsolution_api::Baremetalsolution_apiService<'_> {
        baremetalsolution_api::Baremetalsolution_apiService::new(self)
    }
    /// Get testing_api service handler
    pub fn testing_api(&self) -> testing_api::Testing_apiService<'_> {
        testing_api::Testing_apiService::new(self)
    }
    /// Get recaptchaenterprise_api service handler
    pub fn recaptchaenterprise_api(&self) -> recaptchaenterprise_api::Recaptchaenterprise_apiService<'_> {
        recaptchaenterprise_api::Recaptchaenterprise_apiService::new(self)
    }
    /// Get servicecontrol_api service handler
    pub fn servicecontrol_api(&self) -> servicecontrol_api::Servicecontrol_apiService<'_> {
        servicecontrol_api::Servicecontrol_apiService::new(self)
    }
    /// Get documentai_api service handler
    pub fn documentai_api(&self) -> documentai_api::Documentai_apiService<'_> {
        documentai_api::Documentai_apiService::new(self)
    }
    /// Get firebasedynamiclinks_api service handler
    pub fn firebasedynamiclinks_api(&self) -> firebasedynamiclinks_api::Firebasedynamiclinks_apiService<'_> {
        firebasedynamiclinks_api::Firebasedynamiclinks_apiService::new(self)
    }
    /// Get apigee_api service handler
    pub fn apigee_api(&self) -> apigee_api::Apigee_apiService<'_> {
        apigee_api::Apigee_apiService::new(self)
    }
    /// Get netapp_api service handler
    pub fn netapp_api(&self) -> netapp_api::Netapp_apiService<'_> {
        netapp_api::Netapp_apiService::new(self)
    }
    /// Get identitytoolkit_api service handler
    pub fn identitytoolkit_api(&self) -> identitytoolkit_api::Identitytoolkit_apiService<'_> {
        identitytoolkit_api::Identitytoolkit_apiService::new(self)
    }
    /// Get datastream_api service handler
    pub fn datastream_api(&self) -> datastream_api::Datastream_apiService<'_> {
        datastream_api::Datastream_apiService::new(self)
    }
    /// Get iamcredentials_api service handler
    pub fn iamcredentials_api(&self) -> iamcredentials_api::Iamcredentials_apiService<'_> {
        iamcredentials_api::Iamcredentials_apiService::new(self)
    }
    /// Get datalabeling_api service handler
    pub fn datalabeling_api(&self) -> datalabeling_api::Datalabeling_apiService<'_> {
        datalabeling_api::Datalabeling_apiService::new(self)
    }
    /// Get youtubereporting_api service handler
    pub fn youtubereporting_api(&self) -> youtubereporting_api::Youtubereporting_apiService<'_> {
        youtubereporting_api::Youtubereporting_apiService::new(self)
    }
    /// Get genomics_api service handler
    pub fn genomics_api(&self) -> genomics_api::Genomics_apiService<'_> {
        genomics_api::Genomics_apiService::new(self)
    }
    /// Get lifesciences_api service handler
    pub fn lifesciences_api(&self) -> lifesciences_api::Lifesciences_apiService<'_> {
        lifesciences_api::Lifesciences_apiService::new(self)
    }
    /// Get compute_api service handler
    pub fn compute_api(&self) -> compute_api::Compute_apiService<'_> {
        compute_api::Compute_apiService::new(self)
    }
    /// Get youtubeanalytics_api service handler
    pub fn youtubeanalytics_api(&self) -> youtubeanalytics_api::Youtubeanalytics_apiService<'_> {
        youtubeanalytics_api::Youtubeanalytics_apiService::new(self)
    }
    /// Get factchecktools_api service handler
    pub fn factchecktools_api(&self) -> factchecktools_api::Factchecktools_apiService<'_> {
        factchecktools_api::Factchecktools_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get docs_api service handler
    pub fn docs_api(&self) -> docs_api::Docs_apiService<'_> {
        docs_api::Docs_apiService::new(self)
    }
    /// Get tagmanager_api service handler
    pub fn tagmanager_api(&self) -> tagmanager_api::Tagmanager_apiService<'_> {
        tagmanager_api::Tagmanager_apiService::new(self)
    }
    /// Get texttospeech_api service handler
    pub fn texttospeech_api(&self) -> texttospeech_api::Texttospeech_apiService<'_> {
        texttospeech_api::Texttospeech_apiService::new(self)
    }
    /// Get gmail_api service handler
    pub fn gmail_api(&self) -> gmail_api::Gmail_apiService<'_> {
        gmail_api::Gmail_apiService::new(self)
    }
    /// Get tasks_api service handler
    pub fn tasks_api(&self) -> tasks_api::Tasks_apiService<'_> {
        tasks_api::Tasks_apiService::new(self)
    }
    /// Get securityposture_api service handler
    pub fn securityposture_api(&self) -> securityposture_api::Securityposture_apiService<'_> {
        securityposture_api::Securityposture_apiService::new(self)
    }
    /// Get sql_api service handler
    pub fn sql_api(&self) -> sql_api::Sql_apiService<'_> {
        sql_api::Sql_apiService::new(self)
    }
    /// Get integrations_api service handler
    pub fn integrations_api(&self) -> integrations_api::Integrations_apiService<'_> {
        integrations_api::Integrations_apiService::new(self)
    }
    /// Get essentialcontacts_api service handler
    pub fn essentialcontacts_api(&self) -> essentialcontacts_api::Essentialcontacts_apiService<'_> {
        essentialcontacts_api::Essentialcontacts_apiService::new(self)
    }
    /// Get tpu_api service handler
    pub fn tpu_api(&self) -> tpu_api::Tpu_apiService<'_> {
        tpu_api::Tpu_apiService::new(self)
    }
    /// Get classroom_api service handler
    pub fn classroom_api(&self) -> classroom_api::Classroom_apiService<'_> {
        classroom_api::Classroom_apiService::new(self)
    }
    /// Get mybusinesslodging_api service handler
    pub fn mybusinesslodging_api(&self) -> mybusinesslodging_api::Mybusinesslodging_apiService<'_> {
        mybusinesslodging_api::Mybusinesslodging_apiService::new(self)
    }
    /// Get rapidmigrationassessment_api service handler
    pub fn rapidmigrationassessment_api(&self) -> rapidmigrationassessment_api::Rapidmigrationassessment_apiService<'_> {
        rapidmigrationassessment_api::Rapidmigrationassessment_apiService::new(self)
    }
    /// Get blockchainnodeengine_api service handler
    pub fn blockchainnodeengine_api(&self) -> blockchainnodeengine_api::Blockchainnodeengine_apiService<'_> {
        blockchainnodeengine_api::Blockchainnodeengine_apiService::new(self)
    }


    /// Get reference to the Tokio runtime for executing async operations
    pub(crate) fn runtime(&self) -> &tokio::runtime::Runtime {
        &self.runtime
    }
}

/// Implement ProviderExecutor trait for Hemmer integration
#[async_trait]
impl ProviderExecutor for GcpProvider {
    /// Configure the provider with authentication and settings
    async fn configure(&mut self, config: ProviderConfig) -> Result<()> {
        // Configuration is already handled in new()
        // Additional runtime configuration can be added here
        Ok(())
    }

    /// Plan changes to a resource (diff current vs desired state)
    async fn plan(
        &self,
        resource_type: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // Dispatch to appropriate service based on resource_type
        // Format: "service_name.resource_name" (e.g., "s3.bucket")
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "translate_api" => {
                self.translate_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "area120tables_api" => {
                self.area120tables_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "abusiveexperiencereport_api" => {
                self.abusiveexperiencereport_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "securesourcemanager_api" => {
                self.securesourcemanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "file_api" => {
                self.file_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "oauth2_api" => {
                self.oauth2_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "serviceuser_api" => {
                self.serviceuser_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigquerydatatransfer_api" => {
                self.bigquerydatatransfer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudfunctions_api" => {
                self.cloudfunctions_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkconnectivity_api" => {
                self.networkconnectivity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessverifications_api" => {
                self.mybusinessverifications_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "qpxexpress_api" => {
                self.qpxexpress_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataproc_api" => {
                self.dataproc_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playgrouping_api" => {
                self.playgrouping_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mirror_api" => {
                self.mirror_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adexchangeseller_api" => {
                self.adexchangeseller_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "binaryauthorization_api" => {
                self.binaryauthorization_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketingplatformadmin_api" => {
                self.marketingplatformadmin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudprivatecatalogproducer_api" => {
                self.cloudprivatecatalogproducer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "pubsublite_api" => {
                self.pubsublite_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "slides_api" => {
                self.slides_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "billingbudgets_api" => {
                self.billingbudgets_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "workstations_api" => {
                self.workstations_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "civicinfo_api" => {
                self.civicinfo_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "spectrum_api" => {
                self.spectrum_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "surveys_api" => {
                self.surveys_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataplex_api" => {
                self.dataplex_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "redis_api" => {
                self.redis_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "addressvalidation_api" => {
                self.addressvalidation_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sourcerepo_api" => {
                self.sourcerepo_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "osconfig_api" => {
                self.osconfig_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "domainsrdap_api" => {
                self.domainsrdap_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "speech_api" => {
                self.speech_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "trafficdirector_api" => {
                self.trafficdirector_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "indexing_api" => {
                self.indexing_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "ondemandscanning_api" => {
                self.ondemandscanning_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "drive_api" => {
                self.drive_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessnotifications_api" => {
                self.mybusinessnotifications_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "workflowexecutions_api" => {
                self.workflowexecutions_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "developerconnect_api" => {
                self.developerconnect_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "analyticsadmin_api" => {
                self.analyticsadmin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playdeveloperreporting_api" => {
                self.playdeveloperreporting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "prod_tt_sasportal_api" => {
                self.prod_tt_sasportal_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "searchconsole_api" => {
                self.searchconsole_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaseremoteconfig_api" => {
                self.firebaseremoteconfig_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "publicca_api" => {
                self.publicca_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "discovery_api" => {
                self.discovery_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebasehosting_api" => {
                self.firebasehosting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "memcache_api" => {
                self.memcache_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playablelocations_api" => {
                self.playablelocations_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "jobs_api" => {
                self.jobs_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "iam_api" => {
                self.iam_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "reseller_api" => {
                self.reseller_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "realtimebidding_api" => {
                self.realtimebidding_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datamigration_api" => {
                self.datamigration_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "safebrowsing_api" => {
                self.safebrowsing_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "libraryagent_api" => {
                self.libraryagent_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "css_api" => {
                self.css_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "androidpublisher_api" => {
                self.androidpublisher_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datacatalog_api" => {
                self.datacatalog_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudlocationfinder_api" => {
                self.cloudlocationfinder_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "storagetransfer_api" => {
                self.storagetransfer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "resourcesettings_api" => {
                self.resourcesettings_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebasedatabase_api" => {
                self.firebasedatabase_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "analyticshub_api" => {
                self.analyticshub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "http_body" => {
                self.http_body().plan_resource(resource_name, current_state, desired_input).await
            }
            "managedkafka_api" => {
                self.managedkafka_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "poly_api" => {
                self.poly_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "orgpolicy_api" => {
                self.orgpolicy_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "certificatemanager_api" => {
                self.certificatemanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apigateway_api" => {
                self.apigateway_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datastore_api" => {
                self.datastore_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudtasks_api" => {
                self.cloudtasks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaseappdistribution_api" => {
                self.firebaseappdistribution_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "domains_api" => {
                self.domains_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "policysimulator_api" => {
                self.policysimulator_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apihub_api" => {
                self.apihub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessaccountmanagement_api" => {
                self.mybusinessaccountmanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "smartdevicemanagement_api" => {
                self.smartdevicemanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "tracing_api" => {
                self.tracing_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gamesmanagement_api" => {
                self.gamesmanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firestore_api" => {
                self.firestore_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "language_api" => {
                self.language_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "json_body" => {
                self.json_body().plan_resource(resource_name, current_state, desired_input).await
            }
            "replicapoolupdater_api" => {
                self.replicapoolupdater_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "androidmanagement_api" => {
                self.androidmanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "licensing_api" => {
                self.licensing_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sasportal_api" => {
                self.sasportal_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vmwareengine_api" => {
                self.vmwareengine_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "pollen_api" => {
                self.pollen_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudsupport_api" => {
                self.cloudsupport_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "servicemanagement_api" => {
                self.servicemanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudiot_api" => {
                self.cloudiot_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "androiddeviceprovisioning_api" => {
                self.androiddeviceprovisioning_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gamesconfiguration_api" => {
                self.gamesconfiguration_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "areainsights_api" => {
                self.areainsights_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "meet_api" => {
                self.meet_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adsense_api" => {
                self.adsense_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "toolresults_api" => {
                self.toolresults_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "artifactregistry_api" => {
                self.artifactregistry_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudcommerceprocurement_api" => {
                self.cloudcommerceprocurement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "alertcenter_api" => {
                self.alertcenter_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessbusinessinformation_api" => {
                self.mybusinessbusinessinformation_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigquerydatapolicy_api" => {
                self.bigquerydatapolicy_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "androidenterprise_api" => {
                self.androidenterprise_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playmoviespartner_api" => {
                self.playmoviespartner_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "oracledatabase_api" => {
                self.oracledatabase_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "composer_api" => {
                self.composer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataform_api" => {
                self.dataform_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adsensehost_api" => {
                self.adsensehost_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "deploymentmanager_api" => {
                self.deploymentmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "displayvideo_api" => {
                self.displayvideo_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "chromewebstore_api" => {
                self.chromewebstore_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudkms_api" => {
                self.cloudkms_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigtableadmin_api" => {
                self.bigtableadmin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "drivelabels_api" => {
                self.drivelabels_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vision_api" => {
                self.vision_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "streetviewpublish_api" => {
                self.streetviewpublish_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "clouddeploy_api" => {
                self.clouddeploy_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "biglake_api" => {
                self.biglake_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "eventarc_api" => {
                self.eventarc_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apigeeregistry_api" => {
                self.apigeeregistry_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudbuild_api" => {
                self.cloudbuild_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "secretmanager_api" => {
                self.secretmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sheets_api" => {
                self.sheets_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "recommendationengine_api" => {
                self.recommendationengine_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "appengine_api" => {
                self.appengine_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataportability_api" => {
                self.dataportability_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "remotebuildexecution_api" => {
                self.remotebuildexecution_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "containeranalysis_api" => {
                self.containeranalysis_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "workloadmanager_api" => {
                self.workloadmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudcontrolspartner_api" => {
                self.cloudcontrolspartner_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "monitoring_api" => {
                self.monitoring_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "privateca_api" => {
                self.privateca_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "container_api" => {
                self.container_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "merchantapi_api" => {
                self.merchantapi_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datapipelines_api" => {
                self.datapipelines_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sts_api" => {
                self.sts_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "observability_api" => {
                self.observability_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataflow_api" => {
                self.dataflow_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudresourcemanager_api" => {
                self.cloudresourcemanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaseapphosting_api" => {
                self.firebaseapphosting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "verifiedaccess_api" => {
                self.verifiedaccess_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "accessapproval_api" => {
                self.accessapproval_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebasedataconnect_api" => {
                self.firebasedataconnect_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "assuredworkloads_api" => {
                self.assuredworkloads_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datafusion_api" => {
                self.datafusion_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "doubleclicksearch_api" => {
                self.doubleclicksearch_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "ideahub_api" => {
                self.ideahub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "alloydb_api" => {
                self.alloydb_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "doubleclickbidmanager_api" => {
                self.doubleclickbidmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigqueryreservation_api" => {
                self.bigqueryreservation_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "notebooks_api" => {
                self.notebooks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "readerrevenuesubscriptionlinking_api" => {
                self.readerrevenuesubscriptionlinking_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "authorizedbuyersmarketplace_api" => {
                self.authorizedbuyersmarketplace_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adexchangebuyer2_api" => {
                self.adexchangebuyer2_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adsenseplatform_api" => {
                self.adsenseplatform_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessplaceactions_api" => {
                self.mybusinessplaceactions_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebasestorage_api" => {
                self.firebasestorage_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "calendar_api" => {
                self.calendar_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkmanagement_api" => {
                self.networkmanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "admin_api" => {
                self.admin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "acceleratedmobilepageurl_api" => {
                self.acceleratedmobilepageurl_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "fusiontables_api" => {
                self.fusiontables_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "chromepolicy_api" => {
                self.chromepolicy_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "securitycenter_api" => {
                self.securitycenter_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "analyticsreporting_api" => {
                self.analyticsreporting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "iap_api" => {
                self.iap_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "consumersurveys_api" => {
                self.consumersurveys_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "siteverification_api" => {
                self.siteverification_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "versionhistory_api" => {
                self.versionhistory_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "storagebatchoperations_api" => {
                self.storagebatchoperations_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "acmedns_api" => {
                self.acmedns_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "storage_api" => {
                self.storage_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkservices_api" => {
                self.networkservices_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dfareporting_api" => {
                self.dfareporting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "webfonts_api" => {
                self.webfonts_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apim_api" => {
                self.apim_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "chromeuxreport_api" => {
                self.chromeuxreport_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "aiplatform_api" => {
                self.aiplatform_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "script_api" => {
                self.script_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "contentwarehouse_api" => {
                self.contentwarehouse_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "batch_api" => {
                self.batch_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "kmsinventory_api" => {
                self.kmsinventory_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "run_api" => {
                self.run_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "servicenetworking_api" => {
                self.servicenetworking_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudscheduler_api" => {
                self.cloudscheduler_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudsearch_api" => {
                self.cloudsearch_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "admob_api" => {
                self.admob_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "workspaceevents_api" => {
                self.workspaceevents_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "metastore_api" => {
                self.metastore_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "policytroubleshooter_api" => {
                self.policytroubleshooter_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "analyticsdata_api" => {
                self.analyticsdata_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "contactcenteraiplatform_api" => {
                self.contactcenteraiplatform_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "forms_api" => {
                self.forms_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "clouderrorreporting_api" => {
                self.clouderrorreporting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "pagespeedonline_api" => {
                self.pagespeedonline_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "parallelstore_api" => {
                self.parallelstore_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudbilling_api" => {
                self.cloudbilling_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudidentity_api" => {
                self.cloudidentity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "appsactivity_api" => {
                self.appsactivity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "groupssettings_api" => {
                self.groupssettings_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adexchangebuyer_api" => {
                self.adexchangebuyer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "youtube_api" => {
                self.youtube_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "looker_api" => {
                self.looker_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gkebackup_api" => {
                self.gkebackup_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudasset_api" => {
                self.cloudasset_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "people_api" => {
                self.people_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessqanda_api" => {
                self.mybusinessqanda_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "beyondcorp_api" => {
                self.beyondcorp_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "pubsub_api" => {
                self.pubsub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "servicebroker_api" => {
                self.servicebroker_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "contactcenterinsights_api" => {
                self.contactcenterinsights_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "accesscontextmanager_api" => {
                self.accesscontextmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "searchads360_api" => {
                self.searchads360_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "oslogin_api" => {
                self.oslogin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playcustomapp_api" => {
                self.playcustomapp_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "webmasters_api" => {
                self.webmasters_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "fitness_api" => {
                self.fitness_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "parametermanager_api" => {
                self.parametermanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "businessprofileperformance_api" => {
                self.businessprofileperformance_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "videointelligence_api" => {
                self.videointelligence_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "replicapool_api" => {
                self.replicapool_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dlp_api" => {
                self.dlp_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "migrationcenter_api" => {
                self.migrationcenter_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "kgsearch_api" => {
                self.kgsearch_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudprofiler_api" => {
                self.cloudprofiler_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "healthcare_api" => {
                self.healthcare_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaseappcheck_api" => {
                self.firebaseappcheck_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gkeonprem_api" => {
                self.gkeonprem_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "repeated_any_query_error" => {
                self.repeated_any_query_error().plan_resource(resource_name, current_state, desired_input).await
            }
            "transcoder_api" => {
                self.transcoder_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaserules_api" => {
                self.firebaserules_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "plusdomains_api" => {
                self.plusdomains_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "proximitybeacon_api" => {
                self.proximitybeacon_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebase_api" => {
                self.firebase_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "resource_named_service" => {
                self.resource_named_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "config_api" => {
                self.config_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "games_api" => {
                self.games_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "appstate_api" => {
                self.appstate_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "saasservicemgmt_api" => {
                self.saasservicemgmt_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "test_api" => {
                self.test_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gkehub_api" => {
                self.gkehub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "analytics_api" => {
                self.analytics_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "discoveryengine_api" => {
                self.discoveryengine_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "playintegrity_api" => {
                self.playintegrity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "homegraph_api" => {
                self.homegraph_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apikeys_api" => {
                self.apikeys_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "serviceconsumermanagement_api" => {
                self.serviceconsumermanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datalineage_api" => {
                self.datalineage_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "manufacturers_api" => {
                self.manufacturers_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vectortile_api" => {
                self.vectortile_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "chat_api" => {
                self.chat_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigquery_api" => {
                self.bigquery_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "plus_api" => {
                self.plus_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "bigqueryconnection_api" => {
                self.bigqueryconnection_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "places_api" => {
                self.places_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "logging_api" => {
                self.logging_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "webrisk_api" => {
                self.webrisk_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vpcaccess_api" => {
                self.vpcaccess_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "urlshortener_api" => {
                self.urlshortener_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sqladmin_api" => {
                self.sqladmin_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "keep_api" => {
                self.keep_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "walletobjects_api" => {
                self.walletobjects_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apphub_api" => {
                self.apphub_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vmmigration_api" => {
                self.vmmigration_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "airquality_api" => {
                self.airquality_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "chromemanagement_api" => {
                self.chromemanagement_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "adexperiencereport_api" => {
                self.adexperiencereport_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "recommender_api" => {
                self.recommender_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "driveactivity_api" => {
                self.driveactivity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dns_api" => {
                self.dns_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudshell_api" => {
                self.cloudshell_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "content_api" => {
                self.content_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "commentanalyzer_api" => {
                self.commentanalyzer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "backupdr_api" => {
                self.backupdr_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "groupsmigration_api" => {
                self.groupsmigration_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "connectors_api" => {
                self.connectors_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "solar_api" => {
                self.solar_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "fcm_api" => {
                self.fcm_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "networksecurity_api" => {
                self.networksecurity_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "managedidentities_api" => {
                self.managedidentities_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "serviceusage_api" => {
                self.serviceusage_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "digitalassetlinks_api" => {
                self.digitalassetlinks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "clouddebugger_api" => {
                self.clouddebugger_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datamanager_api" => {
                self.datamanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "ids_api" => {
                self.ids_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "workflows_api" => {
                self.workflows_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudtrace_api" => {
                self.cloudtrace_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "advisorynotifications_api" => {
                self.advisorynotifications_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "fcmdata_api" => {
                self.fcmdata_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "checks_api" => {
                self.checks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "blogger_3" => {
                self.blogger_3().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediaupload" => {
                self.mediaupload().plan_resource(resource_name, current_state, desired_input).await
            }
            "paymentsresellersubscription_api" => {
                self.paymentsresellersubscription_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "spanner_api" => {
                self.spanner_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudchannel_api" => {
                self.cloudchannel_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gameservices_api" => {
                self.gameservices_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudprivatecatalog_api" => {
                self.cloudprivatecatalog_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gmailpostmastertools_api" => {
                self.gmailpostmastertools_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "partners_api" => {
                self.partners_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "any" => {
                self.any().plan_resource(resource_name, current_state, desired_input).await
            }
            "runtimeconfig_api" => {
                self.runtimeconfig_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "retail_api" => {
                self.retail_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "books_api" => {
                self.books_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "websecurityscanner_api" => {
                self.websecurityscanner_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "localservices_api" => {
                self.localservices_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "servicedirectory_api" => {
                self.servicedirectory_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "customsearch_api" => {
                self.customsearch_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "policyanalyzer_api" => {
                self.policyanalyzer_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinessbusinesscalls_api" => {
                self.mybusinessbusinesscalls_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "ml_api" => {
                self.ml_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebaseml_api" => {
                self.firebaseml_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "blogger_api" => {
                self.blogger_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "vault_api" => {
                self.vault_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "baremetalsolution_api" => {
                self.baremetalsolution_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "testing_api" => {
                self.testing_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "recaptchaenterprise_api" => {
                self.recaptchaenterprise_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "servicecontrol_api" => {
                self.servicecontrol_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "documentai_api" => {
                self.documentai_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "firebasedynamiclinks_api" => {
                self.firebasedynamiclinks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "apigee_api" => {
                self.apigee_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "netapp_api" => {
                self.netapp_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "identitytoolkit_api" => {
                self.identitytoolkit_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datastream_api" => {
                self.datastream_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "iamcredentials_api" => {
                self.iamcredentials_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "datalabeling_api" => {
                self.datalabeling_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "youtubereporting_api" => {
                self.youtubereporting_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "genomics_api" => {
                self.genomics_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "lifesciences_api" => {
                self.lifesciences_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "compute_api" => {
                self.compute_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "youtubeanalytics_api" => {
                self.youtubeanalytics_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "factchecktools_api" => {
                self.factchecktools_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "dialogflow_api" => {
                self.dialogflow_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "docs_api" => {
                self.docs_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "tagmanager_api" => {
                self.tagmanager_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "texttospeech_api" => {
                self.texttospeech_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "gmail_api" => {
                self.gmail_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "tasks_api" => {
                self.tasks_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "securityposture_api" => {
                self.securityposture_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "sql_api" => {
                self.sql_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "integrations_api" => {
                self.integrations_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "essentialcontacts_api" => {
                self.essentialcontacts_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "tpu_api" => {
                self.tpu_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "classroom_api" => {
                self.classroom_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "mybusinesslodging_api" => {
                self.mybusinesslodging_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "rapidmigrationassessment_api" => {
                self.rapidmigrationassessment_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "blockchainnodeengine_api" => {
                self.blockchainnodeengine_api().plan_resource(resource_name, current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Create a new resource
    async fn create(&self, resource_type: &str, input: ResourceInput) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "translate_api" => {
                self.translate_api().create_resource(resource_name, input).await
            }
            "area120tables_api" => {
                self.area120tables_api().create_resource(resource_name, input).await
            }
            "abusiveexperiencereport_api" => {
                self.abusiveexperiencereport_api().create_resource(resource_name, input).await
            }
            "securesourcemanager_api" => {
                self.securesourcemanager_api().create_resource(resource_name, input).await
            }
            "file_api" => {
                self.file_api().create_resource(resource_name, input).await
            }
            "oauth2_api" => {
                self.oauth2_api().create_resource(resource_name, input).await
            }
            "serviceuser_api" => {
                self.serviceuser_api().create_resource(resource_name, input).await
            }
            "bigquerydatatransfer_api" => {
                self.bigquerydatatransfer_api().create_resource(resource_name, input).await
            }
            "cloudfunctions_api" => {
                self.cloudfunctions_api().create_resource(resource_name, input).await
            }
            "networkconnectivity_api" => {
                self.networkconnectivity_api().create_resource(resource_name, input).await
            }
            "mybusinessverifications_api" => {
                self.mybusinessverifications_api().create_resource(resource_name, input).await
            }
            "qpxexpress_api" => {
                self.qpxexpress_api().create_resource(resource_name, input).await
            }
            "dataproc_api" => {
                self.dataproc_api().create_resource(resource_name, input).await
            }
            "playgrouping_api" => {
                self.playgrouping_api().create_resource(resource_name, input).await
            }
            "mirror_api" => {
                self.mirror_api().create_resource(resource_name, input).await
            }
            "adexchangeseller_api" => {
                self.adexchangeseller_api().create_resource(resource_name, input).await
            }
            "binaryauthorization_api" => {
                self.binaryauthorization_api().create_resource(resource_name, input).await
            }
            "marketingplatformadmin_api" => {
                self.marketingplatformadmin_api().create_resource(resource_name, input).await
            }
            "cloudprivatecatalogproducer_api" => {
                self.cloudprivatecatalogproducer_api().create_resource(resource_name, input).await
            }
            "pubsublite_api" => {
                self.pubsublite_api().create_resource(resource_name, input).await
            }
            "slides_api" => {
                self.slides_api().create_resource(resource_name, input).await
            }
            "billingbudgets_api" => {
                self.billingbudgets_api().create_resource(resource_name, input).await
            }
            "workstations_api" => {
                self.workstations_api().create_resource(resource_name, input).await
            }
            "civicinfo_api" => {
                self.civicinfo_api().create_resource(resource_name, input).await
            }
            "spectrum_api" => {
                self.spectrum_api().create_resource(resource_name, input).await
            }
            "surveys_api" => {
                self.surveys_api().create_resource(resource_name, input).await
            }
            "dataplex_api" => {
                self.dataplex_api().create_resource(resource_name, input).await
            }
            "redis_api" => {
                self.redis_api().create_resource(resource_name, input).await
            }
            "addressvalidation_api" => {
                self.addressvalidation_api().create_resource(resource_name, input).await
            }
            "sourcerepo_api" => {
                self.sourcerepo_api().create_resource(resource_name, input).await
            }
            "osconfig_api" => {
                self.osconfig_api().create_resource(resource_name, input).await
            }
            "domainsrdap_api" => {
                self.domainsrdap_api().create_resource(resource_name, input).await
            }
            "speech_api" => {
                self.speech_api().create_resource(resource_name, input).await
            }
            "trafficdirector_api" => {
                self.trafficdirector_api().create_resource(resource_name, input).await
            }
            "indexing_api" => {
                self.indexing_api().create_resource(resource_name, input).await
            }
            "ondemandscanning_api" => {
                self.ondemandscanning_api().create_resource(resource_name, input).await
            }
            "drive_api" => {
                self.drive_api().create_resource(resource_name, input).await
            }
            "mybusinessnotifications_api" => {
                self.mybusinessnotifications_api().create_resource(resource_name, input).await
            }
            "workflowexecutions_api" => {
                self.workflowexecutions_api().create_resource(resource_name, input).await
            }
            "developerconnect_api" => {
                self.developerconnect_api().create_resource(resource_name, input).await
            }
            "analyticsadmin_api" => {
                self.analyticsadmin_api().create_resource(resource_name, input).await
            }
            "playdeveloperreporting_api" => {
                self.playdeveloperreporting_api().create_resource(resource_name, input).await
            }
            "prod_tt_sasportal_api" => {
                self.prod_tt_sasportal_api().create_resource(resource_name, input).await
            }
            "searchconsole_api" => {
                self.searchconsole_api().create_resource(resource_name, input).await
            }
            "firebaseremoteconfig_api" => {
                self.firebaseremoteconfig_api().create_resource(resource_name, input).await
            }
            "publicca_api" => {
                self.publicca_api().create_resource(resource_name, input).await
            }
            "discovery_api" => {
                self.discovery_api().create_resource(resource_name, input).await
            }
            "firebasehosting_api" => {
                self.firebasehosting_api().create_resource(resource_name, input).await
            }
            "memcache_api" => {
                self.memcache_api().create_resource(resource_name, input).await
            }
            "playablelocations_api" => {
                self.playablelocations_api().create_resource(resource_name, input).await
            }
            "jobs_api" => {
                self.jobs_api().create_resource(resource_name, input).await
            }
            "iam_api" => {
                self.iam_api().create_resource(resource_name, input).await
            }
            "reseller_api" => {
                self.reseller_api().create_resource(resource_name, input).await
            }
            "realtimebidding_api" => {
                self.realtimebidding_api().create_resource(resource_name, input).await
            }
            "datamigration_api" => {
                self.datamigration_api().create_resource(resource_name, input).await
            }
            "safebrowsing_api" => {
                self.safebrowsing_api().create_resource(resource_name, input).await
            }
            "libraryagent_api" => {
                self.libraryagent_api().create_resource(resource_name, input).await
            }
            "css_api" => {
                self.css_api().create_resource(resource_name, input).await
            }
            "androidpublisher_api" => {
                self.androidpublisher_api().create_resource(resource_name, input).await
            }
            "datacatalog_api" => {
                self.datacatalog_api().create_resource(resource_name, input).await
            }
            "cloudlocationfinder_api" => {
                self.cloudlocationfinder_api().create_resource(resource_name, input).await
            }
            "storagetransfer_api" => {
                self.storagetransfer_api().create_resource(resource_name, input).await
            }
            "resourcesettings_api" => {
                self.resourcesettings_api().create_resource(resource_name, input).await
            }
            "firebasedatabase_api" => {
                self.firebasedatabase_api().create_resource(resource_name, input).await
            }
            "analyticshub_api" => {
                self.analyticshub_api().create_resource(resource_name, input).await
            }
            "http_body" => {
                self.http_body().create_resource(resource_name, input).await
            }
            "managedkafka_api" => {
                self.managedkafka_api().create_resource(resource_name, input).await
            }
            "poly_api" => {
                self.poly_api().create_resource(resource_name, input).await
            }
            "orgpolicy_api" => {
                self.orgpolicy_api().create_resource(resource_name, input).await
            }
            "certificatemanager_api" => {
                self.certificatemanager_api().create_resource(resource_name, input).await
            }
            "apigateway_api" => {
                self.apigateway_api().create_resource(resource_name, input).await
            }
            "datastore_api" => {
                self.datastore_api().create_resource(resource_name, input).await
            }
            "cloudtasks_api" => {
                self.cloudtasks_api().create_resource(resource_name, input).await
            }
            "firebaseappdistribution_api" => {
                self.firebaseappdistribution_api().create_resource(resource_name, input).await
            }
            "domains_api" => {
                self.domains_api().create_resource(resource_name, input).await
            }
            "policysimulator_api" => {
                self.policysimulator_api().create_resource(resource_name, input).await
            }
            "apihub_api" => {
                self.apihub_api().create_resource(resource_name, input).await
            }
            "mybusinessaccountmanagement_api" => {
                self.mybusinessaccountmanagement_api().create_resource(resource_name, input).await
            }
            "smartdevicemanagement_api" => {
                self.smartdevicemanagement_api().create_resource(resource_name, input).await
            }
            "tracing_api" => {
                self.tracing_api().create_resource(resource_name, input).await
            }
            "gamesmanagement_api" => {
                self.gamesmanagement_api().create_resource(resource_name, input).await
            }
            "firestore_api" => {
                self.firestore_api().create_resource(resource_name, input).await
            }
            "language_api" => {
                self.language_api().create_resource(resource_name, input).await
            }
            "json_body" => {
                self.json_body().create_resource(resource_name, input).await
            }
            "replicapoolupdater_api" => {
                self.replicapoolupdater_api().create_resource(resource_name, input).await
            }
            "androidmanagement_api" => {
                self.androidmanagement_api().create_resource(resource_name, input).await
            }
            "licensing_api" => {
                self.licensing_api().create_resource(resource_name, input).await
            }
            "sasportal_api" => {
                self.sasportal_api().create_resource(resource_name, input).await
            }
            "vmwareengine_api" => {
                self.vmwareengine_api().create_resource(resource_name, input).await
            }
            "pollen_api" => {
                self.pollen_api().create_resource(resource_name, input).await
            }
            "cloudsupport_api" => {
                self.cloudsupport_api().create_resource(resource_name, input).await
            }
            "servicemanagement_api" => {
                self.servicemanagement_api().create_resource(resource_name, input).await
            }
            "cloudiot_api" => {
                self.cloudiot_api().create_resource(resource_name, input).await
            }
            "androiddeviceprovisioning_api" => {
                self.androiddeviceprovisioning_api().create_resource(resource_name, input).await
            }
            "gamesconfiguration_api" => {
                self.gamesconfiguration_api().create_resource(resource_name, input).await
            }
            "areainsights_api" => {
                self.areainsights_api().create_resource(resource_name, input).await
            }
            "meet_api" => {
                self.meet_api().create_resource(resource_name, input).await
            }
            "adsense_api" => {
                self.adsense_api().create_resource(resource_name, input).await
            }
            "toolresults_api" => {
                self.toolresults_api().create_resource(resource_name, input).await
            }
            "artifactregistry_api" => {
                self.artifactregistry_api().create_resource(resource_name, input).await
            }
            "cloudcommerceprocurement_api" => {
                self.cloudcommerceprocurement_api().create_resource(resource_name, input).await
            }
            "alertcenter_api" => {
                self.alertcenter_api().create_resource(resource_name, input).await
            }
            "mybusinessbusinessinformation_api" => {
                self.mybusinessbusinessinformation_api().create_resource(resource_name, input).await
            }
            "bigquerydatapolicy_api" => {
                self.bigquerydatapolicy_api().create_resource(resource_name, input).await
            }
            "androidenterprise_api" => {
                self.androidenterprise_api().create_resource(resource_name, input).await
            }
            "playmoviespartner_api" => {
                self.playmoviespartner_api().create_resource(resource_name, input).await
            }
            "oracledatabase_api" => {
                self.oracledatabase_api().create_resource(resource_name, input).await
            }
            "composer_api" => {
                self.composer_api().create_resource(resource_name, input).await
            }
            "dataform_api" => {
                self.dataform_api().create_resource(resource_name, input).await
            }
            "adsensehost_api" => {
                self.adsensehost_api().create_resource(resource_name, input).await
            }
            "deploymentmanager_api" => {
                self.deploymentmanager_api().create_resource(resource_name, input).await
            }
            "displayvideo_api" => {
                self.displayvideo_api().create_resource(resource_name, input).await
            }
            "chromewebstore_api" => {
                self.chromewebstore_api().create_resource(resource_name, input).await
            }
            "cloudkms_api" => {
                self.cloudkms_api().create_resource(resource_name, input).await
            }
            "bigtableadmin_api" => {
                self.bigtableadmin_api().create_resource(resource_name, input).await
            }
            "drivelabels_api" => {
                self.drivelabels_api().create_resource(resource_name, input).await
            }
            "vision_api" => {
                self.vision_api().create_resource(resource_name, input).await
            }
            "streetviewpublish_api" => {
                self.streetviewpublish_api().create_resource(resource_name, input).await
            }
            "clouddeploy_api" => {
                self.clouddeploy_api().create_resource(resource_name, input).await
            }
            "biglake_api" => {
                self.biglake_api().create_resource(resource_name, input).await
            }
            "eventarc_api" => {
                self.eventarc_api().create_resource(resource_name, input).await
            }
            "apigeeregistry_api" => {
                self.apigeeregistry_api().create_resource(resource_name, input).await
            }
            "cloudbuild_api" => {
                self.cloudbuild_api().create_resource(resource_name, input).await
            }
            "secretmanager_api" => {
                self.secretmanager_api().create_resource(resource_name, input).await
            }
            "sheets_api" => {
                self.sheets_api().create_resource(resource_name, input).await
            }
            "recommendationengine_api" => {
                self.recommendationengine_api().create_resource(resource_name, input).await
            }
            "appengine_api" => {
                self.appengine_api().create_resource(resource_name, input).await
            }
            "dataportability_api" => {
                self.dataportability_api().create_resource(resource_name, input).await
            }
            "remotebuildexecution_api" => {
                self.remotebuildexecution_api().create_resource(resource_name, input).await
            }
            "containeranalysis_api" => {
                self.containeranalysis_api().create_resource(resource_name, input).await
            }
            "workloadmanager_api" => {
                self.workloadmanager_api().create_resource(resource_name, input).await
            }
            "cloudcontrolspartner_api" => {
                self.cloudcontrolspartner_api().create_resource(resource_name, input).await
            }
            "monitoring_api" => {
                self.monitoring_api().create_resource(resource_name, input).await
            }
            "privateca_api" => {
                self.privateca_api().create_resource(resource_name, input).await
            }
            "container_api" => {
                self.container_api().create_resource(resource_name, input).await
            }
            "merchantapi_api" => {
                self.merchantapi_api().create_resource(resource_name, input).await
            }
            "datapipelines_api" => {
                self.datapipelines_api().create_resource(resource_name, input).await
            }
            "sts_api" => {
                self.sts_api().create_resource(resource_name, input).await
            }
            "observability_api" => {
                self.observability_api().create_resource(resource_name, input).await
            }
            "dataflow_api" => {
                self.dataflow_api().create_resource(resource_name, input).await
            }
            "cloudresourcemanager_api" => {
                self.cloudresourcemanager_api().create_resource(resource_name, input).await
            }
            "firebaseapphosting_api" => {
                self.firebaseapphosting_api().create_resource(resource_name, input).await
            }
            "verifiedaccess_api" => {
                self.verifiedaccess_api().create_resource(resource_name, input).await
            }
            "accessapproval_api" => {
                self.accessapproval_api().create_resource(resource_name, input).await
            }
            "firebasedataconnect_api" => {
                self.firebasedataconnect_api().create_resource(resource_name, input).await
            }
            "assuredworkloads_api" => {
                self.assuredworkloads_api().create_resource(resource_name, input).await
            }
            "datafusion_api" => {
                self.datafusion_api().create_resource(resource_name, input).await
            }
            "doubleclicksearch_api" => {
                self.doubleclicksearch_api().create_resource(resource_name, input).await
            }
            "ideahub_api" => {
                self.ideahub_api().create_resource(resource_name, input).await
            }
            "alloydb_api" => {
                self.alloydb_api().create_resource(resource_name, input).await
            }
            "doubleclickbidmanager_api" => {
                self.doubleclickbidmanager_api().create_resource(resource_name, input).await
            }
            "bigqueryreservation_api" => {
                self.bigqueryreservation_api().create_resource(resource_name, input).await
            }
            "notebooks_api" => {
                self.notebooks_api().create_resource(resource_name, input).await
            }
            "readerrevenuesubscriptionlinking_api" => {
                self.readerrevenuesubscriptionlinking_api().create_resource(resource_name, input).await
            }
            "authorizedbuyersmarketplace_api" => {
                self.authorizedbuyersmarketplace_api().create_resource(resource_name, input).await
            }
            "adexchangebuyer2_api" => {
                self.adexchangebuyer2_api().create_resource(resource_name, input).await
            }
            "adsenseplatform_api" => {
                self.adsenseplatform_api().create_resource(resource_name, input).await
            }
            "mybusinessplaceactions_api" => {
                self.mybusinessplaceactions_api().create_resource(resource_name, input).await
            }
            "firebasestorage_api" => {
                self.firebasestorage_api().create_resource(resource_name, input).await
            }
            "calendar_api" => {
                self.calendar_api().create_resource(resource_name, input).await
            }
            "networkmanagement_api" => {
                self.networkmanagement_api().create_resource(resource_name, input).await
            }
            "admin_api" => {
                self.admin_api().create_resource(resource_name, input).await
            }
            "acceleratedmobilepageurl_api" => {
                self.acceleratedmobilepageurl_api().create_resource(resource_name, input).await
            }
            "fusiontables_api" => {
                self.fusiontables_api().create_resource(resource_name, input).await
            }
            "chromepolicy_api" => {
                self.chromepolicy_api().create_resource(resource_name, input).await
            }
            "securitycenter_api" => {
                self.securitycenter_api().create_resource(resource_name, input).await
            }
            "analyticsreporting_api" => {
                self.analyticsreporting_api().create_resource(resource_name, input).await
            }
            "iap_api" => {
                self.iap_api().create_resource(resource_name, input).await
            }
            "consumersurveys_api" => {
                self.consumersurveys_api().create_resource(resource_name, input).await
            }
            "siteverification_api" => {
                self.siteverification_api().create_resource(resource_name, input).await
            }
            "versionhistory_api" => {
                self.versionhistory_api().create_resource(resource_name, input).await
            }
            "storagebatchoperations_api" => {
                self.storagebatchoperations_api().create_resource(resource_name, input).await
            }
            "acmedns_api" => {
                self.acmedns_api().create_resource(resource_name, input).await
            }
            "storage_api" => {
                self.storage_api().create_resource(resource_name, input).await
            }
            "networkservices_api" => {
                self.networkservices_api().create_resource(resource_name, input).await
            }
            "dfareporting_api" => {
                self.dfareporting_api().create_resource(resource_name, input).await
            }
            "webfonts_api" => {
                self.webfonts_api().create_resource(resource_name, input).await
            }
            "apim_api" => {
                self.apim_api().create_resource(resource_name, input).await
            }
            "chromeuxreport_api" => {
                self.chromeuxreport_api().create_resource(resource_name, input).await
            }
            "aiplatform_api" => {
                self.aiplatform_api().create_resource(resource_name, input).await
            }
            "script_api" => {
                self.script_api().create_resource(resource_name, input).await
            }
            "contentwarehouse_api" => {
                self.contentwarehouse_api().create_resource(resource_name, input).await
            }
            "batch_api" => {
                self.batch_api().create_resource(resource_name, input).await
            }
            "kmsinventory_api" => {
                self.kmsinventory_api().create_resource(resource_name, input).await
            }
            "run_api" => {
                self.run_api().create_resource(resource_name, input).await
            }
            "servicenetworking_api" => {
                self.servicenetworking_api().create_resource(resource_name, input).await
            }
            "cloudscheduler_api" => {
                self.cloudscheduler_api().create_resource(resource_name, input).await
            }
            "cloudsearch_api" => {
                self.cloudsearch_api().create_resource(resource_name, input).await
            }
            "admob_api" => {
                self.admob_api().create_resource(resource_name, input).await
            }
            "workspaceevents_api" => {
                self.workspaceevents_api().create_resource(resource_name, input).await
            }
            "metastore_api" => {
                self.metastore_api().create_resource(resource_name, input).await
            }
            "policytroubleshooter_api" => {
                self.policytroubleshooter_api().create_resource(resource_name, input).await
            }
            "analyticsdata_api" => {
                self.analyticsdata_api().create_resource(resource_name, input).await
            }
            "contactcenteraiplatform_api" => {
                self.contactcenteraiplatform_api().create_resource(resource_name, input).await
            }
            "forms_api" => {
                self.forms_api().create_resource(resource_name, input).await
            }
            "clouderrorreporting_api" => {
                self.clouderrorreporting_api().create_resource(resource_name, input).await
            }
            "pagespeedonline_api" => {
                self.pagespeedonline_api().create_resource(resource_name, input).await
            }
            "parallelstore_api" => {
                self.parallelstore_api().create_resource(resource_name, input).await
            }
            "cloudbilling_api" => {
                self.cloudbilling_api().create_resource(resource_name, input).await
            }
            "cloudidentity_api" => {
                self.cloudidentity_api().create_resource(resource_name, input).await
            }
            "appsactivity_api" => {
                self.appsactivity_api().create_resource(resource_name, input).await
            }
            "groupssettings_api" => {
                self.groupssettings_api().create_resource(resource_name, input).await
            }
            "adexchangebuyer_api" => {
                self.adexchangebuyer_api().create_resource(resource_name, input).await
            }
            "youtube_api" => {
                self.youtube_api().create_resource(resource_name, input).await
            }
            "looker_api" => {
                self.looker_api().create_resource(resource_name, input).await
            }
            "gkebackup_api" => {
                self.gkebackup_api().create_resource(resource_name, input).await
            }
            "cloudasset_api" => {
                self.cloudasset_api().create_resource(resource_name, input).await
            }
            "people_api" => {
                self.people_api().create_resource(resource_name, input).await
            }
            "mybusinessqanda_api" => {
                self.mybusinessqanda_api().create_resource(resource_name, input).await
            }
            "beyondcorp_api" => {
                self.beyondcorp_api().create_resource(resource_name, input).await
            }
            "pubsub_api" => {
                self.pubsub_api().create_resource(resource_name, input).await
            }
            "servicebroker_api" => {
                self.servicebroker_api().create_resource(resource_name, input).await
            }
            "contactcenterinsights_api" => {
                self.contactcenterinsights_api().create_resource(resource_name, input).await
            }
            "accesscontextmanager_api" => {
                self.accesscontextmanager_api().create_resource(resource_name, input).await
            }
            "searchads360_api" => {
                self.searchads360_api().create_resource(resource_name, input).await
            }
            "oslogin_api" => {
                self.oslogin_api().create_resource(resource_name, input).await
            }
            "playcustomapp_api" => {
                self.playcustomapp_api().create_resource(resource_name, input).await
            }
            "webmasters_api" => {
                self.webmasters_api().create_resource(resource_name, input).await
            }
            "fitness_api" => {
                self.fitness_api().create_resource(resource_name, input).await
            }
            "parametermanager_api" => {
                self.parametermanager_api().create_resource(resource_name, input).await
            }
            "businessprofileperformance_api" => {
                self.businessprofileperformance_api().create_resource(resource_name, input).await
            }
            "videointelligence_api" => {
                self.videointelligence_api().create_resource(resource_name, input).await
            }
            "replicapool_api" => {
                self.replicapool_api().create_resource(resource_name, input).await
            }
            "dlp_api" => {
                self.dlp_api().create_resource(resource_name, input).await
            }
            "migrationcenter_api" => {
                self.migrationcenter_api().create_resource(resource_name, input).await
            }
            "kgsearch_api" => {
                self.kgsearch_api().create_resource(resource_name, input).await
            }
            "cloudprofiler_api" => {
                self.cloudprofiler_api().create_resource(resource_name, input).await
            }
            "healthcare_api" => {
                self.healthcare_api().create_resource(resource_name, input).await
            }
            "firebaseappcheck_api" => {
                self.firebaseappcheck_api().create_resource(resource_name, input).await
            }
            "gkeonprem_api" => {
                self.gkeonprem_api().create_resource(resource_name, input).await
            }
            "repeated_any_query_error" => {
                self.repeated_any_query_error().create_resource(resource_name, input).await
            }
            "transcoder_api" => {
                self.transcoder_api().create_resource(resource_name, input).await
            }
            "firebaserules_api" => {
                self.firebaserules_api().create_resource(resource_name, input).await
            }
            "plusdomains_api" => {
                self.plusdomains_api().create_resource(resource_name, input).await
            }
            "proximitybeacon_api" => {
                self.proximitybeacon_api().create_resource(resource_name, input).await
            }
            "firebase_api" => {
                self.firebase_api().create_resource(resource_name, input).await
            }
            "resource_named_service" => {
                self.resource_named_service().create_resource(resource_name, input).await
            }
            "config_api" => {
                self.config_api().create_resource(resource_name, input).await
            }
            "games_api" => {
                self.games_api().create_resource(resource_name, input).await
            }
            "appstate_api" => {
                self.appstate_api().create_resource(resource_name, input).await
            }
            "saasservicemgmt_api" => {
                self.saasservicemgmt_api().create_resource(resource_name, input).await
            }
            "test_api" => {
                self.test_api().create_resource(resource_name, input).await
            }
            "gkehub_api" => {
                self.gkehub_api().create_resource(resource_name, input).await
            }
            "analytics_api" => {
                self.analytics_api().create_resource(resource_name, input).await
            }
            "discoveryengine_api" => {
                self.discoveryengine_api().create_resource(resource_name, input).await
            }
            "playintegrity_api" => {
                self.playintegrity_api().create_resource(resource_name, input).await
            }
            "homegraph_api" => {
                self.homegraph_api().create_resource(resource_name, input).await
            }
            "apikeys_api" => {
                self.apikeys_api().create_resource(resource_name, input).await
            }
            "serviceconsumermanagement_api" => {
                self.serviceconsumermanagement_api().create_resource(resource_name, input).await
            }
            "datalineage_api" => {
                self.datalineage_api().create_resource(resource_name, input).await
            }
            "manufacturers_api" => {
                self.manufacturers_api().create_resource(resource_name, input).await
            }
            "vectortile_api" => {
                self.vectortile_api().create_resource(resource_name, input).await
            }
            "chat_api" => {
                self.chat_api().create_resource(resource_name, input).await
            }
            "bigquery_api" => {
                self.bigquery_api().create_resource(resource_name, input).await
            }
            "plus_api" => {
                self.plus_api().create_resource(resource_name, input).await
            }
            "bigqueryconnection_api" => {
                self.bigqueryconnection_api().create_resource(resource_name, input).await
            }
            "places_api" => {
                self.places_api().create_resource(resource_name, input).await
            }
            "logging_api" => {
                self.logging_api().create_resource(resource_name, input).await
            }
            "webrisk_api" => {
                self.webrisk_api().create_resource(resource_name, input).await
            }
            "vpcaccess_api" => {
                self.vpcaccess_api().create_resource(resource_name, input).await
            }
            "urlshortener_api" => {
                self.urlshortener_api().create_resource(resource_name, input).await
            }
            "sqladmin_api" => {
                self.sqladmin_api().create_resource(resource_name, input).await
            }
            "keep_api" => {
                self.keep_api().create_resource(resource_name, input).await
            }
            "walletobjects_api" => {
                self.walletobjects_api().create_resource(resource_name, input).await
            }
            "apphub_api" => {
                self.apphub_api().create_resource(resource_name, input).await
            }
            "vmmigration_api" => {
                self.vmmigration_api().create_resource(resource_name, input).await
            }
            "airquality_api" => {
                self.airquality_api().create_resource(resource_name, input).await
            }
            "chromemanagement_api" => {
                self.chromemanagement_api().create_resource(resource_name, input).await
            }
            "adexperiencereport_api" => {
                self.adexperiencereport_api().create_resource(resource_name, input).await
            }
            "recommender_api" => {
                self.recommender_api().create_resource(resource_name, input).await
            }
            "driveactivity_api" => {
                self.driveactivity_api().create_resource(resource_name, input).await
            }
            "dns_api" => {
                self.dns_api().create_resource(resource_name, input).await
            }
            "cloudshell_api" => {
                self.cloudshell_api().create_resource(resource_name, input).await
            }
            "content_api" => {
                self.content_api().create_resource(resource_name, input).await
            }
            "commentanalyzer_api" => {
                self.commentanalyzer_api().create_resource(resource_name, input).await
            }
            "backupdr_api" => {
                self.backupdr_api().create_resource(resource_name, input).await
            }
            "groupsmigration_api" => {
                self.groupsmigration_api().create_resource(resource_name, input).await
            }
            "connectors_api" => {
                self.connectors_api().create_resource(resource_name, input).await
            }
            "solar_api" => {
                self.solar_api().create_resource(resource_name, input).await
            }
            "fcm_api" => {
                self.fcm_api().create_resource(resource_name, input).await
            }
            "networksecurity_api" => {
                self.networksecurity_api().create_resource(resource_name, input).await
            }
            "managedidentities_api" => {
                self.managedidentities_api().create_resource(resource_name, input).await
            }
            "serviceusage_api" => {
                self.serviceusage_api().create_resource(resource_name, input).await
            }
            "digitalassetlinks_api" => {
                self.digitalassetlinks_api().create_resource(resource_name, input).await
            }
            "clouddebugger_api" => {
                self.clouddebugger_api().create_resource(resource_name, input).await
            }
            "datamanager_api" => {
                self.datamanager_api().create_resource(resource_name, input).await
            }
            "ids_api" => {
                self.ids_api().create_resource(resource_name, input).await
            }
            "workflows_api" => {
                self.workflows_api().create_resource(resource_name, input).await
            }
            "cloudtrace_api" => {
                self.cloudtrace_api().create_resource(resource_name, input).await
            }
            "advisorynotifications_api" => {
                self.advisorynotifications_api().create_resource(resource_name, input).await
            }
            "fcmdata_api" => {
                self.fcmdata_api().create_resource(resource_name, input).await
            }
            "checks_api" => {
                self.checks_api().create_resource(resource_name, input).await
            }
            "blogger_3" => {
                self.blogger_3().create_resource(resource_name, input).await
            }
            "mediaupload" => {
                self.mediaupload().create_resource(resource_name, input).await
            }
            "paymentsresellersubscription_api" => {
                self.paymentsresellersubscription_api().create_resource(resource_name, input).await
            }
            "spanner_api" => {
                self.spanner_api().create_resource(resource_name, input).await
            }
            "cloudchannel_api" => {
                self.cloudchannel_api().create_resource(resource_name, input).await
            }
            "gameservices_api" => {
                self.gameservices_api().create_resource(resource_name, input).await
            }
            "cloudprivatecatalog_api" => {
                self.cloudprivatecatalog_api().create_resource(resource_name, input).await
            }
            "gmailpostmastertools_api" => {
                self.gmailpostmastertools_api().create_resource(resource_name, input).await
            }
            "partners_api" => {
                self.partners_api().create_resource(resource_name, input).await
            }
            "any" => {
                self.any().create_resource(resource_name, input).await
            }
            "runtimeconfig_api" => {
                self.runtimeconfig_api().create_resource(resource_name, input).await
            }
            "retail_api" => {
                self.retail_api().create_resource(resource_name, input).await
            }
            "books_api" => {
                self.books_api().create_resource(resource_name, input).await
            }
            "websecurityscanner_api" => {
                self.websecurityscanner_api().create_resource(resource_name, input).await
            }
            "localservices_api" => {
                self.localservices_api().create_resource(resource_name, input).await
            }
            "servicedirectory_api" => {
                self.servicedirectory_api().create_resource(resource_name, input).await
            }
            "customsearch_api" => {
                self.customsearch_api().create_resource(resource_name, input).await
            }
            "policyanalyzer_api" => {
                self.policyanalyzer_api().create_resource(resource_name, input).await
            }
            "mybusinessbusinesscalls_api" => {
                self.mybusinessbusinesscalls_api().create_resource(resource_name, input).await
            }
            "ml_api" => {
                self.ml_api().create_resource(resource_name, input).await
            }
            "firebaseml_api" => {
                self.firebaseml_api().create_resource(resource_name, input).await
            }
            "blogger_api" => {
                self.blogger_api().create_resource(resource_name, input).await
            }
            "vault_api" => {
                self.vault_api().create_resource(resource_name, input).await
            }
            "baremetalsolution_api" => {
                self.baremetalsolution_api().create_resource(resource_name, input).await
            }
            "testing_api" => {
                self.testing_api().create_resource(resource_name, input).await
            }
            "recaptchaenterprise_api" => {
                self.recaptchaenterprise_api().create_resource(resource_name, input).await
            }
            "servicecontrol_api" => {
                self.servicecontrol_api().create_resource(resource_name, input).await
            }
            "documentai_api" => {
                self.documentai_api().create_resource(resource_name, input).await
            }
            "firebasedynamiclinks_api" => {
                self.firebasedynamiclinks_api().create_resource(resource_name, input).await
            }
            "apigee_api" => {
                self.apigee_api().create_resource(resource_name, input).await
            }
            "netapp_api" => {
                self.netapp_api().create_resource(resource_name, input).await
            }
            "identitytoolkit_api" => {
                self.identitytoolkit_api().create_resource(resource_name, input).await
            }
            "datastream_api" => {
                self.datastream_api().create_resource(resource_name, input).await
            }
            "iamcredentials_api" => {
                self.iamcredentials_api().create_resource(resource_name, input).await
            }
            "datalabeling_api" => {
                self.datalabeling_api().create_resource(resource_name, input).await
            }
            "youtubereporting_api" => {
                self.youtubereporting_api().create_resource(resource_name, input).await
            }
            "genomics_api" => {
                self.genomics_api().create_resource(resource_name, input).await
            }
            "lifesciences_api" => {
                self.lifesciences_api().create_resource(resource_name, input).await
            }
            "compute_api" => {
                self.compute_api().create_resource(resource_name, input).await
            }
            "youtubeanalytics_api" => {
                self.youtubeanalytics_api().create_resource(resource_name, input).await
            }
            "factchecktools_api" => {
                self.factchecktools_api().create_resource(resource_name, input).await
            }
            "dialogflow_api" => {
                self.dialogflow_api().create_resource(resource_name, input).await
            }
            "docs_api" => {
                self.docs_api().create_resource(resource_name, input).await
            }
            "tagmanager_api" => {
                self.tagmanager_api().create_resource(resource_name, input).await
            }
            "texttospeech_api" => {
                self.texttospeech_api().create_resource(resource_name, input).await
            }
            "gmail_api" => {
                self.gmail_api().create_resource(resource_name, input).await
            }
            "tasks_api" => {
                self.tasks_api().create_resource(resource_name, input).await
            }
            "securityposture_api" => {
                self.securityposture_api().create_resource(resource_name, input).await
            }
            "sql_api" => {
                self.sql_api().create_resource(resource_name, input).await
            }
            "integrations_api" => {
                self.integrations_api().create_resource(resource_name, input).await
            }
            "essentialcontacts_api" => {
                self.essentialcontacts_api().create_resource(resource_name, input).await
            }
            "tpu_api" => {
                self.tpu_api().create_resource(resource_name, input).await
            }
            "classroom_api" => {
                self.classroom_api().create_resource(resource_name, input).await
            }
            "mybusinesslodging_api" => {
                self.mybusinesslodging_api().create_resource(resource_name, input).await
            }
            "rapidmigrationassessment_api" => {
                self.rapidmigrationassessment_api().create_resource(resource_name, input).await
            }
            "blockchainnodeengine_api" => {
                self.blockchainnodeengine_api().create_resource(resource_name, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Read/refresh resource state
    async fn read(&self, resource_type: &str, id: &str) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "translate_api" => {
                self.translate_api().read_resource(resource_name, id).await
            }
            "area120tables_api" => {
                self.area120tables_api().read_resource(resource_name, id).await
            }
            "abusiveexperiencereport_api" => {
                self.abusiveexperiencereport_api().read_resource(resource_name, id).await
            }
            "securesourcemanager_api" => {
                self.securesourcemanager_api().read_resource(resource_name, id).await
            }
            "file_api" => {
                self.file_api().read_resource(resource_name, id).await
            }
            "oauth2_api" => {
                self.oauth2_api().read_resource(resource_name, id).await
            }
            "serviceuser_api" => {
                self.serviceuser_api().read_resource(resource_name, id).await
            }
            "bigquerydatatransfer_api" => {
                self.bigquerydatatransfer_api().read_resource(resource_name, id).await
            }
            "cloudfunctions_api" => {
                self.cloudfunctions_api().read_resource(resource_name, id).await
            }
            "networkconnectivity_api" => {
                self.networkconnectivity_api().read_resource(resource_name, id).await
            }
            "mybusinessverifications_api" => {
                self.mybusinessverifications_api().read_resource(resource_name, id).await
            }
            "qpxexpress_api" => {
                self.qpxexpress_api().read_resource(resource_name, id).await
            }
            "dataproc_api" => {
                self.dataproc_api().read_resource(resource_name, id).await
            }
            "playgrouping_api" => {
                self.playgrouping_api().read_resource(resource_name, id).await
            }
            "mirror_api" => {
                self.mirror_api().read_resource(resource_name, id).await
            }
            "adexchangeseller_api" => {
                self.adexchangeseller_api().read_resource(resource_name, id).await
            }
            "binaryauthorization_api" => {
                self.binaryauthorization_api().read_resource(resource_name, id).await
            }
            "marketingplatformadmin_api" => {
                self.marketingplatformadmin_api().read_resource(resource_name, id).await
            }
            "cloudprivatecatalogproducer_api" => {
                self.cloudprivatecatalogproducer_api().read_resource(resource_name, id).await
            }
            "pubsublite_api" => {
                self.pubsublite_api().read_resource(resource_name, id).await
            }
            "slides_api" => {
                self.slides_api().read_resource(resource_name, id).await
            }
            "billingbudgets_api" => {
                self.billingbudgets_api().read_resource(resource_name, id).await
            }
            "workstations_api" => {
                self.workstations_api().read_resource(resource_name, id).await
            }
            "civicinfo_api" => {
                self.civicinfo_api().read_resource(resource_name, id).await
            }
            "spectrum_api" => {
                self.spectrum_api().read_resource(resource_name, id).await
            }
            "surveys_api" => {
                self.surveys_api().read_resource(resource_name, id).await
            }
            "dataplex_api" => {
                self.dataplex_api().read_resource(resource_name, id).await
            }
            "redis_api" => {
                self.redis_api().read_resource(resource_name, id).await
            }
            "addressvalidation_api" => {
                self.addressvalidation_api().read_resource(resource_name, id).await
            }
            "sourcerepo_api" => {
                self.sourcerepo_api().read_resource(resource_name, id).await
            }
            "osconfig_api" => {
                self.osconfig_api().read_resource(resource_name, id).await
            }
            "domainsrdap_api" => {
                self.domainsrdap_api().read_resource(resource_name, id).await
            }
            "speech_api" => {
                self.speech_api().read_resource(resource_name, id).await
            }
            "trafficdirector_api" => {
                self.trafficdirector_api().read_resource(resource_name, id).await
            }
            "indexing_api" => {
                self.indexing_api().read_resource(resource_name, id).await
            }
            "ondemandscanning_api" => {
                self.ondemandscanning_api().read_resource(resource_name, id).await
            }
            "drive_api" => {
                self.drive_api().read_resource(resource_name, id).await
            }
            "mybusinessnotifications_api" => {
                self.mybusinessnotifications_api().read_resource(resource_name, id).await
            }
            "workflowexecutions_api" => {
                self.workflowexecutions_api().read_resource(resource_name, id).await
            }
            "developerconnect_api" => {
                self.developerconnect_api().read_resource(resource_name, id).await
            }
            "analyticsadmin_api" => {
                self.analyticsadmin_api().read_resource(resource_name, id).await
            }
            "playdeveloperreporting_api" => {
                self.playdeveloperreporting_api().read_resource(resource_name, id).await
            }
            "prod_tt_sasportal_api" => {
                self.prod_tt_sasportal_api().read_resource(resource_name, id).await
            }
            "searchconsole_api" => {
                self.searchconsole_api().read_resource(resource_name, id).await
            }
            "firebaseremoteconfig_api" => {
                self.firebaseremoteconfig_api().read_resource(resource_name, id).await
            }
            "publicca_api" => {
                self.publicca_api().read_resource(resource_name, id).await
            }
            "discovery_api" => {
                self.discovery_api().read_resource(resource_name, id).await
            }
            "firebasehosting_api" => {
                self.firebasehosting_api().read_resource(resource_name, id).await
            }
            "memcache_api" => {
                self.memcache_api().read_resource(resource_name, id).await
            }
            "playablelocations_api" => {
                self.playablelocations_api().read_resource(resource_name, id).await
            }
            "jobs_api" => {
                self.jobs_api().read_resource(resource_name, id).await
            }
            "iam_api" => {
                self.iam_api().read_resource(resource_name, id).await
            }
            "reseller_api" => {
                self.reseller_api().read_resource(resource_name, id).await
            }
            "realtimebidding_api" => {
                self.realtimebidding_api().read_resource(resource_name, id).await
            }
            "datamigration_api" => {
                self.datamigration_api().read_resource(resource_name, id).await
            }
            "safebrowsing_api" => {
                self.safebrowsing_api().read_resource(resource_name, id).await
            }
            "libraryagent_api" => {
                self.libraryagent_api().read_resource(resource_name, id).await
            }
            "css_api" => {
                self.css_api().read_resource(resource_name, id).await
            }
            "androidpublisher_api" => {
                self.androidpublisher_api().read_resource(resource_name, id).await
            }
            "datacatalog_api" => {
                self.datacatalog_api().read_resource(resource_name, id).await
            }
            "cloudlocationfinder_api" => {
                self.cloudlocationfinder_api().read_resource(resource_name, id).await
            }
            "storagetransfer_api" => {
                self.storagetransfer_api().read_resource(resource_name, id).await
            }
            "resourcesettings_api" => {
                self.resourcesettings_api().read_resource(resource_name, id).await
            }
            "firebasedatabase_api" => {
                self.firebasedatabase_api().read_resource(resource_name, id).await
            }
            "analyticshub_api" => {
                self.analyticshub_api().read_resource(resource_name, id).await
            }
            "http_body" => {
                self.http_body().read_resource(resource_name, id).await
            }
            "managedkafka_api" => {
                self.managedkafka_api().read_resource(resource_name, id).await
            }
            "poly_api" => {
                self.poly_api().read_resource(resource_name, id).await
            }
            "orgpolicy_api" => {
                self.orgpolicy_api().read_resource(resource_name, id).await
            }
            "certificatemanager_api" => {
                self.certificatemanager_api().read_resource(resource_name, id).await
            }
            "apigateway_api" => {
                self.apigateway_api().read_resource(resource_name, id).await
            }
            "datastore_api" => {
                self.datastore_api().read_resource(resource_name, id).await
            }
            "cloudtasks_api" => {
                self.cloudtasks_api().read_resource(resource_name, id).await
            }
            "firebaseappdistribution_api" => {
                self.firebaseappdistribution_api().read_resource(resource_name, id).await
            }
            "domains_api" => {
                self.domains_api().read_resource(resource_name, id).await
            }
            "policysimulator_api" => {
                self.policysimulator_api().read_resource(resource_name, id).await
            }
            "apihub_api" => {
                self.apihub_api().read_resource(resource_name, id).await
            }
            "mybusinessaccountmanagement_api" => {
                self.mybusinessaccountmanagement_api().read_resource(resource_name, id).await
            }
            "smartdevicemanagement_api" => {
                self.smartdevicemanagement_api().read_resource(resource_name, id).await
            }
            "tracing_api" => {
                self.tracing_api().read_resource(resource_name, id).await
            }
            "gamesmanagement_api" => {
                self.gamesmanagement_api().read_resource(resource_name, id).await
            }
            "firestore_api" => {
                self.firestore_api().read_resource(resource_name, id).await
            }
            "language_api" => {
                self.language_api().read_resource(resource_name, id).await
            }
            "json_body" => {
                self.json_body().read_resource(resource_name, id).await
            }
            "replicapoolupdater_api" => {
                self.replicapoolupdater_api().read_resource(resource_name, id).await
            }
            "androidmanagement_api" => {
                self.androidmanagement_api().read_resource(resource_name, id).await
            }
            "licensing_api" => {
                self.licensing_api().read_resource(resource_name, id).await
            }
            "sasportal_api" => {
                self.sasportal_api().read_resource(resource_name, id).await
            }
            "vmwareengine_api" => {
                self.vmwareengine_api().read_resource(resource_name, id).await
            }
            "pollen_api" => {
                self.pollen_api().read_resource(resource_name, id).await
            }
            "cloudsupport_api" => {
                self.cloudsupport_api().read_resource(resource_name, id).await
            }
            "servicemanagement_api" => {
                self.servicemanagement_api().read_resource(resource_name, id).await
            }
            "cloudiot_api" => {
                self.cloudiot_api().read_resource(resource_name, id).await
            }
            "androiddeviceprovisioning_api" => {
                self.androiddeviceprovisioning_api().read_resource(resource_name, id).await
            }
            "gamesconfiguration_api" => {
                self.gamesconfiguration_api().read_resource(resource_name, id).await
            }
            "areainsights_api" => {
                self.areainsights_api().read_resource(resource_name, id).await
            }
            "meet_api" => {
                self.meet_api().read_resource(resource_name, id).await
            }
            "adsense_api" => {
                self.adsense_api().read_resource(resource_name, id).await
            }
            "toolresults_api" => {
                self.toolresults_api().read_resource(resource_name, id).await
            }
            "artifactregistry_api" => {
                self.artifactregistry_api().read_resource(resource_name, id).await
            }
            "cloudcommerceprocurement_api" => {
                self.cloudcommerceprocurement_api().read_resource(resource_name, id).await
            }
            "alertcenter_api" => {
                self.alertcenter_api().read_resource(resource_name, id).await
            }
            "mybusinessbusinessinformation_api" => {
                self.mybusinessbusinessinformation_api().read_resource(resource_name, id).await
            }
            "bigquerydatapolicy_api" => {
                self.bigquerydatapolicy_api().read_resource(resource_name, id).await
            }
            "androidenterprise_api" => {
                self.androidenterprise_api().read_resource(resource_name, id).await
            }
            "playmoviespartner_api" => {
                self.playmoviespartner_api().read_resource(resource_name, id).await
            }
            "oracledatabase_api" => {
                self.oracledatabase_api().read_resource(resource_name, id).await
            }
            "composer_api" => {
                self.composer_api().read_resource(resource_name, id).await
            }
            "dataform_api" => {
                self.dataform_api().read_resource(resource_name, id).await
            }
            "adsensehost_api" => {
                self.adsensehost_api().read_resource(resource_name, id).await
            }
            "deploymentmanager_api" => {
                self.deploymentmanager_api().read_resource(resource_name, id).await
            }
            "displayvideo_api" => {
                self.displayvideo_api().read_resource(resource_name, id).await
            }
            "chromewebstore_api" => {
                self.chromewebstore_api().read_resource(resource_name, id).await
            }
            "cloudkms_api" => {
                self.cloudkms_api().read_resource(resource_name, id).await
            }
            "bigtableadmin_api" => {
                self.bigtableadmin_api().read_resource(resource_name, id).await
            }
            "drivelabels_api" => {
                self.drivelabels_api().read_resource(resource_name, id).await
            }
            "vision_api" => {
                self.vision_api().read_resource(resource_name, id).await
            }
            "streetviewpublish_api" => {
                self.streetviewpublish_api().read_resource(resource_name, id).await
            }
            "clouddeploy_api" => {
                self.clouddeploy_api().read_resource(resource_name, id).await
            }
            "biglake_api" => {
                self.biglake_api().read_resource(resource_name, id).await
            }
            "eventarc_api" => {
                self.eventarc_api().read_resource(resource_name, id).await
            }
            "apigeeregistry_api" => {
                self.apigeeregistry_api().read_resource(resource_name, id).await
            }
            "cloudbuild_api" => {
                self.cloudbuild_api().read_resource(resource_name, id).await
            }
            "secretmanager_api" => {
                self.secretmanager_api().read_resource(resource_name, id).await
            }
            "sheets_api" => {
                self.sheets_api().read_resource(resource_name, id).await
            }
            "recommendationengine_api" => {
                self.recommendationengine_api().read_resource(resource_name, id).await
            }
            "appengine_api" => {
                self.appengine_api().read_resource(resource_name, id).await
            }
            "dataportability_api" => {
                self.dataportability_api().read_resource(resource_name, id).await
            }
            "remotebuildexecution_api" => {
                self.remotebuildexecution_api().read_resource(resource_name, id).await
            }
            "containeranalysis_api" => {
                self.containeranalysis_api().read_resource(resource_name, id).await
            }
            "workloadmanager_api" => {
                self.workloadmanager_api().read_resource(resource_name, id).await
            }
            "cloudcontrolspartner_api" => {
                self.cloudcontrolspartner_api().read_resource(resource_name, id).await
            }
            "monitoring_api" => {
                self.monitoring_api().read_resource(resource_name, id).await
            }
            "privateca_api" => {
                self.privateca_api().read_resource(resource_name, id).await
            }
            "container_api" => {
                self.container_api().read_resource(resource_name, id).await
            }
            "merchantapi_api" => {
                self.merchantapi_api().read_resource(resource_name, id).await
            }
            "datapipelines_api" => {
                self.datapipelines_api().read_resource(resource_name, id).await
            }
            "sts_api" => {
                self.sts_api().read_resource(resource_name, id).await
            }
            "observability_api" => {
                self.observability_api().read_resource(resource_name, id).await
            }
            "dataflow_api" => {
                self.dataflow_api().read_resource(resource_name, id).await
            }
            "cloudresourcemanager_api" => {
                self.cloudresourcemanager_api().read_resource(resource_name, id).await
            }
            "firebaseapphosting_api" => {
                self.firebaseapphosting_api().read_resource(resource_name, id).await
            }
            "verifiedaccess_api" => {
                self.verifiedaccess_api().read_resource(resource_name, id).await
            }
            "accessapproval_api" => {
                self.accessapproval_api().read_resource(resource_name, id).await
            }
            "firebasedataconnect_api" => {
                self.firebasedataconnect_api().read_resource(resource_name, id).await
            }
            "assuredworkloads_api" => {
                self.assuredworkloads_api().read_resource(resource_name, id).await
            }
            "datafusion_api" => {
                self.datafusion_api().read_resource(resource_name, id).await
            }
            "doubleclicksearch_api" => {
                self.doubleclicksearch_api().read_resource(resource_name, id).await
            }
            "ideahub_api" => {
                self.ideahub_api().read_resource(resource_name, id).await
            }
            "alloydb_api" => {
                self.alloydb_api().read_resource(resource_name, id).await
            }
            "doubleclickbidmanager_api" => {
                self.doubleclickbidmanager_api().read_resource(resource_name, id).await
            }
            "bigqueryreservation_api" => {
                self.bigqueryreservation_api().read_resource(resource_name, id).await
            }
            "notebooks_api" => {
                self.notebooks_api().read_resource(resource_name, id).await
            }
            "readerrevenuesubscriptionlinking_api" => {
                self.readerrevenuesubscriptionlinking_api().read_resource(resource_name, id).await
            }
            "authorizedbuyersmarketplace_api" => {
                self.authorizedbuyersmarketplace_api().read_resource(resource_name, id).await
            }
            "adexchangebuyer2_api" => {
                self.adexchangebuyer2_api().read_resource(resource_name, id).await
            }
            "adsenseplatform_api" => {
                self.adsenseplatform_api().read_resource(resource_name, id).await
            }
            "mybusinessplaceactions_api" => {
                self.mybusinessplaceactions_api().read_resource(resource_name, id).await
            }
            "firebasestorage_api" => {
                self.firebasestorage_api().read_resource(resource_name, id).await
            }
            "calendar_api" => {
                self.calendar_api().read_resource(resource_name, id).await
            }
            "networkmanagement_api" => {
                self.networkmanagement_api().read_resource(resource_name, id).await
            }
            "admin_api" => {
                self.admin_api().read_resource(resource_name, id).await
            }
            "acceleratedmobilepageurl_api" => {
                self.acceleratedmobilepageurl_api().read_resource(resource_name, id).await
            }
            "fusiontables_api" => {
                self.fusiontables_api().read_resource(resource_name, id).await
            }
            "chromepolicy_api" => {
                self.chromepolicy_api().read_resource(resource_name, id).await
            }
            "securitycenter_api" => {
                self.securitycenter_api().read_resource(resource_name, id).await
            }
            "analyticsreporting_api" => {
                self.analyticsreporting_api().read_resource(resource_name, id).await
            }
            "iap_api" => {
                self.iap_api().read_resource(resource_name, id).await
            }
            "consumersurveys_api" => {
                self.consumersurveys_api().read_resource(resource_name, id).await
            }
            "siteverification_api" => {
                self.siteverification_api().read_resource(resource_name, id).await
            }
            "versionhistory_api" => {
                self.versionhistory_api().read_resource(resource_name, id).await
            }
            "storagebatchoperations_api" => {
                self.storagebatchoperations_api().read_resource(resource_name, id).await
            }
            "acmedns_api" => {
                self.acmedns_api().read_resource(resource_name, id).await
            }
            "storage_api" => {
                self.storage_api().read_resource(resource_name, id).await
            }
            "networkservices_api" => {
                self.networkservices_api().read_resource(resource_name, id).await
            }
            "dfareporting_api" => {
                self.dfareporting_api().read_resource(resource_name, id).await
            }
            "webfonts_api" => {
                self.webfonts_api().read_resource(resource_name, id).await
            }
            "apim_api" => {
                self.apim_api().read_resource(resource_name, id).await
            }
            "chromeuxreport_api" => {
                self.chromeuxreport_api().read_resource(resource_name, id).await
            }
            "aiplatform_api" => {
                self.aiplatform_api().read_resource(resource_name, id).await
            }
            "script_api" => {
                self.script_api().read_resource(resource_name, id).await
            }
            "contentwarehouse_api" => {
                self.contentwarehouse_api().read_resource(resource_name, id).await
            }
            "batch_api" => {
                self.batch_api().read_resource(resource_name, id).await
            }
            "kmsinventory_api" => {
                self.kmsinventory_api().read_resource(resource_name, id).await
            }
            "run_api" => {
                self.run_api().read_resource(resource_name, id).await
            }
            "servicenetworking_api" => {
                self.servicenetworking_api().read_resource(resource_name, id).await
            }
            "cloudscheduler_api" => {
                self.cloudscheduler_api().read_resource(resource_name, id).await
            }
            "cloudsearch_api" => {
                self.cloudsearch_api().read_resource(resource_name, id).await
            }
            "admob_api" => {
                self.admob_api().read_resource(resource_name, id).await
            }
            "workspaceevents_api" => {
                self.workspaceevents_api().read_resource(resource_name, id).await
            }
            "metastore_api" => {
                self.metastore_api().read_resource(resource_name, id).await
            }
            "policytroubleshooter_api" => {
                self.policytroubleshooter_api().read_resource(resource_name, id).await
            }
            "analyticsdata_api" => {
                self.analyticsdata_api().read_resource(resource_name, id).await
            }
            "contactcenteraiplatform_api" => {
                self.contactcenteraiplatform_api().read_resource(resource_name, id).await
            }
            "forms_api" => {
                self.forms_api().read_resource(resource_name, id).await
            }
            "clouderrorreporting_api" => {
                self.clouderrorreporting_api().read_resource(resource_name, id).await
            }
            "pagespeedonline_api" => {
                self.pagespeedonline_api().read_resource(resource_name, id).await
            }
            "parallelstore_api" => {
                self.parallelstore_api().read_resource(resource_name, id).await
            }
            "cloudbilling_api" => {
                self.cloudbilling_api().read_resource(resource_name, id).await
            }
            "cloudidentity_api" => {
                self.cloudidentity_api().read_resource(resource_name, id).await
            }
            "appsactivity_api" => {
                self.appsactivity_api().read_resource(resource_name, id).await
            }
            "groupssettings_api" => {
                self.groupssettings_api().read_resource(resource_name, id).await
            }
            "adexchangebuyer_api" => {
                self.adexchangebuyer_api().read_resource(resource_name, id).await
            }
            "youtube_api" => {
                self.youtube_api().read_resource(resource_name, id).await
            }
            "looker_api" => {
                self.looker_api().read_resource(resource_name, id).await
            }
            "gkebackup_api" => {
                self.gkebackup_api().read_resource(resource_name, id).await
            }
            "cloudasset_api" => {
                self.cloudasset_api().read_resource(resource_name, id).await
            }
            "people_api" => {
                self.people_api().read_resource(resource_name, id).await
            }
            "mybusinessqanda_api" => {
                self.mybusinessqanda_api().read_resource(resource_name, id).await
            }
            "beyondcorp_api" => {
                self.beyondcorp_api().read_resource(resource_name, id).await
            }
            "pubsub_api" => {
                self.pubsub_api().read_resource(resource_name, id).await
            }
            "servicebroker_api" => {
                self.servicebroker_api().read_resource(resource_name, id).await
            }
            "contactcenterinsights_api" => {
                self.contactcenterinsights_api().read_resource(resource_name, id).await
            }
            "accesscontextmanager_api" => {
                self.accesscontextmanager_api().read_resource(resource_name, id).await
            }
            "searchads360_api" => {
                self.searchads360_api().read_resource(resource_name, id).await
            }
            "oslogin_api" => {
                self.oslogin_api().read_resource(resource_name, id).await
            }
            "playcustomapp_api" => {
                self.playcustomapp_api().read_resource(resource_name, id).await
            }
            "webmasters_api" => {
                self.webmasters_api().read_resource(resource_name, id).await
            }
            "fitness_api" => {
                self.fitness_api().read_resource(resource_name, id).await
            }
            "parametermanager_api" => {
                self.parametermanager_api().read_resource(resource_name, id).await
            }
            "businessprofileperformance_api" => {
                self.businessprofileperformance_api().read_resource(resource_name, id).await
            }
            "videointelligence_api" => {
                self.videointelligence_api().read_resource(resource_name, id).await
            }
            "replicapool_api" => {
                self.replicapool_api().read_resource(resource_name, id).await
            }
            "dlp_api" => {
                self.dlp_api().read_resource(resource_name, id).await
            }
            "migrationcenter_api" => {
                self.migrationcenter_api().read_resource(resource_name, id).await
            }
            "kgsearch_api" => {
                self.kgsearch_api().read_resource(resource_name, id).await
            }
            "cloudprofiler_api" => {
                self.cloudprofiler_api().read_resource(resource_name, id).await
            }
            "healthcare_api" => {
                self.healthcare_api().read_resource(resource_name, id).await
            }
            "firebaseappcheck_api" => {
                self.firebaseappcheck_api().read_resource(resource_name, id).await
            }
            "gkeonprem_api" => {
                self.gkeonprem_api().read_resource(resource_name, id).await
            }
            "repeated_any_query_error" => {
                self.repeated_any_query_error().read_resource(resource_name, id).await
            }
            "transcoder_api" => {
                self.transcoder_api().read_resource(resource_name, id).await
            }
            "firebaserules_api" => {
                self.firebaserules_api().read_resource(resource_name, id).await
            }
            "plusdomains_api" => {
                self.plusdomains_api().read_resource(resource_name, id).await
            }
            "proximitybeacon_api" => {
                self.proximitybeacon_api().read_resource(resource_name, id).await
            }
            "firebase_api" => {
                self.firebase_api().read_resource(resource_name, id).await
            }
            "resource_named_service" => {
                self.resource_named_service().read_resource(resource_name, id).await
            }
            "config_api" => {
                self.config_api().read_resource(resource_name, id).await
            }
            "games_api" => {
                self.games_api().read_resource(resource_name, id).await
            }
            "appstate_api" => {
                self.appstate_api().read_resource(resource_name, id).await
            }
            "saasservicemgmt_api" => {
                self.saasservicemgmt_api().read_resource(resource_name, id).await
            }
            "test_api" => {
                self.test_api().read_resource(resource_name, id).await
            }
            "gkehub_api" => {
                self.gkehub_api().read_resource(resource_name, id).await
            }
            "analytics_api" => {
                self.analytics_api().read_resource(resource_name, id).await
            }
            "discoveryengine_api" => {
                self.discoveryengine_api().read_resource(resource_name, id).await
            }
            "playintegrity_api" => {
                self.playintegrity_api().read_resource(resource_name, id).await
            }
            "homegraph_api" => {
                self.homegraph_api().read_resource(resource_name, id).await
            }
            "apikeys_api" => {
                self.apikeys_api().read_resource(resource_name, id).await
            }
            "serviceconsumermanagement_api" => {
                self.serviceconsumermanagement_api().read_resource(resource_name, id).await
            }
            "datalineage_api" => {
                self.datalineage_api().read_resource(resource_name, id).await
            }
            "manufacturers_api" => {
                self.manufacturers_api().read_resource(resource_name, id).await
            }
            "vectortile_api" => {
                self.vectortile_api().read_resource(resource_name, id).await
            }
            "chat_api" => {
                self.chat_api().read_resource(resource_name, id).await
            }
            "bigquery_api" => {
                self.bigquery_api().read_resource(resource_name, id).await
            }
            "plus_api" => {
                self.plus_api().read_resource(resource_name, id).await
            }
            "bigqueryconnection_api" => {
                self.bigqueryconnection_api().read_resource(resource_name, id).await
            }
            "places_api" => {
                self.places_api().read_resource(resource_name, id).await
            }
            "logging_api" => {
                self.logging_api().read_resource(resource_name, id).await
            }
            "webrisk_api" => {
                self.webrisk_api().read_resource(resource_name, id).await
            }
            "vpcaccess_api" => {
                self.vpcaccess_api().read_resource(resource_name, id).await
            }
            "urlshortener_api" => {
                self.urlshortener_api().read_resource(resource_name, id).await
            }
            "sqladmin_api" => {
                self.sqladmin_api().read_resource(resource_name, id).await
            }
            "keep_api" => {
                self.keep_api().read_resource(resource_name, id).await
            }
            "walletobjects_api" => {
                self.walletobjects_api().read_resource(resource_name, id).await
            }
            "apphub_api" => {
                self.apphub_api().read_resource(resource_name, id).await
            }
            "vmmigration_api" => {
                self.vmmigration_api().read_resource(resource_name, id).await
            }
            "airquality_api" => {
                self.airquality_api().read_resource(resource_name, id).await
            }
            "chromemanagement_api" => {
                self.chromemanagement_api().read_resource(resource_name, id).await
            }
            "adexperiencereport_api" => {
                self.adexperiencereport_api().read_resource(resource_name, id).await
            }
            "recommender_api" => {
                self.recommender_api().read_resource(resource_name, id).await
            }
            "driveactivity_api" => {
                self.driveactivity_api().read_resource(resource_name, id).await
            }
            "dns_api" => {
                self.dns_api().read_resource(resource_name, id).await
            }
            "cloudshell_api" => {
                self.cloudshell_api().read_resource(resource_name, id).await
            }
            "content_api" => {
                self.content_api().read_resource(resource_name, id).await
            }
            "commentanalyzer_api" => {
                self.commentanalyzer_api().read_resource(resource_name, id).await
            }
            "backupdr_api" => {
                self.backupdr_api().read_resource(resource_name, id).await
            }
            "groupsmigration_api" => {
                self.groupsmigration_api().read_resource(resource_name, id).await
            }
            "connectors_api" => {
                self.connectors_api().read_resource(resource_name, id).await
            }
            "solar_api" => {
                self.solar_api().read_resource(resource_name, id).await
            }
            "fcm_api" => {
                self.fcm_api().read_resource(resource_name, id).await
            }
            "networksecurity_api" => {
                self.networksecurity_api().read_resource(resource_name, id).await
            }
            "managedidentities_api" => {
                self.managedidentities_api().read_resource(resource_name, id).await
            }
            "serviceusage_api" => {
                self.serviceusage_api().read_resource(resource_name, id).await
            }
            "digitalassetlinks_api" => {
                self.digitalassetlinks_api().read_resource(resource_name, id).await
            }
            "clouddebugger_api" => {
                self.clouddebugger_api().read_resource(resource_name, id).await
            }
            "datamanager_api" => {
                self.datamanager_api().read_resource(resource_name, id).await
            }
            "ids_api" => {
                self.ids_api().read_resource(resource_name, id).await
            }
            "workflows_api" => {
                self.workflows_api().read_resource(resource_name, id).await
            }
            "cloudtrace_api" => {
                self.cloudtrace_api().read_resource(resource_name, id).await
            }
            "advisorynotifications_api" => {
                self.advisorynotifications_api().read_resource(resource_name, id).await
            }
            "fcmdata_api" => {
                self.fcmdata_api().read_resource(resource_name, id).await
            }
            "checks_api" => {
                self.checks_api().read_resource(resource_name, id).await
            }
            "blogger_3" => {
                self.blogger_3().read_resource(resource_name, id).await
            }
            "mediaupload" => {
                self.mediaupload().read_resource(resource_name, id).await
            }
            "paymentsresellersubscription_api" => {
                self.paymentsresellersubscription_api().read_resource(resource_name, id).await
            }
            "spanner_api" => {
                self.spanner_api().read_resource(resource_name, id).await
            }
            "cloudchannel_api" => {
                self.cloudchannel_api().read_resource(resource_name, id).await
            }
            "gameservices_api" => {
                self.gameservices_api().read_resource(resource_name, id).await
            }
            "cloudprivatecatalog_api" => {
                self.cloudprivatecatalog_api().read_resource(resource_name, id).await
            }
            "gmailpostmastertools_api" => {
                self.gmailpostmastertools_api().read_resource(resource_name, id).await
            }
            "partners_api" => {
                self.partners_api().read_resource(resource_name, id).await
            }
            "any" => {
                self.any().read_resource(resource_name, id).await
            }
            "runtimeconfig_api" => {
                self.runtimeconfig_api().read_resource(resource_name, id).await
            }
            "retail_api" => {
                self.retail_api().read_resource(resource_name, id).await
            }
            "books_api" => {
                self.books_api().read_resource(resource_name, id).await
            }
            "websecurityscanner_api" => {
                self.websecurityscanner_api().read_resource(resource_name, id).await
            }
            "localservices_api" => {
                self.localservices_api().read_resource(resource_name, id).await
            }
            "servicedirectory_api" => {
                self.servicedirectory_api().read_resource(resource_name, id).await
            }
            "customsearch_api" => {
                self.customsearch_api().read_resource(resource_name, id).await
            }
            "policyanalyzer_api" => {
                self.policyanalyzer_api().read_resource(resource_name, id).await
            }
            "mybusinessbusinesscalls_api" => {
                self.mybusinessbusinesscalls_api().read_resource(resource_name, id).await
            }
            "ml_api" => {
                self.ml_api().read_resource(resource_name, id).await
            }
            "firebaseml_api" => {
                self.firebaseml_api().read_resource(resource_name, id).await
            }
            "blogger_api" => {
                self.blogger_api().read_resource(resource_name, id).await
            }
            "vault_api" => {
                self.vault_api().read_resource(resource_name, id).await
            }
            "baremetalsolution_api" => {
                self.baremetalsolution_api().read_resource(resource_name, id).await
            }
            "testing_api" => {
                self.testing_api().read_resource(resource_name, id).await
            }
            "recaptchaenterprise_api" => {
                self.recaptchaenterprise_api().read_resource(resource_name, id).await
            }
            "servicecontrol_api" => {
                self.servicecontrol_api().read_resource(resource_name, id).await
            }
            "documentai_api" => {
                self.documentai_api().read_resource(resource_name, id).await
            }
            "firebasedynamiclinks_api" => {
                self.firebasedynamiclinks_api().read_resource(resource_name, id).await
            }
            "apigee_api" => {
                self.apigee_api().read_resource(resource_name, id).await
            }
            "netapp_api" => {
                self.netapp_api().read_resource(resource_name, id).await
            }
            "identitytoolkit_api" => {
                self.identitytoolkit_api().read_resource(resource_name, id).await
            }
            "datastream_api" => {
                self.datastream_api().read_resource(resource_name, id).await
            }
            "iamcredentials_api" => {
                self.iamcredentials_api().read_resource(resource_name, id).await
            }
            "datalabeling_api" => {
                self.datalabeling_api().read_resource(resource_name, id).await
            }
            "youtubereporting_api" => {
                self.youtubereporting_api().read_resource(resource_name, id).await
            }
            "genomics_api" => {
                self.genomics_api().read_resource(resource_name, id).await
            }
            "lifesciences_api" => {
                self.lifesciences_api().read_resource(resource_name, id).await
            }
            "compute_api" => {
                self.compute_api().read_resource(resource_name, id).await
            }
            "youtubeanalytics_api" => {
                self.youtubeanalytics_api().read_resource(resource_name, id).await
            }
            "factchecktools_api" => {
                self.factchecktools_api().read_resource(resource_name, id).await
            }
            "dialogflow_api" => {
                self.dialogflow_api().read_resource(resource_name, id).await
            }
            "docs_api" => {
                self.docs_api().read_resource(resource_name, id).await
            }
            "tagmanager_api" => {
                self.tagmanager_api().read_resource(resource_name, id).await
            }
            "texttospeech_api" => {
                self.texttospeech_api().read_resource(resource_name, id).await
            }
            "gmail_api" => {
                self.gmail_api().read_resource(resource_name, id).await
            }
            "tasks_api" => {
                self.tasks_api().read_resource(resource_name, id).await
            }
            "securityposture_api" => {
                self.securityposture_api().read_resource(resource_name, id).await
            }
            "sql_api" => {
                self.sql_api().read_resource(resource_name, id).await
            }
            "integrations_api" => {
                self.integrations_api().read_resource(resource_name, id).await
            }
            "essentialcontacts_api" => {
                self.essentialcontacts_api().read_resource(resource_name, id).await
            }
            "tpu_api" => {
                self.tpu_api().read_resource(resource_name, id).await
            }
            "classroom_api" => {
                self.classroom_api().read_resource(resource_name, id).await
            }
            "mybusinesslodging_api" => {
                self.mybusinesslodging_api().read_resource(resource_name, id).await
            }
            "rapidmigrationassessment_api" => {
                self.rapidmigrationassessment_api().read_resource(resource_name, id).await
            }
            "blockchainnodeengine_api" => {
                self.blockchainnodeengine_api().read_resource(resource_name, id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Update an existing resource
    async fn update(
        &self,
        resource_type: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "translate_api" => {
                self.translate_api().update_resource(resource_name, id, input).await
            }
            "area120tables_api" => {
                self.area120tables_api().update_resource(resource_name, id, input).await
            }
            "abusiveexperiencereport_api" => {
                self.abusiveexperiencereport_api().update_resource(resource_name, id, input).await
            }
            "securesourcemanager_api" => {
                self.securesourcemanager_api().update_resource(resource_name, id, input).await
            }
            "file_api" => {
                self.file_api().update_resource(resource_name, id, input).await
            }
            "oauth2_api" => {
                self.oauth2_api().update_resource(resource_name, id, input).await
            }
            "serviceuser_api" => {
                self.serviceuser_api().update_resource(resource_name, id, input).await
            }
            "bigquerydatatransfer_api" => {
                self.bigquerydatatransfer_api().update_resource(resource_name, id, input).await
            }
            "cloudfunctions_api" => {
                self.cloudfunctions_api().update_resource(resource_name, id, input).await
            }
            "networkconnectivity_api" => {
                self.networkconnectivity_api().update_resource(resource_name, id, input).await
            }
            "mybusinessverifications_api" => {
                self.mybusinessverifications_api().update_resource(resource_name, id, input).await
            }
            "qpxexpress_api" => {
                self.qpxexpress_api().update_resource(resource_name, id, input).await
            }
            "dataproc_api" => {
                self.dataproc_api().update_resource(resource_name, id, input).await
            }
            "playgrouping_api" => {
                self.playgrouping_api().update_resource(resource_name, id, input).await
            }
            "mirror_api" => {
                self.mirror_api().update_resource(resource_name, id, input).await
            }
            "adexchangeseller_api" => {
                self.adexchangeseller_api().update_resource(resource_name, id, input).await
            }
            "binaryauthorization_api" => {
                self.binaryauthorization_api().update_resource(resource_name, id, input).await
            }
            "marketingplatformadmin_api" => {
                self.marketingplatformadmin_api().update_resource(resource_name, id, input).await
            }
            "cloudprivatecatalogproducer_api" => {
                self.cloudprivatecatalogproducer_api().update_resource(resource_name, id, input).await
            }
            "pubsublite_api" => {
                self.pubsublite_api().update_resource(resource_name, id, input).await
            }
            "slides_api" => {
                self.slides_api().update_resource(resource_name, id, input).await
            }
            "billingbudgets_api" => {
                self.billingbudgets_api().update_resource(resource_name, id, input).await
            }
            "workstations_api" => {
                self.workstations_api().update_resource(resource_name, id, input).await
            }
            "civicinfo_api" => {
                self.civicinfo_api().update_resource(resource_name, id, input).await
            }
            "spectrum_api" => {
                self.spectrum_api().update_resource(resource_name, id, input).await
            }
            "surveys_api" => {
                self.surveys_api().update_resource(resource_name, id, input).await
            }
            "dataplex_api" => {
                self.dataplex_api().update_resource(resource_name, id, input).await
            }
            "redis_api" => {
                self.redis_api().update_resource(resource_name, id, input).await
            }
            "addressvalidation_api" => {
                self.addressvalidation_api().update_resource(resource_name, id, input).await
            }
            "sourcerepo_api" => {
                self.sourcerepo_api().update_resource(resource_name, id, input).await
            }
            "osconfig_api" => {
                self.osconfig_api().update_resource(resource_name, id, input).await
            }
            "domainsrdap_api" => {
                self.domainsrdap_api().update_resource(resource_name, id, input).await
            }
            "speech_api" => {
                self.speech_api().update_resource(resource_name, id, input).await
            }
            "trafficdirector_api" => {
                self.trafficdirector_api().update_resource(resource_name, id, input).await
            }
            "indexing_api" => {
                self.indexing_api().update_resource(resource_name, id, input).await
            }
            "ondemandscanning_api" => {
                self.ondemandscanning_api().update_resource(resource_name, id, input).await
            }
            "drive_api" => {
                self.drive_api().update_resource(resource_name, id, input).await
            }
            "mybusinessnotifications_api" => {
                self.mybusinessnotifications_api().update_resource(resource_name, id, input).await
            }
            "workflowexecutions_api" => {
                self.workflowexecutions_api().update_resource(resource_name, id, input).await
            }
            "developerconnect_api" => {
                self.developerconnect_api().update_resource(resource_name, id, input).await
            }
            "analyticsadmin_api" => {
                self.analyticsadmin_api().update_resource(resource_name, id, input).await
            }
            "playdeveloperreporting_api" => {
                self.playdeveloperreporting_api().update_resource(resource_name, id, input).await
            }
            "prod_tt_sasportal_api" => {
                self.prod_tt_sasportal_api().update_resource(resource_name, id, input).await
            }
            "searchconsole_api" => {
                self.searchconsole_api().update_resource(resource_name, id, input).await
            }
            "firebaseremoteconfig_api" => {
                self.firebaseremoteconfig_api().update_resource(resource_name, id, input).await
            }
            "publicca_api" => {
                self.publicca_api().update_resource(resource_name, id, input).await
            }
            "discovery_api" => {
                self.discovery_api().update_resource(resource_name, id, input).await
            }
            "firebasehosting_api" => {
                self.firebasehosting_api().update_resource(resource_name, id, input).await
            }
            "memcache_api" => {
                self.memcache_api().update_resource(resource_name, id, input).await
            }
            "playablelocations_api" => {
                self.playablelocations_api().update_resource(resource_name, id, input).await
            }
            "jobs_api" => {
                self.jobs_api().update_resource(resource_name, id, input).await
            }
            "iam_api" => {
                self.iam_api().update_resource(resource_name, id, input).await
            }
            "reseller_api" => {
                self.reseller_api().update_resource(resource_name, id, input).await
            }
            "realtimebidding_api" => {
                self.realtimebidding_api().update_resource(resource_name, id, input).await
            }
            "datamigration_api" => {
                self.datamigration_api().update_resource(resource_name, id, input).await
            }
            "safebrowsing_api" => {
                self.safebrowsing_api().update_resource(resource_name, id, input).await
            }
            "libraryagent_api" => {
                self.libraryagent_api().update_resource(resource_name, id, input).await
            }
            "css_api" => {
                self.css_api().update_resource(resource_name, id, input).await
            }
            "androidpublisher_api" => {
                self.androidpublisher_api().update_resource(resource_name, id, input).await
            }
            "datacatalog_api" => {
                self.datacatalog_api().update_resource(resource_name, id, input).await
            }
            "cloudlocationfinder_api" => {
                self.cloudlocationfinder_api().update_resource(resource_name, id, input).await
            }
            "storagetransfer_api" => {
                self.storagetransfer_api().update_resource(resource_name, id, input).await
            }
            "resourcesettings_api" => {
                self.resourcesettings_api().update_resource(resource_name, id, input).await
            }
            "firebasedatabase_api" => {
                self.firebasedatabase_api().update_resource(resource_name, id, input).await
            }
            "analyticshub_api" => {
                self.analyticshub_api().update_resource(resource_name, id, input).await
            }
            "http_body" => {
                self.http_body().update_resource(resource_name, id, input).await
            }
            "managedkafka_api" => {
                self.managedkafka_api().update_resource(resource_name, id, input).await
            }
            "poly_api" => {
                self.poly_api().update_resource(resource_name, id, input).await
            }
            "orgpolicy_api" => {
                self.orgpolicy_api().update_resource(resource_name, id, input).await
            }
            "certificatemanager_api" => {
                self.certificatemanager_api().update_resource(resource_name, id, input).await
            }
            "apigateway_api" => {
                self.apigateway_api().update_resource(resource_name, id, input).await
            }
            "datastore_api" => {
                self.datastore_api().update_resource(resource_name, id, input).await
            }
            "cloudtasks_api" => {
                self.cloudtasks_api().update_resource(resource_name, id, input).await
            }
            "firebaseappdistribution_api" => {
                self.firebaseappdistribution_api().update_resource(resource_name, id, input).await
            }
            "domains_api" => {
                self.domains_api().update_resource(resource_name, id, input).await
            }
            "policysimulator_api" => {
                self.policysimulator_api().update_resource(resource_name, id, input).await
            }
            "apihub_api" => {
                self.apihub_api().update_resource(resource_name, id, input).await
            }
            "mybusinessaccountmanagement_api" => {
                self.mybusinessaccountmanagement_api().update_resource(resource_name, id, input).await
            }
            "smartdevicemanagement_api" => {
                self.smartdevicemanagement_api().update_resource(resource_name, id, input).await
            }
            "tracing_api" => {
                self.tracing_api().update_resource(resource_name, id, input).await
            }
            "gamesmanagement_api" => {
                self.gamesmanagement_api().update_resource(resource_name, id, input).await
            }
            "firestore_api" => {
                self.firestore_api().update_resource(resource_name, id, input).await
            }
            "language_api" => {
                self.language_api().update_resource(resource_name, id, input).await
            }
            "json_body" => {
                self.json_body().update_resource(resource_name, id, input).await
            }
            "replicapoolupdater_api" => {
                self.replicapoolupdater_api().update_resource(resource_name, id, input).await
            }
            "androidmanagement_api" => {
                self.androidmanagement_api().update_resource(resource_name, id, input).await
            }
            "licensing_api" => {
                self.licensing_api().update_resource(resource_name, id, input).await
            }
            "sasportal_api" => {
                self.sasportal_api().update_resource(resource_name, id, input).await
            }
            "vmwareengine_api" => {
                self.vmwareengine_api().update_resource(resource_name, id, input).await
            }
            "pollen_api" => {
                self.pollen_api().update_resource(resource_name, id, input).await
            }
            "cloudsupport_api" => {
                self.cloudsupport_api().update_resource(resource_name, id, input).await
            }
            "servicemanagement_api" => {
                self.servicemanagement_api().update_resource(resource_name, id, input).await
            }
            "cloudiot_api" => {
                self.cloudiot_api().update_resource(resource_name, id, input).await
            }
            "androiddeviceprovisioning_api" => {
                self.androiddeviceprovisioning_api().update_resource(resource_name, id, input).await
            }
            "gamesconfiguration_api" => {
                self.gamesconfiguration_api().update_resource(resource_name, id, input).await
            }
            "areainsights_api" => {
                self.areainsights_api().update_resource(resource_name, id, input).await
            }
            "meet_api" => {
                self.meet_api().update_resource(resource_name, id, input).await
            }
            "adsense_api" => {
                self.adsense_api().update_resource(resource_name, id, input).await
            }
            "toolresults_api" => {
                self.toolresults_api().update_resource(resource_name, id, input).await
            }
            "artifactregistry_api" => {
                self.artifactregistry_api().update_resource(resource_name, id, input).await
            }
            "cloudcommerceprocurement_api" => {
                self.cloudcommerceprocurement_api().update_resource(resource_name, id, input).await
            }
            "alertcenter_api" => {
                self.alertcenter_api().update_resource(resource_name, id, input).await
            }
            "mybusinessbusinessinformation_api" => {
                self.mybusinessbusinessinformation_api().update_resource(resource_name, id, input).await
            }
            "bigquerydatapolicy_api" => {
                self.bigquerydatapolicy_api().update_resource(resource_name, id, input).await
            }
            "androidenterprise_api" => {
                self.androidenterprise_api().update_resource(resource_name, id, input).await
            }
            "playmoviespartner_api" => {
                self.playmoviespartner_api().update_resource(resource_name, id, input).await
            }
            "oracledatabase_api" => {
                self.oracledatabase_api().update_resource(resource_name, id, input).await
            }
            "composer_api" => {
                self.composer_api().update_resource(resource_name, id, input).await
            }
            "dataform_api" => {
                self.dataform_api().update_resource(resource_name, id, input).await
            }
            "adsensehost_api" => {
                self.adsensehost_api().update_resource(resource_name, id, input).await
            }
            "deploymentmanager_api" => {
                self.deploymentmanager_api().update_resource(resource_name, id, input).await
            }
            "displayvideo_api" => {
                self.displayvideo_api().update_resource(resource_name, id, input).await
            }
            "chromewebstore_api" => {
                self.chromewebstore_api().update_resource(resource_name, id, input).await
            }
            "cloudkms_api" => {
                self.cloudkms_api().update_resource(resource_name, id, input).await
            }
            "bigtableadmin_api" => {
                self.bigtableadmin_api().update_resource(resource_name, id, input).await
            }
            "drivelabels_api" => {
                self.drivelabels_api().update_resource(resource_name, id, input).await
            }
            "vision_api" => {
                self.vision_api().update_resource(resource_name, id, input).await
            }
            "streetviewpublish_api" => {
                self.streetviewpublish_api().update_resource(resource_name, id, input).await
            }
            "clouddeploy_api" => {
                self.clouddeploy_api().update_resource(resource_name, id, input).await
            }
            "biglake_api" => {
                self.biglake_api().update_resource(resource_name, id, input).await
            }
            "eventarc_api" => {
                self.eventarc_api().update_resource(resource_name, id, input).await
            }
            "apigeeregistry_api" => {
                self.apigeeregistry_api().update_resource(resource_name, id, input).await
            }
            "cloudbuild_api" => {
                self.cloudbuild_api().update_resource(resource_name, id, input).await
            }
            "secretmanager_api" => {
                self.secretmanager_api().update_resource(resource_name, id, input).await
            }
            "sheets_api" => {
                self.sheets_api().update_resource(resource_name, id, input).await
            }
            "recommendationengine_api" => {
                self.recommendationengine_api().update_resource(resource_name, id, input).await
            }
            "appengine_api" => {
                self.appengine_api().update_resource(resource_name, id, input).await
            }
            "dataportability_api" => {
                self.dataportability_api().update_resource(resource_name, id, input).await
            }
            "remotebuildexecution_api" => {
                self.remotebuildexecution_api().update_resource(resource_name, id, input).await
            }
            "containeranalysis_api" => {
                self.containeranalysis_api().update_resource(resource_name, id, input).await
            }
            "workloadmanager_api" => {
                self.workloadmanager_api().update_resource(resource_name, id, input).await
            }
            "cloudcontrolspartner_api" => {
                self.cloudcontrolspartner_api().update_resource(resource_name, id, input).await
            }
            "monitoring_api" => {
                self.monitoring_api().update_resource(resource_name, id, input).await
            }
            "privateca_api" => {
                self.privateca_api().update_resource(resource_name, id, input).await
            }
            "container_api" => {
                self.container_api().update_resource(resource_name, id, input).await
            }
            "merchantapi_api" => {
                self.merchantapi_api().update_resource(resource_name, id, input).await
            }
            "datapipelines_api" => {
                self.datapipelines_api().update_resource(resource_name, id, input).await
            }
            "sts_api" => {
                self.sts_api().update_resource(resource_name, id, input).await
            }
            "observability_api" => {
                self.observability_api().update_resource(resource_name, id, input).await
            }
            "dataflow_api" => {
                self.dataflow_api().update_resource(resource_name, id, input).await
            }
            "cloudresourcemanager_api" => {
                self.cloudresourcemanager_api().update_resource(resource_name, id, input).await
            }
            "firebaseapphosting_api" => {
                self.firebaseapphosting_api().update_resource(resource_name, id, input).await
            }
            "verifiedaccess_api" => {
                self.verifiedaccess_api().update_resource(resource_name, id, input).await
            }
            "accessapproval_api" => {
                self.accessapproval_api().update_resource(resource_name, id, input).await
            }
            "firebasedataconnect_api" => {
                self.firebasedataconnect_api().update_resource(resource_name, id, input).await
            }
            "assuredworkloads_api" => {
                self.assuredworkloads_api().update_resource(resource_name, id, input).await
            }
            "datafusion_api" => {
                self.datafusion_api().update_resource(resource_name, id, input).await
            }
            "doubleclicksearch_api" => {
                self.doubleclicksearch_api().update_resource(resource_name, id, input).await
            }
            "ideahub_api" => {
                self.ideahub_api().update_resource(resource_name, id, input).await
            }
            "alloydb_api" => {
                self.alloydb_api().update_resource(resource_name, id, input).await
            }
            "doubleclickbidmanager_api" => {
                self.doubleclickbidmanager_api().update_resource(resource_name, id, input).await
            }
            "bigqueryreservation_api" => {
                self.bigqueryreservation_api().update_resource(resource_name, id, input).await
            }
            "notebooks_api" => {
                self.notebooks_api().update_resource(resource_name, id, input).await
            }
            "readerrevenuesubscriptionlinking_api" => {
                self.readerrevenuesubscriptionlinking_api().update_resource(resource_name, id, input).await
            }
            "authorizedbuyersmarketplace_api" => {
                self.authorizedbuyersmarketplace_api().update_resource(resource_name, id, input).await
            }
            "adexchangebuyer2_api" => {
                self.adexchangebuyer2_api().update_resource(resource_name, id, input).await
            }
            "adsenseplatform_api" => {
                self.adsenseplatform_api().update_resource(resource_name, id, input).await
            }
            "mybusinessplaceactions_api" => {
                self.mybusinessplaceactions_api().update_resource(resource_name, id, input).await
            }
            "firebasestorage_api" => {
                self.firebasestorage_api().update_resource(resource_name, id, input).await
            }
            "calendar_api" => {
                self.calendar_api().update_resource(resource_name, id, input).await
            }
            "networkmanagement_api" => {
                self.networkmanagement_api().update_resource(resource_name, id, input).await
            }
            "admin_api" => {
                self.admin_api().update_resource(resource_name, id, input).await
            }
            "acceleratedmobilepageurl_api" => {
                self.acceleratedmobilepageurl_api().update_resource(resource_name, id, input).await
            }
            "fusiontables_api" => {
                self.fusiontables_api().update_resource(resource_name, id, input).await
            }
            "chromepolicy_api" => {
                self.chromepolicy_api().update_resource(resource_name, id, input).await
            }
            "securitycenter_api" => {
                self.securitycenter_api().update_resource(resource_name, id, input).await
            }
            "analyticsreporting_api" => {
                self.analyticsreporting_api().update_resource(resource_name, id, input).await
            }
            "iap_api" => {
                self.iap_api().update_resource(resource_name, id, input).await
            }
            "consumersurveys_api" => {
                self.consumersurveys_api().update_resource(resource_name, id, input).await
            }
            "siteverification_api" => {
                self.siteverification_api().update_resource(resource_name, id, input).await
            }
            "versionhistory_api" => {
                self.versionhistory_api().update_resource(resource_name, id, input).await
            }
            "storagebatchoperations_api" => {
                self.storagebatchoperations_api().update_resource(resource_name, id, input).await
            }
            "acmedns_api" => {
                self.acmedns_api().update_resource(resource_name, id, input).await
            }
            "storage_api" => {
                self.storage_api().update_resource(resource_name, id, input).await
            }
            "networkservices_api" => {
                self.networkservices_api().update_resource(resource_name, id, input).await
            }
            "dfareporting_api" => {
                self.dfareporting_api().update_resource(resource_name, id, input).await
            }
            "webfonts_api" => {
                self.webfonts_api().update_resource(resource_name, id, input).await
            }
            "apim_api" => {
                self.apim_api().update_resource(resource_name, id, input).await
            }
            "chromeuxreport_api" => {
                self.chromeuxreport_api().update_resource(resource_name, id, input).await
            }
            "aiplatform_api" => {
                self.aiplatform_api().update_resource(resource_name, id, input).await
            }
            "script_api" => {
                self.script_api().update_resource(resource_name, id, input).await
            }
            "contentwarehouse_api" => {
                self.contentwarehouse_api().update_resource(resource_name, id, input).await
            }
            "batch_api" => {
                self.batch_api().update_resource(resource_name, id, input).await
            }
            "kmsinventory_api" => {
                self.kmsinventory_api().update_resource(resource_name, id, input).await
            }
            "run_api" => {
                self.run_api().update_resource(resource_name, id, input).await
            }
            "servicenetworking_api" => {
                self.servicenetworking_api().update_resource(resource_name, id, input).await
            }
            "cloudscheduler_api" => {
                self.cloudscheduler_api().update_resource(resource_name, id, input).await
            }
            "cloudsearch_api" => {
                self.cloudsearch_api().update_resource(resource_name, id, input).await
            }
            "admob_api" => {
                self.admob_api().update_resource(resource_name, id, input).await
            }
            "workspaceevents_api" => {
                self.workspaceevents_api().update_resource(resource_name, id, input).await
            }
            "metastore_api" => {
                self.metastore_api().update_resource(resource_name, id, input).await
            }
            "policytroubleshooter_api" => {
                self.policytroubleshooter_api().update_resource(resource_name, id, input).await
            }
            "analyticsdata_api" => {
                self.analyticsdata_api().update_resource(resource_name, id, input).await
            }
            "contactcenteraiplatform_api" => {
                self.contactcenteraiplatform_api().update_resource(resource_name, id, input).await
            }
            "forms_api" => {
                self.forms_api().update_resource(resource_name, id, input).await
            }
            "clouderrorreporting_api" => {
                self.clouderrorreporting_api().update_resource(resource_name, id, input).await
            }
            "pagespeedonline_api" => {
                self.pagespeedonline_api().update_resource(resource_name, id, input).await
            }
            "parallelstore_api" => {
                self.parallelstore_api().update_resource(resource_name, id, input).await
            }
            "cloudbilling_api" => {
                self.cloudbilling_api().update_resource(resource_name, id, input).await
            }
            "cloudidentity_api" => {
                self.cloudidentity_api().update_resource(resource_name, id, input).await
            }
            "appsactivity_api" => {
                self.appsactivity_api().update_resource(resource_name, id, input).await
            }
            "groupssettings_api" => {
                self.groupssettings_api().update_resource(resource_name, id, input).await
            }
            "adexchangebuyer_api" => {
                self.adexchangebuyer_api().update_resource(resource_name, id, input).await
            }
            "youtube_api" => {
                self.youtube_api().update_resource(resource_name, id, input).await
            }
            "looker_api" => {
                self.looker_api().update_resource(resource_name, id, input).await
            }
            "gkebackup_api" => {
                self.gkebackup_api().update_resource(resource_name, id, input).await
            }
            "cloudasset_api" => {
                self.cloudasset_api().update_resource(resource_name, id, input).await
            }
            "people_api" => {
                self.people_api().update_resource(resource_name, id, input).await
            }
            "mybusinessqanda_api" => {
                self.mybusinessqanda_api().update_resource(resource_name, id, input).await
            }
            "beyondcorp_api" => {
                self.beyondcorp_api().update_resource(resource_name, id, input).await
            }
            "pubsub_api" => {
                self.pubsub_api().update_resource(resource_name, id, input).await
            }
            "servicebroker_api" => {
                self.servicebroker_api().update_resource(resource_name, id, input).await
            }
            "contactcenterinsights_api" => {
                self.contactcenterinsights_api().update_resource(resource_name, id, input).await
            }
            "accesscontextmanager_api" => {
                self.accesscontextmanager_api().update_resource(resource_name, id, input).await
            }
            "searchads360_api" => {
                self.searchads360_api().update_resource(resource_name, id, input).await
            }
            "oslogin_api" => {
                self.oslogin_api().update_resource(resource_name, id, input).await
            }
            "playcustomapp_api" => {
                self.playcustomapp_api().update_resource(resource_name, id, input).await
            }
            "webmasters_api" => {
                self.webmasters_api().update_resource(resource_name, id, input).await
            }
            "fitness_api" => {
                self.fitness_api().update_resource(resource_name, id, input).await
            }
            "parametermanager_api" => {
                self.parametermanager_api().update_resource(resource_name, id, input).await
            }
            "businessprofileperformance_api" => {
                self.businessprofileperformance_api().update_resource(resource_name, id, input).await
            }
            "videointelligence_api" => {
                self.videointelligence_api().update_resource(resource_name, id, input).await
            }
            "replicapool_api" => {
                self.replicapool_api().update_resource(resource_name, id, input).await
            }
            "dlp_api" => {
                self.dlp_api().update_resource(resource_name, id, input).await
            }
            "migrationcenter_api" => {
                self.migrationcenter_api().update_resource(resource_name, id, input).await
            }
            "kgsearch_api" => {
                self.kgsearch_api().update_resource(resource_name, id, input).await
            }
            "cloudprofiler_api" => {
                self.cloudprofiler_api().update_resource(resource_name, id, input).await
            }
            "healthcare_api" => {
                self.healthcare_api().update_resource(resource_name, id, input).await
            }
            "firebaseappcheck_api" => {
                self.firebaseappcheck_api().update_resource(resource_name, id, input).await
            }
            "gkeonprem_api" => {
                self.gkeonprem_api().update_resource(resource_name, id, input).await
            }
            "repeated_any_query_error" => {
                self.repeated_any_query_error().update_resource(resource_name, id, input).await
            }
            "transcoder_api" => {
                self.transcoder_api().update_resource(resource_name, id, input).await
            }
            "firebaserules_api" => {
                self.firebaserules_api().update_resource(resource_name, id, input).await
            }
            "plusdomains_api" => {
                self.plusdomains_api().update_resource(resource_name, id, input).await
            }
            "proximitybeacon_api" => {
                self.proximitybeacon_api().update_resource(resource_name, id, input).await
            }
            "firebase_api" => {
                self.firebase_api().update_resource(resource_name, id, input).await
            }
            "resource_named_service" => {
                self.resource_named_service().update_resource(resource_name, id, input).await
            }
            "config_api" => {
                self.config_api().update_resource(resource_name, id, input).await
            }
            "games_api" => {
                self.games_api().update_resource(resource_name, id, input).await
            }
            "appstate_api" => {
                self.appstate_api().update_resource(resource_name, id, input).await
            }
            "saasservicemgmt_api" => {
                self.saasservicemgmt_api().update_resource(resource_name, id, input).await
            }
            "test_api" => {
                self.test_api().update_resource(resource_name, id, input).await
            }
            "gkehub_api" => {
                self.gkehub_api().update_resource(resource_name, id, input).await
            }
            "analytics_api" => {
                self.analytics_api().update_resource(resource_name, id, input).await
            }
            "discoveryengine_api" => {
                self.discoveryengine_api().update_resource(resource_name, id, input).await
            }
            "playintegrity_api" => {
                self.playintegrity_api().update_resource(resource_name, id, input).await
            }
            "homegraph_api" => {
                self.homegraph_api().update_resource(resource_name, id, input).await
            }
            "apikeys_api" => {
                self.apikeys_api().update_resource(resource_name, id, input).await
            }
            "serviceconsumermanagement_api" => {
                self.serviceconsumermanagement_api().update_resource(resource_name, id, input).await
            }
            "datalineage_api" => {
                self.datalineage_api().update_resource(resource_name, id, input).await
            }
            "manufacturers_api" => {
                self.manufacturers_api().update_resource(resource_name, id, input).await
            }
            "vectortile_api" => {
                self.vectortile_api().update_resource(resource_name, id, input).await
            }
            "chat_api" => {
                self.chat_api().update_resource(resource_name, id, input).await
            }
            "bigquery_api" => {
                self.bigquery_api().update_resource(resource_name, id, input).await
            }
            "plus_api" => {
                self.plus_api().update_resource(resource_name, id, input).await
            }
            "bigqueryconnection_api" => {
                self.bigqueryconnection_api().update_resource(resource_name, id, input).await
            }
            "places_api" => {
                self.places_api().update_resource(resource_name, id, input).await
            }
            "logging_api" => {
                self.logging_api().update_resource(resource_name, id, input).await
            }
            "webrisk_api" => {
                self.webrisk_api().update_resource(resource_name, id, input).await
            }
            "vpcaccess_api" => {
                self.vpcaccess_api().update_resource(resource_name, id, input).await
            }
            "urlshortener_api" => {
                self.urlshortener_api().update_resource(resource_name, id, input).await
            }
            "sqladmin_api" => {
                self.sqladmin_api().update_resource(resource_name, id, input).await
            }
            "keep_api" => {
                self.keep_api().update_resource(resource_name, id, input).await
            }
            "walletobjects_api" => {
                self.walletobjects_api().update_resource(resource_name, id, input).await
            }
            "apphub_api" => {
                self.apphub_api().update_resource(resource_name, id, input).await
            }
            "vmmigration_api" => {
                self.vmmigration_api().update_resource(resource_name, id, input).await
            }
            "airquality_api" => {
                self.airquality_api().update_resource(resource_name, id, input).await
            }
            "chromemanagement_api" => {
                self.chromemanagement_api().update_resource(resource_name, id, input).await
            }
            "adexperiencereport_api" => {
                self.adexperiencereport_api().update_resource(resource_name, id, input).await
            }
            "recommender_api" => {
                self.recommender_api().update_resource(resource_name, id, input).await
            }
            "driveactivity_api" => {
                self.driveactivity_api().update_resource(resource_name, id, input).await
            }
            "dns_api" => {
                self.dns_api().update_resource(resource_name, id, input).await
            }
            "cloudshell_api" => {
                self.cloudshell_api().update_resource(resource_name, id, input).await
            }
            "content_api" => {
                self.content_api().update_resource(resource_name, id, input).await
            }
            "commentanalyzer_api" => {
                self.commentanalyzer_api().update_resource(resource_name, id, input).await
            }
            "backupdr_api" => {
                self.backupdr_api().update_resource(resource_name, id, input).await
            }
            "groupsmigration_api" => {
                self.groupsmigration_api().update_resource(resource_name, id, input).await
            }
            "connectors_api" => {
                self.connectors_api().update_resource(resource_name, id, input).await
            }
            "solar_api" => {
                self.solar_api().update_resource(resource_name, id, input).await
            }
            "fcm_api" => {
                self.fcm_api().update_resource(resource_name, id, input).await
            }
            "networksecurity_api" => {
                self.networksecurity_api().update_resource(resource_name, id, input).await
            }
            "managedidentities_api" => {
                self.managedidentities_api().update_resource(resource_name, id, input).await
            }
            "serviceusage_api" => {
                self.serviceusage_api().update_resource(resource_name, id, input).await
            }
            "digitalassetlinks_api" => {
                self.digitalassetlinks_api().update_resource(resource_name, id, input).await
            }
            "clouddebugger_api" => {
                self.clouddebugger_api().update_resource(resource_name, id, input).await
            }
            "datamanager_api" => {
                self.datamanager_api().update_resource(resource_name, id, input).await
            }
            "ids_api" => {
                self.ids_api().update_resource(resource_name, id, input).await
            }
            "workflows_api" => {
                self.workflows_api().update_resource(resource_name, id, input).await
            }
            "cloudtrace_api" => {
                self.cloudtrace_api().update_resource(resource_name, id, input).await
            }
            "advisorynotifications_api" => {
                self.advisorynotifications_api().update_resource(resource_name, id, input).await
            }
            "fcmdata_api" => {
                self.fcmdata_api().update_resource(resource_name, id, input).await
            }
            "checks_api" => {
                self.checks_api().update_resource(resource_name, id, input).await
            }
            "blogger_3" => {
                self.blogger_3().update_resource(resource_name, id, input).await
            }
            "mediaupload" => {
                self.mediaupload().update_resource(resource_name, id, input).await
            }
            "paymentsresellersubscription_api" => {
                self.paymentsresellersubscription_api().update_resource(resource_name, id, input).await
            }
            "spanner_api" => {
                self.spanner_api().update_resource(resource_name, id, input).await
            }
            "cloudchannel_api" => {
                self.cloudchannel_api().update_resource(resource_name, id, input).await
            }
            "gameservices_api" => {
                self.gameservices_api().update_resource(resource_name, id, input).await
            }
            "cloudprivatecatalog_api" => {
                self.cloudprivatecatalog_api().update_resource(resource_name, id, input).await
            }
            "gmailpostmastertools_api" => {
                self.gmailpostmastertools_api().update_resource(resource_name, id, input).await
            }
            "partners_api" => {
                self.partners_api().update_resource(resource_name, id, input).await
            }
            "any" => {
                self.any().update_resource(resource_name, id, input).await
            }
            "runtimeconfig_api" => {
                self.runtimeconfig_api().update_resource(resource_name, id, input).await
            }
            "retail_api" => {
                self.retail_api().update_resource(resource_name, id, input).await
            }
            "books_api" => {
                self.books_api().update_resource(resource_name, id, input).await
            }
            "websecurityscanner_api" => {
                self.websecurityscanner_api().update_resource(resource_name, id, input).await
            }
            "localservices_api" => {
                self.localservices_api().update_resource(resource_name, id, input).await
            }
            "servicedirectory_api" => {
                self.servicedirectory_api().update_resource(resource_name, id, input).await
            }
            "customsearch_api" => {
                self.customsearch_api().update_resource(resource_name, id, input).await
            }
            "policyanalyzer_api" => {
                self.policyanalyzer_api().update_resource(resource_name, id, input).await
            }
            "mybusinessbusinesscalls_api" => {
                self.mybusinessbusinesscalls_api().update_resource(resource_name, id, input).await
            }
            "ml_api" => {
                self.ml_api().update_resource(resource_name, id, input).await
            }
            "firebaseml_api" => {
                self.firebaseml_api().update_resource(resource_name, id, input).await
            }
            "blogger_api" => {
                self.blogger_api().update_resource(resource_name, id, input).await
            }
            "vault_api" => {
                self.vault_api().update_resource(resource_name, id, input).await
            }
            "baremetalsolution_api" => {
                self.baremetalsolution_api().update_resource(resource_name, id, input).await
            }
            "testing_api" => {
                self.testing_api().update_resource(resource_name, id, input).await
            }
            "recaptchaenterprise_api" => {
                self.recaptchaenterprise_api().update_resource(resource_name, id, input).await
            }
            "servicecontrol_api" => {
                self.servicecontrol_api().update_resource(resource_name, id, input).await
            }
            "documentai_api" => {
                self.documentai_api().update_resource(resource_name, id, input).await
            }
            "firebasedynamiclinks_api" => {
                self.firebasedynamiclinks_api().update_resource(resource_name, id, input).await
            }
            "apigee_api" => {
                self.apigee_api().update_resource(resource_name, id, input).await
            }
            "netapp_api" => {
                self.netapp_api().update_resource(resource_name, id, input).await
            }
            "identitytoolkit_api" => {
                self.identitytoolkit_api().update_resource(resource_name, id, input).await
            }
            "datastream_api" => {
                self.datastream_api().update_resource(resource_name, id, input).await
            }
            "iamcredentials_api" => {
                self.iamcredentials_api().update_resource(resource_name, id, input).await
            }
            "datalabeling_api" => {
                self.datalabeling_api().update_resource(resource_name, id, input).await
            }
            "youtubereporting_api" => {
                self.youtubereporting_api().update_resource(resource_name, id, input).await
            }
            "genomics_api" => {
                self.genomics_api().update_resource(resource_name, id, input).await
            }
            "lifesciences_api" => {
                self.lifesciences_api().update_resource(resource_name, id, input).await
            }
            "compute_api" => {
                self.compute_api().update_resource(resource_name, id, input).await
            }
            "youtubeanalytics_api" => {
                self.youtubeanalytics_api().update_resource(resource_name, id, input).await
            }
            "factchecktools_api" => {
                self.factchecktools_api().update_resource(resource_name, id, input).await
            }
            "dialogflow_api" => {
                self.dialogflow_api().update_resource(resource_name, id, input).await
            }
            "docs_api" => {
                self.docs_api().update_resource(resource_name, id, input).await
            }
            "tagmanager_api" => {
                self.tagmanager_api().update_resource(resource_name, id, input).await
            }
            "texttospeech_api" => {
                self.texttospeech_api().update_resource(resource_name, id, input).await
            }
            "gmail_api" => {
                self.gmail_api().update_resource(resource_name, id, input).await
            }
            "tasks_api" => {
                self.tasks_api().update_resource(resource_name, id, input).await
            }
            "securityposture_api" => {
                self.securityposture_api().update_resource(resource_name, id, input).await
            }
            "sql_api" => {
                self.sql_api().update_resource(resource_name, id, input).await
            }
            "integrations_api" => {
                self.integrations_api().update_resource(resource_name, id, input).await
            }
            "essentialcontacts_api" => {
                self.essentialcontacts_api().update_resource(resource_name, id, input).await
            }
            "tpu_api" => {
                self.tpu_api().update_resource(resource_name, id, input).await
            }
            "classroom_api" => {
                self.classroom_api().update_resource(resource_name, id, input).await
            }
            "mybusinesslodging_api" => {
                self.mybusinesslodging_api().update_resource(resource_name, id, input).await
            }
            "rapidmigrationassessment_api" => {
                self.rapidmigrationassessment_api().update_resource(resource_name, id, input).await
            }
            "blockchainnodeengine_api" => {
                self.blockchainnodeengine_api().update_resource(resource_name, id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Delete a resource
    async fn delete(&self, resource_type: &str, id: &str) -> Result<()> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "translate_api" => {
                self.translate_api().delete_resource(resource_name, id).await
            }
            "area120tables_api" => {
                self.area120tables_api().delete_resource(resource_name, id).await
            }
            "abusiveexperiencereport_api" => {
                self.abusiveexperiencereport_api().delete_resource(resource_name, id).await
            }
            "securesourcemanager_api" => {
                self.securesourcemanager_api().delete_resource(resource_name, id).await
            }
            "file_api" => {
                self.file_api().delete_resource(resource_name, id).await
            }
            "oauth2_api" => {
                self.oauth2_api().delete_resource(resource_name, id).await
            }
            "serviceuser_api" => {
                self.serviceuser_api().delete_resource(resource_name, id).await
            }
            "bigquerydatatransfer_api" => {
                self.bigquerydatatransfer_api().delete_resource(resource_name, id).await
            }
            "cloudfunctions_api" => {
                self.cloudfunctions_api().delete_resource(resource_name, id).await
            }
            "networkconnectivity_api" => {
                self.networkconnectivity_api().delete_resource(resource_name, id).await
            }
            "mybusinessverifications_api" => {
                self.mybusinessverifications_api().delete_resource(resource_name, id).await
            }
            "qpxexpress_api" => {
                self.qpxexpress_api().delete_resource(resource_name, id).await
            }
            "dataproc_api" => {
                self.dataproc_api().delete_resource(resource_name, id).await
            }
            "playgrouping_api" => {
                self.playgrouping_api().delete_resource(resource_name, id).await
            }
            "mirror_api" => {
                self.mirror_api().delete_resource(resource_name, id).await
            }
            "adexchangeseller_api" => {
                self.adexchangeseller_api().delete_resource(resource_name, id).await
            }
            "binaryauthorization_api" => {
                self.binaryauthorization_api().delete_resource(resource_name, id).await
            }
            "marketingplatformadmin_api" => {
                self.marketingplatformadmin_api().delete_resource(resource_name, id).await
            }
            "cloudprivatecatalogproducer_api" => {
                self.cloudprivatecatalogproducer_api().delete_resource(resource_name, id).await
            }
            "pubsublite_api" => {
                self.pubsublite_api().delete_resource(resource_name, id).await
            }
            "slides_api" => {
                self.slides_api().delete_resource(resource_name, id).await
            }
            "billingbudgets_api" => {
                self.billingbudgets_api().delete_resource(resource_name, id).await
            }
            "workstations_api" => {
                self.workstations_api().delete_resource(resource_name, id).await
            }
            "civicinfo_api" => {
                self.civicinfo_api().delete_resource(resource_name, id).await
            }
            "spectrum_api" => {
                self.spectrum_api().delete_resource(resource_name, id).await
            }
            "surveys_api" => {
                self.surveys_api().delete_resource(resource_name, id).await
            }
            "dataplex_api" => {
                self.dataplex_api().delete_resource(resource_name, id).await
            }
            "redis_api" => {
                self.redis_api().delete_resource(resource_name, id).await
            }
            "addressvalidation_api" => {
                self.addressvalidation_api().delete_resource(resource_name, id).await
            }
            "sourcerepo_api" => {
                self.sourcerepo_api().delete_resource(resource_name, id).await
            }
            "osconfig_api" => {
                self.osconfig_api().delete_resource(resource_name, id).await
            }
            "domainsrdap_api" => {
                self.domainsrdap_api().delete_resource(resource_name, id).await
            }
            "speech_api" => {
                self.speech_api().delete_resource(resource_name, id).await
            }
            "trafficdirector_api" => {
                self.trafficdirector_api().delete_resource(resource_name, id).await
            }
            "indexing_api" => {
                self.indexing_api().delete_resource(resource_name, id).await
            }
            "ondemandscanning_api" => {
                self.ondemandscanning_api().delete_resource(resource_name, id).await
            }
            "drive_api" => {
                self.drive_api().delete_resource(resource_name, id).await
            }
            "mybusinessnotifications_api" => {
                self.mybusinessnotifications_api().delete_resource(resource_name, id).await
            }
            "workflowexecutions_api" => {
                self.workflowexecutions_api().delete_resource(resource_name, id).await
            }
            "developerconnect_api" => {
                self.developerconnect_api().delete_resource(resource_name, id).await
            }
            "analyticsadmin_api" => {
                self.analyticsadmin_api().delete_resource(resource_name, id).await
            }
            "playdeveloperreporting_api" => {
                self.playdeveloperreporting_api().delete_resource(resource_name, id).await
            }
            "prod_tt_sasportal_api" => {
                self.prod_tt_sasportal_api().delete_resource(resource_name, id).await
            }
            "searchconsole_api" => {
                self.searchconsole_api().delete_resource(resource_name, id).await
            }
            "firebaseremoteconfig_api" => {
                self.firebaseremoteconfig_api().delete_resource(resource_name, id).await
            }
            "publicca_api" => {
                self.publicca_api().delete_resource(resource_name, id).await
            }
            "discovery_api" => {
                self.discovery_api().delete_resource(resource_name, id).await
            }
            "firebasehosting_api" => {
                self.firebasehosting_api().delete_resource(resource_name, id).await
            }
            "memcache_api" => {
                self.memcache_api().delete_resource(resource_name, id).await
            }
            "playablelocations_api" => {
                self.playablelocations_api().delete_resource(resource_name, id).await
            }
            "jobs_api" => {
                self.jobs_api().delete_resource(resource_name, id).await
            }
            "iam_api" => {
                self.iam_api().delete_resource(resource_name, id).await
            }
            "reseller_api" => {
                self.reseller_api().delete_resource(resource_name, id).await
            }
            "realtimebidding_api" => {
                self.realtimebidding_api().delete_resource(resource_name, id).await
            }
            "datamigration_api" => {
                self.datamigration_api().delete_resource(resource_name, id).await
            }
            "safebrowsing_api" => {
                self.safebrowsing_api().delete_resource(resource_name, id).await
            }
            "libraryagent_api" => {
                self.libraryagent_api().delete_resource(resource_name, id).await
            }
            "css_api" => {
                self.css_api().delete_resource(resource_name, id).await
            }
            "androidpublisher_api" => {
                self.androidpublisher_api().delete_resource(resource_name, id).await
            }
            "datacatalog_api" => {
                self.datacatalog_api().delete_resource(resource_name, id).await
            }
            "cloudlocationfinder_api" => {
                self.cloudlocationfinder_api().delete_resource(resource_name, id).await
            }
            "storagetransfer_api" => {
                self.storagetransfer_api().delete_resource(resource_name, id).await
            }
            "resourcesettings_api" => {
                self.resourcesettings_api().delete_resource(resource_name, id).await
            }
            "firebasedatabase_api" => {
                self.firebasedatabase_api().delete_resource(resource_name, id).await
            }
            "analyticshub_api" => {
                self.analyticshub_api().delete_resource(resource_name, id).await
            }
            "http_body" => {
                self.http_body().delete_resource(resource_name, id).await
            }
            "managedkafka_api" => {
                self.managedkafka_api().delete_resource(resource_name, id).await
            }
            "poly_api" => {
                self.poly_api().delete_resource(resource_name, id).await
            }
            "orgpolicy_api" => {
                self.orgpolicy_api().delete_resource(resource_name, id).await
            }
            "certificatemanager_api" => {
                self.certificatemanager_api().delete_resource(resource_name, id).await
            }
            "apigateway_api" => {
                self.apigateway_api().delete_resource(resource_name, id).await
            }
            "datastore_api" => {
                self.datastore_api().delete_resource(resource_name, id).await
            }
            "cloudtasks_api" => {
                self.cloudtasks_api().delete_resource(resource_name, id).await
            }
            "firebaseappdistribution_api" => {
                self.firebaseappdistribution_api().delete_resource(resource_name, id).await
            }
            "domains_api" => {
                self.domains_api().delete_resource(resource_name, id).await
            }
            "policysimulator_api" => {
                self.policysimulator_api().delete_resource(resource_name, id).await
            }
            "apihub_api" => {
                self.apihub_api().delete_resource(resource_name, id).await
            }
            "mybusinessaccountmanagement_api" => {
                self.mybusinessaccountmanagement_api().delete_resource(resource_name, id).await
            }
            "smartdevicemanagement_api" => {
                self.smartdevicemanagement_api().delete_resource(resource_name, id).await
            }
            "tracing_api" => {
                self.tracing_api().delete_resource(resource_name, id).await
            }
            "gamesmanagement_api" => {
                self.gamesmanagement_api().delete_resource(resource_name, id).await
            }
            "firestore_api" => {
                self.firestore_api().delete_resource(resource_name, id).await
            }
            "language_api" => {
                self.language_api().delete_resource(resource_name, id).await
            }
            "json_body" => {
                self.json_body().delete_resource(resource_name, id).await
            }
            "replicapoolupdater_api" => {
                self.replicapoolupdater_api().delete_resource(resource_name, id).await
            }
            "androidmanagement_api" => {
                self.androidmanagement_api().delete_resource(resource_name, id).await
            }
            "licensing_api" => {
                self.licensing_api().delete_resource(resource_name, id).await
            }
            "sasportal_api" => {
                self.sasportal_api().delete_resource(resource_name, id).await
            }
            "vmwareengine_api" => {
                self.vmwareengine_api().delete_resource(resource_name, id).await
            }
            "pollen_api" => {
                self.pollen_api().delete_resource(resource_name, id).await
            }
            "cloudsupport_api" => {
                self.cloudsupport_api().delete_resource(resource_name, id).await
            }
            "servicemanagement_api" => {
                self.servicemanagement_api().delete_resource(resource_name, id).await
            }
            "cloudiot_api" => {
                self.cloudiot_api().delete_resource(resource_name, id).await
            }
            "androiddeviceprovisioning_api" => {
                self.androiddeviceprovisioning_api().delete_resource(resource_name, id).await
            }
            "gamesconfiguration_api" => {
                self.gamesconfiguration_api().delete_resource(resource_name, id).await
            }
            "areainsights_api" => {
                self.areainsights_api().delete_resource(resource_name, id).await
            }
            "meet_api" => {
                self.meet_api().delete_resource(resource_name, id).await
            }
            "adsense_api" => {
                self.adsense_api().delete_resource(resource_name, id).await
            }
            "toolresults_api" => {
                self.toolresults_api().delete_resource(resource_name, id).await
            }
            "artifactregistry_api" => {
                self.artifactregistry_api().delete_resource(resource_name, id).await
            }
            "cloudcommerceprocurement_api" => {
                self.cloudcommerceprocurement_api().delete_resource(resource_name, id).await
            }
            "alertcenter_api" => {
                self.alertcenter_api().delete_resource(resource_name, id).await
            }
            "mybusinessbusinessinformation_api" => {
                self.mybusinessbusinessinformation_api().delete_resource(resource_name, id).await
            }
            "bigquerydatapolicy_api" => {
                self.bigquerydatapolicy_api().delete_resource(resource_name, id).await
            }
            "androidenterprise_api" => {
                self.androidenterprise_api().delete_resource(resource_name, id).await
            }
            "playmoviespartner_api" => {
                self.playmoviespartner_api().delete_resource(resource_name, id).await
            }
            "oracledatabase_api" => {
                self.oracledatabase_api().delete_resource(resource_name, id).await
            }
            "composer_api" => {
                self.composer_api().delete_resource(resource_name, id).await
            }
            "dataform_api" => {
                self.dataform_api().delete_resource(resource_name, id).await
            }
            "adsensehost_api" => {
                self.adsensehost_api().delete_resource(resource_name, id).await
            }
            "deploymentmanager_api" => {
                self.deploymentmanager_api().delete_resource(resource_name, id).await
            }
            "displayvideo_api" => {
                self.displayvideo_api().delete_resource(resource_name, id).await
            }
            "chromewebstore_api" => {
                self.chromewebstore_api().delete_resource(resource_name, id).await
            }
            "cloudkms_api" => {
                self.cloudkms_api().delete_resource(resource_name, id).await
            }
            "bigtableadmin_api" => {
                self.bigtableadmin_api().delete_resource(resource_name, id).await
            }
            "drivelabels_api" => {
                self.drivelabels_api().delete_resource(resource_name, id).await
            }
            "vision_api" => {
                self.vision_api().delete_resource(resource_name, id).await
            }
            "streetviewpublish_api" => {
                self.streetviewpublish_api().delete_resource(resource_name, id).await
            }
            "clouddeploy_api" => {
                self.clouddeploy_api().delete_resource(resource_name, id).await
            }
            "biglake_api" => {
                self.biglake_api().delete_resource(resource_name, id).await
            }
            "eventarc_api" => {
                self.eventarc_api().delete_resource(resource_name, id).await
            }
            "apigeeregistry_api" => {
                self.apigeeregistry_api().delete_resource(resource_name, id).await
            }
            "cloudbuild_api" => {
                self.cloudbuild_api().delete_resource(resource_name, id).await
            }
            "secretmanager_api" => {
                self.secretmanager_api().delete_resource(resource_name, id).await
            }
            "sheets_api" => {
                self.sheets_api().delete_resource(resource_name, id).await
            }
            "recommendationengine_api" => {
                self.recommendationengine_api().delete_resource(resource_name, id).await
            }
            "appengine_api" => {
                self.appengine_api().delete_resource(resource_name, id).await
            }
            "dataportability_api" => {
                self.dataportability_api().delete_resource(resource_name, id).await
            }
            "remotebuildexecution_api" => {
                self.remotebuildexecution_api().delete_resource(resource_name, id).await
            }
            "containeranalysis_api" => {
                self.containeranalysis_api().delete_resource(resource_name, id).await
            }
            "workloadmanager_api" => {
                self.workloadmanager_api().delete_resource(resource_name, id).await
            }
            "cloudcontrolspartner_api" => {
                self.cloudcontrolspartner_api().delete_resource(resource_name, id).await
            }
            "monitoring_api" => {
                self.monitoring_api().delete_resource(resource_name, id).await
            }
            "privateca_api" => {
                self.privateca_api().delete_resource(resource_name, id).await
            }
            "container_api" => {
                self.container_api().delete_resource(resource_name, id).await
            }
            "merchantapi_api" => {
                self.merchantapi_api().delete_resource(resource_name, id).await
            }
            "datapipelines_api" => {
                self.datapipelines_api().delete_resource(resource_name, id).await
            }
            "sts_api" => {
                self.sts_api().delete_resource(resource_name, id).await
            }
            "observability_api" => {
                self.observability_api().delete_resource(resource_name, id).await
            }
            "dataflow_api" => {
                self.dataflow_api().delete_resource(resource_name, id).await
            }
            "cloudresourcemanager_api" => {
                self.cloudresourcemanager_api().delete_resource(resource_name, id).await
            }
            "firebaseapphosting_api" => {
                self.firebaseapphosting_api().delete_resource(resource_name, id).await
            }
            "verifiedaccess_api" => {
                self.verifiedaccess_api().delete_resource(resource_name, id).await
            }
            "accessapproval_api" => {
                self.accessapproval_api().delete_resource(resource_name, id).await
            }
            "firebasedataconnect_api" => {
                self.firebasedataconnect_api().delete_resource(resource_name, id).await
            }
            "assuredworkloads_api" => {
                self.assuredworkloads_api().delete_resource(resource_name, id).await
            }
            "datafusion_api" => {
                self.datafusion_api().delete_resource(resource_name, id).await
            }
            "doubleclicksearch_api" => {
                self.doubleclicksearch_api().delete_resource(resource_name, id).await
            }
            "ideahub_api" => {
                self.ideahub_api().delete_resource(resource_name, id).await
            }
            "alloydb_api" => {
                self.alloydb_api().delete_resource(resource_name, id).await
            }
            "doubleclickbidmanager_api" => {
                self.doubleclickbidmanager_api().delete_resource(resource_name, id).await
            }
            "bigqueryreservation_api" => {
                self.bigqueryreservation_api().delete_resource(resource_name, id).await
            }
            "notebooks_api" => {
                self.notebooks_api().delete_resource(resource_name, id).await
            }
            "readerrevenuesubscriptionlinking_api" => {
                self.readerrevenuesubscriptionlinking_api().delete_resource(resource_name, id).await
            }
            "authorizedbuyersmarketplace_api" => {
                self.authorizedbuyersmarketplace_api().delete_resource(resource_name, id).await
            }
            "adexchangebuyer2_api" => {
                self.adexchangebuyer2_api().delete_resource(resource_name, id).await
            }
            "adsenseplatform_api" => {
                self.adsenseplatform_api().delete_resource(resource_name, id).await
            }
            "mybusinessplaceactions_api" => {
                self.mybusinessplaceactions_api().delete_resource(resource_name, id).await
            }
            "firebasestorage_api" => {
                self.firebasestorage_api().delete_resource(resource_name, id).await
            }
            "calendar_api" => {
                self.calendar_api().delete_resource(resource_name, id).await
            }
            "networkmanagement_api" => {
                self.networkmanagement_api().delete_resource(resource_name, id).await
            }
            "admin_api" => {
                self.admin_api().delete_resource(resource_name, id).await
            }
            "acceleratedmobilepageurl_api" => {
                self.acceleratedmobilepageurl_api().delete_resource(resource_name, id).await
            }
            "fusiontables_api" => {
                self.fusiontables_api().delete_resource(resource_name, id).await
            }
            "chromepolicy_api" => {
                self.chromepolicy_api().delete_resource(resource_name, id).await
            }
            "securitycenter_api" => {
                self.securitycenter_api().delete_resource(resource_name, id).await
            }
            "analyticsreporting_api" => {
                self.analyticsreporting_api().delete_resource(resource_name, id).await
            }
            "iap_api" => {
                self.iap_api().delete_resource(resource_name, id).await
            }
            "consumersurveys_api" => {
                self.consumersurveys_api().delete_resource(resource_name, id).await
            }
            "siteverification_api" => {
                self.siteverification_api().delete_resource(resource_name, id).await
            }
            "versionhistory_api" => {
                self.versionhistory_api().delete_resource(resource_name, id).await
            }
            "storagebatchoperations_api" => {
                self.storagebatchoperations_api().delete_resource(resource_name, id).await
            }
            "acmedns_api" => {
                self.acmedns_api().delete_resource(resource_name, id).await
            }
            "storage_api" => {
                self.storage_api().delete_resource(resource_name, id).await
            }
            "networkservices_api" => {
                self.networkservices_api().delete_resource(resource_name, id).await
            }
            "dfareporting_api" => {
                self.dfareporting_api().delete_resource(resource_name, id).await
            }
            "webfonts_api" => {
                self.webfonts_api().delete_resource(resource_name, id).await
            }
            "apim_api" => {
                self.apim_api().delete_resource(resource_name, id).await
            }
            "chromeuxreport_api" => {
                self.chromeuxreport_api().delete_resource(resource_name, id).await
            }
            "aiplatform_api" => {
                self.aiplatform_api().delete_resource(resource_name, id).await
            }
            "script_api" => {
                self.script_api().delete_resource(resource_name, id).await
            }
            "contentwarehouse_api" => {
                self.contentwarehouse_api().delete_resource(resource_name, id).await
            }
            "batch_api" => {
                self.batch_api().delete_resource(resource_name, id).await
            }
            "kmsinventory_api" => {
                self.kmsinventory_api().delete_resource(resource_name, id).await
            }
            "run_api" => {
                self.run_api().delete_resource(resource_name, id).await
            }
            "servicenetworking_api" => {
                self.servicenetworking_api().delete_resource(resource_name, id).await
            }
            "cloudscheduler_api" => {
                self.cloudscheduler_api().delete_resource(resource_name, id).await
            }
            "cloudsearch_api" => {
                self.cloudsearch_api().delete_resource(resource_name, id).await
            }
            "admob_api" => {
                self.admob_api().delete_resource(resource_name, id).await
            }
            "workspaceevents_api" => {
                self.workspaceevents_api().delete_resource(resource_name, id).await
            }
            "metastore_api" => {
                self.metastore_api().delete_resource(resource_name, id).await
            }
            "policytroubleshooter_api" => {
                self.policytroubleshooter_api().delete_resource(resource_name, id).await
            }
            "analyticsdata_api" => {
                self.analyticsdata_api().delete_resource(resource_name, id).await
            }
            "contactcenteraiplatform_api" => {
                self.contactcenteraiplatform_api().delete_resource(resource_name, id).await
            }
            "forms_api" => {
                self.forms_api().delete_resource(resource_name, id).await
            }
            "clouderrorreporting_api" => {
                self.clouderrorreporting_api().delete_resource(resource_name, id).await
            }
            "pagespeedonline_api" => {
                self.pagespeedonline_api().delete_resource(resource_name, id).await
            }
            "parallelstore_api" => {
                self.parallelstore_api().delete_resource(resource_name, id).await
            }
            "cloudbilling_api" => {
                self.cloudbilling_api().delete_resource(resource_name, id).await
            }
            "cloudidentity_api" => {
                self.cloudidentity_api().delete_resource(resource_name, id).await
            }
            "appsactivity_api" => {
                self.appsactivity_api().delete_resource(resource_name, id).await
            }
            "groupssettings_api" => {
                self.groupssettings_api().delete_resource(resource_name, id).await
            }
            "adexchangebuyer_api" => {
                self.adexchangebuyer_api().delete_resource(resource_name, id).await
            }
            "youtube_api" => {
                self.youtube_api().delete_resource(resource_name, id).await
            }
            "looker_api" => {
                self.looker_api().delete_resource(resource_name, id).await
            }
            "gkebackup_api" => {
                self.gkebackup_api().delete_resource(resource_name, id).await
            }
            "cloudasset_api" => {
                self.cloudasset_api().delete_resource(resource_name, id).await
            }
            "people_api" => {
                self.people_api().delete_resource(resource_name, id).await
            }
            "mybusinessqanda_api" => {
                self.mybusinessqanda_api().delete_resource(resource_name, id).await
            }
            "beyondcorp_api" => {
                self.beyondcorp_api().delete_resource(resource_name, id).await
            }
            "pubsub_api" => {
                self.pubsub_api().delete_resource(resource_name, id).await
            }
            "servicebroker_api" => {
                self.servicebroker_api().delete_resource(resource_name, id).await
            }
            "contactcenterinsights_api" => {
                self.contactcenterinsights_api().delete_resource(resource_name, id).await
            }
            "accesscontextmanager_api" => {
                self.accesscontextmanager_api().delete_resource(resource_name, id).await
            }
            "searchads360_api" => {
                self.searchads360_api().delete_resource(resource_name, id).await
            }
            "oslogin_api" => {
                self.oslogin_api().delete_resource(resource_name, id).await
            }
            "playcustomapp_api" => {
                self.playcustomapp_api().delete_resource(resource_name, id).await
            }
            "webmasters_api" => {
                self.webmasters_api().delete_resource(resource_name, id).await
            }
            "fitness_api" => {
                self.fitness_api().delete_resource(resource_name, id).await
            }
            "parametermanager_api" => {
                self.parametermanager_api().delete_resource(resource_name, id).await
            }
            "businessprofileperformance_api" => {
                self.businessprofileperformance_api().delete_resource(resource_name, id).await
            }
            "videointelligence_api" => {
                self.videointelligence_api().delete_resource(resource_name, id).await
            }
            "replicapool_api" => {
                self.replicapool_api().delete_resource(resource_name, id).await
            }
            "dlp_api" => {
                self.dlp_api().delete_resource(resource_name, id).await
            }
            "migrationcenter_api" => {
                self.migrationcenter_api().delete_resource(resource_name, id).await
            }
            "kgsearch_api" => {
                self.kgsearch_api().delete_resource(resource_name, id).await
            }
            "cloudprofiler_api" => {
                self.cloudprofiler_api().delete_resource(resource_name, id).await
            }
            "healthcare_api" => {
                self.healthcare_api().delete_resource(resource_name, id).await
            }
            "firebaseappcheck_api" => {
                self.firebaseappcheck_api().delete_resource(resource_name, id).await
            }
            "gkeonprem_api" => {
                self.gkeonprem_api().delete_resource(resource_name, id).await
            }
            "repeated_any_query_error" => {
                self.repeated_any_query_error().delete_resource(resource_name, id).await
            }
            "transcoder_api" => {
                self.transcoder_api().delete_resource(resource_name, id).await
            }
            "firebaserules_api" => {
                self.firebaserules_api().delete_resource(resource_name, id).await
            }
            "plusdomains_api" => {
                self.plusdomains_api().delete_resource(resource_name, id).await
            }
            "proximitybeacon_api" => {
                self.proximitybeacon_api().delete_resource(resource_name, id).await
            }
            "firebase_api" => {
                self.firebase_api().delete_resource(resource_name, id).await
            }
            "resource_named_service" => {
                self.resource_named_service().delete_resource(resource_name, id).await
            }
            "config_api" => {
                self.config_api().delete_resource(resource_name, id).await
            }
            "games_api" => {
                self.games_api().delete_resource(resource_name, id).await
            }
            "appstate_api" => {
                self.appstate_api().delete_resource(resource_name, id).await
            }
            "saasservicemgmt_api" => {
                self.saasservicemgmt_api().delete_resource(resource_name, id).await
            }
            "test_api" => {
                self.test_api().delete_resource(resource_name, id).await
            }
            "gkehub_api" => {
                self.gkehub_api().delete_resource(resource_name, id).await
            }
            "analytics_api" => {
                self.analytics_api().delete_resource(resource_name, id).await
            }
            "discoveryengine_api" => {
                self.discoveryengine_api().delete_resource(resource_name, id).await
            }
            "playintegrity_api" => {
                self.playintegrity_api().delete_resource(resource_name, id).await
            }
            "homegraph_api" => {
                self.homegraph_api().delete_resource(resource_name, id).await
            }
            "apikeys_api" => {
                self.apikeys_api().delete_resource(resource_name, id).await
            }
            "serviceconsumermanagement_api" => {
                self.serviceconsumermanagement_api().delete_resource(resource_name, id).await
            }
            "datalineage_api" => {
                self.datalineage_api().delete_resource(resource_name, id).await
            }
            "manufacturers_api" => {
                self.manufacturers_api().delete_resource(resource_name, id).await
            }
            "vectortile_api" => {
                self.vectortile_api().delete_resource(resource_name, id).await
            }
            "chat_api" => {
                self.chat_api().delete_resource(resource_name, id).await
            }
            "bigquery_api" => {
                self.bigquery_api().delete_resource(resource_name, id).await
            }
            "plus_api" => {
                self.plus_api().delete_resource(resource_name, id).await
            }
            "bigqueryconnection_api" => {
                self.bigqueryconnection_api().delete_resource(resource_name, id).await
            }
            "places_api" => {
                self.places_api().delete_resource(resource_name, id).await
            }
            "logging_api" => {
                self.logging_api().delete_resource(resource_name, id).await
            }
            "webrisk_api" => {
                self.webrisk_api().delete_resource(resource_name, id).await
            }
            "vpcaccess_api" => {
                self.vpcaccess_api().delete_resource(resource_name, id).await
            }
            "urlshortener_api" => {
                self.urlshortener_api().delete_resource(resource_name, id).await
            }
            "sqladmin_api" => {
                self.sqladmin_api().delete_resource(resource_name, id).await
            }
            "keep_api" => {
                self.keep_api().delete_resource(resource_name, id).await
            }
            "walletobjects_api" => {
                self.walletobjects_api().delete_resource(resource_name, id).await
            }
            "apphub_api" => {
                self.apphub_api().delete_resource(resource_name, id).await
            }
            "vmmigration_api" => {
                self.vmmigration_api().delete_resource(resource_name, id).await
            }
            "airquality_api" => {
                self.airquality_api().delete_resource(resource_name, id).await
            }
            "chromemanagement_api" => {
                self.chromemanagement_api().delete_resource(resource_name, id).await
            }
            "adexperiencereport_api" => {
                self.adexperiencereport_api().delete_resource(resource_name, id).await
            }
            "recommender_api" => {
                self.recommender_api().delete_resource(resource_name, id).await
            }
            "driveactivity_api" => {
                self.driveactivity_api().delete_resource(resource_name, id).await
            }
            "dns_api" => {
                self.dns_api().delete_resource(resource_name, id).await
            }
            "cloudshell_api" => {
                self.cloudshell_api().delete_resource(resource_name, id).await
            }
            "content_api" => {
                self.content_api().delete_resource(resource_name, id).await
            }
            "commentanalyzer_api" => {
                self.commentanalyzer_api().delete_resource(resource_name, id).await
            }
            "backupdr_api" => {
                self.backupdr_api().delete_resource(resource_name, id).await
            }
            "groupsmigration_api" => {
                self.groupsmigration_api().delete_resource(resource_name, id).await
            }
            "connectors_api" => {
                self.connectors_api().delete_resource(resource_name, id).await
            }
            "solar_api" => {
                self.solar_api().delete_resource(resource_name, id).await
            }
            "fcm_api" => {
                self.fcm_api().delete_resource(resource_name, id).await
            }
            "networksecurity_api" => {
                self.networksecurity_api().delete_resource(resource_name, id).await
            }
            "managedidentities_api" => {
                self.managedidentities_api().delete_resource(resource_name, id).await
            }
            "serviceusage_api" => {
                self.serviceusage_api().delete_resource(resource_name, id).await
            }
            "digitalassetlinks_api" => {
                self.digitalassetlinks_api().delete_resource(resource_name, id).await
            }
            "clouddebugger_api" => {
                self.clouddebugger_api().delete_resource(resource_name, id).await
            }
            "datamanager_api" => {
                self.datamanager_api().delete_resource(resource_name, id).await
            }
            "ids_api" => {
                self.ids_api().delete_resource(resource_name, id).await
            }
            "workflows_api" => {
                self.workflows_api().delete_resource(resource_name, id).await
            }
            "cloudtrace_api" => {
                self.cloudtrace_api().delete_resource(resource_name, id).await
            }
            "advisorynotifications_api" => {
                self.advisorynotifications_api().delete_resource(resource_name, id).await
            }
            "fcmdata_api" => {
                self.fcmdata_api().delete_resource(resource_name, id).await
            }
            "checks_api" => {
                self.checks_api().delete_resource(resource_name, id).await
            }
            "blogger_3" => {
                self.blogger_3().delete_resource(resource_name, id).await
            }
            "mediaupload" => {
                self.mediaupload().delete_resource(resource_name, id).await
            }
            "paymentsresellersubscription_api" => {
                self.paymentsresellersubscription_api().delete_resource(resource_name, id).await
            }
            "spanner_api" => {
                self.spanner_api().delete_resource(resource_name, id).await
            }
            "cloudchannel_api" => {
                self.cloudchannel_api().delete_resource(resource_name, id).await
            }
            "gameservices_api" => {
                self.gameservices_api().delete_resource(resource_name, id).await
            }
            "cloudprivatecatalog_api" => {
                self.cloudprivatecatalog_api().delete_resource(resource_name, id).await
            }
            "gmailpostmastertools_api" => {
                self.gmailpostmastertools_api().delete_resource(resource_name, id).await
            }
            "partners_api" => {
                self.partners_api().delete_resource(resource_name, id).await
            }
            "any" => {
                self.any().delete_resource(resource_name, id).await
            }
            "runtimeconfig_api" => {
                self.runtimeconfig_api().delete_resource(resource_name, id).await
            }
            "retail_api" => {
                self.retail_api().delete_resource(resource_name, id).await
            }
            "books_api" => {
                self.books_api().delete_resource(resource_name, id).await
            }
            "websecurityscanner_api" => {
                self.websecurityscanner_api().delete_resource(resource_name, id).await
            }
            "localservices_api" => {
                self.localservices_api().delete_resource(resource_name, id).await
            }
            "servicedirectory_api" => {
                self.servicedirectory_api().delete_resource(resource_name, id).await
            }
            "customsearch_api" => {
                self.customsearch_api().delete_resource(resource_name, id).await
            }
            "policyanalyzer_api" => {
                self.policyanalyzer_api().delete_resource(resource_name, id).await
            }
            "mybusinessbusinesscalls_api" => {
                self.mybusinessbusinesscalls_api().delete_resource(resource_name, id).await
            }
            "ml_api" => {
                self.ml_api().delete_resource(resource_name, id).await
            }
            "firebaseml_api" => {
                self.firebaseml_api().delete_resource(resource_name, id).await
            }
            "blogger_api" => {
                self.blogger_api().delete_resource(resource_name, id).await
            }
            "vault_api" => {
                self.vault_api().delete_resource(resource_name, id).await
            }
            "baremetalsolution_api" => {
                self.baremetalsolution_api().delete_resource(resource_name, id).await
            }
            "testing_api" => {
                self.testing_api().delete_resource(resource_name, id).await
            }
            "recaptchaenterprise_api" => {
                self.recaptchaenterprise_api().delete_resource(resource_name, id).await
            }
            "servicecontrol_api" => {
                self.servicecontrol_api().delete_resource(resource_name, id).await
            }
            "documentai_api" => {
                self.documentai_api().delete_resource(resource_name, id).await
            }
            "firebasedynamiclinks_api" => {
                self.firebasedynamiclinks_api().delete_resource(resource_name, id).await
            }
            "apigee_api" => {
                self.apigee_api().delete_resource(resource_name, id).await
            }
            "netapp_api" => {
                self.netapp_api().delete_resource(resource_name, id).await
            }
            "identitytoolkit_api" => {
                self.identitytoolkit_api().delete_resource(resource_name, id).await
            }
            "datastream_api" => {
                self.datastream_api().delete_resource(resource_name, id).await
            }
            "iamcredentials_api" => {
                self.iamcredentials_api().delete_resource(resource_name, id).await
            }
            "datalabeling_api" => {
                self.datalabeling_api().delete_resource(resource_name, id).await
            }
            "youtubereporting_api" => {
                self.youtubereporting_api().delete_resource(resource_name, id).await
            }
            "genomics_api" => {
                self.genomics_api().delete_resource(resource_name, id).await
            }
            "lifesciences_api" => {
                self.lifesciences_api().delete_resource(resource_name, id).await
            }
            "compute_api" => {
                self.compute_api().delete_resource(resource_name, id).await
            }
            "youtubeanalytics_api" => {
                self.youtubeanalytics_api().delete_resource(resource_name, id).await
            }
            "factchecktools_api" => {
                self.factchecktools_api().delete_resource(resource_name, id).await
            }
            "dialogflow_api" => {
                self.dialogflow_api().delete_resource(resource_name, id).await
            }
            "docs_api" => {
                self.docs_api().delete_resource(resource_name, id).await
            }
            "tagmanager_api" => {
                self.tagmanager_api().delete_resource(resource_name, id).await
            }
            "texttospeech_api" => {
                self.texttospeech_api().delete_resource(resource_name, id).await
            }
            "gmail_api" => {
                self.gmail_api().delete_resource(resource_name, id).await
            }
            "tasks_api" => {
                self.tasks_api().delete_resource(resource_name, id).await
            }
            "securityposture_api" => {
                self.securityposture_api().delete_resource(resource_name, id).await
            }
            "sql_api" => {
                self.sql_api().delete_resource(resource_name, id).await
            }
            "integrations_api" => {
                self.integrations_api().delete_resource(resource_name, id).await
            }
            "essentialcontacts_api" => {
                self.essentialcontacts_api().delete_resource(resource_name, id).await
            }
            "tpu_api" => {
                self.tpu_api().delete_resource(resource_name, id).await
            }
            "classroom_api" => {
                self.classroom_api().delete_resource(resource_name, id).await
            }
            "mybusinesslodging_api" => {
                self.mybusinesslodging_api().delete_resource(resource_name, id).await
            }
            "rapidmigrationassessment_api" => {
                self.rapidmigrationassessment_api().delete_resource(resource_name, id).await
            }
            "blockchainnodeengine_api" => {
                self.blockchainnodeengine_api().delete_resource(resource_name, id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }
}

/// Factory function to create a provider instance
///
/// This is the entry point called by Hemmer when loading the provider as a dynamic library.
#[no_mangle]
pub extern "C" fn create_provider() -> *mut dyn ProviderExecutor {
    match GcpProvider::new() {
        Ok(provider) => Box::into_raw(Box::new(provider)) as *mut dyn ProviderExecutor,
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
        // let provider = GcpProvider::new();
        // assert!(provider.is_ok());
    }
}
