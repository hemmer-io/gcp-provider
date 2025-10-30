//! Gcp Provider for Hemmer
//!
//! Auto-generated unified provider from gcp SDK version v1
//!
//! This provider includes multiple services:
//! - factchecktools_api
//! - areainsights_api
//! - billingbudgets_api
//! - billingbudgets_api
//! - dataflow_api
//! - playdeveloperreporting_api
//! - playdeveloperreporting_api
//! - iamcredentials_api
//! - privateca_api
//! - privateca_api
//! - solar_api
//! - cloudfunctions_api
//! - cloudfunctions_api
//! - cloudfunctions_api
//! - cloudfunctions_api
//! - cloudfunctions_api
//! - essentialcontacts_api
//! - orgpolicy_api
//! - remotebuildexecution_api
//! - remotebuildexecution_api
//! - remotebuildexecution_api
//! - urlshortener_api
//! - workstations_api
//! - workstations_api
//! - webrisk_api
//! - keep_api
//! - datastream_api
//! - datastream_api
//! - servicenetworking_api
//! - servicenetworking_api
//! - binaryauthorization_api
//! - binaryauthorization_api
//! - area120tables_api
//! - connectors_api
//! - connectors_api
//! - apihub_api
//! - apigeeregistry_api
//! - beyondcorp_api
//! - beyondcorp_api
//! - toolresults_api
//! - toolresults_api
//! - clouderrorreporting_api
//! - civicinfo_api
//! - apigee_api
//! - mybusinessverifications_api
//! - developerconnect_api
//! - datapipelines_api
//! - firebasedynamiclinks_api
//! - runtimeconfig_api
//! - runtimeconfig_api
//! - storagebatchoperations_api
//! - tracing_api
//! - adexchangebuyer2_api
//! - saasservicemgmt_api
//! - proximitybeacon_api
//! - resourcesettings_api
//! - datalineage_api
//! - libraryagent_api
//! - acmedns_api
//! - publicca_api
//! - publicca_api
//! - publicca_api
//! - networkservices_api
//! - networkservices_api
//! - netapp_api
//! - netapp_api
//! - aiplatform_api
//! - aiplatform_api
//! - pagespeedonline_api
//! - pagespeedonline_api
//! - pagespeedonline_api
//! - pagespeedonline_api
//! - tasks_api
//! - sqladmin_api
//! - sqladmin_api
//! - discoveryengine_api
//! - discoveryengine_api
//! - discoveryengine_api
//! - gkeonprem_api
//! - cloudlocationfinder_api
//! - cloudlocationfinder_api
//! - mybusinesslodging_api
//! - mybusinessaccountmanagement_api
//! - searchads360_api
//! - alertcenter_api
//! - servicemanagement_api
//! - smartdevicemanagement_api
//! - domains_api
//! - domains_api
//! - domains_api
//! - calendar_api
//! - vpcaccess_api
//! - vpcaccess_api
//! - sheets_api
//! - looker_api
//! - forms_api
//! - cloudiot_api
//! - genomics_api
//! - genomics_api
//! - genomics_api
//! - pollen_api
//! - playgrouping_api
//! - oslogin_api
//! - oslogin_api
//! - oslogin_api
//! - gamesmanagement_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - gkehub_api
//! - securityposture_api
//! - networkmanagement_api
//! - networkmanagement_api
//! - securitycenter_api
//! - securitycenter_api
//! - securitycenter_api
//! - securitycenter_api
//! - securitycenter_api
//! - datamanager_api
//! - chat_api
//! - apphub_api
//! - apphub_api
//! - dialogflow_api
//! - dialogflow_api
//! - dialogflow_api
//! - dialogflow_api
//! - dialogflow_api
//! - rapidmigrationassessment_api
//! - documentai_api
//! - documentai_api
//! - documentai_api
//! - artifactregistry_api
//! - artifactregistry_api
//! - artifactregistry_api
//! - qpxexpress_api
//! - analyticsdata_api
//! - analyticsdata_api
//! - file_api
//! - file_api
//! - cloudresourcemanager_api
//! - cloudresourcemanager_api
//! - cloudresourcemanager_api
//! - cloudresourcemanager_api
//! - cloudresourcemanager_api
//! - css_api
//! - walletobjects_api
//! - readerrevenuesubscriptionlinking_api
//! - youtubereporting_api
//! - firestore_api
//! - firestore_api
//! - firestore_api
//! - config_api
//! - translate_api
//! - translate_api
//! - translate_api
//! - redis_api
//! - redis_api
//! - meet_api
//! - dataplex_api
//! - analyticshub_api
//! - analyticshub_api
//! - playintegrity_api
//! - cloudkms_api
//! - fusiontables_api
//! - fusiontables_api
//! - accessapproval_api
//! - checks_api
//! - cloudprivatecatalog_api
//! - addressvalidation_api
//! - chromewebstore_api
//! - chromewebstore_api
//! - websecurityscanner_api
//! - websecurityscanner_api
//! - websecurityscanner_api
//! - places_api
//! - recaptchaenterprise_api
//! - parallelstore_api
//! - parallelstore_api
//! - contactcenterinsights_api
//! - recommendationengine_api
//! - cloudidentity_api
//! - cloudidentity_api
//! - healthcare_api
//! - healthcare_api
//! - healthcare_api
//! - healthcare_api
//! - composer_api
//! - composer_api
//! - searchconsole_api
//! - appengine_api
//! - appengine_api
//! - appengine_api
//! - appengine_api
//! - appengine_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - dfareporting_api
//! - apikeys_api
//! - bigqueryconnection_api
//! - bigqueryconnection_api
//! - firebasedataconnect_api
//! - firebasedataconnect_api
//! - commentanalyzer_api
//! - firebaseapphosting_api
//! - firebaseapphosting_api
//! - doubleclickbidmanager_api
//! - doubleclickbidmanager_api
//! - doubleclickbidmanager_api
//! - fcmdata_api
//! - firebasedatabase_api
//! - ideahub_api
//! - ideahub_api
//! - firebaseremoteconfig_api
//! - prod_tt_sasportal_api
//! - firebaseappdistribution_api
//! - firebaseappdistribution_api
//! - deploymentmanager_api
//! - deploymentmanager_api
//! - deploymentmanager_api
//! - datafusion_api
//! - datafusion_api
//! - tagmanager_api
//! - tagmanager_api
//! - discovery_api
//! - storagetransfer_api
//! - oauth2_api
//! - oauth2_api
//! - managedkafka_api
//! - admin_api
//! - admin_api
//! - admin_api
//! - bigqueryreservation_api
//! - bigqueryreservation_api
//! - bigqueryreservation_api
//! - vmmigration_api
//! - vmmigration_api
//! - contactcenteraiplatform_api
//! - doubleclicksearch_api
//! - backupdr_api
//! - ondemandscanning_api
//! - ondemandscanning_api
//! - cloudbuild_api
//! - cloudbuild_api
//! - cloudbuild_api
//! - cloudbuild_api
//! - cloudbuild_api
//! - policyanalyzer_api
//! - policyanalyzer_api
//! - workflows_api
//! - workflows_api
//! - secretmanager_api
//! - secretmanager_api
//! - secretmanager_api
//! - transcoder_api
//! - transcoder_api
//! - accesscontextmanager_api
//! - accesscontextmanager_api
//! - content_api
//! - content_api
//! - content_api
//! - acceleratedmobilepageurl_api
//! - androidpublisher_api
//! - androidpublisher_api
//! - androidpublisher_api
//! - digitalassetlinks_api
//! - classroom_api
//! - chromeuxreport_api
//! - admob_api
//! - admob_api
//! - gameservices_api
//! - gameservices_api
//! - language_api
//! - language_api
//! - language_api
//! - language_api
//! - retail_api
//! - retail_api
//! - retail_api
//! - bigquery_api
//! - driveactivity_api
//! - serviceuser_api
//! - homegraph_api
//! - vision_api
//! - vision_api
//! - vision_api
//! - firebaseappcheck_api
//! - firebaseappcheck_api
//! - appsactivity_api
//! - cloudsearch_api
//! - books_api
//! - docs_api
//! - firebasehosting_api
//! - firebasehosting_api
//! - youtube_api
//! - observability_api
//! - adsensehost_api
//! - lifesciences_api
//! - adexchangebuyer_api
//! - adexchangebuyer_api
//! - adexchangebuyer_api
//! - streetviewpublish_api
//! - advisorynotifications_api
//! - sourcerepo_api
//! - vectortile_api
//! - paymentsresellersubscription_api
//! - airquality_api
//! - localservices_api
//! - adexchangeseller_api
//! - adexchangeseller_api
//! - adexchangeseller_api
//! - servicebroker_api
//! - servicebroker_api
//! - servicebroker_api
//! - speech_api
//! - speech_api
//! - speech_api
//! - speech_api
//! - speech_api
//! - script_api
//! - realtimebidding_api
//! - realtimebidding_api
//! - chromepolicy_api
//! - storage_api
//! - storage_api
//! - storage_api
//! - slides_api
//! - reseller_api
//! - mybusinessbusinessinformation_api
//! - androidenterprise_api
//! - ids_api
//! - adsense_api
//! - adsense_api
//! - adsense_api
//! - videointelligence_api
//! - videointelligence_api
//! - videointelligence_api
//! - videointelligence_api
//! - videointelligence_api
//! - datamigration_api
//! - datamigration_api
//! - gkebackup_api
//! - adsenseplatform_api
//! - adsenseplatform_api
//! - integrations_api
//! - integrations_api
//! - customsearch_api
//! - kgsearch_api
//! - testing_api
//! - container_api
//! - container_api
//! - datastore_api
//! - datastore_api
//! - datastore_api
//! - playmoviespartner_api
//! - memcache_api
//! - memcache_api
//! - blockchainnodeengine_api
//! - cloudtrace_api
//! - cloudtrace_api
//! - cloudtrace_api
//! - iap_api
//! - iap_api
//! - playcustomapp_api
//! - dataportability_api
//! - dataportability_api
//! - mybusinessbusinesscalls_api
//! - policysimulator_api
//! - policysimulator_api
//! - policysimulator_api
//! - policysimulator_api
//! - firebase_api
//! - iam_api
//! - iam_api
//! - iam_api
//! - bigtableadmin_api
//! - bigtableadmin_api
//! - servicedirectory_api
//! - servicedirectory_api
//! - recommender_api
//! - recommender_api
//! - trafficdirector_api
//! - trafficdirector_api
//! - drivelabels_api
//! - drivelabels_api
//! - cloudasset_api
//! - cloudasset_api
//! - cloudasset_api
//! - cloudasset_api
//! - cloudasset_api
//! - cloudasset_api
//! - cloudbilling_api
//! - cloudbilling_api
//! - webmasters_api
//! - compute_api
//! - compute_api
//! - compute_api
//! - groupssettings_api
//! - workloadmanager_api
//! - abusiveexperiencereport_api
//! - displayvideo_api
//! - displayvideo_api
//! - displayvideo_api
//! - displayvideo_api
//! - displayvideo_api
//! - displayvideo_api
//! - displayvideo_api
//! - ml_api
//! - networkconnectivity_api
//! - networkconnectivity_api
//! - batch_api
//! - webfonts_api
//! - partners_api
//! - cloudprofiler_api
//! - eventarc_api
//! - eventarc_api
//! - contentwarehouse_api
//! - siteverification_api
//! - analyticsadmin_api
//! - analyticsadmin_api
//! - verifiedaccess_api
//! - verifiedaccess_api
//! - safebrowsing_api
//! - safebrowsing_api
//! - people_api
//! - fcm_api
//! - firebasestorage_api
//! - manufacturers_api
//! - appstate_api
//! - servicecontrol_api
//! - servicecontrol_api
//! - datalabeling_api
//! - dlp_api
//! - mybusinessnotifications_api
//! - containeranalysis_api
//! - containeranalysis_api
//! - containeranalysis_api
//! - mybusinessqanda_api
//! - cloudtasks_api
//! - cloudtasks_api
//! - cloudtasks_api
//! - apim_api
//! - surveys_api
//! - pubsub_api
//! - pubsub_api
//! - pubsub_api
//! - texttospeech_api
//! - texttospeech_api
//! - marketingplatformadmin_api
//! - cloudprivatecatalogproducer_api
//! - sasportal_api
//! - spanner_api
//! - clouddeploy_api
//! - biglake_api
//! - serviceusage_api
//! - serviceusage_api
//! - chromemanagement_api
//! - fitness_api
//! - plusdomains_api
//! - androiddeviceprovisioning_api
//! - securesourcemanager_api
//! - kmsinventory_api
//! - cloudcontrolspartner_api
//! - cloudcontrolspartner_api
//! - bigquerydatapolicy_api
//! - bigquerydatapolicy_api
//! - licensing_api
//! - workspaceevents_api
//! - replicapoolupdater_api
//! - indexing_api
//! - alloydb_api
//! - alloydb_api
//! - alloydb_api
//! - spectrum_api
//! - firebaseml_api
//! - firebaseml_api
//! - firebaseml_api
//! - tpu_api
//! - tpu_api
//! - tpu_api
//! - tpu_api
//! - domainsrdap_api
//! - playablelocations_api
//! - groupsmigration_api
//! - androidmanagement_api
//! - run_api
//! - run_api
//! - run_api
//! - run_api
//! - cloudcommerceprocurement_api
//! - jobs_api
//! - jobs_api
//! - jobs_api
//! - jobs_api
//! - networksecurity_api
//! - networksecurity_api
//! - clouddebugger_api
//! - monitoring_api
//! - monitoring_api
//! - youtubeanalytics_api
//! - youtubeanalytics_api
//! - youtubeanalytics_api
//! - policytroubleshooter_api
//! - policytroubleshooter_api
//! - assuredworkloads_api
//! - assuredworkloads_api
//! - versionhistory_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - merchantapi_api
//! - gamesconfiguration_api
//! - cloudscheduler_api
//! - cloudscheduler_api
//! - vmwareengine_api
//! - drive_api
//! - drive_api
//! - dns_api
//! - dns_api
//! - dns_api
//! - dns_api
//! - analyticsreporting_api
//! - blogger_api
//! - blogger_api
//! - replicapool_api
//! - mybusinessplaceactions_api
//! - consumersurveys_api
//! - games_api
//! - gmailpostmastertools_api
//! - gmailpostmastertools_api
//! - oracledatabase_api
//! - dataform_api
//! - dataform_api
//! - authorizedbuyersmarketplace_api
//! - authorizedbuyersmarketplace_api
//! - authorizedbuyersmarketplace_api
//! - serviceconsumermanagement_api
//! - serviceconsumermanagement_api
//! - notebooks_api
//! - notebooks_api
//! - parametermanager_api
//! - logging_api
//! - logging_api
//! - adexperiencereport_api
//! - bigquerydatatransfer_api
//! - datacatalog_api
//! - datacatalog_api
//! - cloudshell_api
//! - cloudshell_api
//! - cloudchannel_api
//! - osconfig_api
//! - osconfig_api
//! - osconfig_api
//! - osconfig_api
//! - osconfig_api
//! - migrationcenter_api
//! - migrationcenter_api
//! - gmail_api
//! - workflowexecutions_api
//! - workflowexecutions_api
//! - apigateway_api
//! - apigateway_api
//! - apigateway_api
//! - apigateway_api
//! - businessprofileperformance_api
//! - metastore_api
//! - metastore_api
//! - metastore_api
//! - metastore_api
//! - metastore_api
//! - metastore_api
//! - pubsublite_api
//! - vault_api
//! - cloudsupport_api
//! - cloudsupport_api
//! - analytics_api
//! - analytics_api
//! - sts_api
//! - sts_api
//! - certificatemanager_api
//! - poly_api
//! - firebaserules_api
//! - plus_api
//! - dataproc_api
//! - dataproc_api
//! - mirror_api
//! - baremetalsolution_api
//! - baremetalsolution_api
//! - baremetalsolution_api
//! - managedidentities_api
//! - managedidentities_api
//! - managedidentities_api
//! - sql_api


pub mod factchecktools_api;
pub mod areainsights_api;
pub mod billingbudgets_api;
pub mod billingbudgets_api;
pub mod dataflow_api;
pub mod playdeveloperreporting_api;
pub mod playdeveloperreporting_api;
pub mod iamcredentials_api;
pub mod privateca_api;
pub mod privateca_api;
pub mod solar_api;
pub mod cloudfunctions_api;
pub mod cloudfunctions_api;
pub mod cloudfunctions_api;
pub mod cloudfunctions_api;
pub mod cloudfunctions_api;
pub mod essentialcontacts_api;
pub mod orgpolicy_api;
pub mod remotebuildexecution_api;
pub mod remotebuildexecution_api;
pub mod remotebuildexecution_api;
pub mod urlshortener_api;
pub mod workstations_api;
pub mod workstations_api;
pub mod webrisk_api;
pub mod keep_api;
pub mod datastream_api;
pub mod datastream_api;
pub mod servicenetworking_api;
pub mod servicenetworking_api;
pub mod binaryauthorization_api;
pub mod binaryauthorization_api;
pub mod area120tables_api;
pub mod connectors_api;
pub mod connectors_api;
pub mod apihub_api;
pub mod apigeeregistry_api;
pub mod beyondcorp_api;
pub mod beyondcorp_api;
pub mod toolresults_api;
pub mod toolresults_api;
pub mod clouderrorreporting_api;
pub mod civicinfo_api;
pub mod apigee_api;
pub mod mybusinessverifications_api;
pub mod developerconnect_api;
pub mod datapipelines_api;
pub mod firebasedynamiclinks_api;
pub mod runtimeconfig_api;
pub mod runtimeconfig_api;
pub mod storagebatchoperations_api;
pub mod tracing_api;
pub mod adexchangebuyer2_api;
pub mod saasservicemgmt_api;
pub mod proximitybeacon_api;
pub mod resourcesettings_api;
pub mod datalineage_api;
pub mod libraryagent_api;
pub mod acmedns_api;
pub mod publicca_api;
pub mod publicca_api;
pub mod publicca_api;
pub mod networkservices_api;
pub mod networkservices_api;
pub mod netapp_api;
pub mod netapp_api;
pub mod aiplatform_api;
pub mod aiplatform_api;
pub mod pagespeedonline_api;
pub mod pagespeedonline_api;
pub mod pagespeedonline_api;
pub mod pagespeedonline_api;
pub mod tasks_api;
pub mod sqladmin_api;
pub mod sqladmin_api;
pub mod discoveryengine_api;
pub mod discoveryengine_api;
pub mod discoveryengine_api;
pub mod gkeonprem_api;
pub mod cloudlocationfinder_api;
pub mod cloudlocationfinder_api;
pub mod mybusinesslodging_api;
pub mod mybusinessaccountmanagement_api;
pub mod searchads360_api;
pub mod alertcenter_api;
pub mod servicemanagement_api;
pub mod smartdevicemanagement_api;
pub mod domains_api;
pub mod domains_api;
pub mod domains_api;
pub mod calendar_api;
pub mod vpcaccess_api;
pub mod vpcaccess_api;
pub mod sheets_api;
pub mod looker_api;
pub mod forms_api;
pub mod cloudiot_api;
pub mod genomics_api;
pub mod genomics_api;
pub mod genomics_api;
pub mod pollen_api;
pub mod playgrouping_api;
pub mod oslogin_api;
pub mod oslogin_api;
pub mod oslogin_api;
pub mod gamesmanagement_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod gkehub_api;
pub mod securityposture_api;
pub mod networkmanagement_api;
pub mod networkmanagement_api;
pub mod securitycenter_api;
pub mod securitycenter_api;
pub mod securitycenter_api;
pub mod securitycenter_api;
pub mod securitycenter_api;
pub mod datamanager_api;
pub mod chat_api;
pub mod apphub_api;
pub mod apphub_api;
pub mod dialogflow_api;
pub mod dialogflow_api;
pub mod dialogflow_api;
pub mod dialogflow_api;
pub mod dialogflow_api;
pub mod rapidmigrationassessment_api;
pub mod documentai_api;
pub mod documentai_api;
pub mod documentai_api;
pub mod artifactregistry_api;
pub mod artifactregistry_api;
pub mod artifactregistry_api;
pub mod qpxexpress_api;
pub mod analyticsdata_api;
pub mod analyticsdata_api;
pub mod file_api;
pub mod file_api;
pub mod cloudresourcemanager_api;
pub mod cloudresourcemanager_api;
pub mod cloudresourcemanager_api;
pub mod cloudresourcemanager_api;
pub mod cloudresourcemanager_api;
pub mod css_api;
pub mod walletobjects_api;
pub mod readerrevenuesubscriptionlinking_api;
pub mod youtubereporting_api;
pub mod firestore_api;
pub mod firestore_api;
pub mod firestore_api;
pub mod config_api;
pub mod translate_api;
pub mod translate_api;
pub mod translate_api;
pub mod redis_api;
pub mod redis_api;
pub mod meet_api;
pub mod dataplex_api;
pub mod analyticshub_api;
pub mod analyticshub_api;
pub mod playintegrity_api;
pub mod cloudkms_api;
pub mod fusiontables_api;
pub mod fusiontables_api;
pub mod accessapproval_api;
pub mod checks_api;
pub mod cloudprivatecatalog_api;
pub mod addressvalidation_api;
pub mod chromewebstore_api;
pub mod chromewebstore_api;
pub mod websecurityscanner_api;
pub mod websecurityscanner_api;
pub mod websecurityscanner_api;
pub mod places_api;
pub mod recaptchaenterprise_api;
pub mod parallelstore_api;
pub mod parallelstore_api;
pub mod contactcenterinsights_api;
pub mod recommendationengine_api;
pub mod cloudidentity_api;
pub mod cloudidentity_api;
pub mod healthcare_api;
pub mod healthcare_api;
pub mod healthcare_api;
pub mod healthcare_api;
pub mod composer_api;
pub mod composer_api;
pub mod searchconsole_api;
pub mod appengine_api;
pub mod appengine_api;
pub mod appengine_api;
pub mod appengine_api;
pub mod appengine_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod dfareporting_api;
pub mod apikeys_api;
pub mod bigqueryconnection_api;
pub mod bigqueryconnection_api;
pub mod firebasedataconnect_api;
pub mod firebasedataconnect_api;
pub mod commentanalyzer_api;
pub mod firebaseapphosting_api;
pub mod firebaseapphosting_api;
pub mod doubleclickbidmanager_api;
pub mod doubleclickbidmanager_api;
pub mod doubleclickbidmanager_api;
pub mod fcmdata_api;
pub mod firebasedatabase_api;
pub mod ideahub_api;
pub mod ideahub_api;
pub mod firebaseremoteconfig_api;
pub mod prod_tt_sasportal_api;
pub mod firebaseappdistribution_api;
pub mod firebaseappdistribution_api;
pub mod deploymentmanager_api;
pub mod deploymentmanager_api;
pub mod deploymentmanager_api;
pub mod datafusion_api;
pub mod datafusion_api;
pub mod tagmanager_api;
pub mod tagmanager_api;
pub mod discovery_api;
pub mod storagetransfer_api;
pub mod oauth2_api;
pub mod oauth2_api;
pub mod managedkafka_api;
pub mod admin_api;
pub mod admin_api;
pub mod admin_api;
pub mod bigqueryreservation_api;
pub mod bigqueryreservation_api;
pub mod bigqueryreservation_api;
pub mod vmmigration_api;
pub mod vmmigration_api;
pub mod contactcenteraiplatform_api;
pub mod doubleclicksearch_api;
pub mod backupdr_api;
pub mod ondemandscanning_api;
pub mod ondemandscanning_api;
pub mod cloudbuild_api;
pub mod cloudbuild_api;
pub mod cloudbuild_api;
pub mod cloudbuild_api;
pub mod cloudbuild_api;
pub mod policyanalyzer_api;
pub mod policyanalyzer_api;
pub mod workflows_api;
pub mod workflows_api;
pub mod secretmanager_api;
pub mod secretmanager_api;
pub mod secretmanager_api;
pub mod transcoder_api;
pub mod transcoder_api;
pub mod accesscontextmanager_api;
pub mod accesscontextmanager_api;
pub mod content_api;
pub mod content_api;
pub mod content_api;
pub mod acceleratedmobilepageurl_api;
pub mod androidpublisher_api;
pub mod androidpublisher_api;
pub mod androidpublisher_api;
pub mod digitalassetlinks_api;
pub mod classroom_api;
pub mod chromeuxreport_api;
pub mod admob_api;
pub mod admob_api;
pub mod gameservices_api;
pub mod gameservices_api;
pub mod language_api;
pub mod language_api;
pub mod language_api;
pub mod language_api;
pub mod retail_api;
pub mod retail_api;
pub mod retail_api;
pub mod bigquery_api;
pub mod driveactivity_api;
pub mod serviceuser_api;
pub mod homegraph_api;
pub mod vision_api;
pub mod vision_api;
pub mod vision_api;
pub mod firebaseappcheck_api;
pub mod firebaseappcheck_api;
pub mod appsactivity_api;
pub mod cloudsearch_api;
pub mod books_api;
pub mod docs_api;
pub mod firebasehosting_api;
pub mod firebasehosting_api;
pub mod youtube_api;
pub mod observability_api;
pub mod adsensehost_api;
pub mod lifesciences_api;
pub mod adexchangebuyer_api;
pub mod adexchangebuyer_api;
pub mod adexchangebuyer_api;
pub mod streetviewpublish_api;
pub mod advisorynotifications_api;
pub mod sourcerepo_api;
pub mod vectortile_api;
pub mod paymentsresellersubscription_api;
pub mod airquality_api;
pub mod localservices_api;
pub mod adexchangeseller_api;
pub mod adexchangeseller_api;
pub mod adexchangeseller_api;
pub mod servicebroker_api;
pub mod servicebroker_api;
pub mod servicebroker_api;
pub mod speech_api;
pub mod speech_api;
pub mod speech_api;
pub mod speech_api;
pub mod speech_api;
pub mod script_api;
pub mod realtimebidding_api;
pub mod realtimebidding_api;
pub mod chromepolicy_api;
pub mod storage_api;
pub mod storage_api;
pub mod storage_api;
pub mod slides_api;
pub mod reseller_api;
pub mod mybusinessbusinessinformation_api;
pub mod androidenterprise_api;
pub mod ids_api;
pub mod adsense_api;
pub mod adsense_api;
pub mod adsense_api;
pub mod videointelligence_api;
pub mod videointelligence_api;
pub mod videointelligence_api;
pub mod videointelligence_api;
pub mod videointelligence_api;
pub mod datamigration_api;
pub mod datamigration_api;
pub mod gkebackup_api;
pub mod adsenseplatform_api;
pub mod adsenseplatform_api;
pub mod integrations_api;
pub mod integrations_api;
pub mod customsearch_api;
pub mod kgsearch_api;
pub mod testing_api;
pub mod container_api;
pub mod container_api;
pub mod datastore_api;
pub mod datastore_api;
pub mod datastore_api;
pub mod playmoviespartner_api;
pub mod memcache_api;
pub mod memcache_api;
pub mod blockchainnodeengine_api;
pub mod cloudtrace_api;
pub mod cloudtrace_api;
pub mod cloudtrace_api;
pub mod iap_api;
pub mod iap_api;
pub mod playcustomapp_api;
pub mod dataportability_api;
pub mod dataportability_api;
pub mod mybusinessbusinesscalls_api;
pub mod policysimulator_api;
pub mod policysimulator_api;
pub mod policysimulator_api;
pub mod policysimulator_api;
pub mod firebase_api;
pub mod iam_api;
pub mod iam_api;
pub mod iam_api;
pub mod bigtableadmin_api;
pub mod bigtableadmin_api;
pub mod servicedirectory_api;
pub mod servicedirectory_api;
pub mod recommender_api;
pub mod recommender_api;
pub mod trafficdirector_api;
pub mod trafficdirector_api;
pub mod drivelabels_api;
pub mod drivelabels_api;
pub mod cloudasset_api;
pub mod cloudasset_api;
pub mod cloudasset_api;
pub mod cloudasset_api;
pub mod cloudasset_api;
pub mod cloudasset_api;
pub mod cloudbilling_api;
pub mod cloudbilling_api;
pub mod webmasters_api;
pub mod compute_api;
pub mod compute_api;
pub mod compute_api;
pub mod groupssettings_api;
pub mod workloadmanager_api;
pub mod abusiveexperiencereport_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod displayvideo_api;
pub mod ml_api;
pub mod networkconnectivity_api;
pub mod networkconnectivity_api;
pub mod batch_api;
pub mod webfonts_api;
pub mod partners_api;
pub mod cloudprofiler_api;
pub mod eventarc_api;
pub mod eventarc_api;
pub mod contentwarehouse_api;
pub mod siteverification_api;
pub mod analyticsadmin_api;
pub mod analyticsadmin_api;
pub mod verifiedaccess_api;
pub mod verifiedaccess_api;
pub mod safebrowsing_api;
pub mod safebrowsing_api;
pub mod people_api;
pub mod fcm_api;
pub mod firebasestorage_api;
pub mod manufacturers_api;
pub mod appstate_api;
pub mod servicecontrol_api;
pub mod servicecontrol_api;
pub mod datalabeling_api;
pub mod dlp_api;
pub mod mybusinessnotifications_api;
pub mod containeranalysis_api;
pub mod containeranalysis_api;
pub mod containeranalysis_api;
pub mod mybusinessqanda_api;
pub mod cloudtasks_api;
pub mod cloudtasks_api;
pub mod cloudtasks_api;
pub mod apim_api;
pub mod surveys_api;
pub mod pubsub_api;
pub mod pubsub_api;
pub mod pubsub_api;
pub mod texttospeech_api;
pub mod texttospeech_api;
pub mod marketingplatformadmin_api;
pub mod cloudprivatecatalogproducer_api;
pub mod sasportal_api;
pub mod spanner_api;
pub mod clouddeploy_api;
pub mod biglake_api;
pub mod serviceusage_api;
pub mod serviceusage_api;
pub mod chromemanagement_api;
pub mod fitness_api;
pub mod plusdomains_api;
pub mod androiddeviceprovisioning_api;
pub mod securesourcemanager_api;
pub mod kmsinventory_api;
pub mod cloudcontrolspartner_api;
pub mod cloudcontrolspartner_api;
pub mod bigquerydatapolicy_api;
pub mod bigquerydatapolicy_api;
pub mod licensing_api;
pub mod workspaceevents_api;
pub mod replicapoolupdater_api;
pub mod indexing_api;
pub mod alloydb_api;
pub mod alloydb_api;
pub mod alloydb_api;
pub mod spectrum_api;
pub mod firebaseml_api;
pub mod firebaseml_api;
pub mod firebaseml_api;
pub mod tpu_api;
pub mod tpu_api;
pub mod tpu_api;
pub mod tpu_api;
pub mod domainsrdap_api;
pub mod playablelocations_api;
pub mod groupsmigration_api;
pub mod androidmanagement_api;
pub mod run_api;
pub mod run_api;
pub mod run_api;
pub mod run_api;
pub mod cloudcommerceprocurement_api;
pub mod jobs_api;
pub mod jobs_api;
pub mod jobs_api;
pub mod jobs_api;
pub mod networksecurity_api;
pub mod networksecurity_api;
pub mod clouddebugger_api;
pub mod monitoring_api;
pub mod monitoring_api;
pub mod youtubeanalytics_api;
pub mod youtubeanalytics_api;
pub mod youtubeanalytics_api;
pub mod policytroubleshooter_api;
pub mod policytroubleshooter_api;
pub mod assuredworkloads_api;
pub mod assuredworkloads_api;
pub mod versionhistory_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod merchantapi_api;
pub mod gamesconfiguration_api;
pub mod cloudscheduler_api;
pub mod cloudscheduler_api;
pub mod vmwareengine_api;
pub mod drive_api;
pub mod drive_api;
pub mod dns_api;
pub mod dns_api;
pub mod dns_api;
pub mod dns_api;
pub mod analyticsreporting_api;
pub mod blogger_api;
pub mod blogger_api;
pub mod replicapool_api;
pub mod mybusinessplaceactions_api;
pub mod consumersurveys_api;
pub mod games_api;
pub mod gmailpostmastertools_api;
pub mod gmailpostmastertools_api;
pub mod oracledatabase_api;
pub mod dataform_api;
pub mod dataform_api;
pub mod authorizedbuyersmarketplace_api;
pub mod authorizedbuyersmarketplace_api;
pub mod authorizedbuyersmarketplace_api;
pub mod serviceconsumermanagement_api;
pub mod serviceconsumermanagement_api;
pub mod notebooks_api;
pub mod notebooks_api;
pub mod parametermanager_api;
pub mod logging_api;
pub mod logging_api;
pub mod adexperiencereport_api;
pub mod bigquerydatatransfer_api;
pub mod datacatalog_api;
pub mod datacatalog_api;
pub mod cloudshell_api;
pub mod cloudshell_api;
pub mod cloudchannel_api;
pub mod osconfig_api;
pub mod osconfig_api;
pub mod osconfig_api;
pub mod osconfig_api;
pub mod osconfig_api;
pub mod migrationcenter_api;
pub mod migrationcenter_api;
pub mod gmail_api;
pub mod workflowexecutions_api;
pub mod workflowexecutions_api;
pub mod apigateway_api;
pub mod apigateway_api;
pub mod apigateway_api;
pub mod apigateway_api;
pub mod businessprofileperformance_api;
pub mod metastore_api;
pub mod metastore_api;
pub mod metastore_api;
pub mod metastore_api;
pub mod metastore_api;
pub mod metastore_api;
pub mod pubsublite_api;
pub mod vault_api;
pub mod cloudsupport_api;
pub mod cloudsupport_api;
pub mod analytics_api;
pub mod analytics_api;
pub mod sts_api;
pub mod sts_api;
pub mod certificatemanager_api;
pub mod poly_api;
pub mod firebaserules_api;
pub mod plus_api;
pub mod dataproc_api;
pub mod dataproc_api;
pub mod mirror_api;
pub mod baremetalsolution_api;
pub mod baremetalsolution_api;
pub mod baremetalsolution_api;
pub mod managedidentities_api;
pub mod managedidentities_api;
pub mod managedidentities_api;
pub mod sql_api;


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
    // factchecktools_api_client: google_factchecktools_api::Client,
    // areainsights_api_client: google_areainsights_api::Client,
    // billingbudgets_api_client: google_billingbudgets_api::Client,
    // billingbudgets_api_client: google_billingbudgets_api::Client,
    // dataflow_api_client: google_dataflow_api::Client,
    // playdeveloperreporting_api_client: google_playdeveloperreporting_api::Client,
    // playdeveloperreporting_api_client: google_playdeveloperreporting_api::Client,
    // iamcredentials_api_client: google_iamcredentials_api::Client,
    // privateca_api_client: google_privateca_api::Client,
    // privateca_api_client: google_privateca_api::Client,
    // solar_api_client: google_solar_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // cloudfunctions_api_client: google_cloudfunctions_api::Client,
    // essentialcontacts_api_client: google_essentialcontacts_api::Client,
    // orgpolicy_api_client: google_orgpolicy_api::Client,
    // remotebuildexecution_api_client: google_remotebuildexecution_api::Client,
    // remotebuildexecution_api_client: google_remotebuildexecution_api::Client,
    // remotebuildexecution_api_client: google_remotebuildexecution_api::Client,
    // urlshortener_api_client: google_urlshortener_api::Client,
    // workstations_api_client: google_workstations_api::Client,
    // workstations_api_client: google_workstations_api::Client,
    // webrisk_api_client: google_webrisk_api::Client,
    // keep_api_client: google_keep_api::Client,
    // datastream_api_client: google_datastream_api::Client,
    // datastream_api_client: google_datastream_api::Client,
    // servicenetworking_api_client: google_servicenetworking_api::Client,
    // servicenetworking_api_client: google_servicenetworking_api::Client,
    // binaryauthorization_api_client: google_binaryauthorization_api::Client,
    // binaryauthorization_api_client: google_binaryauthorization_api::Client,
    // area120tables_api_client: google_area120tables_api::Client,
    // connectors_api_client: google_connectors_api::Client,
    // connectors_api_client: google_connectors_api::Client,
    // apihub_api_client: google_apihub_api::Client,
    // apigeeregistry_api_client: google_apigeeregistry_api::Client,
    // beyondcorp_api_client: google_beyondcorp_api::Client,
    // beyondcorp_api_client: google_beyondcorp_api::Client,
    // toolresults_api_client: google_toolresults_api::Client,
    // toolresults_api_client: google_toolresults_api::Client,
    // clouderrorreporting_api_client: google_clouderrorreporting_api::Client,
    // civicinfo_api_client: google_civicinfo_api::Client,
    // apigee_api_client: google_apigee_api::Client,
    // mybusinessverifications_api_client: google_mybusinessverifications_api::Client,
    // developerconnect_api_client: google_developerconnect_api::Client,
    // datapipelines_api_client: google_datapipelines_api::Client,
    // firebasedynamiclinks_api_client: google_firebasedynamiclinks_api::Client,
    // runtimeconfig_api_client: google_runtimeconfig_api::Client,
    // runtimeconfig_api_client: google_runtimeconfig_api::Client,
    // storagebatchoperations_api_client: google_storagebatchoperations_api::Client,
    // tracing_api_client: google_tracing_api::Client,
    // adexchangebuyer2_api_client: google_adexchangebuyer2_api::Client,
    // saasservicemgmt_api_client: google_saasservicemgmt_api::Client,
    // proximitybeacon_api_client: google_proximitybeacon_api::Client,
    // resourcesettings_api_client: google_resourcesettings_api::Client,
    // datalineage_api_client: google_datalineage_api::Client,
    // libraryagent_api_client: google_libraryagent_api::Client,
    // acmedns_api_client: google_acmedns_api::Client,
    // publicca_api_client: google_publicca_api::Client,
    // publicca_api_client: google_publicca_api::Client,
    // publicca_api_client: google_publicca_api::Client,
    // networkservices_api_client: google_networkservices_api::Client,
    // networkservices_api_client: google_networkservices_api::Client,
    // netapp_api_client: google_netapp_api::Client,
    // netapp_api_client: google_netapp_api::Client,
    // aiplatform_api_client: google_aiplatform_api::Client,
    // aiplatform_api_client: google_aiplatform_api::Client,
    // pagespeedonline_api_client: google_pagespeedonline_api::Client,
    // pagespeedonline_api_client: google_pagespeedonline_api::Client,
    // pagespeedonline_api_client: google_pagespeedonline_api::Client,
    // pagespeedonline_api_client: google_pagespeedonline_api::Client,
    // tasks_api_client: google_tasks_api::Client,
    // sqladmin_api_client: google_sqladmin_api::Client,
    // sqladmin_api_client: google_sqladmin_api::Client,
    // discoveryengine_api_client: google_discoveryengine_api::Client,
    // discoveryengine_api_client: google_discoveryengine_api::Client,
    // discoveryengine_api_client: google_discoveryengine_api::Client,
    // gkeonprem_api_client: google_gkeonprem_api::Client,
    // cloudlocationfinder_api_client: google_cloudlocationfinder_api::Client,
    // cloudlocationfinder_api_client: google_cloudlocationfinder_api::Client,
    // mybusinesslodging_api_client: google_mybusinesslodging_api::Client,
    // mybusinessaccountmanagement_api_client: google_mybusinessaccountmanagement_api::Client,
    // searchads360_api_client: google_searchads360_api::Client,
    // alertcenter_api_client: google_alertcenter_api::Client,
    // servicemanagement_api_client: google_servicemanagement_api::Client,
    // smartdevicemanagement_api_client: google_smartdevicemanagement_api::Client,
    // domains_api_client: google_domains_api::Client,
    // domains_api_client: google_domains_api::Client,
    // domains_api_client: google_domains_api::Client,
    // calendar_api_client: google_calendar_api::Client,
    // vpcaccess_api_client: google_vpcaccess_api::Client,
    // vpcaccess_api_client: google_vpcaccess_api::Client,
    // sheets_api_client: google_sheets_api::Client,
    // looker_api_client: google_looker_api::Client,
    // forms_api_client: google_forms_api::Client,
    // cloudiot_api_client: google_cloudiot_api::Client,
    // genomics_api_client: google_genomics_api::Client,
    // genomics_api_client: google_genomics_api::Client,
    // genomics_api_client: google_genomics_api::Client,
    // pollen_api_client: google_pollen_api::Client,
    // playgrouping_api_client: google_playgrouping_api::Client,
    // oslogin_api_client: google_oslogin_api::Client,
    // oslogin_api_client: google_oslogin_api::Client,
    // oslogin_api_client: google_oslogin_api::Client,
    // gamesmanagement_api_client: google_gamesmanagement_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // gkehub_api_client: google_gkehub_api::Client,
    // securityposture_api_client: google_securityposture_api::Client,
    // networkmanagement_api_client: google_networkmanagement_api::Client,
    // networkmanagement_api_client: google_networkmanagement_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // securitycenter_api_client: google_securitycenter_api::Client,
    // datamanager_api_client: google_datamanager_api::Client,
    // chat_api_client: google_chat_api::Client,
    // apphub_api_client: google_apphub_api::Client,
    // apphub_api_client: google_apphub_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // dialogflow_api_client: google_dialogflow_api::Client,
    // rapidmigrationassessment_api_client: google_rapidmigrationassessment_api::Client,
    // documentai_api_client: google_documentai_api::Client,
    // documentai_api_client: google_documentai_api::Client,
    // documentai_api_client: google_documentai_api::Client,
    // artifactregistry_api_client: google_artifactregistry_api::Client,
    // artifactregistry_api_client: google_artifactregistry_api::Client,
    // artifactregistry_api_client: google_artifactregistry_api::Client,
    // qpxexpress_api_client: google_qpxexpress_api::Client,
    // analyticsdata_api_client: google_analyticsdata_api::Client,
    // analyticsdata_api_client: google_analyticsdata_api::Client,
    // file_api_client: google_file_api::Client,
    // file_api_client: google_file_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // cloudresourcemanager_api_client: google_cloudresourcemanager_api::Client,
    // css_api_client: google_css_api::Client,
    // walletobjects_api_client: google_walletobjects_api::Client,
    // readerrevenuesubscriptionlinking_api_client: google_readerrevenuesubscriptionlinking_api::Client,
    // youtubereporting_api_client: google_youtubereporting_api::Client,
    // firestore_api_client: google_firestore_api::Client,
    // firestore_api_client: google_firestore_api::Client,
    // firestore_api_client: google_firestore_api::Client,
    // config_api_client: google_config_api::Client,
    // translate_api_client: google_translate_api::Client,
    // translate_api_client: google_translate_api::Client,
    // translate_api_client: google_translate_api::Client,
    // redis_api_client: google_redis_api::Client,
    // redis_api_client: google_redis_api::Client,
    // meet_api_client: google_meet_api::Client,
    // dataplex_api_client: google_dataplex_api::Client,
    // analyticshub_api_client: google_analyticshub_api::Client,
    // analyticshub_api_client: google_analyticshub_api::Client,
    // playintegrity_api_client: google_playintegrity_api::Client,
    // cloudkms_api_client: google_cloudkms_api::Client,
    // fusiontables_api_client: google_fusiontables_api::Client,
    // fusiontables_api_client: google_fusiontables_api::Client,
    // accessapproval_api_client: google_accessapproval_api::Client,
    // checks_api_client: google_checks_api::Client,
    // cloudprivatecatalog_api_client: google_cloudprivatecatalog_api::Client,
    // addressvalidation_api_client: google_addressvalidation_api::Client,
    // chromewebstore_api_client: google_chromewebstore_api::Client,
    // chromewebstore_api_client: google_chromewebstore_api::Client,
    // websecurityscanner_api_client: google_websecurityscanner_api::Client,
    // websecurityscanner_api_client: google_websecurityscanner_api::Client,
    // websecurityscanner_api_client: google_websecurityscanner_api::Client,
    // places_api_client: google_places_api::Client,
    // recaptchaenterprise_api_client: google_recaptchaenterprise_api::Client,
    // parallelstore_api_client: google_parallelstore_api::Client,
    // parallelstore_api_client: google_parallelstore_api::Client,
    // contactcenterinsights_api_client: google_contactcenterinsights_api::Client,
    // recommendationengine_api_client: google_recommendationengine_api::Client,
    // cloudidentity_api_client: google_cloudidentity_api::Client,
    // cloudidentity_api_client: google_cloudidentity_api::Client,
    // healthcare_api_client: google_healthcare_api::Client,
    // healthcare_api_client: google_healthcare_api::Client,
    // healthcare_api_client: google_healthcare_api::Client,
    // healthcare_api_client: google_healthcare_api::Client,
    // composer_api_client: google_composer_api::Client,
    // composer_api_client: google_composer_api::Client,
    // searchconsole_api_client: google_searchconsole_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // appengine_api_client: google_appengine_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // dfareporting_api_client: google_dfareporting_api::Client,
    // apikeys_api_client: google_apikeys_api::Client,
    // bigqueryconnection_api_client: google_bigqueryconnection_api::Client,
    // bigqueryconnection_api_client: google_bigqueryconnection_api::Client,
    // firebasedataconnect_api_client: google_firebasedataconnect_api::Client,
    // firebasedataconnect_api_client: google_firebasedataconnect_api::Client,
    // commentanalyzer_api_client: google_commentanalyzer_api::Client,
    // firebaseapphosting_api_client: google_firebaseapphosting_api::Client,
    // firebaseapphosting_api_client: google_firebaseapphosting_api::Client,
    // doubleclickbidmanager_api_client: google_doubleclickbidmanager_api::Client,
    // doubleclickbidmanager_api_client: google_doubleclickbidmanager_api::Client,
    // doubleclickbidmanager_api_client: google_doubleclickbidmanager_api::Client,
    // fcmdata_api_client: google_fcmdata_api::Client,
    // firebasedatabase_api_client: google_firebasedatabase_api::Client,
    // ideahub_api_client: google_ideahub_api::Client,
    // ideahub_api_client: google_ideahub_api::Client,
    // firebaseremoteconfig_api_client: google_firebaseremoteconfig_api::Client,
    // prod_tt_sasportal_api_client: google_prod_tt_sasportal_api::Client,
    // firebaseappdistribution_api_client: google_firebaseappdistribution_api::Client,
    // firebaseappdistribution_api_client: google_firebaseappdistribution_api::Client,
    // deploymentmanager_api_client: google_deploymentmanager_api::Client,
    // deploymentmanager_api_client: google_deploymentmanager_api::Client,
    // deploymentmanager_api_client: google_deploymentmanager_api::Client,
    // datafusion_api_client: google_datafusion_api::Client,
    // datafusion_api_client: google_datafusion_api::Client,
    // tagmanager_api_client: google_tagmanager_api::Client,
    // tagmanager_api_client: google_tagmanager_api::Client,
    // discovery_api_client: google_discovery_api::Client,
    // storagetransfer_api_client: google_storagetransfer_api::Client,
    // oauth2_api_client: google_oauth2_api::Client,
    // oauth2_api_client: google_oauth2_api::Client,
    // managedkafka_api_client: google_managedkafka_api::Client,
    // admin_api_client: google_admin_api::Client,
    // admin_api_client: google_admin_api::Client,
    // admin_api_client: google_admin_api::Client,
    // bigqueryreservation_api_client: google_bigqueryreservation_api::Client,
    // bigqueryreservation_api_client: google_bigqueryreservation_api::Client,
    // bigqueryreservation_api_client: google_bigqueryreservation_api::Client,
    // vmmigration_api_client: google_vmmigration_api::Client,
    // vmmigration_api_client: google_vmmigration_api::Client,
    // contactcenteraiplatform_api_client: google_contactcenteraiplatform_api::Client,
    // doubleclicksearch_api_client: google_doubleclicksearch_api::Client,
    // backupdr_api_client: google_backupdr_api::Client,
    // ondemandscanning_api_client: google_ondemandscanning_api::Client,
    // ondemandscanning_api_client: google_ondemandscanning_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // cloudbuild_api_client: google_cloudbuild_api::Client,
    // policyanalyzer_api_client: google_policyanalyzer_api::Client,
    // policyanalyzer_api_client: google_policyanalyzer_api::Client,
    // workflows_api_client: google_workflows_api::Client,
    // workflows_api_client: google_workflows_api::Client,
    // secretmanager_api_client: google_secretmanager_api::Client,
    // secretmanager_api_client: google_secretmanager_api::Client,
    // secretmanager_api_client: google_secretmanager_api::Client,
    // transcoder_api_client: google_transcoder_api::Client,
    // transcoder_api_client: google_transcoder_api::Client,
    // accesscontextmanager_api_client: google_accesscontextmanager_api::Client,
    // accesscontextmanager_api_client: google_accesscontextmanager_api::Client,
    // content_api_client: google_content_api::Client,
    // content_api_client: google_content_api::Client,
    // content_api_client: google_content_api::Client,
    // acceleratedmobilepageurl_api_client: google_acceleratedmobilepageurl_api::Client,
    // androidpublisher_api_client: google_androidpublisher_api::Client,
    // androidpublisher_api_client: google_androidpublisher_api::Client,
    // androidpublisher_api_client: google_androidpublisher_api::Client,
    // digitalassetlinks_api_client: google_digitalassetlinks_api::Client,
    // classroom_api_client: google_classroom_api::Client,
    // chromeuxreport_api_client: google_chromeuxreport_api::Client,
    // admob_api_client: google_admob_api::Client,
    // admob_api_client: google_admob_api::Client,
    // gameservices_api_client: google_gameservices_api::Client,
    // gameservices_api_client: google_gameservices_api::Client,
    // language_api_client: google_language_api::Client,
    // language_api_client: google_language_api::Client,
    // language_api_client: google_language_api::Client,
    // language_api_client: google_language_api::Client,
    // retail_api_client: google_retail_api::Client,
    // retail_api_client: google_retail_api::Client,
    // retail_api_client: google_retail_api::Client,
    // bigquery_api_client: google_bigquery_api::Client,
    // driveactivity_api_client: google_driveactivity_api::Client,
    // serviceuser_api_client: google_serviceuser_api::Client,
    // homegraph_api_client: google_homegraph_api::Client,
    // vision_api_client: google_vision_api::Client,
    // vision_api_client: google_vision_api::Client,
    // vision_api_client: google_vision_api::Client,
    // firebaseappcheck_api_client: google_firebaseappcheck_api::Client,
    // firebaseappcheck_api_client: google_firebaseappcheck_api::Client,
    // appsactivity_api_client: google_appsactivity_api::Client,
    // cloudsearch_api_client: google_cloudsearch_api::Client,
    // books_api_client: google_books_api::Client,
    // docs_api_client: google_docs_api::Client,
    // firebasehosting_api_client: google_firebasehosting_api::Client,
    // firebasehosting_api_client: google_firebasehosting_api::Client,
    // youtube_api_client: google_youtube_api::Client,
    // observability_api_client: google_observability_api::Client,
    // adsensehost_api_client: google_adsensehost_api::Client,
    // lifesciences_api_client: google_lifesciences_api::Client,
    // adexchangebuyer_api_client: google_adexchangebuyer_api::Client,
    // adexchangebuyer_api_client: google_adexchangebuyer_api::Client,
    // adexchangebuyer_api_client: google_adexchangebuyer_api::Client,
    // streetviewpublish_api_client: google_streetviewpublish_api::Client,
    // advisorynotifications_api_client: google_advisorynotifications_api::Client,
    // sourcerepo_api_client: google_sourcerepo_api::Client,
    // vectortile_api_client: google_vectortile_api::Client,
    // paymentsresellersubscription_api_client: google_paymentsresellersubscription_api::Client,
    // airquality_api_client: google_airquality_api::Client,
    // localservices_api_client: google_localservices_api::Client,
    // adexchangeseller_api_client: google_adexchangeseller_api::Client,
    // adexchangeseller_api_client: google_adexchangeseller_api::Client,
    // adexchangeseller_api_client: google_adexchangeseller_api::Client,
    // servicebroker_api_client: google_servicebroker_api::Client,
    // servicebroker_api_client: google_servicebroker_api::Client,
    // servicebroker_api_client: google_servicebroker_api::Client,
    // speech_api_client: google_speech_api::Client,
    // speech_api_client: google_speech_api::Client,
    // speech_api_client: google_speech_api::Client,
    // speech_api_client: google_speech_api::Client,
    // speech_api_client: google_speech_api::Client,
    // script_api_client: google_script_api::Client,
    // realtimebidding_api_client: google_realtimebidding_api::Client,
    // realtimebidding_api_client: google_realtimebidding_api::Client,
    // chromepolicy_api_client: google_chromepolicy_api::Client,
    // storage_api_client: google_storage_api::Client,
    // storage_api_client: google_storage_api::Client,
    // storage_api_client: google_storage_api::Client,
    // slides_api_client: google_slides_api::Client,
    // reseller_api_client: google_reseller_api::Client,
    // mybusinessbusinessinformation_api_client: google_mybusinessbusinessinformation_api::Client,
    // androidenterprise_api_client: google_androidenterprise_api::Client,
    // ids_api_client: google_ids_api::Client,
    // adsense_api_client: google_adsense_api::Client,
    // adsense_api_client: google_adsense_api::Client,
    // adsense_api_client: google_adsense_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // videointelligence_api_client: google_videointelligence_api::Client,
    // datamigration_api_client: google_datamigration_api::Client,
    // datamigration_api_client: google_datamigration_api::Client,
    // gkebackup_api_client: google_gkebackup_api::Client,
    // adsenseplatform_api_client: google_adsenseplatform_api::Client,
    // adsenseplatform_api_client: google_adsenseplatform_api::Client,
    // integrations_api_client: google_integrations_api::Client,
    // integrations_api_client: google_integrations_api::Client,
    // customsearch_api_client: google_customsearch_api::Client,
    // kgsearch_api_client: google_kgsearch_api::Client,
    // testing_api_client: google_testing_api::Client,
    // container_api_client: google_container_api::Client,
    // container_api_client: google_container_api::Client,
    // datastore_api_client: google_datastore_api::Client,
    // datastore_api_client: google_datastore_api::Client,
    // datastore_api_client: google_datastore_api::Client,
    // playmoviespartner_api_client: google_playmoviespartner_api::Client,
    // memcache_api_client: google_memcache_api::Client,
    // memcache_api_client: google_memcache_api::Client,
    // blockchainnodeengine_api_client: google_blockchainnodeengine_api::Client,
    // cloudtrace_api_client: google_cloudtrace_api::Client,
    // cloudtrace_api_client: google_cloudtrace_api::Client,
    // cloudtrace_api_client: google_cloudtrace_api::Client,
    // iap_api_client: google_iap_api::Client,
    // iap_api_client: google_iap_api::Client,
    // playcustomapp_api_client: google_playcustomapp_api::Client,
    // dataportability_api_client: google_dataportability_api::Client,
    // dataportability_api_client: google_dataportability_api::Client,
    // mybusinessbusinesscalls_api_client: google_mybusinessbusinesscalls_api::Client,
    // policysimulator_api_client: google_policysimulator_api::Client,
    // policysimulator_api_client: google_policysimulator_api::Client,
    // policysimulator_api_client: google_policysimulator_api::Client,
    // policysimulator_api_client: google_policysimulator_api::Client,
    // firebase_api_client: google_firebase_api::Client,
    // iam_api_client: google_iam_api::Client,
    // iam_api_client: google_iam_api::Client,
    // iam_api_client: google_iam_api::Client,
    // bigtableadmin_api_client: google_bigtableadmin_api::Client,
    // bigtableadmin_api_client: google_bigtableadmin_api::Client,
    // servicedirectory_api_client: google_servicedirectory_api::Client,
    // servicedirectory_api_client: google_servicedirectory_api::Client,
    // recommender_api_client: google_recommender_api::Client,
    // recommender_api_client: google_recommender_api::Client,
    // trafficdirector_api_client: google_trafficdirector_api::Client,
    // trafficdirector_api_client: google_trafficdirector_api::Client,
    // drivelabels_api_client: google_drivelabels_api::Client,
    // drivelabels_api_client: google_drivelabels_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudasset_api_client: google_cloudasset_api::Client,
    // cloudbilling_api_client: google_cloudbilling_api::Client,
    // cloudbilling_api_client: google_cloudbilling_api::Client,
    // webmasters_api_client: google_webmasters_api::Client,
    // compute_api_client: google_compute_api::Client,
    // compute_api_client: google_compute_api::Client,
    // compute_api_client: google_compute_api::Client,
    // groupssettings_api_client: google_groupssettings_api::Client,
    // workloadmanager_api_client: google_workloadmanager_api::Client,
    // abusiveexperiencereport_api_client: google_abusiveexperiencereport_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // displayvideo_api_client: google_displayvideo_api::Client,
    // ml_api_client: google_ml_api::Client,
    // networkconnectivity_api_client: google_networkconnectivity_api::Client,
    // networkconnectivity_api_client: google_networkconnectivity_api::Client,
    // batch_api_client: google_batch_api::Client,
    // webfonts_api_client: google_webfonts_api::Client,
    // partners_api_client: google_partners_api::Client,
    // cloudprofiler_api_client: google_cloudprofiler_api::Client,
    // eventarc_api_client: google_eventarc_api::Client,
    // eventarc_api_client: google_eventarc_api::Client,
    // contentwarehouse_api_client: google_contentwarehouse_api::Client,
    // siteverification_api_client: google_siteverification_api::Client,
    // analyticsadmin_api_client: google_analyticsadmin_api::Client,
    // analyticsadmin_api_client: google_analyticsadmin_api::Client,
    // verifiedaccess_api_client: google_verifiedaccess_api::Client,
    // verifiedaccess_api_client: google_verifiedaccess_api::Client,
    // safebrowsing_api_client: google_safebrowsing_api::Client,
    // safebrowsing_api_client: google_safebrowsing_api::Client,
    // people_api_client: google_people_api::Client,
    // fcm_api_client: google_fcm_api::Client,
    // firebasestorage_api_client: google_firebasestorage_api::Client,
    // manufacturers_api_client: google_manufacturers_api::Client,
    // appstate_api_client: google_appstate_api::Client,
    // servicecontrol_api_client: google_servicecontrol_api::Client,
    // servicecontrol_api_client: google_servicecontrol_api::Client,
    // datalabeling_api_client: google_datalabeling_api::Client,
    // dlp_api_client: google_dlp_api::Client,
    // mybusinessnotifications_api_client: google_mybusinessnotifications_api::Client,
    // containeranalysis_api_client: google_containeranalysis_api::Client,
    // containeranalysis_api_client: google_containeranalysis_api::Client,
    // containeranalysis_api_client: google_containeranalysis_api::Client,
    // mybusinessqanda_api_client: google_mybusinessqanda_api::Client,
    // cloudtasks_api_client: google_cloudtasks_api::Client,
    // cloudtasks_api_client: google_cloudtasks_api::Client,
    // cloudtasks_api_client: google_cloudtasks_api::Client,
    // apim_api_client: google_apim_api::Client,
    // surveys_api_client: google_surveys_api::Client,
    // pubsub_api_client: google_pubsub_api::Client,
    // pubsub_api_client: google_pubsub_api::Client,
    // pubsub_api_client: google_pubsub_api::Client,
    // texttospeech_api_client: google_texttospeech_api::Client,
    // texttospeech_api_client: google_texttospeech_api::Client,
    // marketingplatformadmin_api_client: google_marketingplatformadmin_api::Client,
    // cloudprivatecatalogproducer_api_client: google_cloudprivatecatalogproducer_api::Client,
    // sasportal_api_client: google_sasportal_api::Client,
    // spanner_api_client: google_spanner_api::Client,
    // clouddeploy_api_client: google_clouddeploy_api::Client,
    // biglake_api_client: google_biglake_api::Client,
    // serviceusage_api_client: google_serviceusage_api::Client,
    // serviceusage_api_client: google_serviceusage_api::Client,
    // chromemanagement_api_client: google_chromemanagement_api::Client,
    // fitness_api_client: google_fitness_api::Client,
    // plusdomains_api_client: google_plusdomains_api::Client,
    // androiddeviceprovisioning_api_client: google_androiddeviceprovisioning_api::Client,
    // securesourcemanager_api_client: google_securesourcemanager_api::Client,
    // kmsinventory_api_client: google_kmsinventory_api::Client,
    // cloudcontrolspartner_api_client: google_cloudcontrolspartner_api::Client,
    // cloudcontrolspartner_api_client: google_cloudcontrolspartner_api::Client,
    // bigquerydatapolicy_api_client: google_bigquerydatapolicy_api::Client,
    // bigquerydatapolicy_api_client: google_bigquerydatapolicy_api::Client,
    // licensing_api_client: google_licensing_api::Client,
    // workspaceevents_api_client: google_workspaceevents_api::Client,
    // replicapoolupdater_api_client: google_replicapoolupdater_api::Client,
    // indexing_api_client: google_indexing_api::Client,
    // alloydb_api_client: google_alloydb_api::Client,
    // alloydb_api_client: google_alloydb_api::Client,
    // alloydb_api_client: google_alloydb_api::Client,
    // spectrum_api_client: google_spectrum_api::Client,
    // firebaseml_api_client: google_firebaseml_api::Client,
    // firebaseml_api_client: google_firebaseml_api::Client,
    // firebaseml_api_client: google_firebaseml_api::Client,
    // tpu_api_client: google_tpu_api::Client,
    // tpu_api_client: google_tpu_api::Client,
    // tpu_api_client: google_tpu_api::Client,
    // tpu_api_client: google_tpu_api::Client,
    // domainsrdap_api_client: google_domainsrdap_api::Client,
    // playablelocations_api_client: google_playablelocations_api::Client,
    // groupsmigration_api_client: google_groupsmigration_api::Client,
    // androidmanagement_api_client: google_androidmanagement_api::Client,
    // run_api_client: google_run_api::Client,
    // run_api_client: google_run_api::Client,
    // run_api_client: google_run_api::Client,
    // run_api_client: google_run_api::Client,
    // cloudcommerceprocurement_api_client: google_cloudcommerceprocurement_api::Client,
    // jobs_api_client: google_jobs_api::Client,
    // jobs_api_client: google_jobs_api::Client,
    // jobs_api_client: google_jobs_api::Client,
    // jobs_api_client: google_jobs_api::Client,
    // networksecurity_api_client: google_networksecurity_api::Client,
    // networksecurity_api_client: google_networksecurity_api::Client,
    // clouddebugger_api_client: google_clouddebugger_api::Client,
    // monitoring_api_client: google_monitoring_api::Client,
    // monitoring_api_client: google_monitoring_api::Client,
    // youtubeanalytics_api_client: google_youtubeanalytics_api::Client,
    // youtubeanalytics_api_client: google_youtubeanalytics_api::Client,
    // youtubeanalytics_api_client: google_youtubeanalytics_api::Client,
    // policytroubleshooter_api_client: google_policytroubleshooter_api::Client,
    // policytroubleshooter_api_client: google_policytroubleshooter_api::Client,
    // assuredworkloads_api_client: google_assuredworkloads_api::Client,
    // assuredworkloads_api_client: google_assuredworkloads_api::Client,
    // versionhistory_api_client: google_versionhistory_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // merchantapi_api_client: google_merchantapi_api::Client,
    // gamesconfiguration_api_client: google_gamesconfiguration_api::Client,
    // cloudscheduler_api_client: google_cloudscheduler_api::Client,
    // cloudscheduler_api_client: google_cloudscheduler_api::Client,
    // vmwareengine_api_client: google_vmwareengine_api::Client,
    // drive_api_client: google_drive_api::Client,
    // drive_api_client: google_drive_api::Client,
    // dns_api_client: google_dns_api::Client,
    // dns_api_client: google_dns_api::Client,
    // dns_api_client: google_dns_api::Client,
    // dns_api_client: google_dns_api::Client,
    // analyticsreporting_api_client: google_analyticsreporting_api::Client,
    // blogger_api_client: google_blogger_api::Client,
    // blogger_api_client: google_blogger_api::Client,
    // replicapool_api_client: google_replicapool_api::Client,
    // mybusinessplaceactions_api_client: google_mybusinessplaceactions_api::Client,
    // consumersurveys_api_client: google_consumersurveys_api::Client,
    // games_api_client: google_games_api::Client,
    // gmailpostmastertools_api_client: google_gmailpostmastertools_api::Client,
    // gmailpostmastertools_api_client: google_gmailpostmastertools_api::Client,
    // oracledatabase_api_client: google_oracledatabase_api::Client,
    // dataform_api_client: google_dataform_api::Client,
    // dataform_api_client: google_dataform_api::Client,
    // authorizedbuyersmarketplace_api_client: google_authorizedbuyersmarketplace_api::Client,
    // authorizedbuyersmarketplace_api_client: google_authorizedbuyersmarketplace_api::Client,
    // authorizedbuyersmarketplace_api_client: google_authorizedbuyersmarketplace_api::Client,
    // serviceconsumermanagement_api_client: google_serviceconsumermanagement_api::Client,
    // serviceconsumermanagement_api_client: google_serviceconsumermanagement_api::Client,
    // notebooks_api_client: google_notebooks_api::Client,
    // notebooks_api_client: google_notebooks_api::Client,
    // parametermanager_api_client: google_parametermanager_api::Client,
    // logging_api_client: google_logging_api::Client,
    // logging_api_client: google_logging_api::Client,
    // adexperiencereport_api_client: google_adexperiencereport_api::Client,
    // bigquerydatatransfer_api_client: google_bigquerydatatransfer_api::Client,
    // datacatalog_api_client: google_datacatalog_api::Client,
    // datacatalog_api_client: google_datacatalog_api::Client,
    // cloudshell_api_client: google_cloudshell_api::Client,
    // cloudshell_api_client: google_cloudshell_api::Client,
    // cloudchannel_api_client: google_cloudchannel_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // osconfig_api_client: google_osconfig_api::Client,
    // migrationcenter_api_client: google_migrationcenter_api::Client,
    // migrationcenter_api_client: google_migrationcenter_api::Client,
    // gmail_api_client: google_gmail_api::Client,
    // workflowexecutions_api_client: google_workflowexecutions_api::Client,
    // workflowexecutions_api_client: google_workflowexecutions_api::Client,
    // apigateway_api_client: google_apigateway_api::Client,
    // apigateway_api_client: google_apigateway_api::Client,
    // apigateway_api_client: google_apigateway_api::Client,
    // apigateway_api_client: google_apigateway_api::Client,
    // businessprofileperformance_api_client: google_businessprofileperformance_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // metastore_api_client: google_metastore_api::Client,
    // pubsublite_api_client: google_pubsublite_api::Client,
    // vault_api_client: google_vault_api::Client,
    // cloudsupport_api_client: google_cloudsupport_api::Client,
    // cloudsupport_api_client: google_cloudsupport_api::Client,
    // analytics_api_client: google_analytics_api::Client,
    // analytics_api_client: google_analytics_api::Client,
    // sts_api_client: google_sts_api::Client,
    // sts_api_client: google_sts_api::Client,
    // certificatemanager_api_client: google_certificatemanager_api::Client,
    // poly_api_client: google_poly_api::Client,
    // firebaserules_api_client: google_firebaserules_api::Client,
    // plus_api_client: google_plus_api::Client,
    // dataproc_api_client: google_dataproc_api::Client,
    // dataproc_api_client: google_dataproc_api::Client,
    // mirror_api_client: google_mirror_api::Client,
    // baremetalsolution_api_client: google_baremetalsolution_api::Client,
    // baremetalsolution_api_client: google_baremetalsolution_api::Client,
    // baremetalsolution_api_client: google_baremetalsolution_api::Client,
    // managedidentities_api_client: google_managedidentities_api::Client,
    // managedidentities_api_client: google_managedidentities_api::Client,
    // managedidentities_api_client: google_managedidentities_api::Client,
    // sql_api_client: google_sql_api::Client,

}

impl GcpProvider {
    /// Create a new unified provider instance
    pub async fn new() -> Result<Self> {

        Ok(Self {

        })
    }

    /// Get factchecktools_api service handler
    pub fn factchecktools_api(&self) -> factchecktools_api::Factchecktools_apiService<'_> {
        factchecktools_api::Factchecktools_apiService::new(self)
    }
    /// Get areainsights_api service handler
    pub fn areainsights_api(&self) -> areainsights_api::Areainsights_apiService<'_> {
        areainsights_api::Areainsights_apiService::new(self)
    }
    /// Get billingbudgets_api service handler
    pub fn billingbudgets_api(&self) -> billingbudgets_api::Billingbudgets_apiService<'_> {
        billingbudgets_api::Billingbudgets_apiService::new(self)
    }
    /// Get billingbudgets_api service handler
    pub fn billingbudgets_api(&self) -> billingbudgets_api::Billingbudgets_apiService<'_> {
        billingbudgets_api::Billingbudgets_apiService::new(self)
    }
    /// Get dataflow_api service handler
    pub fn dataflow_api(&self) -> dataflow_api::Dataflow_apiService<'_> {
        dataflow_api::Dataflow_apiService::new(self)
    }
    /// Get playdeveloperreporting_api service handler
    pub fn playdeveloperreporting_api(&self) -> playdeveloperreporting_api::Playdeveloperreporting_apiService<'_> {
        playdeveloperreporting_api::Playdeveloperreporting_apiService::new(self)
    }
    /// Get playdeveloperreporting_api service handler
    pub fn playdeveloperreporting_api(&self) -> playdeveloperreporting_api::Playdeveloperreporting_apiService<'_> {
        playdeveloperreporting_api::Playdeveloperreporting_apiService::new(self)
    }
    /// Get iamcredentials_api service handler
    pub fn iamcredentials_api(&self) -> iamcredentials_api::Iamcredentials_apiService<'_> {
        iamcredentials_api::Iamcredentials_apiService::new(self)
    }
    /// Get privateca_api service handler
    pub fn privateca_api(&self) -> privateca_api::Privateca_apiService<'_> {
        privateca_api::Privateca_apiService::new(self)
    }
    /// Get privateca_api service handler
    pub fn privateca_api(&self) -> privateca_api::Privateca_apiService<'_> {
        privateca_api::Privateca_apiService::new(self)
    }
    /// Get solar_api service handler
    pub fn solar_api(&self) -> solar_api::Solar_apiService<'_> {
        solar_api::Solar_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get cloudfunctions_api service handler
    pub fn cloudfunctions_api(&self) -> cloudfunctions_api::Cloudfunctions_apiService<'_> {
        cloudfunctions_api::Cloudfunctions_apiService::new(self)
    }
    /// Get essentialcontacts_api service handler
    pub fn essentialcontacts_api(&self) -> essentialcontacts_api::Essentialcontacts_apiService<'_> {
        essentialcontacts_api::Essentialcontacts_apiService::new(self)
    }
    /// Get orgpolicy_api service handler
    pub fn orgpolicy_api(&self) -> orgpolicy_api::Orgpolicy_apiService<'_> {
        orgpolicy_api::Orgpolicy_apiService::new(self)
    }
    /// Get remotebuildexecution_api service handler
    pub fn remotebuildexecution_api(&self) -> remotebuildexecution_api::Remotebuildexecution_apiService<'_> {
        remotebuildexecution_api::Remotebuildexecution_apiService::new(self)
    }
    /// Get remotebuildexecution_api service handler
    pub fn remotebuildexecution_api(&self) -> remotebuildexecution_api::Remotebuildexecution_apiService<'_> {
        remotebuildexecution_api::Remotebuildexecution_apiService::new(self)
    }
    /// Get remotebuildexecution_api service handler
    pub fn remotebuildexecution_api(&self) -> remotebuildexecution_api::Remotebuildexecution_apiService<'_> {
        remotebuildexecution_api::Remotebuildexecution_apiService::new(self)
    }
    /// Get urlshortener_api service handler
    pub fn urlshortener_api(&self) -> urlshortener_api::Urlshortener_apiService<'_> {
        urlshortener_api::Urlshortener_apiService::new(self)
    }
    /// Get workstations_api service handler
    pub fn workstations_api(&self) -> workstations_api::Workstations_apiService<'_> {
        workstations_api::Workstations_apiService::new(self)
    }
    /// Get workstations_api service handler
    pub fn workstations_api(&self) -> workstations_api::Workstations_apiService<'_> {
        workstations_api::Workstations_apiService::new(self)
    }
    /// Get webrisk_api service handler
    pub fn webrisk_api(&self) -> webrisk_api::Webrisk_apiService<'_> {
        webrisk_api::Webrisk_apiService::new(self)
    }
    /// Get keep_api service handler
    pub fn keep_api(&self) -> keep_api::Keep_apiService<'_> {
        keep_api::Keep_apiService::new(self)
    }
    /// Get datastream_api service handler
    pub fn datastream_api(&self) -> datastream_api::Datastream_apiService<'_> {
        datastream_api::Datastream_apiService::new(self)
    }
    /// Get datastream_api service handler
    pub fn datastream_api(&self) -> datastream_api::Datastream_apiService<'_> {
        datastream_api::Datastream_apiService::new(self)
    }
    /// Get servicenetworking_api service handler
    pub fn servicenetworking_api(&self) -> servicenetworking_api::Servicenetworking_apiService<'_> {
        servicenetworking_api::Servicenetworking_apiService::new(self)
    }
    /// Get servicenetworking_api service handler
    pub fn servicenetworking_api(&self) -> servicenetworking_api::Servicenetworking_apiService<'_> {
        servicenetworking_api::Servicenetworking_apiService::new(self)
    }
    /// Get binaryauthorization_api service handler
    pub fn binaryauthorization_api(&self) -> binaryauthorization_api::Binaryauthorization_apiService<'_> {
        binaryauthorization_api::Binaryauthorization_apiService::new(self)
    }
    /// Get binaryauthorization_api service handler
    pub fn binaryauthorization_api(&self) -> binaryauthorization_api::Binaryauthorization_apiService<'_> {
        binaryauthorization_api::Binaryauthorization_apiService::new(self)
    }
    /// Get area120tables_api service handler
    pub fn area120tables_api(&self) -> area120tables_api::Area120tables_apiService<'_> {
        area120tables_api::Area120tables_apiService::new(self)
    }
    /// Get connectors_api service handler
    pub fn connectors_api(&self) -> connectors_api::Connectors_apiService<'_> {
        connectors_api::Connectors_apiService::new(self)
    }
    /// Get connectors_api service handler
    pub fn connectors_api(&self) -> connectors_api::Connectors_apiService<'_> {
        connectors_api::Connectors_apiService::new(self)
    }
    /// Get apihub_api service handler
    pub fn apihub_api(&self) -> apihub_api::Apihub_apiService<'_> {
        apihub_api::Apihub_apiService::new(self)
    }
    /// Get apigeeregistry_api service handler
    pub fn apigeeregistry_api(&self) -> apigeeregistry_api::Apigeeregistry_apiService<'_> {
        apigeeregistry_api::Apigeeregistry_apiService::new(self)
    }
    /// Get beyondcorp_api service handler
    pub fn beyondcorp_api(&self) -> beyondcorp_api::Beyondcorp_apiService<'_> {
        beyondcorp_api::Beyondcorp_apiService::new(self)
    }
    /// Get beyondcorp_api service handler
    pub fn beyondcorp_api(&self) -> beyondcorp_api::Beyondcorp_apiService<'_> {
        beyondcorp_api::Beyondcorp_apiService::new(self)
    }
    /// Get toolresults_api service handler
    pub fn toolresults_api(&self) -> toolresults_api::Toolresults_apiService<'_> {
        toolresults_api::Toolresults_apiService::new(self)
    }
    /// Get toolresults_api service handler
    pub fn toolresults_api(&self) -> toolresults_api::Toolresults_apiService<'_> {
        toolresults_api::Toolresults_apiService::new(self)
    }
    /// Get clouderrorreporting_api service handler
    pub fn clouderrorreporting_api(&self) -> clouderrorreporting_api::Clouderrorreporting_apiService<'_> {
        clouderrorreporting_api::Clouderrorreporting_apiService::new(self)
    }
    /// Get civicinfo_api service handler
    pub fn civicinfo_api(&self) -> civicinfo_api::Civicinfo_apiService<'_> {
        civicinfo_api::Civicinfo_apiService::new(self)
    }
    /// Get apigee_api service handler
    pub fn apigee_api(&self) -> apigee_api::Apigee_apiService<'_> {
        apigee_api::Apigee_apiService::new(self)
    }
    /// Get mybusinessverifications_api service handler
    pub fn mybusinessverifications_api(&self) -> mybusinessverifications_api::Mybusinessverifications_apiService<'_> {
        mybusinessverifications_api::Mybusinessverifications_apiService::new(self)
    }
    /// Get developerconnect_api service handler
    pub fn developerconnect_api(&self) -> developerconnect_api::Developerconnect_apiService<'_> {
        developerconnect_api::Developerconnect_apiService::new(self)
    }
    /// Get datapipelines_api service handler
    pub fn datapipelines_api(&self) -> datapipelines_api::Datapipelines_apiService<'_> {
        datapipelines_api::Datapipelines_apiService::new(self)
    }
    /// Get firebasedynamiclinks_api service handler
    pub fn firebasedynamiclinks_api(&self) -> firebasedynamiclinks_api::Firebasedynamiclinks_apiService<'_> {
        firebasedynamiclinks_api::Firebasedynamiclinks_apiService::new(self)
    }
    /// Get runtimeconfig_api service handler
    pub fn runtimeconfig_api(&self) -> runtimeconfig_api::Runtimeconfig_apiService<'_> {
        runtimeconfig_api::Runtimeconfig_apiService::new(self)
    }
    /// Get runtimeconfig_api service handler
    pub fn runtimeconfig_api(&self) -> runtimeconfig_api::Runtimeconfig_apiService<'_> {
        runtimeconfig_api::Runtimeconfig_apiService::new(self)
    }
    /// Get storagebatchoperations_api service handler
    pub fn storagebatchoperations_api(&self) -> storagebatchoperations_api::Storagebatchoperations_apiService<'_> {
        storagebatchoperations_api::Storagebatchoperations_apiService::new(self)
    }
    /// Get tracing_api service handler
    pub fn tracing_api(&self) -> tracing_api::Tracing_apiService<'_> {
        tracing_api::Tracing_apiService::new(self)
    }
    /// Get adexchangebuyer2_api service handler
    pub fn adexchangebuyer2_api(&self) -> adexchangebuyer2_api::Adexchangebuyer2_apiService<'_> {
        adexchangebuyer2_api::Adexchangebuyer2_apiService::new(self)
    }
    /// Get saasservicemgmt_api service handler
    pub fn saasservicemgmt_api(&self) -> saasservicemgmt_api::Saasservicemgmt_apiService<'_> {
        saasservicemgmt_api::Saasservicemgmt_apiService::new(self)
    }
    /// Get proximitybeacon_api service handler
    pub fn proximitybeacon_api(&self) -> proximitybeacon_api::Proximitybeacon_apiService<'_> {
        proximitybeacon_api::Proximitybeacon_apiService::new(self)
    }
    /// Get resourcesettings_api service handler
    pub fn resourcesettings_api(&self) -> resourcesettings_api::Resourcesettings_apiService<'_> {
        resourcesettings_api::Resourcesettings_apiService::new(self)
    }
    /// Get datalineage_api service handler
    pub fn datalineage_api(&self) -> datalineage_api::Datalineage_apiService<'_> {
        datalineage_api::Datalineage_apiService::new(self)
    }
    /// Get libraryagent_api service handler
    pub fn libraryagent_api(&self) -> libraryagent_api::Libraryagent_apiService<'_> {
        libraryagent_api::Libraryagent_apiService::new(self)
    }
    /// Get acmedns_api service handler
    pub fn acmedns_api(&self) -> acmedns_api::Acmedns_apiService<'_> {
        acmedns_api::Acmedns_apiService::new(self)
    }
    /// Get publicca_api service handler
    pub fn publicca_api(&self) -> publicca_api::Publicca_apiService<'_> {
        publicca_api::Publicca_apiService::new(self)
    }
    /// Get publicca_api service handler
    pub fn publicca_api(&self) -> publicca_api::Publicca_apiService<'_> {
        publicca_api::Publicca_apiService::new(self)
    }
    /// Get publicca_api service handler
    pub fn publicca_api(&self) -> publicca_api::Publicca_apiService<'_> {
        publicca_api::Publicca_apiService::new(self)
    }
    /// Get networkservices_api service handler
    pub fn networkservices_api(&self) -> networkservices_api::Networkservices_apiService<'_> {
        networkservices_api::Networkservices_apiService::new(self)
    }
    /// Get networkservices_api service handler
    pub fn networkservices_api(&self) -> networkservices_api::Networkservices_apiService<'_> {
        networkservices_api::Networkservices_apiService::new(self)
    }
    /// Get netapp_api service handler
    pub fn netapp_api(&self) -> netapp_api::Netapp_apiService<'_> {
        netapp_api::Netapp_apiService::new(self)
    }
    /// Get netapp_api service handler
    pub fn netapp_api(&self) -> netapp_api::Netapp_apiService<'_> {
        netapp_api::Netapp_apiService::new(self)
    }
    /// Get aiplatform_api service handler
    pub fn aiplatform_api(&self) -> aiplatform_api::Aiplatform_apiService<'_> {
        aiplatform_api::Aiplatform_apiService::new(self)
    }
    /// Get aiplatform_api service handler
    pub fn aiplatform_api(&self) -> aiplatform_api::Aiplatform_apiService<'_> {
        aiplatform_api::Aiplatform_apiService::new(self)
    }
    /// Get pagespeedonline_api service handler
    pub fn pagespeedonline_api(&self) -> pagespeedonline_api::Pagespeedonline_apiService<'_> {
        pagespeedonline_api::Pagespeedonline_apiService::new(self)
    }
    /// Get pagespeedonline_api service handler
    pub fn pagespeedonline_api(&self) -> pagespeedonline_api::Pagespeedonline_apiService<'_> {
        pagespeedonline_api::Pagespeedonline_apiService::new(self)
    }
    /// Get pagespeedonline_api service handler
    pub fn pagespeedonline_api(&self) -> pagespeedonline_api::Pagespeedonline_apiService<'_> {
        pagespeedonline_api::Pagespeedonline_apiService::new(self)
    }
    /// Get pagespeedonline_api service handler
    pub fn pagespeedonline_api(&self) -> pagespeedonline_api::Pagespeedonline_apiService<'_> {
        pagespeedonline_api::Pagespeedonline_apiService::new(self)
    }
    /// Get tasks_api service handler
    pub fn tasks_api(&self) -> tasks_api::Tasks_apiService<'_> {
        tasks_api::Tasks_apiService::new(self)
    }
    /// Get sqladmin_api service handler
    pub fn sqladmin_api(&self) -> sqladmin_api::Sqladmin_apiService<'_> {
        sqladmin_api::Sqladmin_apiService::new(self)
    }
    /// Get sqladmin_api service handler
    pub fn sqladmin_api(&self) -> sqladmin_api::Sqladmin_apiService<'_> {
        sqladmin_api::Sqladmin_apiService::new(self)
    }
    /// Get discoveryengine_api service handler
    pub fn discoveryengine_api(&self) -> discoveryengine_api::Discoveryengine_apiService<'_> {
        discoveryengine_api::Discoveryengine_apiService::new(self)
    }
    /// Get discoveryengine_api service handler
    pub fn discoveryengine_api(&self) -> discoveryengine_api::Discoveryengine_apiService<'_> {
        discoveryengine_api::Discoveryengine_apiService::new(self)
    }
    /// Get discoveryengine_api service handler
    pub fn discoveryengine_api(&self) -> discoveryengine_api::Discoveryengine_apiService<'_> {
        discoveryengine_api::Discoveryengine_apiService::new(self)
    }
    /// Get gkeonprem_api service handler
    pub fn gkeonprem_api(&self) -> gkeonprem_api::Gkeonprem_apiService<'_> {
        gkeonprem_api::Gkeonprem_apiService::new(self)
    }
    /// Get cloudlocationfinder_api service handler
    pub fn cloudlocationfinder_api(&self) -> cloudlocationfinder_api::Cloudlocationfinder_apiService<'_> {
        cloudlocationfinder_api::Cloudlocationfinder_apiService::new(self)
    }
    /// Get cloudlocationfinder_api service handler
    pub fn cloudlocationfinder_api(&self) -> cloudlocationfinder_api::Cloudlocationfinder_apiService<'_> {
        cloudlocationfinder_api::Cloudlocationfinder_apiService::new(self)
    }
    /// Get mybusinesslodging_api service handler
    pub fn mybusinesslodging_api(&self) -> mybusinesslodging_api::Mybusinesslodging_apiService<'_> {
        mybusinesslodging_api::Mybusinesslodging_apiService::new(self)
    }
    /// Get mybusinessaccountmanagement_api service handler
    pub fn mybusinessaccountmanagement_api(&self) -> mybusinessaccountmanagement_api::Mybusinessaccountmanagement_apiService<'_> {
        mybusinessaccountmanagement_api::Mybusinessaccountmanagement_apiService::new(self)
    }
    /// Get searchads360_api service handler
    pub fn searchads360_api(&self) -> searchads360_api::Searchads360_apiService<'_> {
        searchads360_api::Searchads360_apiService::new(self)
    }
    /// Get alertcenter_api service handler
    pub fn alertcenter_api(&self) -> alertcenter_api::Alertcenter_apiService<'_> {
        alertcenter_api::Alertcenter_apiService::new(self)
    }
    /// Get servicemanagement_api service handler
    pub fn servicemanagement_api(&self) -> servicemanagement_api::Servicemanagement_apiService<'_> {
        servicemanagement_api::Servicemanagement_apiService::new(self)
    }
    /// Get smartdevicemanagement_api service handler
    pub fn smartdevicemanagement_api(&self) -> smartdevicemanagement_api::Smartdevicemanagement_apiService<'_> {
        smartdevicemanagement_api::Smartdevicemanagement_apiService::new(self)
    }
    /// Get domains_api service handler
    pub fn domains_api(&self) -> domains_api::Domains_apiService<'_> {
        domains_api::Domains_apiService::new(self)
    }
    /// Get domains_api service handler
    pub fn domains_api(&self) -> domains_api::Domains_apiService<'_> {
        domains_api::Domains_apiService::new(self)
    }
    /// Get domains_api service handler
    pub fn domains_api(&self) -> domains_api::Domains_apiService<'_> {
        domains_api::Domains_apiService::new(self)
    }
    /// Get calendar_api service handler
    pub fn calendar_api(&self) -> calendar_api::Calendar_apiService<'_> {
        calendar_api::Calendar_apiService::new(self)
    }
    /// Get vpcaccess_api service handler
    pub fn vpcaccess_api(&self) -> vpcaccess_api::Vpcaccess_apiService<'_> {
        vpcaccess_api::Vpcaccess_apiService::new(self)
    }
    /// Get vpcaccess_api service handler
    pub fn vpcaccess_api(&self) -> vpcaccess_api::Vpcaccess_apiService<'_> {
        vpcaccess_api::Vpcaccess_apiService::new(self)
    }
    /// Get sheets_api service handler
    pub fn sheets_api(&self) -> sheets_api::Sheets_apiService<'_> {
        sheets_api::Sheets_apiService::new(self)
    }
    /// Get looker_api service handler
    pub fn looker_api(&self) -> looker_api::Looker_apiService<'_> {
        looker_api::Looker_apiService::new(self)
    }
    /// Get forms_api service handler
    pub fn forms_api(&self) -> forms_api::Forms_apiService<'_> {
        forms_api::Forms_apiService::new(self)
    }
    /// Get cloudiot_api service handler
    pub fn cloudiot_api(&self) -> cloudiot_api::Cloudiot_apiService<'_> {
        cloudiot_api::Cloudiot_apiService::new(self)
    }
    /// Get genomics_api service handler
    pub fn genomics_api(&self) -> genomics_api::Genomics_apiService<'_> {
        genomics_api::Genomics_apiService::new(self)
    }
    /// Get genomics_api service handler
    pub fn genomics_api(&self) -> genomics_api::Genomics_apiService<'_> {
        genomics_api::Genomics_apiService::new(self)
    }
    /// Get genomics_api service handler
    pub fn genomics_api(&self) -> genomics_api::Genomics_apiService<'_> {
        genomics_api::Genomics_apiService::new(self)
    }
    /// Get pollen_api service handler
    pub fn pollen_api(&self) -> pollen_api::Pollen_apiService<'_> {
        pollen_api::Pollen_apiService::new(self)
    }
    /// Get playgrouping_api service handler
    pub fn playgrouping_api(&self) -> playgrouping_api::Playgrouping_apiService<'_> {
        playgrouping_api::Playgrouping_apiService::new(self)
    }
    /// Get oslogin_api service handler
    pub fn oslogin_api(&self) -> oslogin_api::Oslogin_apiService<'_> {
        oslogin_api::Oslogin_apiService::new(self)
    }
    /// Get oslogin_api service handler
    pub fn oslogin_api(&self) -> oslogin_api::Oslogin_apiService<'_> {
        oslogin_api::Oslogin_apiService::new(self)
    }
    /// Get oslogin_api service handler
    pub fn oslogin_api(&self) -> oslogin_api::Oslogin_apiService<'_> {
        oslogin_api::Oslogin_apiService::new(self)
    }
    /// Get gamesmanagement_api service handler
    pub fn gamesmanagement_api(&self) -> gamesmanagement_api::Gamesmanagement_apiService<'_> {
        gamesmanagement_api::Gamesmanagement_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get gkehub_api service handler
    pub fn gkehub_api(&self) -> gkehub_api::Gkehub_apiService<'_> {
        gkehub_api::Gkehub_apiService::new(self)
    }
    /// Get securityposture_api service handler
    pub fn securityposture_api(&self) -> securityposture_api::Securityposture_apiService<'_> {
        securityposture_api::Securityposture_apiService::new(self)
    }
    /// Get networkmanagement_api service handler
    pub fn networkmanagement_api(&self) -> networkmanagement_api::Networkmanagement_apiService<'_> {
        networkmanagement_api::Networkmanagement_apiService::new(self)
    }
    /// Get networkmanagement_api service handler
    pub fn networkmanagement_api(&self) -> networkmanagement_api::Networkmanagement_apiService<'_> {
        networkmanagement_api::Networkmanagement_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get securitycenter_api service handler
    pub fn securitycenter_api(&self) -> securitycenter_api::Securitycenter_apiService<'_> {
        securitycenter_api::Securitycenter_apiService::new(self)
    }
    /// Get datamanager_api service handler
    pub fn datamanager_api(&self) -> datamanager_api::Datamanager_apiService<'_> {
        datamanager_api::Datamanager_apiService::new(self)
    }
    /// Get chat_api service handler
    pub fn chat_api(&self) -> chat_api::Chat_apiService<'_> {
        chat_api::Chat_apiService::new(self)
    }
    /// Get apphub_api service handler
    pub fn apphub_api(&self) -> apphub_api::Apphub_apiService<'_> {
        apphub_api::Apphub_apiService::new(self)
    }
    /// Get apphub_api service handler
    pub fn apphub_api(&self) -> apphub_api::Apphub_apiService<'_> {
        apphub_api::Apphub_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get dialogflow_api service handler
    pub fn dialogflow_api(&self) -> dialogflow_api::Dialogflow_apiService<'_> {
        dialogflow_api::Dialogflow_apiService::new(self)
    }
    /// Get rapidmigrationassessment_api service handler
    pub fn rapidmigrationassessment_api(&self) -> rapidmigrationassessment_api::Rapidmigrationassessment_apiService<'_> {
        rapidmigrationassessment_api::Rapidmigrationassessment_apiService::new(self)
    }
    /// Get documentai_api service handler
    pub fn documentai_api(&self) -> documentai_api::Documentai_apiService<'_> {
        documentai_api::Documentai_apiService::new(self)
    }
    /// Get documentai_api service handler
    pub fn documentai_api(&self) -> documentai_api::Documentai_apiService<'_> {
        documentai_api::Documentai_apiService::new(self)
    }
    /// Get documentai_api service handler
    pub fn documentai_api(&self) -> documentai_api::Documentai_apiService<'_> {
        documentai_api::Documentai_apiService::new(self)
    }
    /// Get artifactregistry_api service handler
    pub fn artifactregistry_api(&self) -> artifactregistry_api::Artifactregistry_apiService<'_> {
        artifactregistry_api::Artifactregistry_apiService::new(self)
    }
    /// Get artifactregistry_api service handler
    pub fn artifactregistry_api(&self) -> artifactregistry_api::Artifactregistry_apiService<'_> {
        artifactregistry_api::Artifactregistry_apiService::new(self)
    }
    /// Get artifactregistry_api service handler
    pub fn artifactregistry_api(&self) -> artifactregistry_api::Artifactregistry_apiService<'_> {
        artifactregistry_api::Artifactregistry_apiService::new(self)
    }
    /// Get qpxexpress_api service handler
    pub fn qpxexpress_api(&self) -> qpxexpress_api::Qpxexpress_apiService<'_> {
        qpxexpress_api::Qpxexpress_apiService::new(self)
    }
    /// Get analyticsdata_api service handler
    pub fn analyticsdata_api(&self) -> analyticsdata_api::Analyticsdata_apiService<'_> {
        analyticsdata_api::Analyticsdata_apiService::new(self)
    }
    /// Get analyticsdata_api service handler
    pub fn analyticsdata_api(&self) -> analyticsdata_api::Analyticsdata_apiService<'_> {
        analyticsdata_api::Analyticsdata_apiService::new(self)
    }
    /// Get file_api service handler
    pub fn file_api(&self) -> file_api::File_apiService<'_> {
        file_api::File_apiService::new(self)
    }
    /// Get file_api service handler
    pub fn file_api(&self) -> file_api::File_apiService<'_> {
        file_api::File_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get cloudresourcemanager_api service handler
    pub fn cloudresourcemanager_api(&self) -> cloudresourcemanager_api::Cloudresourcemanager_apiService<'_> {
        cloudresourcemanager_api::Cloudresourcemanager_apiService::new(self)
    }
    /// Get css_api service handler
    pub fn css_api(&self) -> css_api::Css_apiService<'_> {
        css_api::Css_apiService::new(self)
    }
    /// Get walletobjects_api service handler
    pub fn walletobjects_api(&self) -> walletobjects_api::Walletobjects_apiService<'_> {
        walletobjects_api::Walletobjects_apiService::new(self)
    }
    /// Get readerrevenuesubscriptionlinking_api service handler
    pub fn readerrevenuesubscriptionlinking_api(&self) -> readerrevenuesubscriptionlinking_api::Readerrevenuesubscriptionlinking_apiService<'_> {
        readerrevenuesubscriptionlinking_api::Readerrevenuesubscriptionlinking_apiService::new(self)
    }
    /// Get youtubereporting_api service handler
    pub fn youtubereporting_api(&self) -> youtubereporting_api::Youtubereporting_apiService<'_> {
        youtubereporting_api::Youtubereporting_apiService::new(self)
    }
    /// Get firestore_api service handler
    pub fn firestore_api(&self) -> firestore_api::Firestore_apiService<'_> {
        firestore_api::Firestore_apiService::new(self)
    }
    /// Get firestore_api service handler
    pub fn firestore_api(&self) -> firestore_api::Firestore_apiService<'_> {
        firestore_api::Firestore_apiService::new(self)
    }
    /// Get firestore_api service handler
    pub fn firestore_api(&self) -> firestore_api::Firestore_apiService<'_> {
        firestore_api::Firestore_apiService::new(self)
    }
    /// Get config_api service handler
    pub fn config_api(&self) -> config_api::Config_apiService<'_> {
        config_api::Config_apiService::new(self)
    }
    /// Get translate_api service handler
    pub fn translate_api(&self) -> translate_api::Translate_apiService<'_> {
        translate_api::Translate_apiService::new(self)
    }
    /// Get translate_api service handler
    pub fn translate_api(&self) -> translate_api::Translate_apiService<'_> {
        translate_api::Translate_apiService::new(self)
    }
    /// Get translate_api service handler
    pub fn translate_api(&self) -> translate_api::Translate_apiService<'_> {
        translate_api::Translate_apiService::new(self)
    }
    /// Get redis_api service handler
    pub fn redis_api(&self) -> redis_api::Redis_apiService<'_> {
        redis_api::Redis_apiService::new(self)
    }
    /// Get redis_api service handler
    pub fn redis_api(&self) -> redis_api::Redis_apiService<'_> {
        redis_api::Redis_apiService::new(self)
    }
    /// Get meet_api service handler
    pub fn meet_api(&self) -> meet_api::Meet_apiService<'_> {
        meet_api::Meet_apiService::new(self)
    }
    /// Get dataplex_api service handler
    pub fn dataplex_api(&self) -> dataplex_api::Dataplex_apiService<'_> {
        dataplex_api::Dataplex_apiService::new(self)
    }
    /// Get analyticshub_api service handler
    pub fn analyticshub_api(&self) -> analyticshub_api::Analyticshub_apiService<'_> {
        analyticshub_api::Analyticshub_apiService::new(self)
    }
    /// Get analyticshub_api service handler
    pub fn analyticshub_api(&self) -> analyticshub_api::Analyticshub_apiService<'_> {
        analyticshub_api::Analyticshub_apiService::new(self)
    }
    /// Get playintegrity_api service handler
    pub fn playintegrity_api(&self) -> playintegrity_api::Playintegrity_apiService<'_> {
        playintegrity_api::Playintegrity_apiService::new(self)
    }
    /// Get cloudkms_api service handler
    pub fn cloudkms_api(&self) -> cloudkms_api::Cloudkms_apiService<'_> {
        cloudkms_api::Cloudkms_apiService::new(self)
    }
    /// Get fusiontables_api service handler
    pub fn fusiontables_api(&self) -> fusiontables_api::Fusiontables_apiService<'_> {
        fusiontables_api::Fusiontables_apiService::new(self)
    }
    /// Get fusiontables_api service handler
    pub fn fusiontables_api(&self) -> fusiontables_api::Fusiontables_apiService<'_> {
        fusiontables_api::Fusiontables_apiService::new(self)
    }
    /// Get accessapproval_api service handler
    pub fn accessapproval_api(&self) -> accessapproval_api::Accessapproval_apiService<'_> {
        accessapproval_api::Accessapproval_apiService::new(self)
    }
    /// Get checks_api service handler
    pub fn checks_api(&self) -> checks_api::Checks_apiService<'_> {
        checks_api::Checks_apiService::new(self)
    }
    /// Get cloudprivatecatalog_api service handler
    pub fn cloudprivatecatalog_api(&self) -> cloudprivatecatalog_api::Cloudprivatecatalog_apiService<'_> {
        cloudprivatecatalog_api::Cloudprivatecatalog_apiService::new(self)
    }
    /// Get addressvalidation_api service handler
    pub fn addressvalidation_api(&self) -> addressvalidation_api::Addressvalidation_apiService<'_> {
        addressvalidation_api::Addressvalidation_apiService::new(self)
    }
    /// Get chromewebstore_api service handler
    pub fn chromewebstore_api(&self) -> chromewebstore_api::Chromewebstore_apiService<'_> {
        chromewebstore_api::Chromewebstore_apiService::new(self)
    }
    /// Get chromewebstore_api service handler
    pub fn chromewebstore_api(&self) -> chromewebstore_api::Chromewebstore_apiService<'_> {
        chromewebstore_api::Chromewebstore_apiService::new(self)
    }
    /// Get websecurityscanner_api service handler
    pub fn websecurityscanner_api(&self) -> websecurityscanner_api::Websecurityscanner_apiService<'_> {
        websecurityscanner_api::Websecurityscanner_apiService::new(self)
    }
    /// Get websecurityscanner_api service handler
    pub fn websecurityscanner_api(&self) -> websecurityscanner_api::Websecurityscanner_apiService<'_> {
        websecurityscanner_api::Websecurityscanner_apiService::new(self)
    }
    /// Get websecurityscanner_api service handler
    pub fn websecurityscanner_api(&self) -> websecurityscanner_api::Websecurityscanner_apiService<'_> {
        websecurityscanner_api::Websecurityscanner_apiService::new(self)
    }
    /// Get places_api service handler
    pub fn places_api(&self) -> places_api::Places_apiService<'_> {
        places_api::Places_apiService::new(self)
    }
    /// Get recaptchaenterprise_api service handler
    pub fn recaptchaenterprise_api(&self) -> recaptchaenterprise_api::Recaptchaenterprise_apiService<'_> {
        recaptchaenterprise_api::Recaptchaenterprise_apiService::new(self)
    }
    /// Get parallelstore_api service handler
    pub fn parallelstore_api(&self) -> parallelstore_api::Parallelstore_apiService<'_> {
        parallelstore_api::Parallelstore_apiService::new(self)
    }
    /// Get parallelstore_api service handler
    pub fn parallelstore_api(&self) -> parallelstore_api::Parallelstore_apiService<'_> {
        parallelstore_api::Parallelstore_apiService::new(self)
    }
    /// Get contactcenterinsights_api service handler
    pub fn contactcenterinsights_api(&self) -> contactcenterinsights_api::Contactcenterinsights_apiService<'_> {
        contactcenterinsights_api::Contactcenterinsights_apiService::new(self)
    }
    /// Get recommendationengine_api service handler
    pub fn recommendationengine_api(&self) -> recommendationengine_api::Recommendationengine_apiService<'_> {
        recommendationengine_api::Recommendationengine_apiService::new(self)
    }
    /// Get cloudidentity_api service handler
    pub fn cloudidentity_api(&self) -> cloudidentity_api::Cloudidentity_apiService<'_> {
        cloudidentity_api::Cloudidentity_apiService::new(self)
    }
    /// Get cloudidentity_api service handler
    pub fn cloudidentity_api(&self) -> cloudidentity_api::Cloudidentity_apiService<'_> {
        cloudidentity_api::Cloudidentity_apiService::new(self)
    }
    /// Get healthcare_api service handler
    pub fn healthcare_api(&self) -> healthcare_api::Healthcare_apiService<'_> {
        healthcare_api::Healthcare_apiService::new(self)
    }
    /// Get healthcare_api service handler
    pub fn healthcare_api(&self) -> healthcare_api::Healthcare_apiService<'_> {
        healthcare_api::Healthcare_apiService::new(self)
    }
    /// Get healthcare_api service handler
    pub fn healthcare_api(&self) -> healthcare_api::Healthcare_apiService<'_> {
        healthcare_api::Healthcare_apiService::new(self)
    }
    /// Get healthcare_api service handler
    pub fn healthcare_api(&self) -> healthcare_api::Healthcare_apiService<'_> {
        healthcare_api::Healthcare_apiService::new(self)
    }
    /// Get composer_api service handler
    pub fn composer_api(&self) -> composer_api::Composer_apiService<'_> {
        composer_api::Composer_apiService::new(self)
    }
    /// Get composer_api service handler
    pub fn composer_api(&self) -> composer_api::Composer_apiService<'_> {
        composer_api::Composer_apiService::new(self)
    }
    /// Get searchconsole_api service handler
    pub fn searchconsole_api(&self) -> searchconsole_api::Searchconsole_apiService<'_> {
        searchconsole_api::Searchconsole_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get appengine_api service handler
    pub fn appengine_api(&self) -> appengine_api::Appengine_apiService<'_> {
        appengine_api::Appengine_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get dfareporting_api service handler
    pub fn dfareporting_api(&self) -> dfareporting_api::Dfareporting_apiService<'_> {
        dfareporting_api::Dfareporting_apiService::new(self)
    }
    /// Get apikeys_api service handler
    pub fn apikeys_api(&self) -> apikeys_api::Apikeys_apiService<'_> {
        apikeys_api::Apikeys_apiService::new(self)
    }
    /// Get bigqueryconnection_api service handler
    pub fn bigqueryconnection_api(&self) -> bigqueryconnection_api::Bigqueryconnection_apiService<'_> {
        bigqueryconnection_api::Bigqueryconnection_apiService::new(self)
    }
    /// Get bigqueryconnection_api service handler
    pub fn bigqueryconnection_api(&self) -> bigqueryconnection_api::Bigqueryconnection_apiService<'_> {
        bigqueryconnection_api::Bigqueryconnection_apiService::new(self)
    }
    /// Get firebasedataconnect_api service handler
    pub fn firebasedataconnect_api(&self) -> firebasedataconnect_api::Firebasedataconnect_apiService<'_> {
        firebasedataconnect_api::Firebasedataconnect_apiService::new(self)
    }
    /// Get firebasedataconnect_api service handler
    pub fn firebasedataconnect_api(&self) -> firebasedataconnect_api::Firebasedataconnect_apiService<'_> {
        firebasedataconnect_api::Firebasedataconnect_apiService::new(self)
    }
    /// Get commentanalyzer_api service handler
    pub fn commentanalyzer_api(&self) -> commentanalyzer_api::Commentanalyzer_apiService<'_> {
        commentanalyzer_api::Commentanalyzer_apiService::new(self)
    }
    /// Get firebaseapphosting_api service handler
    pub fn firebaseapphosting_api(&self) -> firebaseapphosting_api::Firebaseapphosting_apiService<'_> {
        firebaseapphosting_api::Firebaseapphosting_apiService::new(self)
    }
    /// Get firebaseapphosting_api service handler
    pub fn firebaseapphosting_api(&self) -> firebaseapphosting_api::Firebaseapphosting_apiService<'_> {
        firebaseapphosting_api::Firebaseapphosting_apiService::new(self)
    }
    /// Get doubleclickbidmanager_api service handler
    pub fn doubleclickbidmanager_api(&self) -> doubleclickbidmanager_api::Doubleclickbidmanager_apiService<'_> {
        doubleclickbidmanager_api::Doubleclickbidmanager_apiService::new(self)
    }
    /// Get doubleclickbidmanager_api service handler
    pub fn doubleclickbidmanager_api(&self) -> doubleclickbidmanager_api::Doubleclickbidmanager_apiService<'_> {
        doubleclickbidmanager_api::Doubleclickbidmanager_apiService::new(self)
    }
    /// Get doubleclickbidmanager_api service handler
    pub fn doubleclickbidmanager_api(&self) -> doubleclickbidmanager_api::Doubleclickbidmanager_apiService<'_> {
        doubleclickbidmanager_api::Doubleclickbidmanager_apiService::new(self)
    }
    /// Get fcmdata_api service handler
    pub fn fcmdata_api(&self) -> fcmdata_api::Fcmdata_apiService<'_> {
        fcmdata_api::Fcmdata_apiService::new(self)
    }
    /// Get firebasedatabase_api service handler
    pub fn firebasedatabase_api(&self) -> firebasedatabase_api::Firebasedatabase_apiService<'_> {
        firebasedatabase_api::Firebasedatabase_apiService::new(self)
    }
    /// Get ideahub_api service handler
    pub fn ideahub_api(&self) -> ideahub_api::Ideahub_apiService<'_> {
        ideahub_api::Ideahub_apiService::new(self)
    }
    /// Get ideahub_api service handler
    pub fn ideahub_api(&self) -> ideahub_api::Ideahub_apiService<'_> {
        ideahub_api::Ideahub_apiService::new(self)
    }
    /// Get firebaseremoteconfig_api service handler
    pub fn firebaseremoteconfig_api(&self) -> firebaseremoteconfig_api::Firebaseremoteconfig_apiService<'_> {
        firebaseremoteconfig_api::Firebaseremoteconfig_apiService::new(self)
    }
    /// Get prod_tt_sasportal_api service handler
    pub fn prod_tt_sasportal_api(&self) -> prod_tt_sasportal_api::Prod_tt_sasportal_apiService<'_> {
        prod_tt_sasportal_api::Prod_tt_sasportal_apiService::new(self)
    }
    /// Get firebaseappdistribution_api service handler
    pub fn firebaseappdistribution_api(&self) -> firebaseappdistribution_api::Firebaseappdistribution_apiService<'_> {
        firebaseappdistribution_api::Firebaseappdistribution_apiService::new(self)
    }
    /// Get firebaseappdistribution_api service handler
    pub fn firebaseappdistribution_api(&self) -> firebaseappdistribution_api::Firebaseappdistribution_apiService<'_> {
        firebaseappdistribution_api::Firebaseappdistribution_apiService::new(self)
    }
    /// Get deploymentmanager_api service handler
    pub fn deploymentmanager_api(&self) -> deploymentmanager_api::Deploymentmanager_apiService<'_> {
        deploymentmanager_api::Deploymentmanager_apiService::new(self)
    }
    /// Get deploymentmanager_api service handler
    pub fn deploymentmanager_api(&self) -> deploymentmanager_api::Deploymentmanager_apiService<'_> {
        deploymentmanager_api::Deploymentmanager_apiService::new(self)
    }
    /// Get deploymentmanager_api service handler
    pub fn deploymentmanager_api(&self) -> deploymentmanager_api::Deploymentmanager_apiService<'_> {
        deploymentmanager_api::Deploymentmanager_apiService::new(self)
    }
    /// Get datafusion_api service handler
    pub fn datafusion_api(&self) -> datafusion_api::Datafusion_apiService<'_> {
        datafusion_api::Datafusion_apiService::new(self)
    }
    /// Get datafusion_api service handler
    pub fn datafusion_api(&self) -> datafusion_api::Datafusion_apiService<'_> {
        datafusion_api::Datafusion_apiService::new(self)
    }
    /// Get tagmanager_api service handler
    pub fn tagmanager_api(&self) -> tagmanager_api::Tagmanager_apiService<'_> {
        tagmanager_api::Tagmanager_apiService::new(self)
    }
    /// Get tagmanager_api service handler
    pub fn tagmanager_api(&self) -> tagmanager_api::Tagmanager_apiService<'_> {
        tagmanager_api::Tagmanager_apiService::new(self)
    }
    /// Get discovery_api service handler
    pub fn discovery_api(&self) -> discovery_api::Discovery_apiService<'_> {
        discovery_api::Discovery_apiService::new(self)
    }
    /// Get storagetransfer_api service handler
    pub fn storagetransfer_api(&self) -> storagetransfer_api::Storagetransfer_apiService<'_> {
        storagetransfer_api::Storagetransfer_apiService::new(self)
    }
    /// Get oauth2_api service handler
    pub fn oauth2_api(&self) -> oauth2_api::Oauth2_apiService<'_> {
        oauth2_api::Oauth2_apiService::new(self)
    }
    /// Get oauth2_api service handler
    pub fn oauth2_api(&self) -> oauth2_api::Oauth2_apiService<'_> {
        oauth2_api::Oauth2_apiService::new(self)
    }
    /// Get managedkafka_api service handler
    pub fn managedkafka_api(&self) -> managedkafka_api::Managedkafka_apiService<'_> {
        managedkafka_api::Managedkafka_apiService::new(self)
    }
    /// Get admin_api service handler
    pub fn admin_api(&self) -> admin_api::Admin_apiService<'_> {
        admin_api::Admin_apiService::new(self)
    }
    /// Get admin_api service handler
    pub fn admin_api(&self) -> admin_api::Admin_apiService<'_> {
        admin_api::Admin_apiService::new(self)
    }
    /// Get admin_api service handler
    pub fn admin_api(&self) -> admin_api::Admin_apiService<'_> {
        admin_api::Admin_apiService::new(self)
    }
    /// Get bigqueryreservation_api service handler
    pub fn bigqueryreservation_api(&self) -> bigqueryreservation_api::Bigqueryreservation_apiService<'_> {
        bigqueryreservation_api::Bigqueryreservation_apiService::new(self)
    }
    /// Get bigqueryreservation_api service handler
    pub fn bigqueryreservation_api(&self) -> bigqueryreservation_api::Bigqueryreservation_apiService<'_> {
        bigqueryreservation_api::Bigqueryreservation_apiService::new(self)
    }
    /// Get bigqueryreservation_api service handler
    pub fn bigqueryreservation_api(&self) -> bigqueryreservation_api::Bigqueryreservation_apiService<'_> {
        bigqueryreservation_api::Bigqueryreservation_apiService::new(self)
    }
    /// Get vmmigration_api service handler
    pub fn vmmigration_api(&self) -> vmmigration_api::Vmmigration_apiService<'_> {
        vmmigration_api::Vmmigration_apiService::new(self)
    }
    /// Get vmmigration_api service handler
    pub fn vmmigration_api(&self) -> vmmigration_api::Vmmigration_apiService<'_> {
        vmmigration_api::Vmmigration_apiService::new(self)
    }
    /// Get contactcenteraiplatform_api service handler
    pub fn contactcenteraiplatform_api(&self) -> contactcenteraiplatform_api::Contactcenteraiplatform_apiService<'_> {
        contactcenteraiplatform_api::Contactcenteraiplatform_apiService::new(self)
    }
    /// Get doubleclicksearch_api service handler
    pub fn doubleclicksearch_api(&self) -> doubleclicksearch_api::Doubleclicksearch_apiService<'_> {
        doubleclicksearch_api::Doubleclicksearch_apiService::new(self)
    }
    /// Get backupdr_api service handler
    pub fn backupdr_api(&self) -> backupdr_api::Backupdr_apiService<'_> {
        backupdr_api::Backupdr_apiService::new(self)
    }
    /// Get ondemandscanning_api service handler
    pub fn ondemandscanning_api(&self) -> ondemandscanning_api::Ondemandscanning_apiService<'_> {
        ondemandscanning_api::Ondemandscanning_apiService::new(self)
    }
    /// Get ondemandscanning_api service handler
    pub fn ondemandscanning_api(&self) -> ondemandscanning_api::Ondemandscanning_apiService<'_> {
        ondemandscanning_api::Ondemandscanning_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get cloudbuild_api service handler
    pub fn cloudbuild_api(&self) -> cloudbuild_api::Cloudbuild_apiService<'_> {
        cloudbuild_api::Cloudbuild_apiService::new(self)
    }
    /// Get policyanalyzer_api service handler
    pub fn policyanalyzer_api(&self) -> policyanalyzer_api::Policyanalyzer_apiService<'_> {
        policyanalyzer_api::Policyanalyzer_apiService::new(self)
    }
    /// Get policyanalyzer_api service handler
    pub fn policyanalyzer_api(&self) -> policyanalyzer_api::Policyanalyzer_apiService<'_> {
        policyanalyzer_api::Policyanalyzer_apiService::new(self)
    }
    /// Get workflows_api service handler
    pub fn workflows_api(&self) -> workflows_api::Workflows_apiService<'_> {
        workflows_api::Workflows_apiService::new(self)
    }
    /// Get workflows_api service handler
    pub fn workflows_api(&self) -> workflows_api::Workflows_apiService<'_> {
        workflows_api::Workflows_apiService::new(self)
    }
    /// Get secretmanager_api service handler
    pub fn secretmanager_api(&self) -> secretmanager_api::Secretmanager_apiService<'_> {
        secretmanager_api::Secretmanager_apiService::new(self)
    }
    /// Get secretmanager_api service handler
    pub fn secretmanager_api(&self) -> secretmanager_api::Secretmanager_apiService<'_> {
        secretmanager_api::Secretmanager_apiService::new(self)
    }
    /// Get secretmanager_api service handler
    pub fn secretmanager_api(&self) -> secretmanager_api::Secretmanager_apiService<'_> {
        secretmanager_api::Secretmanager_apiService::new(self)
    }
    /// Get transcoder_api service handler
    pub fn transcoder_api(&self) -> transcoder_api::Transcoder_apiService<'_> {
        transcoder_api::Transcoder_apiService::new(self)
    }
    /// Get transcoder_api service handler
    pub fn transcoder_api(&self) -> transcoder_api::Transcoder_apiService<'_> {
        transcoder_api::Transcoder_apiService::new(self)
    }
    /// Get accesscontextmanager_api service handler
    pub fn accesscontextmanager_api(&self) -> accesscontextmanager_api::Accesscontextmanager_apiService<'_> {
        accesscontextmanager_api::Accesscontextmanager_apiService::new(self)
    }
    /// Get accesscontextmanager_api service handler
    pub fn accesscontextmanager_api(&self) -> accesscontextmanager_api::Accesscontextmanager_apiService<'_> {
        accesscontextmanager_api::Accesscontextmanager_apiService::new(self)
    }
    /// Get content_api service handler
    pub fn content_api(&self) -> content_api::Content_apiService<'_> {
        content_api::Content_apiService::new(self)
    }
    /// Get content_api service handler
    pub fn content_api(&self) -> content_api::Content_apiService<'_> {
        content_api::Content_apiService::new(self)
    }
    /// Get content_api service handler
    pub fn content_api(&self) -> content_api::Content_apiService<'_> {
        content_api::Content_apiService::new(self)
    }
    /// Get acceleratedmobilepageurl_api service handler
    pub fn acceleratedmobilepageurl_api(&self) -> acceleratedmobilepageurl_api::Acceleratedmobilepageurl_apiService<'_> {
        acceleratedmobilepageurl_api::Acceleratedmobilepageurl_apiService::new(self)
    }
    /// Get androidpublisher_api service handler
    pub fn androidpublisher_api(&self) -> androidpublisher_api::Androidpublisher_apiService<'_> {
        androidpublisher_api::Androidpublisher_apiService::new(self)
    }
    /// Get androidpublisher_api service handler
    pub fn androidpublisher_api(&self) -> androidpublisher_api::Androidpublisher_apiService<'_> {
        androidpublisher_api::Androidpublisher_apiService::new(self)
    }
    /// Get androidpublisher_api service handler
    pub fn androidpublisher_api(&self) -> androidpublisher_api::Androidpublisher_apiService<'_> {
        androidpublisher_api::Androidpublisher_apiService::new(self)
    }
    /// Get digitalassetlinks_api service handler
    pub fn digitalassetlinks_api(&self) -> digitalassetlinks_api::Digitalassetlinks_apiService<'_> {
        digitalassetlinks_api::Digitalassetlinks_apiService::new(self)
    }
    /// Get classroom_api service handler
    pub fn classroom_api(&self) -> classroom_api::Classroom_apiService<'_> {
        classroom_api::Classroom_apiService::new(self)
    }
    /// Get chromeuxreport_api service handler
    pub fn chromeuxreport_api(&self) -> chromeuxreport_api::Chromeuxreport_apiService<'_> {
        chromeuxreport_api::Chromeuxreport_apiService::new(self)
    }
    /// Get admob_api service handler
    pub fn admob_api(&self) -> admob_api::Admob_apiService<'_> {
        admob_api::Admob_apiService::new(self)
    }
    /// Get admob_api service handler
    pub fn admob_api(&self) -> admob_api::Admob_apiService<'_> {
        admob_api::Admob_apiService::new(self)
    }
    /// Get gameservices_api service handler
    pub fn gameservices_api(&self) -> gameservices_api::Gameservices_apiService<'_> {
        gameservices_api::Gameservices_apiService::new(self)
    }
    /// Get gameservices_api service handler
    pub fn gameservices_api(&self) -> gameservices_api::Gameservices_apiService<'_> {
        gameservices_api::Gameservices_apiService::new(self)
    }
    /// Get language_api service handler
    pub fn language_api(&self) -> language_api::Language_apiService<'_> {
        language_api::Language_apiService::new(self)
    }
    /// Get language_api service handler
    pub fn language_api(&self) -> language_api::Language_apiService<'_> {
        language_api::Language_apiService::new(self)
    }
    /// Get language_api service handler
    pub fn language_api(&self) -> language_api::Language_apiService<'_> {
        language_api::Language_apiService::new(self)
    }
    /// Get language_api service handler
    pub fn language_api(&self) -> language_api::Language_apiService<'_> {
        language_api::Language_apiService::new(self)
    }
    /// Get retail_api service handler
    pub fn retail_api(&self) -> retail_api::Retail_apiService<'_> {
        retail_api::Retail_apiService::new(self)
    }
    /// Get retail_api service handler
    pub fn retail_api(&self) -> retail_api::Retail_apiService<'_> {
        retail_api::Retail_apiService::new(self)
    }
    /// Get retail_api service handler
    pub fn retail_api(&self) -> retail_api::Retail_apiService<'_> {
        retail_api::Retail_apiService::new(self)
    }
    /// Get bigquery_api service handler
    pub fn bigquery_api(&self) -> bigquery_api::Bigquery_apiService<'_> {
        bigquery_api::Bigquery_apiService::new(self)
    }
    /// Get driveactivity_api service handler
    pub fn driveactivity_api(&self) -> driveactivity_api::Driveactivity_apiService<'_> {
        driveactivity_api::Driveactivity_apiService::new(self)
    }
    /// Get serviceuser_api service handler
    pub fn serviceuser_api(&self) -> serviceuser_api::Serviceuser_apiService<'_> {
        serviceuser_api::Serviceuser_apiService::new(self)
    }
    /// Get homegraph_api service handler
    pub fn homegraph_api(&self) -> homegraph_api::Homegraph_apiService<'_> {
        homegraph_api::Homegraph_apiService::new(self)
    }
    /// Get vision_api service handler
    pub fn vision_api(&self) -> vision_api::Vision_apiService<'_> {
        vision_api::Vision_apiService::new(self)
    }
    /// Get vision_api service handler
    pub fn vision_api(&self) -> vision_api::Vision_apiService<'_> {
        vision_api::Vision_apiService::new(self)
    }
    /// Get vision_api service handler
    pub fn vision_api(&self) -> vision_api::Vision_apiService<'_> {
        vision_api::Vision_apiService::new(self)
    }
    /// Get firebaseappcheck_api service handler
    pub fn firebaseappcheck_api(&self) -> firebaseappcheck_api::Firebaseappcheck_apiService<'_> {
        firebaseappcheck_api::Firebaseappcheck_apiService::new(self)
    }
    /// Get firebaseappcheck_api service handler
    pub fn firebaseappcheck_api(&self) -> firebaseappcheck_api::Firebaseappcheck_apiService<'_> {
        firebaseappcheck_api::Firebaseappcheck_apiService::new(self)
    }
    /// Get appsactivity_api service handler
    pub fn appsactivity_api(&self) -> appsactivity_api::Appsactivity_apiService<'_> {
        appsactivity_api::Appsactivity_apiService::new(self)
    }
    /// Get cloudsearch_api service handler
    pub fn cloudsearch_api(&self) -> cloudsearch_api::Cloudsearch_apiService<'_> {
        cloudsearch_api::Cloudsearch_apiService::new(self)
    }
    /// Get books_api service handler
    pub fn books_api(&self) -> books_api::Books_apiService<'_> {
        books_api::Books_apiService::new(self)
    }
    /// Get docs_api service handler
    pub fn docs_api(&self) -> docs_api::Docs_apiService<'_> {
        docs_api::Docs_apiService::new(self)
    }
    /// Get firebasehosting_api service handler
    pub fn firebasehosting_api(&self) -> firebasehosting_api::Firebasehosting_apiService<'_> {
        firebasehosting_api::Firebasehosting_apiService::new(self)
    }
    /// Get firebasehosting_api service handler
    pub fn firebasehosting_api(&self) -> firebasehosting_api::Firebasehosting_apiService<'_> {
        firebasehosting_api::Firebasehosting_apiService::new(self)
    }
    /// Get youtube_api service handler
    pub fn youtube_api(&self) -> youtube_api::Youtube_apiService<'_> {
        youtube_api::Youtube_apiService::new(self)
    }
    /// Get observability_api service handler
    pub fn observability_api(&self) -> observability_api::Observability_apiService<'_> {
        observability_api::Observability_apiService::new(self)
    }
    /// Get adsensehost_api service handler
    pub fn adsensehost_api(&self) -> adsensehost_api::Adsensehost_apiService<'_> {
        adsensehost_api::Adsensehost_apiService::new(self)
    }
    /// Get lifesciences_api service handler
    pub fn lifesciences_api(&self) -> lifesciences_api::Lifesciences_apiService<'_> {
        lifesciences_api::Lifesciences_apiService::new(self)
    }
    /// Get adexchangebuyer_api service handler
    pub fn adexchangebuyer_api(&self) -> adexchangebuyer_api::Adexchangebuyer_apiService<'_> {
        adexchangebuyer_api::Adexchangebuyer_apiService::new(self)
    }
    /// Get adexchangebuyer_api service handler
    pub fn adexchangebuyer_api(&self) -> adexchangebuyer_api::Adexchangebuyer_apiService<'_> {
        adexchangebuyer_api::Adexchangebuyer_apiService::new(self)
    }
    /// Get adexchangebuyer_api service handler
    pub fn adexchangebuyer_api(&self) -> adexchangebuyer_api::Adexchangebuyer_apiService<'_> {
        adexchangebuyer_api::Adexchangebuyer_apiService::new(self)
    }
    /// Get streetviewpublish_api service handler
    pub fn streetviewpublish_api(&self) -> streetviewpublish_api::Streetviewpublish_apiService<'_> {
        streetviewpublish_api::Streetviewpublish_apiService::new(self)
    }
    /// Get advisorynotifications_api service handler
    pub fn advisorynotifications_api(&self) -> advisorynotifications_api::Advisorynotifications_apiService<'_> {
        advisorynotifications_api::Advisorynotifications_apiService::new(self)
    }
    /// Get sourcerepo_api service handler
    pub fn sourcerepo_api(&self) -> sourcerepo_api::Sourcerepo_apiService<'_> {
        sourcerepo_api::Sourcerepo_apiService::new(self)
    }
    /// Get vectortile_api service handler
    pub fn vectortile_api(&self) -> vectortile_api::Vectortile_apiService<'_> {
        vectortile_api::Vectortile_apiService::new(self)
    }
    /// Get paymentsresellersubscription_api service handler
    pub fn paymentsresellersubscription_api(&self) -> paymentsresellersubscription_api::Paymentsresellersubscription_apiService<'_> {
        paymentsresellersubscription_api::Paymentsresellersubscription_apiService::new(self)
    }
    /// Get airquality_api service handler
    pub fn airquality_api(&self) -> airquality_api::Airquality_apiService<'_> {
        airquality_api::Airquality_apiService::new(self)
    }
    /// Get localservices_api service handler
    pub fn localservices_api(&self) -> localservices_api::Localservices_apiService<'_> {
        localservices_api::Localservices_apiService::new(self)
    }
    /// Get adexchangeseller_api service handler
    pub fn adexchangeseller_api(&self) -> adexchangeseller_api::Adexchangeseller_apiService<'_> {
        adexchangeseller_api::Adexchangeseller_apiService::new(self)
    }
    /// Get adexchangeseller_api service handler
    pub fn adexchangeseller_api(&self) -> adexchangeseller_api::Adexchangeseller_apiService<'_> {
        adexchangeseller_api::Adexchangeseller_apiService::new(self)
    }
    /// Get adexchangeseller_api service handler
    pub fn adexchangeseller_api(&self) -> adexchangeseller_api::Adexchangeseller_apiService<'_> {
        adexchangeseller_api::Adexchangeseller_apiService::new(self)
    }
    /// Get servicebroker_api service handler
    pub fn servicebroker_api(&self) -> servicebroker_api::Servicebroker_apiService<'_> {
        servicebroker_api::Servicebroker_apiService::new(self)
    }
    /// Get servicebroker_api service handler
    pub fn servicebroker_api(&self) -> servicebroker_api::Servicebroker_apiService<'_> {
        servicebroker_api::Servicebroker_apiService::new(self)
    }
    /// Get servicebroker_api service handler
    pub fn servicebroker_api(&self) -> servicebroker_api::Servicebroker_apiService<'_> {
        servicebroker_api::Servicebroker_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get speech_api service handler
    pub fn speech_api(&self) -> speech_api::Speech_apiService<'_> {
        speech_api::Speech_apiService::new(self)
    }
    /// Get script_api service handler
    pub fn script_api(&self) -> script_api::Script_apiService<'_> {
        script_api::Script_apiService::new(self)
    }
    /// Get realtimebidding_api service handler
    pub fn realtimebidding_api(&self) -> realtimebidding_api::Realtimebidding_apiService<'_> {
        realtimebidding_api::Realtimebidding_apiService::new(self)
    }
    /// Get realtimebidding_api service handler
    pub fn realtimebidding_api(&self) -> realtimebidding_api::Realtimebidding_apiService<'_> {
        realtimebidding_api::Realtimebidding_apiService::new(self)
    }
    /// Get chromepolicy_api service handler
    pub fn chromepolicy_api(&self) -> chromepolicy_api::Chromepolicy_apiService<'_> {
        chromepolicy_api::Chromepolicy_apiService::new(self)
    }
    /// Get storage_api service handler
    pub fn storage_api(&self) -> storage_api::Storage_apiService<'_> {
        storage_api::Storage_apiService::new(self)
    }
    /// Get storage_api service handler
    pub fn storage_api(&self) -> storage_api::Storage_apiService<'_> {
        storage_api::Storage_apiService::new(self)
    }
    /// Get storage_api service handler
    pub fn storage_api(&self) -> storage_api::Storage_apiService<'_> {
        storage_api::Storage_apiService::new(self)
    }
    /// Get slides_api service handler
    pub fn slides_api(&self) -> slides_api::Slides_apiService<'_> {
        slides_api::Slides_apiService::new(self)
    }
    /// Get reseller_api service handler
    pub fn reseller_api(&self) -> reseller_api::Reseller_apiService<'_> {
        reseller_api::Reseller_apiService::new(self)
    }
    /// Get mybusinessbusinessinformation_api service handler
    pub fn mybusinessbusinessinformation_api(&self) -> mybusinessbusinessinformation_api::Mybusinessbusinessinformation_apiService<'_> {
        mybusinessbusinessinformation_api::Mybusinessbusinessinformation_apiService::new(self)
    }
    /// Get androidenterprise_api service handler
    pub fn androidenterprise_api(&self) -> androidenterprise_api::Androidenterprise_apiService<'_> {
        androidenterprise_api::Androidenterprise_apiService::new(self)
    }
    /// Get ids_api service handler
    pub fn ids_api(&self) -> ids_api::Ids_apiService<'_> {
        ids_api::Ids_apiService::new(self)
    }
    /// Get adsense_api service handler
    pub fn adsense_api(&self) -> adsense_api::Adsense_apiService<'_> {
        adsense_api::Adsense_apiService::new(self)
    }
    /// Get adsense_api service handler
    pub fn adsense_api(&self) -> adsense_api::Adsense_apiService<'_> {
        adsense_api::Adsense_apiService::new(self)
    }
    /// Get adsense_api service handler
    pub fn adsense_api(&self) -> adsense_api::Adsense_apiService<'_> {
        adsense_api::Adsense_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get videointelligence_api service handler
    pub fn videointelligence_api(&self) -> videointelligence_api::Videointelligence_apiService<'_> {
        videointelligence_api::Videointelligence_apiService::new(self)
    }
    /// Get datamigration_api service handler
    pub fn datamigration_api(&self) -> datamigration_api::Datamigration_apiService<'_> {
        datamigration_api::Datamigration_apiService::new(self)
    }
    /// Get datamigration_api service handler
    pub fn datamigration_api(&self) -> datamigration_api::Datamigration_apiService<'_> {
        datamigration_api::Datamigration_apiService::new(self)
    }
    /// Get gkebackup_api service handler
    pub fn gkebackup_api(&self) -> gkebackup_api::Gkebackup_apiService<'_> {
        gkebackup_api::Gkebackup_apiService::new(self)
    }
    /// Get adsenseplatform_api service handler
    pub fn adsenseplatform_api(&self) -> adsenseplatform_api::Adsenseplatform_apiService<'_> {
        adsenseplatform_api::Adsenseplatform_apiService::new(self)
    }
    /// Get adsenseplatform_api service handler
    pub fn adsenseplatform_api(&self) -> adsenseplatform_api::Adsenseplatform_apiService<'_> {
        adsenseplatform_api::Adsenseplatform_apiService::new(self)
    }
    /// Get integrations_api service handler
    pub fn integrations_api(&self) -> integrations_api::Integrations_apiService<'_> {
        integrations_api::Integrations_apiService::new(self)
    }
    /// Get integrations_api service handler
    pub fn integrations_api(&self) -> integrations_api::Integrations_apiService<'_> {
        integrations_api::Integrations_apiService::new(self)
    }
    /// Get customsearch_api service handler
    pub fn customsearch_api(&self) -> customsearch_api::Customsearch_apiService<'_> {
        customsearch_api::Customsearch_apiService::new(self)
    }
    /// Get kgsearch_api service handler
    pub fn kgsearch_api(&self) -> kgsearch_api::Kgsearch_apiService<'_> {
        kgsearch_api::Kgsearch_apiService::new(self)
    }
    /// Get testing_api service handler
    pub fn testing_api(&self) -> testing_api::Testing_apiService<'_> {
        testing_api::Testing_apiService::new(self)
    }
    /// Get container_api service handler
    pub fn container_api(&self) -> container_api::Container_apiService<'_> {
        container_api::Container_apiService::new(self)
    }
    /// Get container_api service handler
    pub fn container_api(&self) -> container_api::Container_apiService<'_> {
        container_api::Container_apiService::new(self)
    }
    /// Get datastore_api service handler
    pub fn datastore_api(&self) -> datastore_api::Datastore_apiService<'_> {
        datastore_api::Datastore_apiService::new(self)
    }
    /// Get datastore_api service handler
    pub fn datastore_api(&self) -> datastore_api::Datastore_apiService<'_> {
        datastore_api::Datastore_apiService::new(self)
    }
    /// Get datastore_api service handler
    pub fn datastore_api(&self) -> datastore_api::Datastore_apiService<'_> {
        datastore_api::Datastore_apiService::new(self)
    }
    /// Get playmoviespartner_api service handler
    pub fn playmoviespartner_api(&self) -> playmoviespartner_api::Playmoviespartner_apiService<'_> {
        playmoviespartner_api::Playmoviespartner_apiService::new(self)
    }
    /// Get memcache_api service handler
    pub fn memcache_api(&self) -> memcache_api::Memcache_apiService<'_> {
        memcache_api::Memcache_apiService::new(self)
    }
    /// Get memcache_api service handler
    pub fn memcache_api(&self) -> memcache_api::Memcache_apiService<'_> {
        memcache_api::Memcache_apiService::new(self)
    }
    /// Get blockchainnodeengine_api service handler
    pub fn blockchainnodeengine_api(&self) -> blockchainnodeengine_api::Blockchainnodeengine_apiService<'_> {
        blockchainnodeengine_api::Blockchainnodeengine_apiService::new(self)
    }
    /// Get cloudtrace_api service handler
    pub fn cloudtrace_api(&self) -> cloudtrace_api::Cloudtrace_apiService<'_> {
        cloudtrace_api::Cloudtrace_apiService::new(self)
    }
    /// Get cloudtrace_api service handler
    pub fn cloudtrace_api(&self) -> cloudtrace_api::Cloudtrace_apiService<'_> {
        cloudtrace_api::Cloudtrace_apiService::new(self)
    }
    /// Get cloudtrace_api service handler
    pub fn cloudtrace_api(&self) -> cloudtrace_api::Cloudtrace_apiService<'_> {
        cloudtrace_api::Cloudtrace_apiService::new(self)
    }
    /// Get iap_api service handler
    pub fn iap_api(&self) -> iap_api::Iap_apiService<'_> {
        iap_api::Iap_apiService::new(self)
    }
    /// Get iap_api service handler
    pub fn iap_api(&self) -> iap_api::Iap_apiService<'_> {
        iap_api::Iap_apiService::new(self)
    }
    /// Get playcustomapp_api service handler
    pub fn playcustomapp_api(&self) -> playcustomapp_api::Playcustomapp_apiService<'_> {
        playcustomapp_api::Playcustomapp_apiService::new(self)
    }
    /// Get dataportability_api service handler
    pub fn dataportability_api(&self) -> dataportability_api::Dataportability_apiService<'_> {
        dataportability_api::Dataportability_apiService::new(self)
    }
    /// Get dataportability_api service handler
    pub fn dataportability_api(&self) -> dataportability_api::Dataportability_apiService<'_> {
        dataportability_api::Dataportability_apiService::new(self)
    }
    /// Get mybusinessbusinesscalls_api service handler
    pub fn mybusinessbusinesscalls_api(&self) -> mybusinessbusinesscalls_api::Mybusinessbusinesscalls_apiService<'_> {
        mybusinessbusinesscalls_api::Mybusinessbusinesscalls_apiService::new(self)
    }
    /// Get policysimulator_api service handler
    pub fn policysimulator_api(&self) -> policysimulator_api::Policysimulator_apiService<'_> {
        policysimulator_api::Policysimulator_apiService::new(self)
    }
    /// Get policysimulator_api service handler
    pub fn policysimulator_api(&self) -> policysimulator_api::Policysimulator_apiService<'_> {
        policysimulator_api::Policysimulator_apiService::new(self)
    }
    /// Get policysimulator_api service handler
    pub fn policysimulator_api(&self) -> policysimulator_api::Policysimulator_apiService<'_> {
        policysimulator_api::Policysimulator_apiService::new(self)
    }
    /// Get policysimulator_api service handler
    pub fn policysimulator_api(&self) -> policysimulator_api::Policysimulator_apiService<'_> {
        policysimulator_api::Policysimulator_apiService::new(self)
    }
    /// Get firebase_api service handler
    pub fn firebase_api(&self) -> firebase_api::Firebase_apiService<'_> {
        firebase_api::Firebase_apiService::new(self)
    }
    /// Get iam_api service handler
    pub fn iam_api(&self) -> iam_api::Iam_apiService<'_> {
        iam_api::Iam_apiService::new(self)
    }
    /// Get iam_api service handler
    pub fn iam_api(&self) -> iam_api::Iam_apiService<'_> {
        iam_api::Iam_apiService::new(self)
    }
    /// Get iam_api service handler
    pub fn iam_api(&self) -> iam_api::Iam_apiService<'_> {
        iam_api::Iam_apiService::new(self)
    }
    /// Get bigtableadmin_api service handler
    pub fn bigtableadmin_api(&self) -> bigtableadmin_api::Bigtableadmin_apiService<'_> {
        bigtableadmin_api::Bigtableadmin_apiService::new(self)
    }
    /// Get bigtableadmin_api service handler
    pub fn bigtableadmin_api(&self) -> bigtableadmin_api::Bigtableadmin_apiService<'_> {
        bigtableadmin_api::Bigtableadmin_apiService::new(self)
    }
    /// Get servicedirectory_api service handler
    pub fn servicedirectory_api(&self) -> servicedirectory_api::Servicedirectory_apiService<'_> {
        servicedirectory_api::Servicedirectory_apiService::new(self)
    }
    /// Get servicedirectory_api service handler
    pub fn servicedirectory_api(&self) -> servicedirectory_api::Servicedirectory_apiService<'_> {
        servicedirectory_api::Servicedirectory_apiService::new(self)
    }
    /// Get recommender_api service handler
    pub fn recommender_api(&self) -> recommender_api::Recommender_apiService<'_> {
        recommender_api::Recommender_apiService::new(self)
    }
    /// Get recommender_api service handler
    pub fn recommender_api(&self) -> recommender_api::Recommender_apiService<'_> {
        recommender_api::Recommender_apiService::new(self)
    }
    /// Get trafficdirector_api service handler
    pub fn trafficdirector_api(&self) -> trafficdirector_api::Trafficdirector_apiService<'_> {
        trafficdirector_api::Trafficdirector_apiService::new(self)
    }
    /// Get trafficdirector_api service handler
    pub fn trafficdirector_api(&self) -> trafficdirector_api::Trafficdirector_apiService<'_> {
        trafficdirector_api::Trafficdirector_apiService::new(self)
    }
    /// Get drivelabels_api service handler
    pub fn drivelabels_api(&self) -> drivelabels_api::Drivelabels_apiService<'_> {
        drivelabels_api::Drivelabels_apiService::new(self)
    }
    /// Get drivelabels_api service handler
    pub fn drivelabels_api(&self) -> drivelabels_api::Drivelabels_apiService<'_> {
        drivelabels_api::Drivelabels_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudasset_api service handler
    pub fn cloudasset_api(&self) -> cloudasset_api::Cloudasset_apiService<'_> {
        cloudasset_api::Cloudasset_apiService::new(self)
    }
    /// Get cloudbilling_api service handler
    pub fn cloudbilling_api(&self) -> cloudbilling_api::Cloudbilling_apiService<'_> {
        cloudbilling_api::Cloudbilling_apiService::new(self)
    }
    /// Get cloudbilling_api service handler
    pub fn cloudbilling_api(&self) -> cloudbilling_api::Cloudbilling_apiService<'_> {
        cloudbilling_api::Cloudbilling_apiService::new(self)
    }
    /// Get webmasters_api service handler
    pub fn webmasters_api(&self) -> webmasters_api::Webmasters_apiService<'_> {
        webmasters_api::Webmasters_apiService::new(self)
    }
    /// Get compute_api service handler
    pub fn compute_api(&self) -> compute_api::Compute_apiService<'_> {
        compute_api::Compute_apiService::new(self)
    }
    /// Get compute_api service handler
    pub fn compute_api(&self) -> compute_api::Compute_apiService<'_> {
        compute_api::Compute_apiService::new(self)
    }
    /// Get compute_api service handler
    pub fn compute_api(&self) -> compute_api::Compute_apiService<'_> {
        compute_api::Compute_apiService::new(self)
    }
    /// Get groupssettings_api service handler
    pub fn groupssettings_api(&self) -> groupssettings_api::Groupssettings_apiService<'_> {
        groupssettings_api::Groupssettings_apiService::new(self)
    }
    /// Get workloadmanager_api service handler
    pub fn workloadmanager_api(&self) -> workloadmanager_api::Workloadmanager_apiService<'_> {
        workloadmanager_api::Workloadmanager_apiService::new(self)
    }
    /// Get abusiveexperiencereport_api service handler
    pub fn abusiveexperiencereport_api(&self) -> abusiveexperiencereport_api::Abusiveexperiencereport_apiService<'_> {
        abusiveexperiencereport_api::Abusiveexperiencereport_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get displayvideo_api service handler
    pub fn displayvideo_api(&self) -> displayvideo_api::Displayvideo_apiService<'_> {
        displayvideo_api::Displayvideo_apiService::new(self)
    }
    /// Get ml_api service handler
    pub fn ml_api(&self) -> ml_api::Ml_apiService<'_> {
        ml_api::Ml_apiService::new(self)
    }
    /// Get networkconnectivity_api service handler
    pub fn networkconnectivity_api(&self) -> networkconnectivity_api::Networkconnectivity_apiService<'_> {
        networkconnectivity_api::Networkconnectivity_apiService::new(self)
    }
    /// Get networkconnectivity_api service handler
    pub fn networkconnectivity_api(&self) -> networkconnectivity_api::Networkconnectivity_apiService<'_> {
        networkconnectivity_api::Networkconnectivity_apiService::new(self)
    }
    /// Get batch_api service handler
    pub fn batch_api(&self) -> batch_api::Batch_apiService<'_> {
        batch_api::Batch_apiService::new(self)
    }
    /// Get webfonts_api service handler
    pub fn webfonts_api(&self) -> webfonts_api::Webfonts_apiService<'_> {
        webfonts_api::Webfonts_apiService::new(self)
    }
    /// Get partners_api service handler
    pub fn partners_api(&self) -> partners_api::Partners_apiService<'_> {
        partners_api::Partners_apiService::new(self)
    }
    /// Get cloudprofiler_api service handler
    pub fn cloudprofiler_api(&self) -> cloudprofiler_api::Cloudprofiler_apiService<'_> {
        cloudprofiler_api::Cloudprofiler_apiService::new(self)
    }
    /// Get eventarc_api service handler
    pub fn eventarc_api(&self) -> eventarc_api::Eventarc_apiService<'_> {
        eventarc_api::Eventarc_apiService::new(self)
    }
    /// Get eventarc_api service handler
    pub fn eventarc_api(&self) -> eventarc_api::Eventarc_apiService<'_> {
        eventarc_api::Eventarc_apiService::new(self)
    }
    /// Get contentwarehouse_api service handler
    pub fn contentwarehouse_api(&self) -> contentwarehouse_api::Contentwarehouse_apiService<'_> {
        contentwarehouse_api::Contentwarehouse_apiService::new(self)
    }
    /// Get siteverification_api service handler
    pub fn siteverification_api(&self) -> siteverification_api::Siteverification_apiService<'_> {
        siteverification_api::Siteverification_apiService::new(self)
    }
    /// Get analyticsadmin_api service handler
    pub fn analyticsadmin_api(&self) -> analyticsadmin_api::Analyticsadmin_apiService<'_> {
        analyticsadmin_api::Analyticsadmin_apiService::new(self)
    }
    /// Get analyticsadmin_api service handler
    pub fn analyticsadmin_api(&self) -> analyticsadmin_api::Analyticsadmin_apiService<'_> {
        analyticsadmin_api::Analyticsadmin_apiService::new(self)
    }
    /// Get verifiedaccess_api service handler
    pub fn verifiedaccess_api(&self) -> verifiedaccess_api::Verifiedaccess_apiService<'_> {
        verifiedaccess_api::Verifiedaccess_apiService::new(self)
    }
    /// Get verifiedaccess_api service handler
    pub fn verifiedaccess_api(&self) -> verifiedaccess_api::Verifiedaccess_apiService<'_> {
        verifiedaccess_api::Verifiedaccess_apiService::new(self)
    }
    /// Get safebrowsing_api service handler
    pub fn safebrowsing_api(&self) -> safebrowsing_api::Safebrowsing_apiService<'_> {
        safebrowsing_api::Safebrowsing_apiService::new(self)
    }
    /// Get safebrowsing_api service handler
    pub fn safebrowsing_api(&self) -> safebrowsing_api::Safebrowsing_apiService<'_> {
        safebrowsing_api::Safebrowsing_apiService::new(self)
    }
    /// Get people_api service handler
    pub fn people_api(&self) -> people_api::People_apiService<'_> {
        people_api::People_apiService::new(self)
    }
    /// Get fcm_api service handler
    pub fn fcm_api(&self) -> fcm_api::Fcm_apiService<'_> {
        fcm_api::Fcm_apiService::new(self)
    }
    /// Get firebasestorage_api service handler
    pub fn firebasestorage_api(&self) -> firebasestorage_api::Firebasestorage_apiService<'_> {
        firebasestorage_api::Firebasestorage_apiService::new(self)
    }
    /// Get manufacturers_api service handler
    pub fn manufacturers_api(&self) -> manufacturers_api::Manufacturers_apiService<'_> {
        manufacturers_api::Manufacturers_apiService::new(self)
    }
    /// Get appstate_api service handler
    pub fn appstate_api(&self) -> appstate_api::Appstate_apiService<'_> {
        appstate_api::Appstate_apiService::new(self)
    }
    /// Get servicecontrol_api service handler
    pub fn servicecontrol_api(&self) -> servicecontrol_api::Servicecontrol_apiService<'_> {
        servicecontrol_api::Servicecontrol_apiService::new(self)
    }
    /// Get servicecontrol_api service handler
    pub fn servicecontrol_api(&self) -> servicecontrol_api::Servicecontrol_apiService<'_> {
        servicecontrol_api::Servicecontrol_apiService::new(self)
    }
    /// Get datalabeling_api service handler
    pub fn datalabeling_api(&self) -> datalabeling_api::Datalabeling_apiService<'_> {
        datalabeling_api::Datalabeling_apiService::new(self)
    }
    /// Get dlp_api service handler
    pub fn dlp_api(&self) -> dlp_api::Dlp_apiService<'_> {
        dlp_api::Dlp_apiService::new(self)
    }
    /// Get mybusinessnotifications_api service handler
    pub fn mybusinessnotifications_api(&self) -> mybusinessnotifications_api::Mybusinessnotifications_apiService<'_> {
        mybusinessnotifications_api::Mybusinessnotifications_apiService::new(self)
    }
    /// Get containeranalysis_api service handler
    pub fn containeranalysis_api(&self) -> containeranalysis_api::Containeranalysis_apiService<'_> {
        containeranalysis_api::Containeranalysis_apiService::new(self)
    }
    /// Get containeranalysis_api service handler
    pub fn containeranalysis_api(&self) -> containeranalysis_api::Containeranalysis_apiService<'_> {
        containeranalysis_api::Containeranalysis_apiService::new(self)
    }
    /// Get containeranalysis_api service handler
    pub fn containeranalysis_api(&self) -> containeranalysis_api::Containeranalysis_apiService<'_> {
        containeranalysis_api::Containeranalysis_apiService::new(self)
    }
    /// Get mybusinessqanda_api service handler
    pub fn mybusinessqanda_api(&self) -> mybusinessqanda_api::Mybusinessqanda_apiService<'_> {
        mybusinessqanda_api::Mybusinessqanda_apiService::new(self)
    }
    /// Get cloudtasks_api service handler
    pub fn cloudtasks_api(&self) -> cloudtasks_api::Cloudtasks_apiService<'_> {
        cloudtasks_api::Cloudtasks_apiService::new(self)
    }
    /// Get cloudtasks_api service handler
    pub fn cloudtasks_api(&self) -> cloudtasks_api::Cloudtasks_apiService<'_> {
        cloudtasks_api::Cloudtasks_apiService::new(self)
    }
    /// Get cloudtasks_api service handler
    pub fn cloudtasks_api(&self) -> cloudtasks_api::Cloudtasks_apiService<'_> {
        cloudtasks_api::Cloudtasks_apiService::new(self)
    }
    /// Get apim_api service handler
    pub fn apim_api(&self) -> apim_api::Apim_apiService<'_> {
        apim_api::Apim_apiService::new(self)
    }
    /// Get surveys_api service handler
    pub fn surveys_api(&self) -> surveys_api::Surveys_apiService<'_> {
        surveys_api::Surveys_apiService::new(self)
    }
    /// Get pubsub_api service handler
    pub fn pubsub_api(&self) -> pubsub_api::Pubsub_apiService<'_> {
        pubsub_api::Pubsub_apiService::new(self)
    }
    /// Get pubsub_api service handler
    pub fn pubsub_api(&self) -> pubsub_api::Pubsub_apiService<'_> {
        pubsub_api::Pubsub_apiService::new(self)
    }
    /// Get pubsub_api service handler
    pub fn pubsub_api(&self) -> pubsub_api::Pubsub_apiService<'_> {
        pubsub_api::Pubsub_apiService::new(self)
    }
    /// Get texttospeech_api service handler
    pub fn texttospeech_api(&self) -> texttospeech_api::Texttospeech_apiService<'_> {
        texttospeech_api::Texttospeech_apiService::new(self)
    }
    /// Get texttospeech_api service handler
    pub fn texttospeech_api(&self) -> texttospeech_api::Texttospeech_apiService<'_> {
        texttospeech_api::Texttospeech_apiService::new(self)
    }
    /// Get marketingplatformadmin_api service handler
    pub fn marketingplatformadmin_api(&self) -> marketingplatformadmin_api::Marketingplatformadmin_apiService<'_> {
        marketingplatformadmin_api::Marketingplatformadmin_apiService::new(self)
    }
    /// Get cloudprivatecatalogproducer_api service handler
    pub fn cloudprivatecatalogproducer_api(&self) -> cloudprivatecatalogproducer_api::Cloudprivatecatalogproducer_apiService<'_> {
        cloudprivatecatalogproducer_api::Cloudprivatecatalogproducer_apiService::new(self)
    }
    /// Get sasportal_api service handler
    pub fn sasportal_api(&self) -> sasportal_api::Sasportal_apiService<'_> {
        sasportal_api::Sasportal_apiService::new(self)
    }
    /// Get spanner_api service handler
    pub fn spanner_api(&self) -> spanner_api::Spanner_apiService<'_> {
        spanner_api::Spanner_apiService::new(self)
    }
    /// Get clouddeploy_api service handler
    pub fn clouddeploy_api(&self) -> clouddeploy_api::Clouddeploy_apiService<'_> {
        clouddeploy_api::Clouddeploy_apiService::new(self)
    }
    /// Get biglake_api service handler
    pub fn biglake_api(&self) -> biglake_api::Biglake_apiService<'_> {
        biglake_api::Biglake_apiService::new(self)
    }
    /// Get serviceusage_api service handler
    pub fn serviceusage_api(&self) -> serviceusage_api::Serviceusage_apiService<'_> {
        serviceusage_api::Serviceusage_apiService::new(self)
    }
    /// Get serviceusage_api service handler
    pub fn serviceusage_api(&self) -> serviceusage_api::Serviceusage_apiService<'_> {
        serviceusage_api::Serviceusage_apiService::new(self)
    }
    /// Get chromemanagement_api service handler
    pub fn chromemanagement_api(&self) -> chromemanagement_api::Chromemanagement_apiService<'_> {
        chromemanagement_api::Chromemanagement_apiService::new(self)
    }
    /// Get fitness_api service handler
    pub fn fitness_api(&self) -> fitness_api::Fitness_apiService<'_> {
        fitness_api::Fitness_apiService::new(self)
    }
    /// Get plusdomains_api service handler
    pub fn plusdomains_api(&self) -> plusdomains_api::Plusdomains_apiService<'_> {
        plusdomains_api::Plusdomains_apiService::new(self)
    }
    /// Get androiddeviceprovisioning_api service handler
    pub fn androiddeviceprovisioning_api(&self) -> androiddeviceprovisioning_api::Androiddeviceprovisioning_apiService<'_> {
        androiddeviceprovisioning_api::Androiddeviceprovisioning_apiService::new(self)
    }
    /// Get securesourcemanager_api service handler
    pub fn securesourcemanager_api(&self) -> securesourcemanager_api::Securesourcemanager_apiService<'_> {
        securesourcemanager_api::Securesourcemanager_apiService::new(self)
    }
    /// Get kmsinventory_api service handler
    pub fn kmsinventory_api(&self) -> kmsinventory_api::Kmsinventory_apiService<'_> {
        kmsinventory_api::Kmsinventory_apiService::new(self)
    }
    /// Get cloudcontrolspartner_api service handler
    pub fn cloudcontrolspartner_api(&self) -> cloudcontrolspartner_api::Cloudcontrolspartner_apiService<'_> {
        cloudcontrolspartner_api::Cloudcontrolspartner_apiService::new(self)
    }
    /// Get cloudcontrolspartner_api service handler
    pub fn cloudcontrolspartner_api(&self) -> cloudcontrolspartner_api::Cloudcontrolspartner_apiService<'_> {
        cloudcontrolspartner_api::Cloudcontrolspartner_apiService::new(self)
    }
    /// Get bigquerydatapolicy_api service handler
    pub fn bigquerydatapolicy_api(&self) -> bigquerydatapolicy_api::Bigquerydatapolicy_apiService<'_> {
        bigquerydatapolicy_api::Bigquerydatapolicy_apiService::new(self)
    }
    /// Get bigquerydatapolicy_api service handler
    pub fn bigquerydatapolicy_api(&self) -> bigquerydatapolicy_api::Bigquerydatapolicy_apiService<'_> {
        bigquerydatapolicy_api::Bigquerydatapolicy_apiService::new(self)
    }
    /// Get licensing_api service handler
    pub fn licensing_api(&self) -> licensing_api::Licensing_apiService<'_> {
        licensing_api::Licensing_apiService::new(self)
    }
    /// Get workspaceevents_api service handler
    pub fn workspaceevents_api(&self) -> workspaceevents_api::Workspaceevents_apiService<'_> {
        workspaceevents_api::Workspaceevents_apiService::new(self)
    }
    /// Get replicapoolupdater_api service handler
    pub fn replicapoolupdater_api(&self) -> replicapoolupdater_api::Replicapoolupdater_apiService<'_> {
        replicapoolupdater_api::Replicapoolupdater_apiService::new(self)
    }
    /// Get indexing_api service handler
    pub fn indexing_api(&self) -> indexing_api::Indexing_apiService<'_> {
        indexing_api::Indexing_apiService::new(self)
    }
    /// Get alloydb_api service handler
    pub fn alloydb_api(&self) -> alloydb_api::Alloydb_apiService<'_> {
        alloydb_api::Alloydb_apiService::new(self)
    }
    /// Get alloydb_api service handler
    pub fn alloydb_api(&self) -> alloydb_api::Alloydb_apiService<'_> {
        alloydb_api::Alloydb_apiService::new(self)
    }
    /// Get alloydb_api service handler
    pub fn alloydb_api(&self) -> alloydb_api::Alloydb_apiService<'_> {
        alloydb_api::Alloydb_apiService::new(self)
    }
    /// Get spectrum_api service handler
    pub fn spectrum_api(&self) -> spectrum_api::Spectrum_apiService<'_> {
        spectrum_api::Spectrum_apiService::new(self)
    }
    /// Get firebaseml_api service handler
    pub fn firebaseml_api(&self) -> firebaseml_api::Firebaseml_apiService<'_> {
        firebaseml_api::Firebaseml_apiService::new(self)
    }
    /// Get firebaseml_api service handler
    pub fn firebaseml_api(&self) -> firebaseml_api::Firebaseml_apiService<'_> {
        firebaseml_api::Firebaseml_apiService::new(self)
    }
    /// Get firebaseml_api service handler
    pub fn firebaseml_api(&self) -> firebaseml_api::Firebaseml_apiService<'_> {
        firebaseml_api::Firebaseml_apiService::new(self)
    }
    /// Get tpu_api service handler
    pub fn tpu_api(&self) -> tpu_api::Tpu_apiService<'_> {
        tpu_api::Tpu_apiService::new(self)
    }
    /// Get tpu_api service handler
    pub fn tpu_api(&self) -> tpu_api::Tpu_apiService<'_> {
        tpu_api::Tpu_apiService::new(self)
    }
    /// Get tpu_api service handler
    pub fn tpu_api(&self) -> tpu_api::Tpu_apiService<'_> {
        tpu_api::Tpu_apiService::new(self)
    }
    /// Get tpu_api service handler
    pub fn tpu_api(&self) -> tpu_api::Tpu_apiService<'_> {
        tpu_api::Tpu_apiService::new(self)
    }
    /// Get domainsrdap_api service handler
    pub fn domainsrdap_api(&self) -> domainsrdap_api::Domainsrdap_apiService<'_> {
        domainsrdap_api::Domainsrdap_apiService::new(self)
    }
    /// Get playablelocations_api service handler
    pub fn playablelocations_api(&self) -> playablelocations_api::Playablelocations_apiService<'_> {
        playablelocations_api::Playablelocations_apiService::new(self)
    }
    /// Get groupsmigration_api service handler
    pub fn groupsmigration_api(&self) -> groupsmigration_api::Groupsmigration_apiService<'_> {
        groupsmigration_api::Groupsmigration_apiService::new(self)
    }
    /// Get androidmanagement_api service handler
    pub fn androidmanagement_api(&self) -> androidmanagement_api::Androidmanagement_apiService<'_> {
        androidmanagement_api::Androidmanagement_apiService::new(self)
    }
    /// Get run_api service handler
    pub fn run_api(&self) -> run_api::Run_apiService<'_> {
        run_api::Run_apiService::new(self)
    }
    /// Get run_api service handler
    pub fn run_api(&self) -> run_api::Run_apiService<'_> {
        run_api::Run_apiService::new(self)
    }
    /// Get run_api service handler
    pub fn run_api(&self) -> run_api::Run_apiService<'_> {
        run_api::Run_apiService::new(self)
    }
    /// Get run_api service handler
    pub fn run_api(&self) -> run_api::Run_apiService<'_> {
        run_api::Run_apiService::new(self)
    }
    /// Get cloudcommerceprocurement_api service handler
    pub fn cloudcommerceprocurement_api(&self) -> cloudcommerceprocurement_api::Cloudcommerceprocurement_apiService<'_> {
        cloudcommerceprocurement_api::Cloudcommerceprocurement_apiService::new(self)
    }
    /// Get jobs_api service handler
    pub fn jobs_api(&self) -> jobs_api::Jobs_apiService<'_> {
        jobs_api::Jobs_apiService::new(self)
    }
    /// Get jobs_api service handler
    pub fn jobs_api(&self) -> jobs_api::Jobs_apiService<'_> {
        jobs_api::Jobs_apiService::new(self)
    }
    /// Get jobs_api service handler
    pub fn jobs_api(&self) -> jobs_api::Jobs_apiService<'_> {
        jobs_api::Jobs_apiService::new(self)
    }
    /// Get jobs_api service handler
    pub fn jobs_api(&self) -> jobs_api::Jobs_apiService<'_> {
        jobs_api::Jobs_apiService::new(self)
    }
    /// Get networksecurity_api service handler
    pub fn networksecurity_api(&self) -> networksecurity_api::Networksecurity_apiService<'_> {
        networksecurity_api::Networksecurity_apiService::new(self)
    }
    /// Get networksecurity_api service handler
    pub fn networksecurity_api(&self) -> networksecurity_api::Networksecurity_apiService<'_> {
        networksecurity_api::Networksecurity_apiService::new(self)
    }
    /// Get clouddebugger_api service handler
    pub fn clouddebugger_api(&self) -> clouddebugger_api::Clouddebugger_apiService<'_> {
        clouddebugger_api::Clouddebugger_apiService::new(self)
    }
    /// Get monitoring_api service handler
    pub fn monitoring_api(&self) -> monitoring_api::Monitoring_apiService<'_> {
        monitoring_api::Monitoring_apiService::new(self)
    }
    /// Get monitoring_api service handler
    pub fn monitoring_api(&self) -> monitoring_api::Monitoring_apiService<'_> {
        monitoring_api::Monitoring_apiService::new(self)
    }
    /// Get youtubeanalytics_api service handler
    pub fn youtubeanalytics_api(&self) -> youtubeanalytics_api::Youtubeanalytics_apiService<'_> {
        youtubeanalytics_api::Youtubeanalytics_apiService::new(self)
    }
    /// Get youtubeanalytics_api service handler
    pub fn youtubeanalytics_api(&self) -> youtubeanalytics_api::Youtubeanalytics_apiService<'_> {
        youtubeanalytics_api::Youtubeanalytics_apiService::new(self)
    }
    /// Get youtubeanalytics_api service handler
    pub fn youtubeanalytics_api(&self) -> youtubeanalytics_api::Youtubeanalytics_apiService<'_> {
        youtubeanalytics_api::Youtubeanalytics_apiService::new(self)
    }
    /// Get policytroubleshooter_api service handler
    pub fn policytroubleshooter_api(&self) -> policytroubleshooter_api::Policytroubleshooter_apiService<'_> {
        policytroubleshooter_api::Policytroubleshooter_apiService::new(self)
    }
    /// Get policytroubleshooter_api service handler
    pub fn policytroubleshooter_api(&self) -> policytroubleshooter_api::Policytroubleshooter_apiService<'_> {
        policytroubleshooter_api::Policytroubleshooter_apiService::new(self)
    }
    /// Get assuredworkloads_api service handler
    pub fn assuredworkloads_api(&self) -> assuredworkloads_api::Assuredworkloads_apiService<'_> {
        assuredworkloads_api::Assuredworkloads_apiService::new(self)
    }
    /// Get assuredworkloads_api service handler
    pub fn assuredworkloads_api(&self) -> assuredworkloads_api::Assuredworkloads_apiService<'_> {
        assuredworkloads_api::Assuredworkloads_apiService::new(self)
    }
    /// Get versionhistory_api service handler
    pub fn versionhistory_api(&self) -> versionhistory_api::Versionhistory_apiService<'_> {
        versionhistory_api::Versionhistory_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get merchantapi_api service handler
    pub fn merchantapi_api(&self) -> merchantapi_api::Merchantapi_apiService<'_> {
        merchantapi_api::Merchantapi_apiService::new(self)
    }
    /// Get gamesconfiguration_api service handler
    pub fn gamesconfiguration_api(&self) -> gamesconfiguration_api::Gamesconfiguration_apiService<'_> {
        gamesconfiguration_api::Gamesconfiguration_apiService::new(self)
    }
    /// Get cloudscheduler_api service handler
    pub fn cloudscheduler_api(&self) -> cloudscheduler_api::Cloudscheduler_apiService<'_> {
        cloudscheduler_api::Cloudscheduler_apiService::new(self)
    }
    /// Get cloudscheduler_api service handler
    pub fn cloudscheduler_api(&self) -> cloudscheduler_api::Cloudscheduler_apiService<'_> {
        cloudscheduler_api::Cloudscheduler_apiService::new(self)
    }
    /// Get vmwareengine_api service handler
    pub fn vmwareengine_api(&self) -> vmwareengine_api::Vmwareengine_apiService<'_> {
        vmwareengine_api::Vmwareengine_apiService::new(self)
    }
    /// Get drive_api service handler
    pub fn drive_api(&self) -> drive_api::Drive_apiService<'_> {
        drive_api::Drive_apiService::new(self)
    }
    /// Get drive_api service handler
    pub fn drive_api(&self) -> drive_api::Drive_apiService<'_> {
        drive_api::Drive_apiService::new(self)
    }
    /// Get dns_api service handler
    pub fn dns_api(&self) -> dns_api::Dns_apiService<'_> {
        dns_api::Dns_apiService::new(self)
    }
    /// Get dns_api service handler
    pub fn dns_api(&self) -> dns_api::Dns_apiService<'_> {
        dns_api::Dns_apiService::new(self)
    }
    /// Get dns_api service handler
    pub fn dns_api(&self) -> dns_api::Dns_apiService<'_> {
        dns_api::Dns_apiService::new(self)
    }
    /// Get dns_api service handler
    pub fn dns_api(&self) -> dns_api::Dns_apiService<'_> {
        dns_api::Dns_apiService::new(self)
    }
    /// Get analyticsreporting_api service handler
    pub fn analyticsreporting_api(&self) -> analyticsreporting_api::Analyticsreporting_apiService<'_> {
        analyticsreporting_api::Analyticsreporting_apiService::new(self)
    }
    /// Get blogger_api service handler
    pub fn blogger_api(&self) -> blogger_api::Blogger_apiService<'_> {
        blogger_api::Blogger_apiService::new(self)
    }
    /// Get blogger_api service handler
    pub fn blogger_api(&self) -> blogger_api::Blogger_apiService<'_> {
        blogger_api::Blogger_apiService::new(self)
    }
    /// Get replicapool_api service handler
    pub fn replicapool_api(&self) -> replicapool_api::Replicapool_apiService<'_> {
        replicapool_api::Replicapool_apiService::new(self)
    }
    /// Get mybusinessplaceactions_api service handler
    pub fn mybusinessplaceactions_api(&self) -> mybusinessplaceactions_api::Mybusinessplaceactions_apiService<'_> {
        mybusinessplaceactions_api::Mybusinessplaceactions_apiService::new(self)
    }
    /// Get consumersurveys_api service handler
    pub fn consumersurveys_api(&self) -> consumersurveys_api::Consumersurveys_apiService<'_> {
        consumersurveys_api::Consumersurveys_apiService::new(self)
    }
    /// Get games_api service handler
    pub fn games_api(&self) -> games_api::Games_apiService<'_> {
        games_api::Games_apiService::new(self)
    }
    /// Get gmailpostmastertools_api service handler
    pub fn gmailpostmastertools_api(&self) -> gmailpostmastertools_api::Gmailpostmastertools_apiService<'_> {
        gmailpostmastertools_api::Gmailpostmastertools_apiService::new(self)
    }
    /// Get gmailpostmastertools_api service handler
    pub fn gmailpostmastertools_api(&self) -> gmailpostmastertools_api::Gmailpostmastertools_apiService<'_> {
        gmailpostmastertools_api::Gmailpostmastertools_apiService::new(self)
    }
    /// Get oracledatabase_api service handler
    pub fn oracledatabase_api(&self) -> oracledatabase_api::Oracledatabase_apiService<'_> {
        oracledatabase_api::Oracledatabase_apiService::new(self)
    }
    /// Get dataform_api service handler
    pub fn dataform_api(&self) -> dataform_api::Dataform_apiService<'_> {
        dataform_api::Dataform_apiService::new(self)
    }
    /// Get dataform_api service handler
    pub fn dataform_api(&self) -> dataform_api::Dataform_apiService<'_> {
        dataform_api::Dataform_apiService::new(self)
    }
    /// Get authorizedbuyersmarketplace_api service handler
    pub fn authorizedbuyersmarketplace_api(&self) -> authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService<'_> {
        authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService::new(self)
    }
    /// Get authorizedbuyersmarketplace_api service handler
    pub fn authorizedbuyersmarketplace_api(&self) -> authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService<'_> {
        authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService::new(self)
    }
    /// Get authorizedbuyersmarketplace_api service handler
    pub fn authorizedbuyersmarketplace_api(&self) -> authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService<'_> {
        authorizedbuyersmarketplace_api::Authorizedbuyersmarketplace_apiService::new(self)
    }
    /// Get serviceconsumermanagement_api service handler
    pub fn serviceconsumermanagement_api(&self) -> serviceconsumermanagement_api::Serviceconsumermanagement_apiService<'_> {
        serviceconsumermanagement_api::Serviceconsumermanagement_apiService::new(self)
    }
    /// Get serviceconsumermanagement_api service handler
    pub fn serviceconsumermanagement_api(&self) -> serviceconsumermanagement_api::Serviceconsumermanagement_apiService<'_> {
        serviceconsumermanagement_api::Serviceconsumermanagement_apiService::new(self)
    }
    /// Get notebooks_api service handler
    pub fn notebooks_api(&self) -> notebooks_api::Notebooks_apiService<'_> {
        notebooks_api::Notebooks_apiService::new(self)
    }
    /// Get notebooks_api service handler
    pub fn notebooks_api(&self) -> notebooks_api::Notebooks_apiService<'_> {
        notebooks_api::Notebooks_apiService::new(self)
    }
    /// Get parametermanager_api service handler
    pub fn parametermanager_api(&self) -> parametermanager_api::Parametermanager_apiService<'_> {
        parametermanager_api::Parametermanager_apiService::new(self)
    }
    /// Get logging_api service handler
    pub fn logging_api(&self) -> logging_api::Logging_apiService<'_> {
        logging_api::Logging_apiService::new(self)
    }
    /// Get logging_api service handler
    pub fn logging_api(&self) -> logging_api::Logging_apiService<'_> {
        logging_api::Logging_apiService::new(self)
    }
    /// Get adexperiencereport_api service handler
    pub fn adexperiencereport_api(&self) -> adexperiencereport_api::Adexperiencereport_apiService<'_> {
        adexperiencereport_api::Adexperiencereport_apiService::new(self)
    }
    /// Get bigquerydatatransfer_api service handler
    pub fn bigquerydatatransfer_api(&self) -> bigquerydatatransfer_api::Bigquerydatatransfer_apiService<'_> {
        bigquerydatatransfer_api::Bigquerydatatransfer_apiService::new(self)
    }
    /// Get datacatalog_api service handler
    pub fn datacatalog_api(&self) -> datacatalog_api::Datacatalog_apiService<'_> {
        datacatalog_api::Datacatalog_apiService::new(self)
    }
    /// Get datacatalog_api service handler
    pub fn datacatalog_api(&self) -> datacatalog_api::Datacatalog_apiService<'_> {
        datacatalog_api::Datacatalog_apiService::new(self)
    }
    /// Get cloudshell_api service handler
    pub fn cloudshell_api(&self) -> cloudshell_api::Cloudshell_apiService<'_> {
        cloudshell_api::Cloudshell_apiService::new(self)
    }
    /// Get cloudshell_api service handler
    pub fn cloudshell_api(&self) -> cloudshell_api::Cloudshell_apiService<'_> {
        cloudshell_api::Cloudshell_apiService::new(self)
    }
    /// Get cloudchannel_api service handler
    pub fn cloudchannel_api(&self) -> cloudchannel_api::Cloudchannel_apiService<'_> {
        cloudchannel_api::Cloudchannel_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get osconfig_api service handler
    pub fn osconfig_api(&self) -> osconfig_api::Osconfig_apiService<'_> {
        osconfig_api::Osconfig_apiService::new(self)
    }
    /// Get migrationcenter_api service handler
    pub fn migrationcenter_api(&self) -> migrationcenter_api::Migrationcenter_apiService<'_> {
        migrationcenter_api::Migrationcenter_apiService::new(self)
    }
    /// Get migrationcenter_api service handler
    pub fn migrationcenter_api(&self) -> migrationcenter_api::Migrationcenter_apiService<'_> {
        migrationcenter_api::Migrationcenter_apiService::new(self)
    }
    /// Get gmail_api service handler
    pub fn gmail_api(&self) -> gmail_api::Gmail_apiService<'_> {
        gmail_api::Gmail_apiService::new(self)
    }
    /// Get workflowexecutions_api service handler
    pub fn workflowexecutions_api(&self) -> workflowexecutions_api::Workflowexecutions_apiService<'_> {
        workflowexecutions_api::Workflowexecutions_apiService::new(self)
    }
    /// Get workflowexecutions_api service handler
    pub fn workflowexecutions_api(&self) -> workflowexecutions_api::Workflowexecutions_apiService<'_> {
        workflowexecutions_api::Workflowexecutions_apiService::new(self)
    }
    /// Get apigateway_api service handler
    pub fn apigateway_api(&self) -> apigateway_api::Apigateway_apiService<'_> {
        apigateway_api::Apigateway_apiService::new(self)
    }
    /// Get apigateway_api service handler
    pub fn apigateway_api(&self) -> apigateway_api::Apigateway_apiService<'_> {
        apigateway_api::Apigateway_apiService::new(self)
    }
    /// Get apigateway_api service handler
    pub fn apigateway_api(&self) -> apigateway_api::Apigateway_apiService<'_> {
        apigateway_api::Apigateway_apiService::new(self)
    }
    /// Get apigateway_api service handler
    pub fn apigateway_api(&self) -> apigateway_api::Apigateway_apiService<'_> {
        apigateway_api::Apigateway_apiService::new(self)
    }
    /// Get businessprofileperformance_api service handler
    pub fn businessprofileperformance_api(&self) -> businessprofileperformance_api::Businessprofileperformance_apiService<'_> {
        businessprofileperformance_api::Businessprofileperformance_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get metastore_api service handler
    pub fn metastore_api(&self) -> metastore_api::Metastore_apiService<'_> {
        metastore_api::Metastore_apiService::new(self)
    }
    /// Get pubsublite_api service handler
    pub fn pubsublite_api(&self) -> pubsublite_api::Pubsublite_apiService<'_> {
        pubsublite_api::Pubsublite_apiService::new(self)
    }
    /// Get vault_api service handler
    pub fn vault_api(&self) -> vault_api::Vault_apiService<'_> {
        vault_api::Vault_apiService::new(self)
    }
    /// Get cloudsupport_api service handler
    pub fn cloudsupport_api(&self) -> cloudsupport_api::Cloudsupport_apiService<'_> {
        cloudsupport_api::Cloudsupport_apiService::new(self)
    }
    /// Get cloudsupport_api service handler
    pub fn cloudsupport_api(&self) -> cloudsupport_api::Cloudsupport_apiService<'_> {
        cloudsupport_api::Cloudsupport_apiService::new(self)
    }
    /// Get analytics_api service handler
    pub fn analytics_api(&self) -> analytics_api::Analytics_apiService<'_> {
        analytics_api::Analytics_apiService::new(self)
    }
    /// Get analytics_api service handler
    pub fn analytics_api(&self) -> analytics_api::Analytics_apiService<'_> {
        analytics_api::Analytics_apiService::new(self)
    }
    /// Get sts_api service handler
    pub fn sts_api(&self) -> sts_api::Sts_apiService<'_> {
        sts_api::Sts_apiService::new(self)
    }
    /// Get sts_api service handler
    pub fn sts_api(&self) -> sts_api::Sts_apiService<'_> {
        sts_api::Sts_apiService::new(self)
    }
    /// Get certificatemanager_api service handler
    pub fn certificatemanager_api(&self) -> certificatemanager_api::Certificatemanager_apiService<'_> {
        certificatemanager_api::Certificatemanager_apiService::new(self)
    }
    /// Get poly_api service handler
    pub fn poly_api(&self) -> poly_api::Poly_apiService<'_> {
        poly_api::Poly_apiService::new(self)
    }
    /// Get firebaserules_api service handler
    pub fn firebaserules_api(&self) -> firebaserules_api::Firebaserules_apiService<'_> {
        firebaserules_api::Firebaserules_apiService::new(self)
    }
    /// Get plus_api service handler
    pub fn plus_api(&self) -> plus_api::Plus_apiService<'_> {
        plus_api::Plus_apiService::new(self)
    }
    /// Get dataproc_api service handler
    pub fn dataproc_api(&self) -> dataproc_api::Dataproc_apiService<'_> {
        dataproc_api::Dataproc_apiService::new(self)
    }
    /// Get dataproc_api service handler
    pub fn dataproc_api(&self) -> dataproc_api::Dataproc_apiService<'_> {
        dataproc_api::Dataproc_apiService::new(self)
    }
    /// Get mirror_api service handler
    pub fn mirror_api(&self) -> mirror_api::Mirror_apiService<'_> {
        mirror_api::Mirror_apiService::new(self)
    }
    /// Get baremetalsolution_api service handler
    pub fn baremetalsolution_api(&self) -> baremetalsolution_api::Baremetalsolution_apiService<'_> {
        baremetalsolution_api::Baremetalsolution_apiService::new(self)
    }
    /// Get baremetalsolution_api service handler
    pub fn baremetalsolution_api(&self) -> baremetalsolution_api::Baremetalsolution_apiService<'_> {
        baremetalsolution_api::Baremetalsolution_apiService::new(self)
    }
    /// Get baremetalsolution_api service handler
    pub fn baremetalsolution_api(&self) -> baremetalsolution_api::Baremetalsolution_apiService<'_> {
        baremetalsolution_api::Baremetalsolution_apiService::new(self)
    }
    /// Get managedidentities_api service handler
    pub fn managedidentities_api(&self) -> managedidentities_api::Managedidentities_apiService<'_> {
        managedidentities_api::Managedidentities_apiService::new(self)
    }
    /// Get managedidentities_api service handler
    pub fn managedidentities_api(&self) -> managedidentities_api::Managedidentities_apiService<'_> {
        managedidentities_api::Managedidentities_apiService::new(self)
    }
    /// Get managedidentities_api service handler
    pub fn managedidentities_api(&self) -> managedidentities_api::Managedidentities_apiService<'_> {
        managedidentities_api::Managedidentities_apiService::new(self)
    }
    /// Get sql_api service handler
    pub fn sql_api(&self) -> sql_api::Sql_apiService<'_> {
        sql_api::Sql_apiService::new(self)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
    }
}
