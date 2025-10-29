//! Gcp Provider for Hemmer
//!
//! Auto-generated unified provider from gcp SDK version v1
//!
//! This provider includes multiple services:
//! - factchecktools
//! - areainsights
//! - billingbudgets
//! - billingbudgets
//! - dataflow
//! - playdeveloperreporting
//! - playdeveloperreporting
//! - iamcredentials
//! - privateca
//! - privateca
//! - solar
//! - cloudfunctions
//! - cloudfunctions
//! - cloudfunctions
//! - cloudfunctions
//! - cloudfunctions
//! - essentialcontacts
//! - orgpolicy
//! - remotebuildexecution
//! - remotebuildexecution
//! - remotebuildexecution
//! - urlshortener
//! - workstations
//! - workstations
//! - webrisk
//! - keep
//! - datastream
//! - datastream
//! - servicenetworking
//! - servicenetworking
//! - binaryauthorization
//! - binaryauthorization
//! - area120tables
//! - connectors
//! - connectors
//! - apihub
//! - apigeeregistry
//! - beyondcorp
//! - beyondcorp
//! - toolresults
//! - toolresults
//! - clouderrorreporting
//! - civicinfo
//! - apigee
//! - mybusinessverifications
//! - developerconnect
//! - datapipelines
//! - firebasedynamiclinks
//! - runtimeconfig
//! - runtimeconfig
//! - storagebatchoperations
//! - tracing
//! - adexchangebuyer2
//! - saasservicemgmt
//! - proximitybeacon
//! - resourcesettings
//! - datalineage
//! - libraryagent
//! - acmedns
//! - publicca
//! - publicca
//! - publicca
//! - networkservices
//! - networkservices
//! - netapp
//! - netapp
//! - aiplatform
//! - aiplatform
//! - pagespeedonline
//! - pagespeedonline
//! - pagespeedonline
//! - pagespeedonline
//! - tasks
//! - sqladmin
//! - sqladmin
//! - discoveryengine
//! - discoveryengine
//! - discoveryengine
//! - gkeonprem
//! - cloudlocationfinder
//! - cloudlocationfinder
//! - mybusinesslodging
//! - mybusinessaccountmanagement
//! - searchads360
//! - alertcenter
//! - servicemanagement
//! - smartdevicemanagement
//! - domains
//! - domains
//! - domains
//! - calendar
//! - vpcaccess
//! - vpcaccess
//! - sheets
//! - looker
//! - forms
//! - cloudiot
//! - genomics
//! - genomics
//! - genomics
//! - pollen
//! - playgrouping
//! - oslogin
//! - oslogin
//! - oslogin
//! - gamesmanagement
//! - gkehub
//! - gkehub
//! - gkehub
//! - gkehub
//! - gkehub
//! - gkehub
//! - gkehub
//! - gkehub
//! - securityposture
//! - networkmanagement
//! - networkmanagement
//! - securitycenter
//! - securitycenter
//! - securitycenter
//! - securitycenter
//! - securitycenter
//! - datamanager
//! - chat
//! - apphub
//! - apphub
//! - dialogflow
//! - dialogflow
//! - dialogflow
//! - dialogflow
//! - dialogflow
//! - rapidmigrationassessment
//! - documentai
//! - documentai
//! - documentai
//! - artifactregistry
//! - artifactregistry
//! - artifactregistry
//! - qpxexpress
//! - analyticsdata
//! - analyticsdata
//! - file
//! - file
//! - cloudresourcemanager
//! - cloudresourcemanager
//! - cloudresourcemanager
//! - cloudresourcemanager
//! - cloudresourcemanager
//! - css
//! - walletobjects
//! - readerrevenuesubscriptionlinking
//! - youtubereporting
//! - firestore
//! - firestore
//! - firestore
//! - config
//! - translate
//! - translate
//! - translate
//! - redis
//! - redis
//! - meet
//! - dataplex
//! - analyticshub
//! - analyticshub
//! - playintegrity
//! - cloudkms
//! - fusiontables
//! - fusiontables
//! - accessapproval
//! - checks
//! - cloudprivatecatalog
//! - addressvalidation
//! - chromewebstore
//! - chromewebstore
//! - websecurityscanner
//! - websecurityscanner
//! - websecurityscanner
//! - places
//! - recaptchaenterprise
//! - parallelstore
//! - parallelstore
//! - contactcenterinsights
//! - recommendationengine
//! - cloudidentity
//! - cloudidentity
//! - healthcare
//! - healthcare
//! - healthcare
//! - healthcare
//! - composer
//! - composer
//! - searchconsole
//! - appengine
//! - appengine
//! - appengine
//! - appengine
//! - appengine
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - dfareporting
//! - apikeys
//! - bigqueryconnection
//! - bigqueryconnection
//! - firebasedataconnect
//! - firebasedataconnect
//! - commentanalyzer
//! - firebaseapphosting
//! - firebaseapphosting
//! - doubleclickbidmanager
//! - doubleclickbidmanager
//! - doubleclickbidmanager
//! - fcmdata
//! - firebasedatabase
//! - ideahub
//! - ideahub
//! - firebaseremoteconfig
//! - prod_tt_sasportal
//! - firebaseappdistribution
//! - firebaseappdistribution
//! - deploymentmanager
//! - deploymentmanager
//! - deploymentmanager
//! - datafusion
//! - datafusion
//! - tagmanager
//! - tagmanager
//! - discovery
//! - storagetransfer
//! - oauth2
//! - oauth2
//! - managedkafka
//! - admin
//! - admin
//! - admin
//! - bigqueryreservation
//! - bigqueryreservation
//! - bigqueryreservation
//! - vmmigration
//! - vmmigration
//! - contactcenteraiplatform
//! - doubleclicksearch
//! - backupdr
//! - ondemandscanning
//! - ondemandscanning
//! - cloudbuild
//! - cloudbuild
//! - cloudbuild
//! - cloudbuild
//! - cloudbuild
//! - policyanalyzer
//! - policyanalyzer
//! - workflows
//! - workflows
//! - secretmanager
//! - secretmanager
//! - secretmanager
//! - transcoder
//! - transcoder
//! - accesscontextmanager
//! - accesscontextmanager
//! - content
//! - content
//! - content
//! - acceleratedmobilepageurl
//! - androidpublisher
//! - androidpublisher
//! - androidpublisher
//! - digitalassetlinks
//! - classroom
//! - chromeuxreport
//! - admob
//! - admob
//! - gameservices
//! - gameservices
//! - language
//! - language
//! - language
//! - language
//! - retail
//! - retail
//! - retail
//! - bigquery
//! - driveactivity
//! - serviceuser
//! - homegraph
//! - vision
//! - vision
//! - vision
//! - firebaseappcheck
//! - firebaseappcheck
//! - appsactivity
//! - cloudsearch
//! - books
//! - docs
//! - firebasehosting
//! - firebasehosting
//! - youtube
//! - observability
//! - adsensehost
//! - lifesciences
//! - adexchangebuyer
//! - adexchangebuyer
//! - adexchangebuyer
//! - streetviewpublish
//! - advisorynotifications
//! - sourcerepo
//! - vectortile
//! - paymentsresellersubscription
//! - airquality
//! - localservices
//! - adexchangeseller
//! - adexchangeseller
//! - adexchangeseller
//! - servicebroker
//! - servicebroker
//! - servicebroker
//! - speech
//! - speech
//! - speech
//! - speech
//! - speech
//! - script
//! - realtimebidding
//! - realtimebidding
//! - chromepolicy
//! - storage
//! - storage
//! - storage
//! - slides
//! - reseller
//! - mybusinessbusinessinformation
//! - androidenterprise
//! - ids
//! - adsense
//! - adsense
//! - adsense
//! - videointelligence
//! - videointelligence
//! - videointelligence
//! - videointelligence
//! - videointelligence
//! - datamigration
//! - datamigration
//! - gkebackup
//! - adsenseplatform
//! - adsenseplatform
//! - integrations
//! - integrations
//! - customsearch
//! - kgsearch
//! - testing
//! - container
//! - container
//! - datastore
//! - datastore
//! - datastore
//! - playmoviespartner
//! - memcache
//! - memcache
//! - blockchainnodeengine
//! - cloudtrace
//! - cloudtrace
//! - cloudtrace
//! - iap
//! - iap
//! - playcustomapp
//! - dataportability
//! - dataportability
//! - mybusinessbusinesscalls
//! - policysimulator
//! - policysimulator
//! - policysimulator
//! - policysimulator
//! - firebase
//! - iam
//! - iam
//! - iam
//! - bigtableadmin
//! - bigtableadmin
//! - servicedirectory
//! - servicedirectory
//! - recommender
//! - recommender
//! - trafficdirector
//! - trafficdirector
//! - drivelabels
//! - drivelabels
//! - cloudasset
//! - cloudasset
//! - cloudasset
//! - cloudasset
//! - cloudasset
//! - cloudasset
//! - cloudbilling
//! - cloudbilling
//! - webmasters
//! - compute
//! - compute
//! - compute
//! - groupssettings
//! - workloadmanager
//! - abusiveexperiencereport
//! - displayvideo
//! - displayvideo
//! - displayvideo
//! - displayvideo
//! - displayvideo
//! - displayvideo
//! - displayvideo
//! - ml
//! - networkconnectivity
//! - networkconnectivity
//! - batch
//! - webfonts
//! - partners
//! - cloudprofiler
//! - eventarc
//! - eventarc
//! - contentwarehouse
//! - siteverification
//! - analyticsadmin
//! - analyticsadmin
//! - verifiedaccess
//! - verifiedaccess
//! - safebrowsing
//! - safebrowsing
//! - people
//! - fcm
//! - firebasestorage
//! - manufacturers
//! - appstate
//! - servicecontrol
//! - servicecontrol
//! - datalabeling
//! - dlp
//! - mybusinessnotifications
//! - containeranalysis
//! - containeranalysis
//! - containeranalysis
//! - mybusinessqanda
//! - cloudtasks
//! - cloudtasks
//! - cloudtasks
//! - apim
//! - surveys
//! - pubsub
//! - pubsub
//! - pubsub
//! - texttospeech
//! - texttospeech
//! - marketingplatformadmin
//! - cloudprivatecatalogproducer
//! - sasportal
//! - spanner
//! - clouddeploy
//! - biglake
//! - serviceusage
//! - serviceusage
//! - chromemanagement
//! - fitness
//! - plusdomains
//! - androiddeviceprovisioning
//! - securesourcemanager
//! - kmsinventory
//! - cloudcontrolspartner
//! - cloudcontrolspartner
//! - bigquerydatapolicy
//! - bigquerydatapolicy
//! - licensing
//! - workspaceevents
//! - replicapoolupdater
//! - indexing
//! - alloydb
//! - alloydb
//! - alloydb
//! - spectrum
//! - firebaseml
//! - firebaseml
//! - firebaseml
//! - tpu
//! - tpu
//! - tpu
//! - tpu
//! - domainsrdap
//! - playablelocations
//! - groupsmigration
//! - androidmanagement
//! - run
//! - run
//! - run
//! - run
//! - cloudcommerceprocurement
//! - jobs
//! - jobs
//! - jobs
//! - jobs
//! - networksecurity
//! - networksecurity
//! - clouddebugger
//! - monitoring
//! - monitoring
//! - youtubeanalytics
//! - youtubeanalytics
//! - youtubeanalytics
//! - policytroubleshooter
//! - policytroubleshooter
//! - assuredworkloads
//! - assuredworkloads
//! - versionhistory
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - merchantapi
//! - gamesconfiguration
//! - cloudscheduler
//! - cloudscheduler
//! - vmwareengine
//! - drive
//! - drive
//! - dns
//! - dns
//! - dns
//! - dns
//! - analyticsreporting
//! - blogger
//! - blogger
//! - replicapool
//! - mybusinessplaceactions
//! - consumersurveys
//! - games
//! - gmailpostmastertools
//! - gmailpostmastertools
//! - oracledatabase
//! - dataform
//! - dataform
//! - authorizedbuyersmarketplace
//! - authorizedbuyersmarketplace
//! - authorizedbuyersmarketplace
//! - serviceconsumermanagement
//! - serviceconsumermanagement
//! - notebooks
//! - notebooks
//! - parametermanager
//! - logging
//! - logging
//! - adexperiencereport
//! - bigquerydatatransfer
//! - datacatalog
//! - datacatalog
//! - cloudshell
//! - cloudshell
//! - cloudchannel
//! - osconfig
//! - osconfig
//! - osconfig
//! - osconfig
//! - osconfig
//! - migrationcenter
//! - migrationcenter
//! - gmail
//! - workflowexecutions
//! - workflowexecutions
//! - apigateway
//! - apigateway
//! - apigateway
//! - apigateway
//! - businessprofileperformance
//! - metastore
//! - metastore
//! - metastore
//! - metastore
//! - metastore
//! - metastore
//! - pubsublite
//! - vault
//! - cloudsupport
//! - cloudsupport
//! - analytics
//! - analytics
//! - sts
//! - sts
//! - certificatemanager
//! - poly
//! - firebaserules
//! - plus
//! - dataproc
//! - dataproc
//! - mirror
//! - baremetalsolution
//! - baremetalsolution
//! - baremetalsolution
//! - managedidentities
//! - managedidentities
//! - managedidentities
//! - sql


pub mod factchecktools;
pub mod areainsights;
pub mod billingbudgets;
pub mod billingbudgets;
pub mod dataflow;
pub mod playdeveloperreporting;
pub mod playdeveloperreporting;
pub mod iamcredentials;
pub mod privateca;
pub mod privateca;
pub mod solar;
pub mod cloudfunctions;
pub mod cloudfunctions;
pub mod cloudfunctions;
pub mod cloudfunctions;
pub mod cloudfunctions;
pub mod essentialcontacts;
pub mod orgpolicy;
pub mod remotebuildexecution;
pub mod remotebuildexecution;
pub mod remotebuildexecution;
pub mod urlshortener;
pub mod workstations;
pub mod workstations;
pub mod webrisk;
pub mod keep;
pub mod datastream;
pub mod datastream;
pub mod servicenetworking;
pub mod servicenetworking;
pub mod binaryauthorization;
pub mod binaryauthorization;
pub mod area120tables;
pub mod connectors;
pub mod connectors;
pub mod apihub;
pub mod apigeeregistry;
pub mod beyondcorp;
pub mod beyondcorp;
pub mod toolresults;
pub mod toolresults;
pub mod clouderrorreporting;
pub mod civicinfo;
pub mod apigee;
pub mod mybusinessverifications;
pub mod developerconnect;
pub mod datapipelines;
pub mod firebasedynamiclinks;
pub mod runtimeconfig;
pub mod runtimeconfig;
pub mod storagebatchoperations;
pub mod tracing;
pub mod adexchangebuyer2;
pub mod saasservicemgmt;
pub mod proximitybeacon;
pub mod resourcesettings;
pub mod datalineage;
pub mod libraryagent;
pub mod acmedns;
pub mod publicca;
pub mod publicca;
pub mod publicca;
pub mod networkservices;
pub mod networkservices;
pub mod netapp;
pub mod netapp;
pub mod aiplatform;
pub mod aiplatform;
pub mod pagespeedonline;
pub mod pagespeedonline;
pub mod pagespeedonline;
pub mod pagespeedonline;
pub mod tasks;
pub mod sqladmin;
pub mod sqladmin;
pub mod discoveryengine;
pub mod discoveryengine;
pub mod discoveryengine;
pub mod gkeonprem;
pub mod cloudlocationfinder;
pub mod cloudlocationfinder;
pub mod mybusinesslodging;
pub mod mybusinessaccountmanagement;
pub mod searchads360;
pub mod alertcenter;
pub mod servicemanagement;
pub mod smartdevicemanagement;
pub mod domains;
pub mod domains;
pub mod domains;
pub mod calendar;
pub mod vpcaccess;
pub mod vpcaccess;
pub mod sheets;
pub mod looker;
pub mod forms;
pub mod cloudiot;
pub mod genomics;
pub mod genomics;
pub mod genomics;
pub mod pollen;
pub mod playgrouping;
pub mod oslogin;
pub mod oslogin;
pub mod oslogin;
pub mod gamesmanagement;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod gkehub;
pub mod securityposture;
pub mod networkmanagement;
pub mod networkmanagement;
pub mod securitycenter;
pub mod securitycenter;
pub mod securitycenter;
pub mod securitycenter;
pub mod securitycenter;
pub mod datamanager;
pub mod chat;
pub mod apphub;
pub mod apphub;
pub mod dialogflow;
pub mod dialogflow;
pub mod dialogflow;
pub mod dialogflow;
pub mod dialogflow;
pub mod rapidmigrationassessment;
pub mod documentai;
pub mod documentai;
pub mod documentai;
pub mod artifactregistry;
pub mod artifactregistry;
pub mod artifactregistry;
pub mod qpxexpress;
pub mod analyticsdata;
pub mod analyticsdata;
pub mod file;
pub mod file;
pub mod cloudresourcemanager;
pub mod cloudresourcemanager;
pub mod cloudresourcemanager;
pub mod cloudresourcemanager;
pub mod cloudresourcemanager;
pub mod css;
pub mod walletobjects;
pub mod readerrevenuesubscriptionlinking;
pub mod youtubereporting;
pub mod firestore;
pub mod firestore;
pub mod firestore;
pub mod config;
pub mod translate;
pub mod translate;
pub mod translate;
pub mod redis;
pub mod redis;
pub mod meet;
pub mod dataplex;
pub mod analyticshub;
pub mod analyticshub;
pub mod playintegrity;
pub mod cloudkms;
pub mod fusiontables;
pub mod fusiontables;
pub mod accessapproval;
pub mod checks;
pub mod cloudprivatecatalog;
pub mod addressvalidation;
pub mod chromewebstore;
pub mod chromewebstore;
pub mod websecurityscanner;
pub mod websecurityscanner;
pub mod websecurityscanner;
pub mod places;
pub mod recaptchaenterprise;
pub mod parallelstore;
pub mod parallelstore;
pub mod contactcenterinsights;
pub mod recommendationengine;
pub mod cloudidentity;
pub mod cloudidentity;
pub mod healthcare;
pub mod healthcare;
pub mod healthcare;
pub mod healthcare;
pub mod composer;
pub mod composer;
pub mod searchconsole;
pub mod appengine;
pub mod appengine;
pub mod appengine;
pub mod appengine;
pub mod appengine;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod dfareporting;
pub mod apikeys;
pub mod bigqueryconnection;
pub mod bigqueryconnection;
pub mod firebasedataconnect;
pub mod firebasedataconnect;
pub mod commentanalyzer;
pub mod firebaseapphosting;
pub mod firebaseapphosting;
pub mod doubleclickbidmanager;
pub mod doubleclickbidmanager;
pub mod doubleclickbidmanager;
pub mod fcmdata;
pub mod firebasedatabase;
pub mod ideahub;
pub mod ideahub;
pub mod firebaseremoteconfig;
pub mod prod_tt_sasportal;
pub mod firebaseappdistribution;
pub mod firebaseappdistribution;
pub mod deploymentmanager;
pub mod deploymentmanager;
pub mod deploymentmanager;
pub mod datafusion;
pub mod datafusion;
pub mod tagmanager;
pub mod tagmanager;
pub mod discovery;
pub mod storagetransfer;
pub mod oauth2;
pub mod oauth2;
pub mod managedkafka;
pub mod admin;
pub mod admin;
pub mod admin;
pub mod bigqueryreservation;
pub mod bigqueryreservation;
pub mod bigqueryreservation;
pub mod vmmigration;
pub mod vmmigration;
pub mod contactcenteraiplatform;
pub mod doubleclicksearch;
pub mod backupdr;
pub mod ondemandscanning;
pub mod ondemandscanning;
pub mod cloudbuild;
pub mod cloudbuild;
pub mod cloudbuild;
pub mod cloudbuild;
pub mod cloudbuild;
pub mod policyanalyzer;
pub mod policyanalyzer;
pub mod workflows;
pub mod workflows;
pub mod secretmanager;
pub mod secretmanager;
pub mod secretmanager;
pub mod transcoder;
pub mod transcoder;
pub mod accesscontextmanager;
pub mod accesscontextmanager;
pub mod content;
pub mod content;
pub mod content;
pub mod acceleratedmobilepageurl;
pub mod androidpublisher;
pub mod androidpublisher;
pub mod androidpublisher;
pub mod digitalassetlinks;
pub mod classroom;
pub mod chromeuxreport;
pub mod admob;
pub mod admob;
pub mod gameservices;
pub mod gameservices;
pub mod language;
pub mod language;
pub mod language;
pub mod language;
pub mod retail;
pub mod retail;
pub mod retail;
pub mod bigquery;
pub mod driveactivity;
pub mod serviceuser;
pub mod homegraph;
pub mod vision;
pub mod vision;
pub mod vision;
pub mod firebaseappcheck;
pub mod firebaseappcheck;
pub mod appsactivity;
pub mod cloudsearch;
pub mod books;
pub mod docs;
pub mod firebasehosting;
pub mod firebasehosting;
pub mod youtube;
pub mod observability;
pub mod adsensehost;
pub mod lifesciences;
pub mod adexchangebuyer;
pub mod adexchangebuyer;
pub mod adexchangebuyer;
pub mod streetviewpublish;
pub mod advisorynotifications;
pub mod sourcerepo;
pub mod vectortile;
pub mod paymentsresellersubscription;
pub mod airquality;
pub mod localservices;
pub mod adexchangeseller;
pub mod adexchangeseller;
pub mod adexchangeseller;
pub mod servicebroker;
pub mod servicebroker;
pub mod servicebroker;
pub mod speech;
pub mod speech;
pub mod speech;
pub mod speech;
pub mod speech;
pub mod script;
pub mod realtimebidding;
pub mod realtimebidding;
pub mod chromepolicy;
pub mod storage;
pub mod storage;
pub mod storage;
pub mod slides;
pub mod reseller;
pub mod mybusinessbusinessinformation;
pub mod androidenterprise;
pub mod ids;
pub mod adsense;
pub mod adsense;
pub mod adsense;
pub mod videointelligence;
pub mod videointelligence;
pub mod videointelligence;
pub mod videointelligence;
pub mod videointelligence;
pub mod datamigration;
pub mod datamigration;
pub mod gkebackup;
pub mod adsenseplatform;
pub mod adsenseplatform;
pub mod integrations;
pub mod integrations;
pub mod customsearch;
pub mod kgsearch;
pub mod testing;
pub mod container;
pub mod container;
pub mod datastore;
pub mod datastore;
pub mod datastore;
pub mod playmoviespartner;
pub mod memcache;
pub mod memcache;
pub mod blockchainnodeengine;
pub mod cloudtrace;
pub mod cloudtrace;
pub mod cloudtrace;
pub mod iap;
pub mod iap;
pub mod playcustomapp;
pub mod dataportability;
pub mod dataportability;
pub mod mybusinessbusinesscalls;
pub mod policysimulator;
pub mod policysimulator;
pub mod policysimulator;
pub mod policysimulator;
pub mod firebase;
pub mod iam;
pub mod iam;
pub mod iam;
pub mod bigtableadmin;
pub mod bigtableadmin;
pub mod servicedirectory;
pub mod servicedirectory;
pub mod recommender;
pub mod recommender;
pub mod trafficdirector;
pub mod trafficdirector;
pub mod drivelabels;
pub mod drivelabels;
pub mod cloudasset;
pub mod cloudasset;
pub mod cloudasset;
pub mod cloudasset;
pub mod cloudasset;
pub mod cloudasset;
pub mod cloudbilling;
pub mod cloudbilling;
pub mod webmasters;
pub mod compute;
pub mod compute;
pub mod compute;
pub mod groupssettings;
pub mod workloadmanager;
pub mod abusiveexperiencereport;
pub mod displayvideo;
pub mod displayvideo;
pub mod displayvideo;
pub mod displayvideo;
pub mod displayvideo;
pub mod displayvideo;
pub mod displayvideo;
pub mod ml;
pub mod networkconnectivity;
pub mod networkconnectivity;
pub mod batch;
pub mod webfonts;
pub mod partners;
pub mod cloudprofiler;
pub mod eventarc;
pub mod eventarc;
pub mod contentwarehouse;
pub mod siteverification;
pub mod analyticsadmin;
pub mod analyticsadmin;
pub mod verifiedaccess;
pub mod verifiedaccess;
pub mod safebrowsing;
pub mod safebrowsing;
pub mod people;
pub mod fcm;
pub mod firebasestorage;
pub mod manufacturers;
pub mod appstate;
pub mod servicecontrol;
pub mod servicecontrol;
pub mod datalabeling;
pub mod dlp;
pub mod mybusinessnotifications;
pub mod containeranalysis;
pub mod containeranalysis;
pub mod containeranalysis;
pub mod mybusinessqanda;
pub mod cloudtasks;
pub mod cloudtasks;
pub mod cloudtasks;
pub mod apim;
pub mod surveys;
pub mod pubsub;
pub mod pubsub;
pub mod pubsub;
pub mod texttospeech;
pub mod texttospeech;
pub mod marketingplatformadmin;
pub mod cloudprivatecatalogproducer;
pub mod sasportal;
pub mod spanner;
pub mod clouddeploy;
pub mod biglake;
pub mod serviceusage;
pub mod serviceusage;
pub mod chromemanagement;
pub mod fitness;
pub mod plusdomains;
pub mod androiddeviceprovisioning;
pub mod securesourcemanager;
pub mod kmsinventory;
pub mod cloudcontrolspartner;
pub mod cloudcontrolspartner;
pub mod bigquerydatapolicy;
pub mod bigquerydatapolicy;
pub mod licensing;
pub mod workspaceevents;
pub mod replicapoolupdater;
pub mod indexing;
pub mod alloydb;
pub mod alloydb;
pub mod alloydb;
pub mod spectrum;
pub mod firebaseml;
pub mod firebaseml;
pub mod firebaseml;
pub mod tpu;
pub mod tpu;
pub mod tpu;
pub mod tpu;
pub mod domainsrdap;
pub mod playablelocations;
pub mod groupsmigration;
pub mod androidmanagement;
pub mod run;
pub mod run;
pub mod run;
pub mod run;
pub mod cloudcommerceprocurement;
pub mod jobs;
pub mod jobs;
pub mod jobs;
pub mod jobs;
pub mod networksecurity;
pub mod networksecurity;
pub mod clouddebugger;
pub mod monitoring;
pub mod monitoring;
pub mod youtubeanalytics;
pub mod youtubeanalytics;
pub mod youtubeanalytics;
pub mod policytroubleshooter;
pub mod policytroubleshooter;
pub mod assuredworkloads;
pub mod assuredworkloads;
pub mod versionhistory;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod merchantapi;
pub mod gamesconfiguration;
pub mod cloudscheduler;
pub mod cloudscheduler;
pub mod vmwareengine;
pub mod drive;
pub mod drive;
pub mod dns;
pub mod dns;
pub mod dns;
pub mod dns;
pub mod analyticsreporting;
pub mod blogger;
pub mod blogger;
pub mod replicapool;
pub mod mybusinessplaceactions;
pub mod consumersurveys;
pub mod games;
pub mod gmailpostmastertools;
pub mod gmailpostmastertools;
pub mod oracledatabase;
pub mod dataform;
pub mod dataform;
pub mod authorizedbuyersmarketplace;
pub mod authorizedbuyersmarketplace;
pub mod authorizedbuyersmarketplace;
pub mod serviceconsumermanagement;
pub mod serviceconsumermanagement;
pub mod notebooks;
pub mod notebooks;
pub mod parametermanager;
pub mod logging;
pub mod logging;
pub mod adexperiencereport;
pub mod bigquerydatatransfer;
pub mod datacatalog;
pub mod datacatalog;
pub mod cloudshell;
pub mod cloudshell;
pub mod cloudchannel;
pub mod osconfig;
pub mod osconfig;
pub mod osconfig;
pub mod osconfig;
pub mod osconfig;
pub mod migrationcenter;
pub mod migrationcenter;
pub mod gmail;
pub mod workflowexecutions;
pub mod workflowexecutions;
pub mod apigateway;
pub mod apigateway;
pub mod apigateway;
pub mod apigateway;
pub mod businessprofileperformance;
pub mod metastore;
pub mod metastore;
pub mod metastore;
pub mod metastore;
pub mod metastore;
pub mod metastore;
pub mod pubsublite;
pub mod vault;
pub mod cloudsupport;
pub mod cloudsupport;
pub mod analytics;
pub mod analytics;
pub mod sts;
pub mod sts;
pub mod certificatemanager;
pub mod poly;
pub mod firebaserules;
pub mod plus;
pub mod dataproc;
pub mod dataproc;
pub mod mirror;
pub mod baremetalsolution;
pub mod baremetalsolution;
pub mod baremetalsolution;
pub mod managedidentities;
pub mod managedidentities;
pub mod managedidentities;
pub mod sql;


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
    // factchecktools_client: google_factchecktools::Client,
    // areainsights_client: google_areainsights::Client,
    // billingbudgets_client: google_billingbudgets::Client,
    // billingbudgets_client: google_billingbudgets::Client,
    // dataflow_client: google_dataflow::Client,
    // playdeveloperreporting_client: google_playdeveloperreporting::Client,
    // playdeveloperreporting_client: google_playdeveloperreporting::Client,
    // iamcredentials_client: google_iamcredentials::Client,
    // privateca_client: google_privateca::Client,
    // privateca_client: google_privateca::Client,
    // solar_client: google_solar::Client,
    // cloudfunctions_client: google_cloudfunctions::Client,
    // cloudfunctions_client: google_cloudfunctions::Client,
    // cloudfunctions_client: google_cloudfunctions::Client,
    // cloudfunctions_client: google_cloudfunctions::Client,
    // cloudfunctions_client: google_cloudfunctions::Client,
    // essentialcontacts_client: google_essentialcontacts::Client,
    // orgpolicy_client: google_orgpolicy::Client,
    // remotebuildexecution_client: google_remotebuildexecution::Client,
    // remotebuildexecution_client: google_remotebuildexecution::Client,
    // remotebuildexecution_client: google_remotebuildexecution::Client,
    // urlshortener_client: google_urlshortener::Client,
    // workstations_client: google_workstations::Client,
    // workstations_client: google_workstations::Client,
    // webrisk_client: google_webrisk::Client,
    // keep_client: google_keep::Client,
    // datastream_client: google_datastream::Client,
    // datastream_client: google_datastream::Client,
    // servicenetworking_client: google_servicenetworking::Client,
    // servicenetworking_client: google_servicenetworking::Client,
    // binaryauthorization_client: google_binaryauthorization::Client,
    // binaryauthorization_client: google_binaryauthorization::Client,
    // area120tables_client: google_area120tables::Client,
    // connectors_client: google_connectors::Client,
    // connectors_client: google_connectors::Client,
    // apihub_client: google_apihub::Client,
    // apigeeregistry_client: google_apigeeregistry::Client,
    // beyondcorp_client: google_beyondcorp::Client,
    // beyondcorp_client: google_beyondcorp::Client,
    // toolresults_client: google_toolresults::Client,
    // toolresults_client: google_toolresults::Client,
    // clouderrorreporting_client: google_clouderrorreporting::Client,
    // civicinfo_client: google_civicinfo::Client,
    // apigee_client: google_apigee::Client,
    // mybusinessverifications_client: google_mybusinessverifications::Client,
    // developerconnect_client: google_developerconnect::Client,
    // datapipelines_client: google_datapipelines::Client,
    // firebasedynamiclinks_client: google_firebasedynamiclinks::Client,
    // runtimeconfig_client: google_runtimeconfig::Client,
    // runtimeconfig_client: google_runtimeconfig::Client,
    // storagebatchoperations_client: google_storagebatchoperations::Client,
    // tracing_client: google_tracing::Client,
    // adexchangebuyer2_client: google_adexchangebuyer2::Client,
    // saasservicemgmt_client: google_saasservicemgmt::Client,
    // proximitybeacon_client: google_proximitybeacon::Client,
    // resourcesettings_client: google_resourcesettings::Client,
    // datalineage_client: google_datalineage::Client,
    // libraryagent_client: google_libraryagent::Client,
    // acmedns_client: google_acmedns::Client,
    // publicca_client: google_publicca::Client,
    // publicca_client: google_publicca::Client,
    // publicca_client: google_publicca::Client,
    // networkservices_client: google_networkservices::Client,
    // networkservices_client: google_networkservices::Client,
    // netapp_client: google_netapp::Client,
    // netapp_client: google_netapp::Client,
    // aiplatform_client: google_aiplatform::Client,
    // aiplatform_client: google_aiplatform::Client,
    // pagespeedonline_client: google_pagespeedonline::Client,
    // pagespeedonline_client: google_pagespeedonline::Client,
    // pagespeedonline_client: google_pagespeedonline::Client,
    // pagespeedonline_client: google_pagespeedonline::Client,
    // tasks_client: google_tasks::Client,
    // sqladmin_client: google_sqladmin::Client,
    // sqladmin_client: google_sqladmin::Client,
    // discoveryengine_client: google_discoveryengine::Client,
    // discoveryengine_client: google_discoveryengine::Client,
    // discoveryengine_client: google_discoveryengine::Client,
    // gkeonprem_client: google_gkeonprem::Client,
    // cloudlocationfinder_client: google_cloudlocationfinder::Client,
    // cloudlocationfinder_client: google_cloudlocationfinder::Client,
    // mybusinesslodging_client: google_mybusinesslodging::Client,
    // mybusinessaccountmanagement_client: google_mybusinessaccountmanagement::Client,
    // searchads360_client: google_searchads360::Client,
    // alertcenter_client: google_alertcenter::Client,
    // servicemanagement_client: google_servicemanagement::Client,
    // smartdevicemanagement_client: google_smartdevicemanagement::Client,
    // domains_client: google_domains::Client,
    // domains_client: google_domains::Client,
    // domains_client: google_domains::Client,
    // calendar_client: google_calendar::Client,
    // vpcaccess_client: google_vpcaccess::Client,
    // vpcaccess_client: google_vpcaccess::Client,
    // sheets_client: google_sheets::Client,
    // looker_client: google_looker::Client,
    // forms_client: google_forms::Client,
    // cloudiot_client: google_cloudiot::Client,
    // genomics_client: google_genomics::Client,
    // genomics_client: google_genomics::Client,
    // genomics_client: google_genomics::Client,
    // pollen_client: google_pollen::Client,
    // playgrouping_client: google_playgrouping::Client,
    // oslogin_client: google_oslogin::Client,
    // oslogin_client: google_oslogin::Client,
    // oslogin_client: google_oslogin::Client,
    // gamesmanagement_client: google_gamesmanagement::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // gkehub_client: google_gkehub::Client,
    // securityposture_client: google_securityposture::Client,
    // networkmanagement_client: google_networkmanagement::Client,
    // networkmanagement_client: google_networkmanagement::Client,
    // securitycenter_client: google_securitycenter::Client,
    // securitycenter_client: google_securitycenter::Client,
    // securitycenter_client: google_securitycenter::Client,
    // securitycenter_client: google_securitycenter::Client,
    // securitycenter_client: google_securitycenter::Client,
    // datamanager_client: google_datamanager::Client,
    // chat_client: google_chat::Client,
    // apphub_client: google_apphub::Client,
    // apphub_client: google_apphub::Client,
    // dialogflow_client: google_dialogflow::Client,
    // dialogflow_client: google_dialogflow::Client,
    // dialogflow_client: google_dialogflow::Client,
    // dialogflow_client: google_dialogflow::Client,
    // dialogflow_client: google_dialogflow::Client,
    // rapidmigrationassessment_client: google_rapidmigrationassessment::Client,
    // documentai_client: google_documentai::Client,
    // documentai_client: google_documentai::Client,
    // documentai_client: google_documentai::Client,
    // artifactregistry_client: google_artifactregistry::Client,
    // artifactregistry_client: google_artifactregistry::Client,
    // artifactregistry_client: google_artifactregistry::Client,
    // qpxexpress_client: google_qpxexpress::Client,
    // analyticsdata_client: google_analyticsdata::Client,
    // analyticsdata_client: google_analyticsdata::Client,
    // file_client: google_file::Client,
    // file_client: google_file::Client,
    // cloudresourcemanager_client: google_cloudresourcemanager::Client,
    // cloudresourcemanager_client: google_cloudresourcemanager::Client,
    // cloudresourcemanager_client: google_cloudresourcemanager::Client,
    // cloudresourcemanager_client: google_cloudresourcemanager::Client,
    // cloudresourcemanager_client: google_cloudresourcemanager::Client,
    // css_client: google_css::Client,
    // walletobjects_client: google_walletobjects::Client,
    // readerrevenuesubscriptionlinking_client: google_readerrevenuesubscriptionlinking::Client,
    // youtubereporting_client: google_youtubereporting::Client,
    // firestore_client: google_firestore::Client,
    // firestore_client: google_firestore::Client,
    // firestore_client: google_firestore::Client,
    // config_client: google_config::Client,
    // translate_client: google_translate::Client,
    // translate_client: google_translate::Client,
    // translate_client: google_translate::Client,
    // redis_client: google_redis::Client,
    // redis_client: google_redis::Client,
    // meet_client: google_meet::Client,
    // dataplex_client: google_dataplex::Client,
    // analyticshub_client: google_analyticshub::Client,
    // analyticshub_client: google_analyticshub::Client,
    // playintegrity_client: google_playintegrity::Client,
    // cloudkms_client: google_cloudkms::Client,
    // fusiontables_client: google_fusiontables::Client,
    // fusiontables_client: google_fusiontables::Client,
    // accessapproval_client: google_accessapproval::Client,
    // checks_client: google_checks::Client,
    // cloudprivatecatalog_client: google_cloudprivatecatalog::Client,
    // addressvalidation_client: google_addressvalidation::Client,
    // chromewebstore_client: google_chromewebstore::Client,
    // chromewebstore_client: google_chromewebstore::Client,
    // websecurityscanner_client: google_websecurityscanner::Client,
    // websecurityscanner_client: google_websecurityscanner::Client,
    // websecurityscanner_client: google_websecurityscanner::Client,
    // places_client: google_places::Client,
    // recaptchaenterprise_client: google_recaptchaenterprise::Client,
    // parallelstore_client: google_parallelstore::Client,
    // parallelstore_client: google_parallelstore::Client,
    // contactcenterinsights_client: google_contactcenterinsights::Client,
    // recommendationengine_client: google_recommendationengine::Client,
    // cloudidentity_client: google_cloudidentity::Client,
    // cloudidentity_client: google_cloudidentity::Client,
    // healthcare_client: google_healthcare::Client,
    // healthcare_client: google_healthcare::Client,
    // healthcare_client: google_healthcare::Client,
    // healthcare_client: google_healthcare::Client,
    // composer_client: google_composer::Client,
    // composer_client: google_composer::Client,
    // searchconsole_client: google_searchconsole::Client,
    // appengine_client: google_appengine::Client,
    // appengine_client: google_appengine::Client,
    // appengine_client: google_appengine::Client,
    // appengine_client: google_appengine::Client,
    // appengine_client: google_appengine::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // dfareporting_client: google_dfareporting::Client,
    // apikeys_client: google_apikeys::Client,
    // bigqueryconnection_client: google_bigqueryconnection::Client,
    // bigqueryconnection_client: google_bigqueryconnection::Client,
    // firebasedataconnect_client: google_firebasedataconnect::Client,
    // firebasedataconnect_client: google_firebasedataconnect::Client,
    // commentanalyzer_client: google_commentanalyzer::Client,
    // firebaseapphosting_client: google_firebaseapphosting::Client,
    // firebaseapphosting_client: google_firebaseapphosting::Client,
    // doubleclickbidmanager_client: google_doubleclickbidmanager::Client,
    // doubleclickbidmanager_client: google_doubleclickbidmanager::Client,
    // doubleclickbidmanager_client: google_doubleclickbidmanager::Client,
    // fcmdata_client: google_fcmdata::Client,
    // firebasedatabase_client: google_firebasedatabase::Client,
    // ideahub_client: google_ideahub::Client,
    // ideahub_client: google_ideahub::Client,
    // firebaseremoteconfig_client: google_firebaseremoteconfig::Client,
    // prod_tt_sasportal_client: google_prod_tt_sasportal::Client,
    // firebaseappdistribution_client: google_firebaseappdistribution::Client,
    // firebaseappdistribution_client: google_firebaseappdistribution::Client,
    // deploymentmanager_client: google_deploymentmanager::Client,
    // deploymentmanager_client: google_deploymentmanager::Client,
    // deploymentmanager_client: google_deploymentmanager::Client,
    // datafusion_client: google_datafusion::Client,
    // datafusion_client: google_datafusion::Client,
    // tagmanager_client: google_tagmanager::Client,
    // tagmanager_client: google_tagmanager::Client,
    // discovery_client: google_discovery::Client,
    // storagetransfer_client: google_storagetransfer::Client,
    // oauth2_client: google_oauth2::Client,
    // oauth2_client: google_oauth2::Client,
    // managedkafka_client: google_managedkafka::Client,
    // admin_client: google_admin::Client,
    // admin_client: google_admin::Client,
    // admin_client: google_admin::Client,
    // bigqueryreservation_client: google_bigqueryreservation::Client,
    // bigqueryreservation_client: google_bigqueryreservation::Client,
    // bigqueryreservation_client: google_bigqueryreservation::Client,
    // vmmigration_client: google_vmmigration::Client,
    // vmmigration_client: google_vmmigration::Client,
    // contactcenteraiplatform_client: google_contactcenteraiplatform::Client,
    // doubleclicksearch_client: google_doubleclicksearch::Client,
    // backupdr_client: google_backupdr::Client,
    // ondemandscanning_client: google_ondemandscanning::Client,
    // ondemandscanning_client: google_ondemandscanning::Client,
    // cloudbuild_client: google_cloudbuild::Client,
    // cloudbuild_client: google_cloudbuild::Client,
    // cloudbuild_client: google_cloudbuild::Client,
    // cloudbuild_client: google_cloudbuild::Client,
    // cloudbuild_client: google_cloudbuild::Client,
    // policyanalyzer_client: google_policyanalyzer::Client,
    // policyanalyzer_client: google_policyanalyzer::Client,
    // workflows_client: google_workflows::Client,
    // workflows_client: google_workflows::Client,
    // secretmanager_client: google_secretmanager::Client,
    // secretmanager_client: google_secretmanager::Client,
    // secretmanager_client: google_secretmanager::Client,
    // transcoder_client: google_transcoder::Client,
    // transcoder_client: google_transcoder::Client,
    // accesscontextmanager_client: google_accesscontextmanager::Client,
    // accesscontextmanager_client: google_accesscontextmanager::Client,
    // content_client: google_content::Client,
    // content_client: google_content::Client,
    // content_client: google_content::Client,
    // acceleratedmobilepageurl_client: google_acceleratedmobilepageurl::Client,
    // androidpublisher_client: google_androidpublisher::Client,
    // androidpublisher_client: google_androidpublisher::Client,
    // androidpublisher_client: google_androidpublisher::Client,
    // digitalassetlinks_client: google_digitalassetlinks::Client,
    // classroom_client: google_classroom::Client,
    // chromeuxreport_client: google_chromeuxreport::Client,
    // admob_client: google_admob::Client,
    // admob_client: google_admob::Client,
    // gameservices_client: google_gameservices::Client,
    // gameservices_client: google_gameservices::Client,
    // language_client: google_language::Client,
    // language_client: google_language::Client,
    // language_client: google_language::Client,
    // language_client: google_language::Client,
    // retail_client: google_retail::Client,
    // retail_client: google_retail::Client,
    // retail_client: google_retail::Client,
    // bigquery_client: google_bigquery::Client,
    // driveactivity_client: google_driveactivity::Client,
    // serviceuser_client: google_serviceuser::Client,
    // homegraph_client: google_homegraph::Client,
    // vision_client: google_vision::Client,
    // vision_client: google_vision::Client,
    // vision_client: google_vision::Client,
    // firebaseappcheck_client: google_firebaseappcheck::Client,
    // firebaseappcheck_client: google_firebaseappcheck::Client,
    // appsactivity_client: google_appsactivity::Client,
    // cloudsearch_client: google_cloudsearch::Client,
    // books_client: google_books::Client,
    // docs_client: google_docs::Client,
    // firebasehosting_client: google_firebasehosting::Client,
    // firebasehosting_client: google_firebasehosting::Client,
    // youtube_client: google_youtube::Client,
    // observability_client: google_observability::Client,
    // adsensehost_client: google_adsensehost::Client,
    // lifesciences_client: google_lifesciences::Client,
    // adexchangebuyer_client: google_adexchangebuyer::Client,
    // adexchangebuyer_client: google_adexchangebuyer::Client,
    // adexchangebuyer_client: google_adexchangebuyer::Client,
    // streetviewpublish_client: google_streetviewpublish::Client,
    // advisorynotifications_client: google_advisorynotifications::Client,
    // sourcerepo_client: google_sourcerepo::Client,
    // vectortile_client: google_vectortile::Client,
    // paymentsresellersubscription_client: google_paymentsresellersubscription::Client,
    // airquality_client: google_airquality::Client,
    // localservices_client: google_localservices::Client,
    // adexchangeseller_client: google_adexchangeseller::Client,
    // adexchangeseller_client: google_adexchangeseller::Client,
    // adexchangeseller_client: google_adexchangeseller::Client,
    // servicebroker_client: google_servicebroker::Client,
    // servicebroker_client: google_servicebroker::Client,
    // servicebroker_client: google_servicebroker::Client,
    // speech_client: google_speech::Client,
    // speech_client: google_speech::Client,
    // speech_client: google_speech::Client,
    // speech_client: google_speech::Client,
    // speech_client: google_speech::Client,
    // script_client: google_script::Client,
    // realtimebidding_client: google_realtimebidding::Client,
    // realtimebidding_client: google_realtimebidding::Client,
    // chromepolicy_client: google_chromepolicy::Client,
    // storage_client: google_storage::Client,
    // storage_client: google_storage::Client,
    // storage_client: google_storage::Client,
    // slides_client: google_slides::Client,
    // reseller_client: google_reseller::Client,
    // mybusinessbusinessinformation_client: google_mybusinessbusinessinformation::Client,
    // androidenterprise_client: google_androidenterprise::Client,
    // ids_client: google_ids::Client,
    // adsense_client: google_adsense::Client,
    // adsense_client: google_adsense::Client,
    // adsense_client: google_adsense::Client,
    // videointelligence_client: google_videointelligence::Client,
    // videointelligence_client: google_videointelligence::Client,
    // videointelligence_client: google_videointelligence::Client,
    // videointelligence_client: google_videointelligence::Client,
    // videointelligence_client: google_videointelligence::Client,
    // datamigration_client: google_datamigration::Client,
    // datamigration_client: google_datamigration::Client,
    // gkebackup_client: google_gkebackup::Client,
    // adsenseplatform_client: google_adsenseplatform::Client,
    // adsenseplatform_client: google_adsenseplatform::Client,
    // integrations_client: google_integrations::Client,
    // integrations_client: google_integrations::Client,
    // customsearch_client: google_customsearch::Client,
    // kgsearch_client: google_kgsearch::Client,
    // testing_client: google_testing::Client,
    // container_client: google_container::Client,
    // container_client: google_container::Client,
    // datastore_client: google_datastore::Client,
    // datastore_client: google_datastore::Client,
    // datastore_client: google_datastore::Client,
    // playmoviespartner_client: google_playmoviespartner::Client,
    // memcache_client: google_memcache::Client,
    // memcache_client: google_memcache::Client,
    // blockchainnodeengine_client: google_blockchainnodeengine::Client,
    // cloudtrace_client: google_cloudtrace::Client,
    // cloudtrace_client: google_cloudtrace::Client,
    // cloudtrace_client: google_cloudtrace::Client,
    // iap_client: google_iap::Client,
    // iap_client: google_iap::Client,
    // playcustomapp_client: google_playcustomapp::Client,
    // dataportability_client: google_dataportability::Client,
    // dataportability_client: google_dataportability::Client,
    // mybusinessbusinesscalls_client: google_mybusinessbusinesscalls::Client,
    // policysimulator_client: google_policysimulator::Client,
    // policysimulator_client: google_policysimulator::Client,
    // policysimulator_client: google_policysimulator::Client,
    // policysimulator_client: google_policysimulator::Client,
    // firebase_client: google_firebase::Client,
    // iam_client: google_iam::Client,
    // iam_client: google_iam::Client,
    // iam_client: google_iam::Client,
    // bigtableadmin_client: google_bigtableadmin::Client,
    // bigtableadmin_client: google_bigtableadmin::Client,
    // servicedirectory_client: google_servicedirectory::Client,
    // servicedirectory_client: google_servicedirectory::Client,
    // recommender_client: google_recommender::Client,
    // recommender_client: google_recommender::Client,
    // trafficdirector_client: google_trafficdirector::Client,
    // trafficdirector_client: google_trafficdirector::Client,
    // drivelabels_client: google_drivelabels::Client,
    // drivelabels_client: google_drivelabels::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudasset_client: google_cloudasset::Client,
    // cloudbilling_client: google_cloudbilling::Client,
    // cloudbilling_client: google_cloudbilling::Client,
    // webmasters_client: google_webmasters::Client,
    // compute_client: google_compute::Client,
    // compute_client: google_compute::Client,
    // compute_client: google_compute::Client,
    // groupssettings_client: google_groupssettings::Client,
    // workloadmanager_client: google_workloadmanager::Client,
    // abusiveexperiencereport_client: google_abusiveexperiencereport::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // displayvideo_client: google_displayvideo::Client,
    // ml_client: google_ml::Client,
    // networkconnectivity_client: google_networkconnectivity::Client,
    // networkconnectivity_client: google_networkconnectivity::Client,
    // batch_client: google_batch::Client,
    // webfonts_client: google_webfonts::Client,
    // partners_client: google_partners::Client,
    // cloudprofiler_client: google_cloudprofiler::Client,
    // eventarc_client: google_eventarc::Client,
    // eventarc_client: google_eventarc::Client,
    // contentwarehouse_client: google_contentwarehouse::Client,
    // siteverification_client: google_siteverification::Client,
    // analyticsadmin_client: google_analyticsadmin::Client,
    // analyticsadmin_client: google_analyticsadmin::Client,
    // verifiedaccess_client: google_verifiedaccess::Client,
    // verifiedaccess_client: google_verifiedaccess::Client,
    // safebrowsing_client: google_safebrowsing::Client,
    // safebrowsing_client: google_safebrowsing::Client,
    // people_client: google_people::Client,
    // fcm_client: google_fcm::Client,
    // firebasestorage_client: google_firebasestorage::Client,
    // manufacturers_client: google_manufacturers::Client,
    // appstate_client: google_appstate::Client,
    // servicecontrol_client: google_servicecontrol::Client,
    // servicecontrol_client: google_servicecontrol::Client,
    // datalabeling_client: google_datalabeling::Client,
    // dlp_client: google_dlp::Client,
    // mybusinessnotifications_client: google_mybusinessnotifications::Client,
    // containeranalysis_client: google_containeranalysis::Client,
    // containeranalysis_client: google_containeranalysis::Client,
    // containeranalysis_client: google_containeranalysis::Client,
    // mybusinessqanda_client: google_mybusinessqanda::Client,
    // cloudtasks_client: google_cloudtasks::Client,
    // cloudtasks_client: google_cloudtasks::Client,
    // cloudtasks_client: google_cloudtasks::Client,
    // apim_client: google_apim::Client,
    // surveys_client: google_surveys::Client,
    // pubsub_client: google_pubsub::Client,
    // pubsub_client: google_pubsub::Client,
    // pubsub_client: google_pubsub::Client,
    // texttospeech_client: google_texttospeech::Client,
    // texttospeech_client: google_texttospeech::Client,
    // marketingplatformadmin_client: google_marketingplatformadmin::Client,
    // cloudprivatecatalogproducer_client: google_cloudprivatecatalogproducer::Client,
    // sasportal_client: google_sasportal::Client,
    // spanner_client: google_spanner::Client,
    // clouddeploy_client: google_clouddeploy::Client,
    // biglake_client: google_biglake::Client,
    // serviceusage_client: google_serviceusage::Client,
    // serviceusage_client: google_serviceusage::Client,
    // chromemanagement_client: google_chromemanagement::Client,
    // fitness_client: google_fitness::Client,
    // plusdomains_client: google_plusdomains::Client,
    // androiddeviceprovisioning_client: google_androiddeviceprovisioning::Client,
    // securesourcemanager_client: google_securesourcemanager::Client,
    // kmsinventory_client: google_kmsinventory::Client,
    // cloudcontrolspartner_client: google_cloudcontrolspartner::Client,
    // cloudcontrolspartner_client: google_cloudcontrolspartner::Client,
    // bigquerydatapolicy_client: google_bigquerydatapolicy::Client,
    // bigquerydatapolicy_client: google_bigquerydatapolicy::Client,
    // licensing_client: google_licensing::Client,
    // workspaceevents_client: google_workspaceevents::Client,
    // replicapoolupdater_client: google_replicapoolupdater::Client,
    // indexing_client: google_indexing::Client,
    // alloydb_client: google_alloydb::Client,
    // alloydb_client: google_alloydb::Client,
    // alloydb_client: google_alloydb::Client,
    // spectrum_client: google_spectrum::Client,
    // firebaseml_client: google_firebaseml::Client,
    // firebaseml_client: google_firebaseml::Client,
    // firebaseml_client: google_firebaseml::Client,
    // tpu_client: google_tpu::Client,
    // tpu_client: google_tpu::Client,
    // tpu_client: google_tpu::Client,
    // tpu_client: google_tpu::Client,
    // domainsrdap_client: google_domainsrdap::Client,
    // playablelocations_client: google_playablelocations::Client,
    // groupsmigration_client: google_groupsmigration::Client,
    // androidmanagement_client: google_androidmanagement::Client,
    // run_client: google_run::Client,
    // run_client: google_run::Client,
    // run_client: google_run::Client,
    // run_client: google_run::Client,
    // cloudcommerceprocurement_client: google_cloudcommerceprocurement::Client,
    // jobs_client: google_jobs::Client,
    // jobs_client: google_jobs::Client,
    // jobs_client: google_jobs::Client,
    // jobs_client: google_jobs::Client,
    // networksecurity_client: google_networksecurity::Client,
    // networksecurity_client: google_networksecurity::Client,
    // clouddebugger_client: google_clouddebugger::Client,
    // monitoring_client: google_monitoring::Client,
    // monitoring_client: google_monitoring::Client,
    // youtubeanalytics_client: google_youtubeanalytics::Client,
    // youtubeanalytics_client: google_youtubeanalytics::Client,
    // youtubeanalytics_client: google_youtubeanalytics::Client,
    // policytroubleshooter_client: google_policytroubleshooter::Client,
    // policytroubleshooter_client: google_policytroubleshooter::Client,
    // assuredworkloads_client: google_assuredworkloads::Client,
    // assuredworkloads_client: google_assuredworkloads::Client,
    // versionhistory_client: google_versionhistory::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // merchantapi_client: google_merchantapi::Client,
    // gamesconfiguration_client: google_gamesconfiguration::Client,
    // cloudscheduler_client: google_cloudscheduler::Client,
    // cloudscheduler_client: google_cloudscheduler::Client,
    // vmwareengine_client: google_vmwareengine::Client,
    // drive_client: google_drive::Client,
    // drive_client: google_drive::Client,
    // dns_client: google_dns::Client,
    // dns_client: google_dns::Client,
    // dns_client: google_dns::Client,
    // dns_client: google_dns::Client,
    // analyticsreporting_client: google_analyticsreporting::Client,
    // blogger_client: google_blogger::Client,
    // blogger_client: google_blogger::Client,
    // replicapool_client: google_replicapool::Client,
    // mybusinessplaceactions_client: google_mybusinessplaceactions::Client,
    // consumersurveys_client: google_consumersurveys::Client,
    // games_client: google_games::Client,
    // gmailpostmastertools_client: google_gmailpostmastertools::Client,
    // gmailpostmastertools_client: google_gmailpostmastertools::Client,
    // oracledatabase_client: google_oracledatabase::Client,
    // dataform_client: google_dataform::Client,
    // dataform_client: google_dataform::Client,
    // authorizedbuyersmarketplace_client: google_authorizedbuyersmarketplace::Client,
    // authorizedbuyersmarketplace_client: google_authorizedbuyersmarketplace::Client,
    // authorizedbuyersmarketplace_client: google_authorizedbuyersmarketplace::Client,
    // serviceconsumermanagement_client: google_serviceconsumermanagement::Client,
    // serviceconsumermanagement_client: google_serviceconsumermanagement::Client,
    // notebooks_client: google_notebooks::Client,
    // notebooks_client: google_notebooks::Client,
    // parametermanager_client: google_parametermanager::Client,
    // logging_client: google_logging::Client,
    // logging_client: google_logging::Client,
    // adexperiencereport_client: google_adexperiencereport::Client,
    // bigquerydatatransfer_client: google_bigquerydatatransfer::Client,
    // datacatalog_client: google_datacatalog::Client,
    // datacatalog_client: google_datacatalog::Client,
    // cloudshell_client: google_cloudshell::Client,
    // cloudshell_client: google_cloudshell::Client,
    // cloudchannel_client: google_cloudchannel::Client,
    // osconfig_client: google_osconfig::Client,
    // osconfig_client: google_osconfig::Client,
    // osconfig_client: google_osconfig::Client,
    // osconfig_client: google_osconfig::Client,
    // osconfig_client: google_osconfig::Client,
    // migrationcenter_client: google_migrationcenter::Client,
    // migrationcenter_client: google_migrationcenter::Client,
    // gmail_client: google_gmail::Client,
    // workflowexecutions_client: google_workflowexecutions::Client,
    // workflowexecutions_client: google_workflowexecutions::Client,
    // apigateway_client: google_apigateway::Client,
    // apigateway_client: google_apigateway::Client,
    // apigateway_client: google_apigateway::Client,
    // apigateway_client: google_apigateway::Client,
    // businessprofileperformance_client: google_businessprofileperformance::Client,
    // metastore_client: google_metastore::Client,
    // metastore_client: google_metastore::Client,
    // metastore_client: google_metastore::Client,
    // metastore_client: google_metastore::Client,
    // metastore_client: google_metastore::Client,
    // metastore_client: google_metastore::Client,
    // pubsublite_client: google_pubsublite::Client,
    // vault_client: google_vault::Client,
    // cloudsupport_client: google_cloudsupport::Client,
    // cloudsupport_client: google_cloudsupport::Client,
    // analytics_client: google_analytics::Client,
    // analytics_client: google_analytics::Client,
    // sts_client: google_sts::Client,
    // sts_client: google_sts::Client,
    // certificatemanager_client: google_certificatemanager::Client,
    // poly_client: google_poly::Client,
    // firebaserules_client: google_firebaserules::Client,
    // plus_client: google_plus::Client,
    // dataproc_client: google_dataproc::Client,
    // dataproc_client: google_dataproc::Client,
    // mirror_client: google_mirror::Client,
    // baremetalsolution_client: google_baremetalsolution::Client,
    // baremetalsolution_client: google_baremetalsolution::Client,
    // baremetalsolution_client: google_baremetalsolution::Client,
    // managedidentities_client: google_managedidentities::Client,
    // managedidentities_client: google_managedidentities::Client,
    // managedidentities_client: google_managedidentities::Client,
    // sql_client: google_sql::Client,

}

impl GcpProvider {
    /// Create a new unified provider instance
    pub async fn new() -> Result<Self> {

        Ok(Self {

        })
    }

    /// Get factchecktools service handler
    pub fn factchecktools(&self) -> factchecktools::FactchecktoolsService<'_> {
        factchecktools::FactchecktoolsService::new(self)
    }
    /// Get areainsights service handler
    pub fn areainsights(&self) -> areainsights::AreainsightsService<'_> {
        areainsights::AreainsightsService::new(self)
    }
    /// Get billingbudgets service handler
    pub fn billingbudgets(&self) -> billingbudgets::BillingbudgetsService<'_> {
        billingbudgets::BillingbudgetsService::new(self)
    }
    /// Get billingbudgets service handler
    pub fn billingbudgets(&self) -> billingbudgets::BillingbudgetsService<'_> {
        billingbudgets::BillingbudgetsService::new(self)
    }
    /// Get dataflow service handler
    pub fn dataflow(&self) -> dataflow::DataflowService<'_> {
        dataflow::DataflowService::new(self)
    }
    /// Get playdeveloperreporting service handler
    pub fn playdeveloperreporting(&self) -> playdeveloperreporting::PlaydeveloperreportingService<'_> {
        playdeveloperreporting::PlaydeveloperreportingService::new(self)
    }
    /// Get playdeveloperreporting service handler
    pub fn playdeveloperreporting(&self) -> playdeveloperreporting::PlaydeveloperreportingService<'_> {
        playdeveloperreporting::PlaydeveloperreportingService::new(self)
    }
    /// Get iamcredentials service handler
    pub fn iamcredentials(&self) -> iamcredentials::IamcredentialsService<'_> {
        iamcredentials::IamcredentialsService::new(self)
    }
    /// Get privateca service handler
    pub fn privateca(&self) -> privateca::PrivatecaService<'_> {
        privateca::PrivatecaService::new(self)
    }
    /// Get privateca service handler
    pub fn privateca(&self) -> privateca::PrivatecaService<'_> {
        privateca::PrivatecaService::new(self)
    }
    /// Get solar service handler
    pub fn solar(&self) -> solar::SolarService<'_> {
        solar::SolarService::new(self)
    }
    /// Get cloudfunctions service handler
    pub fn cloudfunctions(&self) -> cloudfunctions::CloudfunctionsService<'_> {
        cloudfunctions::CloudfunctionsService::new(self)
    }
    /// Get cloudfunctions service handler
    pub fn cloudfunctions(&self) -> cloudfunctions::CloudfunctionsService<'_> {
        cloudfunctions::CloudfunctionsService::new(self)
    }
    /// Get cloudfunctions service handler
    pub fn cloudfunctions(&self) -> cloudfunctions::CloudfunctionsService<'_> {
        cloudfunctions::CloudfunctionsService::new(self)
    }
    /// Get cloudfunctions service handler
    pub fn cloudfunctions(&self) -> cloudfunctions::CloudfunctionsService<'_> {
        cloudfunctions::CloudfunctionsService::new(self)
    }
    /// Get cloudfunctions service handler
    pub fn cloudfunctions(&self) -> cloudfunctions::CloudfunctionsService<'_> {
        cloudfunctions::CloudfunctionsService::new(self)
    }
    /// Get essentialcontacts service handler
    pub fn essentialcontacts(&self) -> essentialcontacts::EssentialcontactsService<'_> {
        essentialcontacts::EssentialcontactsService::new(self)
    }
    /// Get orgpolicy service handler
    pub fn orgpolicy(&self) -> orgpolicy::OrgpolicyService<'_> {
        orgpolicy::OrgpolicyService::new(self)
    }
    /// Get remotebuildexecution service handler
    pub fn remotebuildexecution(&self) -> remotebuildexecution::RemotebuildexecutionService<'_> {
        remotebuildexecution::RemotebuildexecutionService::new(self)
    }
    /// Get remotebuildexecution service handler
    pub fn remotebuildexecution(&self) -> remotebuildexecution::RemotebuildexecutionService<'_> {
        remotebuildexecution::RemotebuildexecutionService::new(self)
    }
    /// Get remotebuildexecution service handler
    pub fn remotebuildexecution(&self) -> remotebuildexecution::RemotebuildexecutionService<'_> {
        remotebuildexecution::RemotebuildexecutionService::new(self)
    }
    /// Get urlshortener service handler
    pub fn urlshortener(&self) -> urlshortener::UrlshortenerService<'_> {
        urlshortener::UrlshortenerService::new(self)
    }
    /// Get workstations service handler
    pub fn workstations(&self) -> workstations::WorkstationsService<'_> {
        workstations::WorkstationsService::new(self)
    }
    /// Get workstations service handler
    pub fn workstations(&self) -> workstations::WorkstationsService<'_> {
        workstations::WorkstationsService::new(self)
    }
    /// Get webrisk service handler
    pub fn webrisk(&self) -> webrisk::WebriskService<'_> {
        webrisk::WebriskService::new(self)
    }
    /// Get keep service handler
    pub fn keep(&self) -> keep::KeepService<'_> {
        keep::KeepService::new(self)
    }
    /// Get datastream service handler
    pub fn datastream(&self) -> datastream::DatastreamService<'_> {
        datastream::DatastreamService::new(self)
    }
    /// Get datastream service handler
    pub fn datastream(&self) -> datastream::DatastreamService<'_> {
        datastream::DatastreamService::new(self)
    }
    /// Get servicenetworking service handler
    pub fn servicenetworking(&self) -> servicenetworking::ServicenetworkingService<'_> {
        servicenetworking::ServicenetworkingService::new(self)
    }
    /// Get servicenetworking service handler
    pub fn servicenetworking(&self) -> servicenetworking::ServicenetworkingService<'_> {
        servicenetworking::ServicenetworkingService::new(self)
    }
    /// Get binaryauthorization service handler
    pub fn binaryauthorization(&self) -> binaryauthorization::BinaryauthorizationService<'_> {
        binaryauthorization::BinaryauthorizationService::new(self)
    }
    /// Get binaryauthorization service handler
    pub fn binaryauthorization(&self) -> binaryauthorization::BinaryauthorizationService<'_> {
        binaryauthorization::BinaryauthorizationService::new(self)
    }
    /// Get area120tables service handler
    pub fn area120tables(&self) -> area120tables::Area120tablesService<'_> {
        area120tables::Area120tablesService::new(self)
    }
    /// Get connectors service handler
    pub fn connectors(&self) -> connectors::ConnectorsService<'_> {
        connectors::ConnectorsService::new(self)
    }
    /// Get connectors service handler
    pub fn connectors(&self) -> connectors::ConnectorsService<'_> {
        connectors::ConnectorsService::new(self)
    }
    /// Get apihub service handler
    pub fn apihub(&self) -> apihub::ApihubService<'_> {
        apihub::ApihubService::new(self)
    }
    /// Get apigeeregistry service handler
    pub fn apigeeregistry(&self) -> apigeeregistry::ApigeeregistryService<'_> {
        apigeeregistry::ApigeeregistryService::new(self)
    }
    /// Get beyondcorp service handler
    pub fn beyondcorp(&self) -> beyondcorp::BeyondcorpService<'_> {
        beyondcorp::BeyondcorpService::new(self)
    }
    /// Get beyondcorp service handler
    pub fn beyondcorp(&self) -> beyondcorp::BeyondcorpService<'_> {
        beyondcorp::BeyondcorpService::new(self)
    }
    /// Get toolresults service handler
    pub fn toolresults(&self) -> toolresults::ToolresultsService<'_> {
        toolresults::ToolresultsService::new(self)
    }
    /// Get toolresults service handler
    pub fn toolresults(&self) -> toolresults::ToolresultsService<'_> {
        toolresults::ToolresultsService::new(self)
    }
    /// Get clouderrorreporting service handler
    pub fn clouderrorreporting(&self) -> clouderrorreporting::ClouderrorreportingService<'_> {
        clouderrorreporting::ClouderrorreportingService::new(self)
    }
    /// Get civicinfo service handler
    pub fn civicinfo(&self) -> civicinfo::CivicinfoService<'_> {
        civicinfo::CivicinfoService::new(self)
    }
    /// Get apigee service handler
    pub fn apigee(&self) -> apigee::ApigeeService<'_> {
        apigee::ApigeeService::new(self)
    }
    /// Get mybusinessverifications service handler
    pub fn mybusinessverifications(&self) -> mybusinessverifications::MybusinessverificationsService<'_> {
        mybusinessverifications::MybusinessverificationsService::new(self)
    }
    /// Get developerconnect service handler
    pub fn developerconnect(&self) -> developerconnect::DeveloperconnectService<'_> {
        developerconnect::DeveloperconnectService::new(self)
    }
    /// Get datapipelines service handler
    pub fn datapipelines(&self) -> datapipelines::DatapipelinesService<'_> {
        datapipelines::DatapipelinesService::new(self)
    }
    /// Get firebasedynamiclinks service handler
    pub fn firebasedynamiclinks(&self) -> firebasedynamiclinks::FirebasedynamiclinksService<'_> {
        firebasedynamiclinks::FirebasedynamiclinksService::new(self)
    }
    /// Get runtimeconfig service handler
    pub fn runtimeconfig(&self) -> runtimeconfig::RuntimeconfigService<'_> {
        runtimeconfig::RuntimeconfigService::new(self)
    }
    /// Get runtimeconfig service handler
    pub fn runtimeconfig(&self) -> runtimeconfig::RuntimeconfigService<'_> {
        runtimeconfig::RuntimeconfigService::new(self)
    }
    /// Get storagebatchoperations service handler
    pub fn storagebatchoperations(&self) -> storagebatchoperations::StoragebatchoperationsService<'_> {
        storagebatchoperations::StoragebatchoperationsService::new(self)
    }
    /// Get tracing service handler
    pub fn tracing(&self) -> tracing::TracingService<'_> {
        tracing::TracingService::new(self)
    }
    /// Get adexchangebuyer2 service handler
    pub fn adexchangebuyer2(&self) -> adexchangebuyer2::Adexchangebuyer2Service<'_> {
        adexchangebuyer2::Adexchangebuyer2Service::new(self)
    }
    /// Get saasservicemgmt service handler
    pub fn saasservicemgmt(&self) -> saasservicemgmt::SaasservicemgmtService<'_> {
        saasservicemgmt::SaasservicemgmtService::new(self)
    }
    /// Get proximitybeacon service handler
    pub fn proximitybeacon(&self) -> proximitybeacon::ProximitybeaconService<'_> {
        proximitybeacon::ProximitybeaconService::new(self)
    }
    /// Get resourcesettings service handler
    pub fn resourcesettings(&self) -> resourcesettings::ResourcesettingsService<'_> {
        resourcesettings::ResourcesettingsService::new(self)
    }
    /// Get datalineage service handler
    pub fn datalineage(&self) -> datalineage::DatalineageService<'_> {
        datalineage::DatalineageService::new(self)
    }
    /// Get libraryagent service handler
    pub fn libraryagent(&self) -> libraryagent::LibraryagentService<'_> {
        libraryagent::LibraryagentService::new(self)
    }
    /// Get acmedns service handler
    pub fn acmedns(&self) -> acmedns::AcmednsService<'_> {
        acmedns::AcmednsService::new(self)
    }
    /// Get publicca service handler
    pub fn publicca(&self) -> publicca::PubliccaService<'_> {
        publicca::PubliccaService::new(self)
    }
    /// Get publicca service handler
    pub fn publicca(&self) -> publicca::PubliccaService<'_> {
        publicca::PubliccaService::new(self)
    }
    /// Get publicca service handler
    pub fn publicca(&self) -> publicca::PubliccaService<'_> {
        publicca::PubliccaService::new(self)
    }
    /// Get networkservices service handler
    pub fn networkservices(&self) -> networkservices::NetworkservicesService<'_> {
        networkservices::NetworkservicesService::new(self)
    }
    /// Get networkservices service handler
    pub fn networkservices(&self) -> networkservices::NetworkservicesService<'_> {
        networkservices::NetworkservicesService::new(self)
    }
    /// Get netapp service handler
    pub fn netapp(&self) -> netapp::NetappService<'_> {
        netapp::NetappService::new(self)
    }
    /// Get netapp service handler
    pub fn netapp(&self) -> netapp::NetappService<'_> {
        netapp::NetappService::new(self)
    }
    /// Get aiplatform service handler
    pub fn aiplatform(&self) -> aiplatform::AiplatformService<'_> {
        aiplatform::AiplatformService::new(self)
    }
    /// Get aiplatform service handler
    pub fn aiplatform(&self) -> aiplatform::AiplatformService<'_> {
        aiplatform::AiplatformService::new(self)
    }
    /// Get pagespeedonline service handler
    pub fn pagespeedonline(&self) -> pagespeedonline::PagespeedonlineService<'_> {
        pagespeedonline::PagespeedonlineService::new(self)
    }
    /// Get pagespeedonline service handler
    pub fn pagespeedonline(&self) -> pagespeedonline::PagespeedonlineService<'_> {
        pagespeedonline::PagespeedonlineService::new(self)
    }
    /// Get pagespeedonline service handler
    pub fn pagespeedonline(&self) -> pagespeedonline::PagespeedonlineService<'_> {
        pagespeedonline::PagespeedonlineService::new(self)
    }
    /// Get pagespeedonline service handler
    pub fn pagespeedonline(&self) -> pagespeedonline::PagespeedonlineService<'_> {
        pagespeedonline::PagespeedonlineService::new(self)
    }
    /// Get tasks service handler
    pub fn tasks(&self) -> tasks::TasksService<'_> {
        tasks::TasksService::new(self)
    }
    /// Get sqladmin service handler
    pub fn sqladmin(&self) -> sqladmin::SqladminService<'_> {
        sqladmin::SqladminService::new(self)
    }
    /// Get sqladmin service handler
    pub fn sqladmin(&self) -> sqladmin::SqladminService<'_> {
        sqladmin::SqladminService::new(self)
    }
    /// Get discoveryengine service handler
    pub fn discoveryengine(&self) -> discoveryengine::DiscoveryengineService<'_> {
        discoveryengine::DiscoveryengineService::new(self)
    }
    /// Get discoveryengine service handler
    pub fn discoveryengine(&self) -> discoveryengine::DiscoveryengineService<'_> {
        discoveryengine::DiscoveryengineService::new(self)
    }
    /// Get discoveryengine service handler
    pub fn discoveryengine(&self) -> discoveryengine::DiscoveryengineService<'_> {
        discoveryengine::DiscoveryengineService::new(self)
    }
    /// Get gkeonprem service handler
    pub fn gkeonprem(&self) -> gkeonprem::GkeonpremService<'_> {
        gkeonprem::GkeonpremService::new(self)
    }
    /// Get cloudlocationfinder service handler
    pub fn cloudlocationfinder(&self) -> cloudlocationfinder::CloudlocationfinderService<'_> {
        cloudlocationfinder::CloudlocationfinderService::new(self)
    }
    /// Get cloudlocationfinder service handler
    pub fn cloudlocationfinder(&self) -> cloudlocationfinder::CloudlocationfinderService<'_> {
        cloudlocationfinder::CloudlocationfinderService::new(self)
    }
    /// Get mybusinesslodging service handler
    pub fn mybusinesslodging(&self) -> mybusinesslodging::MybusinesslodgingService<'_> {
        mybusinesslodging::MybusinesslodgingService::new(self)
    }
    /// Get mybusinessaccountmanagement service handler
    pub fn mybusinessaccountmanagement(&self) -> mybusinessaccountmanagement::MybusinessaccountmanagementService<'_> {
        mybusinessaccountmanagement::MybusinessaccountmanagementService::new(self)
    }
    /// Get searchads360 service handler
    pub fn searchads360(&self) -> searchads360::Searchads360Service<'_> {
        searchads360::Searchads360Service::new(self)
    }
    /// Get alertcenter service handler
    pub fn alertcenter(&self) -> alertcenter::AlertcenterService<'_> {
        alertcenter::AlertcenterService::new(self)
    }
    /// Get servicemanagement service handler
    pub fn servicemanagement(&self) -> servicemanagement::ServicemanagementService<'_> {
        servicemanagement::ServicemanagementService::new(self)
    }
    /// Get smartdevicemanagement service handler
    pub fn smartdevicemanagement(&self) -> smartdevicemanagement::SmartdevicemanagementService<'_> {
        smartdevicemanagement::SmartdevicemanagementService::new(self)
    }
    /// Get domains service handler
    pub fn domains(&self) -> domains::DomainsService<'_> {
        domains::DomainsService::new(self)
    }
    /// Get domains service handler
    pub fn domains(&self) -> domains::DomainsService<'_> {
        domains::DomainsService::new(self)
    }
    /// Get domains service handler
    pub fn domains(&self) -> domains::DomainsService<'_> {
        domains::DomainsService::new(self)
    }
    /// Get calendar service handler
    pub fn calendar(&self) -> calendar::CalendarService<'_> {
        calendar::CalendarService::new(self)
    }
    /// Get vpcaccess service handler
    pub fn vpcaccess(&self) -> vpcaccess::VpcaccessService<'_> {
        vpcaccess::VpcaccessService::new(self)
    }
    /// Get vpcaccess service handler
    pub fn vpcaccess(&self) -> vpcaccess::VpcaccessService<'_> {
        vpcaccess::VpcaccessService::new(self)
    }
    /// Get sheets service handler
    pub fn sheets(&self) -> sheets::SheetsService<'_> {
        sheets::SheetsService::new(self)
    }
    /// Get looker service handler
    pub fn looker(&self) -> looker::LookerService<'_> {
        looker::LookerService::new(self)
    }
    /// Get forms service handler
    pub fn forms(&self) -> forms::FormsService<'_> {
        forms::FormsService::new(self)
    }
    /// Get cloudiot service handler
    pub fn cloudiot(&self) -> cloudiot::CloudiotService<'_> {
        cloudiot::CloudiotService::new(self)
    }
    /// Get genomics service handler
    pub fn genomics(&self) -> genomics::GenomicsService<'_> {
        genomics::GenomicsService::new(self)
    }
    /// Get genomics service handler
    pub fn genomics(&self) -> genomics::GenomicsService<'_> {
        genomics::GenomicsService::new(self)
    }
    /// Get genomics service handler
    pub fn genomics(&self) -> genomics::GenomicsService<'_> {
        genomics::GenomicsService::new(self)
    }
    /// Get pollen service handler
    pub fn pollen(&self) -> pollen::PollenService<'_> {
        pollen::PollenService::new(self)
    }
    /// Get playgrouping service handler
    pub fn playgrouping(&self) -> playgrouping::PlaygroupingService<'_> {
        playgrouping::PlaygroupingService::new(self)
    }
    /// Get oslogin service handler
    pub fn oslogin(&self) -> oslogin::OsloginService<'_> {
        oslogin::OsloginService::new(self)
    }
    /// Get oslogin service handler
    pub fn oslogin(&self) -> oslogin::OsloginService<'_> {
        oslogin::OsloginService::new(self)
    }
    /// Get oslogin service handler
    pub fn oslogin(&self) -> oslogin::OsloginService<'_> {
        oslogin::OsloginService::new(self)
    }
    /// Get gamesmanagement service handler
    pub fn gamesmanagement(&self) -> gamesmanagement::GamesmanagementService<'_> {
        gamesmanagement::GamesmanagementService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get gkehub service handler
    pub fn gkehub(&self) -> gkehub::GkehubService<'_> {
        gkehub::GkehubService::new(self)
    }
    /// Get securityposture service handler
    pub fn securityposture(&self) -> securityposture::SecuritypostureService<'_> {
        securityposture::SecuritypostureService::new(self)
    }
    /// Get networkmanagement service handler
    pub fn networkmanagement(&self) -> networkmanagement::NetworkmanagementService<'_> {
        networkmanagement::NetworkmanagementService::new(self)
    }
    /// Get networkmanagement service handler
    pub fn networkmanagement(&self) -> networkmanagement::NetworkmanagementService<'_> {
        networkmanagement::NetworkmanagementService::new(self)
    }
    /// Get securitycenter service handler
    pub fn securitycenter(&self) -> securitycenter::SecuritycenterService<'_> {
        securitycenter::SecuritycenterService::new(self)
    }
    /// Get securitycenter service handler
    pub fn securitycenter(&self) -> securitycenter::SecuritycenterService<'_> {
        securitycenter::SecuritycenterService::new(self)
    }
    /// Get securitycenter service handler
    pub fn securitycenter(&self) -> securitycenter::SecuritycenterService<'_> {
        securitycenter::SecuritycenterService::new(self)
    }
    /// Get securitycenter service handler
    pub fn securitycenter(&self) -> securitycenter::SecuritycenterService<'_> {
        securitycenter::SecuritycenterService::new(self)
    }
    /// Get securitycenter service handler
    pub fn securitycenter(&self) -> securitycenter::SecuritycenterService<'_> {
        securitycenter::SecuritycenterService::new(self)
    }
    /// Get datamanager service handler
    pub fn datamanager(&self) -> datamanager::DatamanagerService<'_> {
        datamanager::DatamanagerService::new(self)
    }
    /// Get chat service handler
    pub fn chat(&self) -> chat::ChatService<'_> {
        chat::ChatService::new(self)
    }
    /// Get apphub service handler
    pub fn apphub(&self) -> apphub::ApphubService<'_> {
        apphub::ApphubService::new(self)
    }
    /// Get apphub service handler
    pub fn apphub(&self) -> apphub::ApphubService<'_> {
        apphub::ApphubService::new(self)
    }
    /// Get dialogflow service handler
    pub fn dialogflow(&self) -> dialogflow::DialogflowService<'_> {
        dialogflow::DialogflowService::new(self)
    }
    /// Get dialogflow service handler
    pub fn dialogflow(&self) -> dialogflow::DialogflowService<'_> {
        dialogflow::DialogflowService::new(self)
    }
    /// Get dialogflow service handler
    pub fn dialogflow(&self) -> dialogflow::DialogflowService<'_> {
        dialogflow::DialogflowService::new(self)
    }
    /// Get dialogflow service handler
    pub fn dialogflow(&self) -> dialogflow::DialogflowService<'_> {
        dialogflow::DialogflowService::new(self)
    }
    /// Get dialogflow service handler
    pub fn dialogflow(&self) -> dialogflow::DialogflowService<'_> {
        dialogflow::DialogflowService::new(self)
    }
    /// Get rapidmigrationassessment service handler
    pub fn rapidmigrationassessment(&self) -> rapidmigrationassessment::RapidmigrationassessmentService<'_> {
        rapidmigrationassessment::RapidmigrationassessmentService::new(self)
    }
    /// Get documentai service handler
    pub fn documentai(&self) -> documentai::DocumentaiService<'_> {
        documentai::DocumentaiService::new(self)
    }
    /// Get documentai service handler
    pub fn documentai(&self) -> documentai::DocumentaiService<'_> {
        documentai::DocumentaiService::new(self)
    }
    /// Get documentai service handler
    pub fn documentai(&self) -> documentai::DocumentaiService<'_> {
        documentai::DocumentaiService::new(self)
    }
    /// Get artifactregistry service handler
    pub fn artifactregistry(&self) -> artifactregistry::ArtifactregistryService<'_> {
        artifactregistry::ArtifactregistryService::new(self)
    }
    /// Get artifactregistry service handler
    pub fn artifactregistry(&self) -> artifactregistry::ArtifactregistryService<'_> {
        artifactregistry::ArtifactregistryService::new(self)
    }
    /// Get artifactregistry service handler
    pub fn artifactregistry(&self) -> artifactregistry::ArtifactregistryService<'_> {
        artifactregistry::ArtifactregistryService::new(self)
    }
    /// Get qpxexpress service handler
    pub fn qpxexpress(&self) -> qpxexpress::QpxexpressService<'_> {
        qpxexpress::QpxexpressService::new(self)
    }
    /// Get analyticsdata service handler
    pub fn analyticsdata(&self) -> analyticsdata::AnalyticsdataService<'_> {
        analyticsdata::AnalyticsdataService::new(self)
    }
    /// Get analyticsdata service handler
    pub fn analyticsdata(&self) -> analyticsdata::AnalyticsdataService<'_> {
        analyticsdata::AnalyticsdataService::new(self)
    }
    /// Get file service handler
    pub fn file(&self) -> file::FileService<'_> {
        file::FileService::new(self)
    }
    /// Get file service handler
    pub fn file(&self) -> file::FileService<'_> {
        file::FileService::new(self)
    }
    /// Get cloudresourcemanager service handler
    pub fn cloudresourcemanager(&self) -> cloudresourcemanager::CloudresourcemanagerService<'_> {
        cloudresourcemanager::CloudresourcemanagerService::new(self)
    }
    /// Get cloudresourcemanager service handler
    pub fn cloudresourcemanager(&self) -> cloudresourcemanager::CloudresourcemanagerService<'_> {
        cloudresourcemanager::CloudresourcemanagerService::new(self)
    }
    /// Get cloudresourcemanager service handler
    pub fn cloudresourcemanager(&self) -> cloudresourcemanager::CloudresourcemanagerService<'_> {
        cloudresourcemanager::CloudresourcemanagerService::new(self)
    }
    /// Get cloudresourcemanager service handler
    pub fn cloudresourcemanager(&self) -> cloudresourcemanager::CloudresourcemanagerService<'_> {
        cloudresourcemanager::CloudresourcemanagerService::new(self)
    }
    /// Get cloudresourcemanager service handler
    pub fn cloudresourcemanager(&self) -> cloudresourcemanager::CloudresourcemanagerService<'_> {
        cloudresourcemanager::CloudresourcemanagerService::new(self)
    }
    /// Get css service handler
    pub fn css(&self) -> css::CssService<'_> {
        css::CssService::new(self)
    }
    /// Get walletobjects service handler
    pub fn walletobjects(&self) -> walletobjects::WalletobjectsService<'_> {
        walletobjects::WalletobjectsService::new(self)
    }
    /// Get readerrevenuesubscriptionlinking service handler
    pub fn readerrevenuesubscriptionlinking(&self) -> readerrevenuesubscriptionlinking::ReaderrevenuesubscriptionlinkingService<'_> {
        readerrevenuesubscriptionlinking::ReaderrevenuesubscriptionlinkingService::new(self)
    }
    /// Get youtubereporting service handler
    pub fn youtubereporting(&self) -> youtubereporting::YoutubereportingService<'_> {
        youtubereporting::YoutubereportingService::new(self)
    }
    /// Get firestore service handler
    pub fn firestore(&self) -> firestore::FirestoreService<'_> {
        firestore::FirestoreService::new(self)
    }
    /// Get firestore service handler
    pub fn firestore(&self) -> firestore::FirestoreService<'_> {
        firestore::FirestoreService::new(self)
    }
    /// Get firestore service handler
    pub fn firestore(&self) -> firestore::FirestoreService<'_> {
        firestore::FirestoreService::new(self)
    }
    /// Get config service handler
    pub fn config(&self) -> config::ConfigService<'_> {
        config::ConfigService::new(self)
    }
    /// Get translate service handler
    pub fn translate(&self) -> translate::TranslateService<'_> {
        translate::TranslateService::new(self)
    }
    /// Get translate service handler
    pub fn translate(&self) -> translate::TranslateService<'_> {
        translate::TranslateService::new(self)
    }
    /// Get translate service handler
    pub fn translate(&self) -> translate::TranslateService<'_> {
        translate::TranslateService::new(self)
    }
    /// Get redis service handler
    pub fn redis(&self) -> redis::RedisService<'_> {
        redis::RedisService::new(self)
    }
    /// Get redis service handler
    pub fn redis(&self) -> redis::RedisService<'_> {
        redis::RedisService::new(self)
    }
    /// Get meet service handler
    pub fn meet(&self) -> meet::MeetService<'_> {
        meet::MeetService::new(self)
    }
    /// Get dataplex service handler
    pub fn dataplex(&self) -> dataplex::DataplexService<'_> {
        dataplex::DataplexService::new(self)
    }
    /// Get analyticshub service handler
    pub fn analyticshub(&self) -> analyticshub::AnalyticshubService<'_> {
        analyticshub::AnalyticshubService::new(self)
    }
    /// Get analyticshub service handler
    pub fn analyticshub(&self) -> analyticshub::AnalyticshubService<'_> {
        analyticshub::AnalyticshubService::new(self)
    }
    /// Get playintegrity service handler
    pub fn playintegrity(&self) -> playintegrity::PlayintegrityService<'_> {
        playintegrity::PlayintegrityService::new(self)
    }
    /// Get cloudkms service handler
    pub fn cloudkms(&self) -> cloudkms::CloudkmsService<'_> {
        cloudkms::CloudkmsService::new(self)
    }
    /// Get fusiontables service handler
    pub fn fusiontables(&self) -> fusiontables::FusiontablesService<'_> {
        fusiontables::FusiontablesService::new(self)
    }
    /// Get fusiontables service handler
    pub fn fusiontables(&self) -> fusiontables::FusiontablesService<'_> {
        fusiontables::FusiontablesService::new(self)
    }
    /// Get accessapproval service handler
    pub fn accessapproval(&self) -> accessapproval::AccessapprovalService<'_> {
        accessapproval::AccessapprovalService::new(self)
    }
    /// Get checks service handler
    pub fn checks(&self) -> checks::ChecksService<'_> {
        checks::ChecksService::new(self)
    }
    /// Get cloudprivatecatalog service handler
    pub fn cloudprivatecatalog(&self) -> cloudprivatecatalog::CloudprivatecatalogService<'_> {
        cloudprivatecatalog::CloudprivatecatalogService::new(self)
    }
    /// Get addressvalidation service handler
    pub fn addressvalidation(&self) -> addressvalidation::AddressvalidationService<'_> {
        addressvalidation::AddressvalidationService::new(self)
    }
    /// Get chromewebstore service handler
    pub fn chromewebstore(&self) -> chromewebstore::ChromewebstoreService<'_> {
        chromewebstore::ChromewebstoreService::new(self)
    }
    /// Get chromewebstore service handler
    pub fn chromewebstore(&self) -> chromewebstore::ChromewebstoreService<'_> {
        chromewebstore::ChromewebstoreService::new(self)
    }
    /// Get websecurityscanner service handler
    pub fn websecurityscanner(&self) -> websecurityscanner::WebsecurityscannerService<'_> {
        websecurityscanner::WebsecurityscannerService::new(self)
    }
    /// Get websecurityscanner service handler
    pub fn websecurityscanner(&self) -> websecurityscanner::WebsecurityscannerService<'_> {
        websecurityscanner::WebsecurityscannerService::new(self)
    }
    /// Get websecurityscanner service handler
    pub fn websecurityscanner(&self) -> websecurityscanner::WebsecurityscannerService<'_> {
        websecurityscanner::WebsecurityscannerService::new(self)
    }
    /// Get places service handler
    pub fn places(&self) -> places::PlacesService<'_> {
        places::PlacesService::new(self)
    }
    /// Get recaptchaenterprise service handler
    pub fn recaptchaenterprise(&self) -> recaptchaenterprise::RecaptchaenterpriseService<'_> {
        recaptchaenterprise::RecaptchaenterpriseService::new(self)
    }
    /// Get parallelstore service handler
    pub fn parallelstore(&self) -> parallelstore::ParallelstoreService<'_> {
        parallelstore::ParallelstoreService::new(self)
    }
    /// Get parallelstore service handler
    pub fn parallelstore(&self) -> parallelstore::ParallelstoreService<'_> {
        parallelstore::ParallelstoreService::new(self)
    }
    /// Get contactcenterinsights service handler
    pub fn contactcenterinsights(&self) -> contactcenterinsights::ContactcenterinsightsService<'_> {
        contactcenterinsights::ContactcenterinsightsService::new(self)
    }
    /// Get recommendationengine service handler
    pub fn recommendationengine(&self) -> recommendationengine::RecommendationengineService<'_> {
        recommendationengine::RecommendationengineService::new(self)
    }
    /// Get cloudidentity service handler
    pub fn cloudidentity(&self) -> cloudidentity::CloudidentityService<'_> {
        cloudidentity::CloudidentityService::new(self)
    }
    /// Get cloudidentity service handler
    pub fn cloudidentity(&self) -> cloudidentity::CloudidentityService<'_> {
        cloudidentity::CloudidentityService::new(self)
    }
    /// Get healthcare service handler
    pub fn healthcare(&self) -> healthcare::HealthcareService<'_> {
        healthcare::HealthcareService::new(self)
    }
    /// Get healthcare service handler
    pub fn healthcare(&self) -> healthcare::HealthcareService<'_> {
        healthcare::HealthcareService::new(self)
    }
    /// Get healthcare service handler
    pub fn healthcare(&self) -> healthcare::HealthcareService<'_> {
        healthcare::HealthcareService::new(self)
    }
    /// Get healthcare service handler
    pub fn healthcare(&self) -> healthcare::HealthcareService<'_> {
        healthcare::HealthcareService::new(self)
    }
    /// Get composer service handler
    pub fn composer(&self) -> composer::ComposerService<'_> {
        composer::ComposerService::new(self)
    }
    /// Get composer service handler
    pub fn composer(&self) -> composer::ComposerService<'_> {
        composer::ComposerService::new(self)
    }
    /// Get searchconsole service handler
    pub fn searchconsole(&self) -> searchconsole::SearchconsoleService<'_> {
        searchconsole::SearchconsoleService::new(self)
    }
    /// Get appengine service handler
    pub fn appengine(&self) -> appengine::AppengineService<'_> {
        appengine::AppengineService::new(self)
    }
    /// Get appengine service handler
    pub fn appengine(&self) -> appengine::AppengineService<'_> {
        appengine::AppengineService::new(self)
    }
    /// Get appengine service handler
    pub fn appengine(&self) -> appengine::AppengineService<'_> {
        appengine::AppengineService::new(self)
    }
    /// Get appengine service handler
    pub fn appengine(&self) -> appengine::AppengineService<'_> {
        appengine::AppengineService::new(self)
    }
    /// Get appengine service handler
    pub fn appengine(&self) -> appengine::AppengineService<'_> {
        appengine::AppengineService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get dfareporting service handler
    pub fn dfareporting(&self) -> dfareporting::DfareportingService<'_> {
        dfareporting::DfareportingService::new(self)
    }
    /// Get apikeys service handler
    pub fn apikeys(&self) -> apikeys::ApikeysService<'_> {
        apikeys::ApikeysService::new(self)
    }
    /// Get bigqueryconnection service handler
    pub fn bigqueryconnection(&self) -> bigqueryconnection::BigqueryconnectionService<'_> {
        bigqueryconnection::BigqueryconnectionService::new(self)
    }
    /// Get bigqueryconnection service handler
    pub fn bigqueryconnection(&self) -> bigqueryconnection::BigqueryconnectionService<'_> {
        bigqueryconnection::BigqueryconnectionService::new(self)
    }
    /// Get firebasedataconnect service handler
    pub fn firebasedataconnect(&self) -> firebasedataconnect::FirebasedataconnectService<'_> {
        firebasedataconnect::FirebasedataconnectService::new(self)
    }
    /// Get firebasedataconnect service handler
    pub fn firebasedataconnect(&self) -> firebasedataconnect::FirebasedataconnectService<'_> {
        firebasedataconnect::FirebasedataconnectService::new(self)
    }
    /// Get commentanalyzer service handler
    pub fn commentanalyzer(&self) -> commentanalyzer::CommentanalyzerService<'_> {
        commentanalyzer::CommentanalyzerService::new(self)
    }
    /// Get firebaseapphosting service handler
    pub fn firebaseapphosting(&self) -> firebaseapphosting::FirebaseapphostingService<'_> {
        firebaseapphosting::FirebaseapphostingService::new(self)
    }
    /// Get firebaseapphosting service handler
    pub fn firebaseapphosting(&self) -> firebaseapphosting::FirebaseapphostingService<'_> {
        firebaseapphosting::FirebaseapphostingService::new(self)
    }
    /// Get doubleclickbidmanager service handler
    pub fn doubleclickbidmanager(&self) -> doubleclickbidmanager::DoubleclickbidmanagerService<'_> {
        doubleclickbidmanager::DoubleclickbidmanagerService::new(self)
    }
    /// Get doubleclickbidmanager service handler
    pub fn doubleclickbidmanager(&self) -> doubleclickbidmanager::DoubleclickbidmanagerService<'_> {
        doubleclickbidmanager::DoubleclickbidmanagerService::new(self)
    }
    /// Get doubleclickbidmanager service handler
    pub fn doubleclickbidmanager(&self) -> doubleclickbidmanager::DoubleclickbidmanagerService<'_> {
        doubleclickbidmanager::DoubleclickbidmanagerService::new(self)
    }
    /// Get fcmdata service handler
    pub fn fcmdata(&self) -> fcmdata::FcmdataService<'_> {
        fcmdata::FcmdataService::new(self)
    }
    /// Get firebasedatabase service handler
    pub fn firebasedatabase(&self) -> firebasedatabase::FirebasedatabaseService<'_> {
        firebasedatabase::FirebasedatabaseService::new(self)
    }
    /// Get ideahub service handler
    pub fn ideahub(&self) -> ideahub::IdeahubService<'_> {
        ideahub::IdeahubService::new(self)
    }
    /// Get ideahub service handler
    pub fn ideahub(&self) -> ideahub::IdeahubService<'_> {
        ideahub::IdeahubService::new(self)
    }
    /// Get firebaseremoteconfig service handler
    pub fn firebaseremoteconfig(&self) -> firebaseremoteconfig::FirebaseremoteconfigService<'_> {
        firebaseremoteconfig::FirebaseremoteconfigService::new(self)
    }
    /// Get prod_tt_sasportal service handler
    pub fn prod_tt_sasportal(&self) -> prod_tt_sasportal::Prod_tt_sasportalService<'_> {
        prod_tt_sasportal::Prod_tt_sasportalService::new(self)
    }
    /// Get firebaseappdistribution service handler
    pub fn firebaseappdistribution(&self) -> firebaseappdistribution::FirebaseappdistributionService<'_> {
        firebaseappdistribution::FirebaseappdistributionService::new(self)
    }
    /// Get firebaseappdistribution service handler
    pub fn firebaseappdistribution(&self) -> firebaseappdistribution::FirebaseappdistributionService<'_> {
        firebaseappdistribution::FirebaseappdistributionService::new(self)
    }
    /// Get deploymentmanager service handler
    pub fn deploymentmanager(&self) -> deploymentmanager::DeploymentmanagerService<'_> {
        deploymentmanager::DeploymentmanagerService::new(self)
    }
    /// Get deploymentmanager service handler
    pub fn deploymentmanager(&self) -> deploymentmanager::DeploymentmanagerService<'_> {
        deploymentmanager::DeploymentmanagerService::new(self)
    }
    /// Get deploymentmanager service handler
    pub fn deploymentmanager(&self) -> deploymentmanager::DeploymentmanagerService<'_> {
        deploymentmanager::DeploymentmanagerService::new(self)
    }
    /// Get datafusion service handler
    pub fn datafusion(&self) -> datafusion::DatafusionService<'_> {
        datafusion::DatafusionService::new(self)
    }
    /// Get datafusion service handler
    pub fn datafusion(&self) -> datafusion::DatafusionService<'_> {
        datafusion::DatafusionService::new(self)
    }
    /// Get tagmanager service handler
    pub fn tagmanager(&self) -> tagmanager::TagmanagerService<'_> {
        tagmanager::TagmanagerService::new(self)
    }
    /// Get tagmanager service handler
    pub fn tagmanager(&self) -> tagmanager::TagmanagerService<'_> {
        tagmanager::TagmanagerService::new(self)
    }
    /// Get discovery service handler
    pub fn discovery(&self) -> discovery::DiscoveryService<'_> {
        discovery::DiscoveryService::new(self)
    }
    /// Get storagetransfer service handler
    pub fn storagetransfer(&self) -> storagetransfer::StoragetransferService<'_> {
        storagetransfer::StoragetransferService::new(self)
    }
    /// Get oauth2 service handler
    pub fn oauth2(&self) -> oauth2::Oauth2Service<'_> {
        oauth2::Oauth2Service::new(self)
    }
    /// Get oauth2 service handler
    pub fn oauth2(&self) -> oauth2::Oauth2Service<'_> {
        oauth2::Oauth2Service::new(self)
    }
    /// Get managedkafka service handler
    pub fn managedkafka(&self) -> managedkafka::ManagedkafkaService<'_> {
        managedkafka::ManagedkafkaService::new(self)
    }
    /// Get admin service handler
    pub fn admin(&self) -> admin::AdminService<'_> {
        admin::AdminService::new(self)
    }
    /// Get admin service handler
    pub fn admin(&self) -> admin::AdminService<'_> {
        admin::AdminService::new(self)
    }
    /// Get admin service handler
    pub fn admin(&self) -> admin::AdminService<'_> {
        admin::AdminService::new(self)
    }
    /// Get bigqueryreservation service handler
    pub fn bigqueryreservation(&self) -> bigqueryreservation::BigqueryreservationService<'_> {
        bigqueryreservation::BigqueryreservationService::new(self)
    }
    /// Get bigqueryreservation service handler
    pub fn bigqueryreservation(&self) -> bigqueryreservation::BigqueryreservationService<'_> {
        bigqueryreservation::BigqueryreservationService::new(self)
    }
    /// Get bigqueryreservation service handler
    pub fn bigqueryreservation(&self) -> bigqueryreservation::BigqueryreservationService<'_> {
        bigqueryreservation::BigqueryreservationService::new(self)
    }
    /// Get vmmigration service handler
    pub fn vmmigration(&self) -> vmmigration::VmmigrationService<'_> {
        vmmigration::VmmigrationService::new(self)
    }
    /// Get vmmigration service handler
    pub fn vmmigration(&self) -> vmmigration::VmmigrationService<'_> {
        vmmigration::VmmigrationService::new(self)
    }
    /// Get contactcenteraiplatform service handler
    pub fn contactcenteraiplatform(&self) -> contactcenteraiplatform::ContactcenteraiplatformService<'_> {
        contactcenteraiplatform::ContactcenteraiplatformService::new(self)
    }
    /// Get doubleclicksearch service handler
    pub fn doubleclicksearch(&self) -> doubleclicksearch::DoubleclicksearchService<'_> {
        doubleclicksearch::DoubleclicksearchService::new(self)
    }
    /// Get backupdr service handler
    pub fn backupdr(&self) -> backupdr::BackupdrService<'_> {
        backupdr::BackupdrService::new(self)
    }
    /// Get ondemandscanning service handler
    pub fn ondemandscanning(&self) -> ondemandscanning::OndemandscanningService<'_> {
        ondemandscanning::OndemandscanningService::new(self)
    }
    /// Get ondemandscanning service handler
    pub fn ondemandscanning(&self) -> ondemandscanning::OndemandscanningService<'_> {
        ondemandscanning::OndemandscanningService::new(self)
    }
    /// Get cloudbuild service handler
    pub fn cloudbuild(&self) -> cloudbuild::CloudbuildService<'_> {
        cloudbuild::CloudbuildService::new(self)
    }
    /// Get cloudbuild service handler
    pub fn cloudbuild(&self) -> cloudbuild::CloudbuildService<'_> {
        cloudbuild::CloudbuildService::new(self)
    }
    /// Get cloudbuild service handler
    pub fn cloudbuild(&self) -> cloudbuild::CloudbuildService<'_> {
        cloudbuild::CloudbuildService::new(self)
    }
    /// Get cloudbuild service handler
    pub fn cloudbuild(&self) -> cloudbuild::CloudbuildService<'_> {
        cloudbuild::CloudbuildService::new(self)
    }
    /// Get cloudbuild service handler
    pub fn cloudbuild(&self) -> cloudbuild::CloudbuildService<'_> {
        cloudbuild::CloudbuildService::new(self)
    }
    /// Get policyanalyzer service handler
    pub fn policyanalyzer(&self) -> policyanalyzer::PolicyanalyzerService<'_> {
        policyanalyzer::PolicyanalyzerService::new(self)
    }
    /// Get policyanalyzer service handler
    pub fn policyanalyzer(&self) -> policyanalyzer::PolicyanalyzerService<'_> {
        policyanalyzer::PolicyanalyzerService::new(self)
    }
    /// Get workflows service handler
    pub fn workflows(&self) -> workflows::WorkflowsService<'_> {
        workflows::WorkflowsService::new(self)
    }
    /// Get workflows service handler
    pub fn workflows(&self) -> workflows::WorkflowsService<'_> {
        workflows::WorkflowsService::new(self)
    }
    /// Get secretmanager service handler
    pub fn secretmanager(&self) -> secretmanager::SecretmanagerService<'_> {
        secretmanager::SecretmanagerService::new(self)
    }
    /// Get secretmanager service handler
    pub fn secretmanager(&self) -> secretmanager::SecretmanagerService<'_> {
        secretmanager::SecretmanagerService::new(self)
    }
    /// Get secretmanager service handler
    pub fn secretmanager(&self) -> secretmanager::SecretmanagerService<'_> {
        secretmanager::SecretmanagerService::new(self)
    }
    /// Get transcoder service handler
    pub fn transcoder(&self) -> transcoder::TranscoderService<'_> {
        transcoder::TranscoderService::new(self)
    }
    /// Get transcoder service handler
    pub fn transcoder(&self) -> transcoder::TranscoderService<'_> {
        transcoder::TranscoderService::new(self)
    }
    /// Get accesscontextmanager service handler
    pub fn accesscontextmanager(&self) -> accesscontextmanager::AccesscontextmanagerService<'_> {
        accesscontextmanager::AccesscontextmanagerService::new(self)
    }
    /// Get accesscontextmanager service handler
    pub fn accesscontextmanager(&self) -> accesscontextmanager::AccesscontextmanagerService<'_> {
        accesscontextmanager::AccesscontextmanagerService::new(self)
    }
    /// Get content service handler
    pub fn content(&self) -> content::ContentService<'_> {
        content::ContentService::new(self)
    }
    /// Get content service handler
    pub fn content(&self) -> content::ContentService<'_> {
        content::ContentService::new(self)
    }
    /// Get content service handler
    pub fn content(&self) -> content::ContentService<'_> {
        content::ContentService::new(self)
    }
    /// Get acceleratedmobilepageurl service handler
    pub fn acceleratedmobilepageurl(&self) -> acceleratedmobilepageurl::AcceleratedmobilepageurlService<'_> {
        acceleratedmobilepageurl::AcceleratedmobilepageurlService::new(self)
    }
    /// Get androidpublisher service handler
    pub fn androidpublisher(&self) -> androidpublisher::AndroidpublisherService<'_> {
        androidpublisher::AndroidpublisherService::new(self)
    }
    /// Get androidpublisher service handler
    pub fn androidpublisher(&self) -> androidpublisher::AndroidpublisherService<'_> {
        androidpublisher::AndroidpublisherService::new(self)
    }
    /// Get androidpublisher service handler
    pub fn androidpublisher(&self) -> androidpublisher::AndroidpublisherService<'_> {
        androidpublisher::AndroidpublisherService::new(self)
    }
    /// Get digitalassetlinks service handler
    pub fn digitalassetlinks(&self) -> digitalassetlinks::DigitalassetlinksService<'_> {
        digitalassetlinks::DigitalassetlinksService::new(self)
    }
    /// Get classroom service handler
    pub fn classroom(&self) -> classroom::ClassroomService<'_> {
        classroom::ClassroomService::new(self)
    }
    /// Get chromeuxreport service handler
    pub fn chromeuxreport(&self) -> chromeuxreport::ChromeuxreportService<'_> {
        chromeuxreport::ChromeuxreportService::new(self)
    }
    /// Get admob service handler
    pub fn admob(&self) -> admob::AdmobService<'_> {
        admob::AdmobService::new(self)
    }
    /// Get admob service handler
    pub fn admob(&self) -> admob::AdmobService<'_> {
        admob::AdmobService::new(self)
    }
    /// Get gameservices service handler
    pub fn gameservices(&self) -> gameservices::GameservicesService<'_> {
        gameservices::GameservicesService::new(self)
    }
    /// Get gameservices service handler
    pub fn gameservices(&self) -> gameservices::GameservicesService<'_> {
        gameservices::GameservicesService::new(self)
    }
    /// Get language service handler
    pub fn language(&self) -> language::LanguageService<'_> {
        language::LanguageService::new(self)
    }
    /// Get language service handler
    pub fn language(&self) -> language::LanguageService<'_> {
        language::LanguageService::new(self)
    }
    /// Get language service handler
    pub fn language(&self) -> language::LanguageService<'_> {
        language::LanguageService::new(self)
    }
    /// Get language service handler
    pub fn language(&self) -> language::LanguageService<'_> {
        language::LanguageService::new(self)
    }
    /// Get retail service handler
    pub fn retail(&self) -> retail::RetailService<'_> {
        retail::RetailService::new(self)
    }
    /// Get retail service handler
    pub fn retail(&self) -> retail::RetailService<'_> {
        retail::RetailService::new(self)
    }
    /// Get retail service handler
    pub fn retail(&self) -> retail::RetailService<'_> {
        retail::RetailService::new(self)
    }
    /// Get bigquery service handler
    pub fn bigquery(&self) -> bigquery::BigqueryService<'_> {
        bigquery::BigqueryService::new(self)
    }
    /// Get driveactivity service handler
    pub fn driveactivity(&self) -> driveactivity::DriveactivityService<'_> {
        driveactivity::DriveactivityService::new(self)
    }
    /// Get serviceuser service handler
    pub fn serviceuser(&self) -> serviceuser::ServiceuserService<'_> {
        serviceuser::ServiceuserService::new(self)
    }
    /// Get homegraph service handler
    pub fn homegraph(&self) -> homegraph::HomegraphService<'_> {
        homegraph::HomegraphService::new(self)
    }
    /// Get vision service handler
    pub fn vision(&self) -> vision::VisionService<'_> {
        vision::VisionService::new(self)
    }
    /// Get vision service handler
    pub fn vision(&self) -> vision::VisionService<'_> {
        vision::VisionService::new(self)
    }
    /// Get vision service handler
    pub fn vision(&self) -> vision::VisionService<'_> {
        vision::VisionService::new(self)
    }
    /// Get firebaseappcheck service handler
    pub fn firebaseappcheck(&self) -> firebaseappcheck::FirebaseappcheckService<'_> {
        firebaseappcheck::FirebaseappcheckService::new(self)
    }
    /// Get firebaseappcheck service handler
    pub fn firebaseappcheck(&self) -> firebaseappcheck::FirebaseappcheckService<'_> {
        firebaseappcheck::FirebaseappcheckService::new(self)
    }
    /// Get appsactivity service handler
    pub fn appsactivity(&self) -> appsactivity::AppsactivityService<'_> {
        appsactivity::AppsactivityService::new(self)
    }
    /// Get cloudsearch service handler
    pub fn cloudsearch(&self) -> cloudsearch::CloudsearchService<'_> {
        cloudsearch::CloudsearchService::new(self)
    }
    /// Get books service handler
    pub fn books(&self) -> books::BooksService<'_> {
        books::BooksService::new(self)
    }
    /// Get docs service handler
    pub fn docs(&self) -> docs::DocsService<'_> {
        docs::DocsService::new(self)
    }
    /// Get firebasehosting service handler
    pub fn firebasehosting(&self) -> firebasehosting::FirebasehostingService<'_> {
        firebasehosting::FirebasehostingService::new(self)
    }
    /// Get firebasehosting service handler
    pub fn firebasehosting(&self) -> firebasehosting::FirebasehostingService<'_> {
        firebasehosting::FirebasehostingService::new(self)
    }
    /// Get youtube service handler
    pub fn youtube(&self) -> youtube::YoutubeService<'_> {
        youtube::YoutubeService::new(self)
    }
    /// Get observability service handler
    pub fn observability(&self) -> observability::ObservabilityService<'_> {
        observability::ObservabilityService::new(self)
    }
    /// Get adsensehost service handler
    pub fn adsensehost(&self) -> adsensehost::AdsensehostService<'_> {
        adsensehost::AdsensehostService::new(self)
    }
    /// Get lifesciences service handler
    pub fn lifesciences(&self) -> lifesciences::LifesciencesService<'_> {
        lifesciences::LifesciencesService::new(self)
    }
    /// Get adexchangebuyer service handler
    pub fn adexchangebuyer(&self) -> adexchangebuyer::AdexchangebuyerService<'_> {
        adexchangebuyer::AdexchangebuyerService::new(self)
    }
    /// Get adexchangebuyer service handler
    pub fn adexchangebuyer(&self) -> adexchangebuyer::AdexchangebuyerService<'_> {
        adexchangebuyer::AdexchangebuyerService::new(self)
    }
    /// Get adexchangebuyer service handler
    pub fn adexchangebuyer(&self) -> adexchangebuyer::AdexchangebuyerService<'_> {
        adexchangebuyer::AdexchangebuyerService::new(self)
    }
    /// Get streetviewpublish service handler
    pub fn streetviewpublish(&self) -> streetviewpublish::StreetviewpublishService<'_> {
        streetviewpublish::StreetviewpublishService::new(self)
    }
    /// Get advisorynotifications service handler
    pub fn advisorynotifications(&self) -> advisorynotifications::AdvisorynotificationsService<'_> {
        advisorynotifications::AdvisorynotificationsService::new(self)
    }
    /// Get sourcerepo service handler
    pub fn sourcerepo(&self) -> sourcerepo::SourcerepoService<'_> {
        sourcerepo::SourcerepoService::new(self)
    }
    /// Get vectortile service handler
    pub fn vectortile(&self) -> vectortile::VectortileService<'_> {
        vectortile::VectortileService::new(self)
    }
    /// Get paymentsresellersubscription service handler
    pub fn paymentsresellersubscription(&self) -> paymentsresellersubscription::PaymentsresellersubscriptionService<'_> {
        paymentsresellersubscription::PaymentsresellersubscriptionService::new(self)
    }
    /// Get airquality service handler
    pub fn airquality(&self) -> airquality::AirqualityService<'_> {
        airquality::AirqualityService::new(self)
    }
    /// Get localservices service handler
    pub fn localservices(&self) -> localservices::LocalservicesService<'_> {
        localservices::LocalservicesService::new(self)
    }
    /// Get adexchangeseller service handler
    pub fn adexchangeseller(&self) -> adexchangeseller::AdexchangesellerService<'_> {
        adexchangeseller::AdexchangesellerService::new(self)
    }
    /// Get adexchangeseller service handler
    pub fn adexchangeseller(&self) -> adexchangeseller::AdexchangesellerService<'_> {
        adexchangeseller::AdexchangesellerService::new(self)
    }
    /// Get adexchangeseller service handler
    pub fn adexchangeseller(&self) -> adexchangeseller::AdexchangesellerService<'_> {
        adexchangeseller::AdexchangesellerService::new(self)
    }
    /// Get servicebroker service handler
    pub fn servicebroker(&self) -> servicebroker::ServicebrokerService<'_> {
        servicebroker::ServicebrokerService::new(self)
    }
    /// Get servicebroker service handler
    pub fn servicebroker(&self) -> servicebroker::ServicebrokerService<'_> {
        servicebroker::ServicebrokerService::new(self)
    }
    /// Get servicebroker service handler
    pub fn servicebroker(&self) -> servicebroker::ServicebrokerService<'_> {
        servicebroker::ServicebrokerService::new(self)
    }
    /// Get speech service handler
    pub fn speech(&self) -> speech::SpeechService<'_> {
        speech::SpeechService::new(self)
    }
    /// Get speech service handler
    pub fn speech(&self) -> speech::SpeechService<'_> {
        speech::SpeechService::new(self)
    }
    /// Get speech service handler
    pub fn speech(&self) -> speech::SpeechService<'_> {
        speech::SpeechService::new(self)
    }
    /// Get speech service handler
    pub fn speech(&self) -> speech::SpeechService<'_> {
        speech::SpeechService::new(self)
    }
    /// Get speech service handler
    pub fn speech(&self) -> speech::SpeechService<'_> {
        speech::SpeechService::new(self)
    }
    /// Get script service handler
    pub fn script(&self) -> script::ScriptService<'_> {
        script::ScriptService::new(self)
    }
    /// Get realtimebidding service handler
    pub fn realtimebidding(&self) -> realtimebidding::RealtimebiddingService<'_> {
        realtimebidding::RealtimebiddingService::new(self)
    }
    /// Get realtimebidding service handler
    pub fn realtimebidding(&self) -> realtimebidding::RealtimebiddingService<'_> {
        realtimebidding::RealtimebiddingService::new(self)
    }
    /// Get chromepolicy service handler
    pub fn chromepolicy(&self) -> chromepolicy::ChromepolicyService<'_> {
        chromepolicy::ChromepolicyService::new(self)
    }
    /// Get storage service handler
    pub fn storage(&self) -> storage::StorageService<'_> {
        storage::StorageService::new(self)
    }
    /// Get storage service handler
    pub fn storage(&self) -> storage::StorageService<'_> {
        storage::StorageService::new(self)
    }
    /// Get storage service handler
    pub fn storage(&self) -> storage::StorageService<'_> {
        storage::StorageService::new(self)
    }
    /// Get slides service handler
    pub fn slides(&self) -> slides::SlidesService<'_> {
        slides::SlidesService::new(self)
    }
    /// Get reseller service handler
    pub fn reseller(&self) -> reseller::ResellerService<'_> {
        reseller::ResellerService::new(self)
    }
    /// Get mybusinessbusinessinformation service handler
    pub fn mybusinessbusinessinformation(&self) -> mybusinessbusinessinformation::MybusinessbusinessinformationService<'_> {
        mybusinessbusinessinformation::MybusinessbusinessinformationService::new(self)
    }
    /// Get androidenterprise service handler
    pub fn androidenterprise(&self) -> androidenterprise::AndroidenterpriseService<'_> {
        androidenterprise::AndroidenterpriseService::new(self)
    }
    /// Get ids service handler
    pub fn ids(&self) -> ids::IdsService<'_> {
        ids::IdsService::new(self)
    }
    /// Get adsense service handler
    pub fn adsense(&self) -> adsense::AdsenseService<'_> {
        adsense::AdsenseService::new(self)
    }
    /// Get adsense service handler
    pub fn adsense(&self) -> adsense::AdsenseService<'_> {
        adsense::AdsenseService::new(self)
    }
    /// Get adsense service handler
    pub fn adsense(&self) -> adsense::AdsenseService<'_> {
        adsense::AdsenseService::new(self)
    }
    /// Get videointelligence service handler
    pub fn videointelligence(&self) -> videointelligence::VideointelligenceService<'_> {
        videointelligence::VideointelligenceService::new(self)
    }
    /// Get videointelligence service handler
    pub fn videointelligence(&self) -> videointelligence::VideointelligenceService<'_> {
        videointelligence::VideointelligenceService::new(self)
    }
    /// Get videointelligence service handler
    pub fn videointelligence(&self) -> videointelligence::VideointelligenceService<'_> {
        videointelligence::VideointelligenceService::new(self)
    }
    /// Get videointelligence service handler
    pub fn videointelligence(&self) -> videointelligence::VideointelligenceService<'_> {
        videointelligence::VideointelligenceService::new(self)
    }
    /// Get videointelligence service handler
    pub fn videointelligence(&self) -> videointelligence::VideointelligenceService<'_> {
        videointelligence::VideointelligenceService::new(self)
    }
    /// Get datamigration service handler
    pub fn datamigration(&self) -> datamigration::DatamigrationService<'_> {
        datamigration::DatamigrationService::new(self)
    }
    /// Get datamigration service handler
    pub fn datamigration(&self) -> datamigration::DatamigrationService<'_> {
        datamigration::DatamigrationService::new(self)
    }
    /// Get gkebackup service handler
    pub fn gkebackup(&self) -> gkebackup::GkebackupService<'_> {
        gkebackup::GkebackupService::new(self)
    }
    /// Get adsenseplatform service handler
    pub fn adsenseplatform(&self) -> adsenseplatform::AdsenseplatformService<'_> {
        adsenseplatform::AdsenseplatformService::new(self)
    }
    /// Get adsenseplatform service handler
    pub fn adsenseplatform(&self) -> adsenseplatform::AdsenseplatformService<'_> {
        adsenseplatform::AdsenseplatformService::new(self)
    }
    /// Get integrations service handler
    pub fn integrations(&self) -> integrations::IntegrationsService<'_> {
        integrations::IntegrationsService::new(self)
    }
    /// Get integrations service handler
    pub fn integrations(&self) -> integrations::IntegrationsService<'_> {
        integrations::IntegrationsService::new(self)
    }
    /// Get customsearch service handler
    pub fn customsearch(&self) -> customsearch::CustomsearchService<'_> {
        customsearch::CustomsearchService::new(self)
    }
    /// Get kgsearch service handler
    pub fn kgsearch(&self) -> kgsearch::KgsearchService<'_> {
        kgsearch::KgsearchService::new(self)
    }
    /// Get testing service handler
    pub fn testing(&self) -> testing::TestingService<'_> {
        testing::TestingService::new(self)
    }
    /// Get container service handler
    pub fn container(&self) -> container::ContainerService<'_> {
        container::ContainerService::new(self)
    }
    /// Get container service handler
    pub fn container(&self) -> container::ContainerService<'_> {
        container::ContainerService::new(self)
    }
    /// Get datastore service handler
    pub fn datastore(&self) -> datastore::DatastoreService<'_> {
        datastore::DatastoreService::new(self)
    }
    /// Get datastore service handler
    pub fn datastore(&self) -> datastore::DatastoreService<'_> {
        datastore::DatastoreService::new(self)
    }
    /// Get datastore service handler
    pub fn datastore(&self) -> datastore::DatastoreService<'_> {
        datastore::DatastoreService::new(self)
    }
    /// Get playmoviespartner service handler
    pub fn playmoviespartner(&self) -> playmoviespartner::PlaymoviespartnerService<'_> {
        playmoviespartner::PlaymoviespartnerService::new(self)
    }
    /// Get memcache service handler
    pub fn memcache(&self) -> memcache::MemcacheService<'_> {
        memcache::MemcacheService::new(self)
    }
    /// Get memcache service handler
    pub fn memcache(&self) -> memcache::MemcacheService<'_> {
        memcache::MemcacheService::new(self)
    }
    /// Get blockchainnodeengine service handler
    pub fn blockchainnodeengine(&self) -> blockchainnodeengine::BlockchainnodeengineService<'_> {
        blockchainnodeengine::BlockchainnodeengineService::new(self)
    }
    /// Get cloudtrace service handler
    pub fn cloudtrace(&self) -> cloudtrace::CloudtraceService<'_> {
        cloudtrace::CloudtraceService::new(self)
    }
    /// Get cloudtrace service handler
    pub fn cloudtrace(&self) -> cloudtrace::CloudtraceService<'_> {
        cloudtrace::CloudtraceService::new(self)
    }
    /// Get cloudtrace service handler
    pub fn cloudtrace(&self) -> cloudtrace::CloudtraceService<'_> {
        cloudtrace::CloudtraceService::new(self)
    }
    /// Get iap service handler
    pub fn iap(&self) -> iap::IapService<'_> {
        iap::IapService::new(self)
    }
    /// Get iap service handler
    pub fn iap(&self) -> iap::IapService<'_> {
        iap::IapService::new(self)
    }
    /// Get playcustomapp service handler
    pub fn playcustomapp(&self) -> playcustomapp::PlaycustomappService<'_> {
        playcustomapp::PlaycustomappService::new(self)
    }
    /// Get dataportability service handler
    pub fn dataportability(&self) -> dataportability::DataportabilityService<'_> {
        dataportability::DataportabilityService::new(self)
    }
    /// Get dataportability service handler
    pub fn dataportability(&self) -> dataportability::DataportabilityService<'_> {
        dataportability::DataportabilityService::new(self)
    }
    /// Get mybusinessbusinesscalls service handler
    pub fn mybusinessbusinesscalls(&self) -> mybusinessbusinesscalls::MybusinessbusinesscallsService<'_> {
        mybusinessbusinesscalls::MybusinessbusinesscallsService::new(self)
    }
    /// Get policysimulator service handler
    pub fn policysimulator(&self) -> policysimulator::PolicysimulatorService<'_> {
        policysimulator::PolicysimulatorService::new(self)
    }
    /// Get policysimulator service handler
    pub fn policysimulator(&self) -> policysimulator::PolicysimulatorService<'_> {
        policysimulator::PolicysimulatorService::new(self)
    }
    /// Get policysimulator service handler
    pub fn policysimulator(&self) -> policysimulator::PolicysimulatorService<'_> {
        policysimulator::PolicysimulatorService::new(self)
    }
    /// Get policysimulator service handler
    pub fn policysimulator(&self) -> policysimulator::PolicysimulatorService<'_> {
        policysimulator::PolicysimulatorService::new(self)
    }
    /// Get firebase service handler
    pub fn firebase(&self) -> firebase::FirebaseService<'_> {
        firebase::FirebaseService::new(self)
    }
    /// Get iam service handler
    pub fn iam(&self) -> iam::IamService<'_> {
        iam::IamService::new(self)
    }
    /// Get iam service handler
    pub fn iam(&self) -> iam::IamService<'_> {
        iam::IamService::new(self)
    }
    /// Get iam service handler
    pub fn iam(&self) -> iam::IamService<'_> {
        iam::IamService::new(self)
    }
    /// Get bigtableadmin service handler
    pub fn bigtableadmin(&self) -> bigtableadmin::BigtableadminService<'_> {
        bigtableadmin::BigtableadminService::new(self)
    }
    /// Get bigtableadmin service handler
    pub fn bigtableadmin(&self) -> bigtableadmin::BigtableadminService<'_> {
        bigtableadmin::BigtableadminService::new(self)
    }
    /// Get servicedirectory service handler
    pub fn servicedirectory(&self) -> servicedirectory::ServicedirectoryService<'_> {
        servicedirectory::ServicedirectoryService::new(self)
    }
    /// Get servicedirectory service handler
    pub fn servicedirectory(&self) -> servicedirectory::ServicedirectoryService<'_> {
        servicedirectory::ServicedirectoryService::new(self)
    }
    /// Get recommender service handler
    pub fn recommender(&self) -> recommender::RecommenderService<'_> {
        recommender::RecommenderService::new(self)
    }
    /// Get recommender service handler
    pub fn recommender(&self) -> recommender::RecommenderService<'_> {
        recommender::RecommenderService::new(self)
    }
    /// Get trafficdirector service handler
    pub fn trafficdirector(&self) -> trafficdirector::TrafficdirectorService<'_> {
        trafficdirector::TrafficdirectorService::new(self)
    }
    /// Get trafficdirector service handler
    pub fn trafficdirector(&self) -> trafficdirector::TrafficdirectorService<'_> {
        trafficdirector::TrafficdirectorService::new(self)
    }
    /// Get drivelabels service handler
    pub fn drivelabels(&self) -> drivelabels::DrivelabelsService<'_> {
        drivelabels::DrivelabelsService::new(self)
    }
    /// Get drivelabels service handler
    pub fn drivelabels(&self) -> drivelabels::DrivelabelsService<'_> {
        drivelabels::DrivelabelsService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudasset service handler
    pub fn cloudasset(&self) -> cloudasset::CloudassetService<'_> {
        cloudasset::CloudassetService::new(self)
    }
    /// Get cloudbilling service handler
    pub fn cloudbilling(&self) -> cloudbilling::CloudbillingService<'_> {
        cloudbilling::CloudbillingService::new(self)
    }
    /// Get cloudbilling service handler
    pub fn cloudbilling(&self) -> cloudbilling::CloudbillingService<'_> {
        cloudbilling::CloudbillingService::new(self)
    }
    /// Get webmasters service handler
    pub fn webmasters(&self) -> webmasters::WebmastersService<'_> {
        webmasters::WebmastersService::new(self)
    }
    /// Get compute service handler
    pub fn compute(&self) -> compute::ComputeService<'_> {
        compute::ComputeService::new(self)
    }
    /// Get compute service handler
    pub fn compute(&self) -> compute::ComputeService<'_> {
        compute::ComputeService::new(self)
    }
    /// Get compute service handler
    pub fn compute(&self) -> compute::ComputeService<'_> {
        compute::ComputeService::new(self)
    }
    /// Get groupssettings service handler
    pub fn groupssettings(&self) -> groupssettings::GroupssettingsService<'_> {
        groupssettings::GroupssettingsService::new(self)
    }
    /// Get workloadmanager service handler
    pub fn workloadmanager(&self) -> workloadmanager::WorkloadmanagerService<'_> {
        workloadmanager::WorkloadmanagerService::new(self)
    }
    /// Get abusiveexperiencereport service handler
    pub fn abusiveexperiencereport(&self) -> abusiveexperiencereport::AbusiveexperiencereportService<'_> {
        abusiveexperiencereport::AbusiveexperiencereportService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get displayvideo service handler
    pub fn displayvideo(&self) -> displayvideo::DisplayvideoService<'_> {
        displayvideo::DisplayvideoService::new(self)
    }
    /// Get ml service handler
    pub fn ml(&self) -> ml::MlService<'_> {
        ml::MlService::new(self)
    }
    /// Get networkconnectivity service handler
    pub fn networkconnectivity(&self) -> networkconnectivity::NetworkconnectivityService<'_> {
        networkconnectivity::NetworkconnectivityService::new(self)
    }
    /// Get networkconnectivity service handler
    pub fn networkconnectivity(&self) -> networkconnectivity::NetworkconnectivityService<'_> {
        networkconnectivity::NetworkconnectivityService::new(self)
    }
    /// Get batch service handler
    pub fn batch(&self) -> batch::BatchService<'_> {
        batch::BatchService::new(self)
    }
    /// Get webfonts service handler
    pub fn webfonts(&self) -> webfonts::WebfontsService<'_> {
        webfonts::WebfontsService::new(self)
    }
    /// Get partners service handler
    pub fn partners(&self) -> partners::PartnersService<'_> {
        partners::PartnersService::new(self)
    }
    /// Get cloudprofiler service handler
    pub fn cloudprofiler(&self) -> cloudprofiler::CloudprofilerService<'_> {
        cloudprofiler::CloudprofilerService::new(self)
    }
    /// Get eventarc service handler
    pub fn eventarc(&self) -> eventarc::EventarcService<'_> {
        eventarc::EventarcService::new(self)
    }
    /// Get eventarc service handler
    pub fn eventarc(&self) -> eventarc::EventarcService<'_> {
        eventarc::EventarcService::new(self)
    }
    /// Get contentwarehouse service handler
    pub fn contentwarehouse(&self) -> contentwarehouse::ContentwarehouseService<'_> {
        contentwarehouse::ContentwarehouseService::new(self)
    }
    /// Get siteverification service handler
    pub fn siteverification(&self) -> siteverification::SiteverificationService<'_> {
        siteverification::SiteverificationService::new(self)
    }
    /// Get analyticsadmin service handler
    pub fn analyticsadmin(&self) -> analyticsadmin::AnalyticsadminService<'_> {
        analyticsadmin::AnalyticsadminService::new(self)
    }
    /// Get analyticsadmin service handler
    pub fn analyticsadmin(&self) -> analyticsadmin::AnalyticsadminService<'_> {
        analyticsadmin::AnalyticsadminService::new(self)
    }
    /// Get verifiedaccess service handler
    pub fn verifiedaccess(&self) -> verifiedaccess::VerifiedaccessService<'_> {
        verifiedaccess::VerifiedaccessService::new(self)
    }
    /// Get verifiedaccess service handler
    pub fn verifiedaccess(&self) -> verifiedaccess::VerifiedaccessService<'_> {
        verifiedaccess::VerifiedaccessService::new(self)
    }
    /// Get safebrowsing service handler
    pub fn safebrowsing(&self) -> safebrowsing::SafebrowsingService<'_> {
        safebrowsing::SafebrowsingService::new(self)
    }
    /// Get safebrowsing service handler
    pub fn safebrowsing(&self) -> safebrowsing::SafebrowsingService<'_> {
        safebrowsing::SafebrowsingService::new(self)
    }
    /// Get people service handler
    pub fn people(&self) -> people::PeopleService<'_> {
        people::PeopleService::new(self)
    }
    /// Get fcm service handler
    pub fn fcm(&self) -> fcm::FcmService<'_> {
        fcm::FcmService::new(self)
    }
    /// Get firebasestorage service handler
    pub fn firebasestorage(&self) -> firebasestorage::FirebasestorageService<'_> {
        firebasestorage::FirebasestorageService::new(self)
    }
    /// Get manufacturers service handler
    pub fn manufacturers(&self) -> manufacturers::ManufacturersService<'_> {
        manufacturers::ManufacturersService::new(self)
    }
    /// Get appstate service handler
    pub fn appstate(&self) -> appstate::AppstateService<'_> {
        appstate::AppstateService::new(self)
    }
    /// Get servicecontrol service handler
    pub fn servicecontrol(&self) -> servicecontrol::ServicecontrolService<'_> {
        servicecontrol::ServicecontrolService::new(self)
    }
    /// Get servicecontrol service handler
    pub fn servicecontrol(&self) -> servicecontrol::ServicecontrolService<'_> {
        servicecontrol::ServicecontrolService::new(self)
    }
    /// Get datalabeling service handler
    pub fn datalabeling(&self) -> datalabeling::DatalabelingService<'_> {
        datalabeling::DatalabelingService::new(self)
    }
    /// Get dlp service handler
    pub fn dlp(&self) -> dlp::DlpService<'_> {
        dlp::DlpService::new(self)
    }
    /// Get mybusinessnotifications service handler
    pub fn mybusinessnotifications(&self) -> mybusinessnotifications::MybusinessnotificationsService<'_> {
        mybusinessnotifications::MybusinessnotificationsService::new(self)
    }
    /// Get containeranalysis service handler
    pub fn containeranalysis(&self) -> containeranalysis::ContaineranalysisService<'_> {
        containeranalysis::ContaineranalysisService::new(self)
    }
    /// Get containeranalysis service handler
    pub fn containeranalysis(&self) -> containeranalysis::ContaineranalysisService<'_> {
        containeranalysis::ContaineranalysisService::new(self)
    }
    /// Get containeranalysis service handler
    pub fn containeranalysis(&self) -> containeranalysis::ContaineranalysisService<'_> {
        containeranalysis::ContaineranalysisService::new(self)
    }
    /// Get mybusinessqanda service handler
    pub fn mybusinessqanda(&self) -> mybusinessqanda::MybusinessqandaService<'_> {
        mybusinessqanda::MybusinessqandaService::new(self)
    }
    /// Get cloudtasks service handler
    pub fn cloudtasks(&self) -> cloudtasks::CloudtasksService<'_> {
        cloudtasks::CloudtasksService::new(self)
    }
    /// Get cloudtasks service handler
    pub fn cloudtasks(&self) -> cloudtasks::CloudtasksService<'_> {
        cloudtasks::CloudtasksService::new(self)
    }
    /// Get cloudtasks service handler
    pub fn cloudtasks(&self) -> cloudtasks::CloudtasksService<'_> {
        cloudtasks::CloudtasksService::new(self)
    }
    /// Get apim service handler
    pub fn apim(&self) -> apim::ApimService<'_> {
        apim::ApimService::new(self)
    }
    /// Get surveys service handler
    pub fn surveys(&self) -> surveys::SurveysService<'_> {
        surveys::SurveysService::new(self)
    }
    /// Get pubsub service handler
    pub fn pubsub(&self) -> pubsub::PubsubService<'_> {
        pubsub::PubsubService::new(self)
    }
    /// Get pubsub service handler
    pub fn pubsub(&self) -> pubsub::PubsubService<'_> {
        pubsub::PubsubService::new(self)
    }
    /// Get pubsub service handler
    pub fn pubsub(&self) -> pubsub::PubsubService<'_> {
        pubsub::PubsubService::new(self)
    }
    /// Get texttospeech service handler
    pub fn texttospeech(&self) -> texttospeech::TexttospeechService<'_> {
        texttospeech::TexttospeechService::new(self)
    }
    /// Get texttospeech service handler
    pub fn texttospeech(&self) -> texttospeech::TexttospeechService<'_> {
        texttospeech::TexttospeechService::new(self)
    }
    /// Get marketingplatformadmin service handler
    pub fn marketingplatformadmin(&self) -> marketingplatformadmin::MarketingplatformadminService<'_> {
        marketingplatformadmin::MarketingplatformadminService::new(self)
    }
    /// Get cloudprivatecatalogproducer service handler
    pub fn cloudprivatecatalogproducer(&self) -> cloudprivatecatalogproducer::CloudprivatecatalogproducerService<'_> {
        cloudprivatecatalogproducer::CloudprivatecatalogproducerService::new(self)
    }
    /// Get sasportal service handler
    pub fn sasportal(&self) -> sasportal::SasportalService<'_> {
        sasportal::SasportalService::new(self)
    }
    /// Get spanner service handler
    pub fn spanner(&self) -> spanner::SpannerService<'_> {
        spanner::SpannerService::new(self)
    }
    /// Get clouddeploy service handler
    pub fn clouddeploy(&self) -> clouddeploy::ClouddeployService<'_> {
        clouddeploy::ClouddeployService::new(self)
    }
    /// Get biglake service handler
    pub fn biglake(&self) -> biglake::BiglakeService<'_> {
        biglake::BiglakeService::new(self)
    }
    /// Get serviceusage service handler
    pub fn serviceusage(&self) -> serviceusage::ServiceusageService<'_> {
        serviceusage::ServiceusageService::new(self)
    }
    /// Get serviceusage service handler
    pub fn serviceusage(&self) -> serviceusage::ServiceusageService<'_> {
        serviceusage::ServiceusageService::new(self)
    }
    /// Get chromemanagement service handler
    pub fn chromemanagement(&self) -> chromemanagement::ChromemanagementService<'_> {
        chromemanagement::ChromemanagementService::new(self)
    }
    /// Get fitness service handler
    pub fn fitness(&self) -> fitness::FitnessService<'_> {
        fitness::FitnessService::new(self)
    }
    /// Get plusdomains service handler
    pub fn plusdomains(&self) -> plusdomains::PlusdomainsService<'_> {
        plusdomains::PlusdomainsService::new(self)
    }
    /// Get androiddeviceprovisioning service handler
    pub fn androiddeviceprovisioning(&self) -> androiddeviceprovisioning::AndroiddeviceprovisioningService<'_> {
        androiddeviceprovisioning::AndroiddeviceprovisioningService::new(self)
    }
    /// Get securesourcemanager service handler
    pub fn securesourcemanager(&self) -> securesourcemanager::SecuresourcemanagerService<'_> {
        securesourcemanager::SecuresourcemanagerService::new(self)
    }
    /// Get kmsinventory service handler
    pub fn kmsinventory(&self) -> kmsinventory::KmsinventoryService<'_> {
        kmsinventory::KmsinventoryService::new(self)
    }
    /// Get cloudcontrolspartner service handler
    pub fn cloudcontrolspartner(&self) -> cloudcontrolspartner::CloudcontrolspartnerService<'_> {
        cloudcontrolspartner::CloudcontrolspartnerService::new(self)
    }
    /// Get cloudcontrolspartner service handler
    pub fn cloudcontrolspartner(&self) -> cloudcontrolspartner::CloudcontrolspartnerService<'_> {
        cloudcontrolspartner::CloudcontrolspartnerService::new(self)
    }
    /// Get bigquerydatapolicy service handler
    pub fn bigquerydatapolicy(&self) -> bigquerydatapolicy::BigquerydatapolicyService<'_> {
        bigquerydatapolicy::BigquerydatapolicyService::new(self)
    }
    /// Get bigquerydatapolicy service handler
    pub fn bigquerydatapolicy(&self) -> bigquerydatapolicy::BigquerydatapolicyService<'_> {
        bigquerydatapolicy::BigquerydatapolicyService::new(self)
    }
    /// Get licensing service handler
    pub fn licensing(&self) -> licensing::LicensingService<'_> {
        licensing::LicensingService::new(self)
    }
    /// Get workspaceevents service handler
    pub fn workspaceevents(&self) -> workspaceevents::WorkspaceeventsService<'_> {
        workspaceevents::WorkspaceeventsService::new(self)
    }
    /// Get replicapoolupdater service handler
    pub fn replicapoolupdater(&self) -> replicapoolupdater::ReplicapoolupdaterService<'_> {
        replicapoolupdater::ReplicapoolupdaterService::new(self)
    }
    /// Get indexing service handler
    pub fn indexing(&self) -> indexing::IndexingService<'_> {
        indexing::IndexingService::new(self)
    }
    /// Get alloydb service handler
    pub fn alloydb(&self) -> alloydb::AlloydbService<'_> {
        alloydb::AlloydbService::new(self)
    }
    /// Get alloydb service handler
    pub fn alloydb(&self) -> alloydb::AlloydbService<'_> {
        alloydb::AlloydbService::new(self)
    }
    /// Get alloydb service handler
    pub fn alloydb(&self) -> alloydb::AlloydbService<'_> {
        alloydb::AlloydbService::new(self)
    }
    /// Get spectrum service handler
    pub fn spectrum(&self) -> spectrum::SpectrumService<'_> {
        spectrum::SpectrumService::new(self)
    }
    /// Get firebaseml service handler
    pub fn firebaseml(&self) -> firebaseml::FirebasemlService<'_> {
        firebaseml::FirebasemlService::new(self)
    }
    /// Get firebaseml service handler
    pub fn firebaseml(&self) -> firebaseml::FirebasemlService<'_> {
        firebaseml::FirebasemlService::new(self)
    }
    /// Get firebaseml service handler
    pub fn firebaseml(&self) -> firebaseml::FirebasemlService<'_> {
        firebaseml::FirebasemlService::new(self)
    }
    /// Get tpu service handler
    pub fn tpu(&self) -> tpu::TpuService<'_> {
        tpu::TpuService::new(self)
    }
    /// Get tpu service handler
    pub fn tpu(&self) -> tpu::TpuService<'_> {
        tpu::TpuService::new(self)
    }
    /// Get tpu service handler
    pub fn tpu(&self) -> tpu::TpuService<'_> {
        tpu::TpuService::new(self)
    }
    /// Get tpu service handler
    pub fn tpu(&self) -> tpu::TpuService<'_> {
        tpu::TpuService::new(self)
    }
    /// Get domainsrdap service handler
    pub fn domainsrdap(&self) -> domainsrdap::DomainsrdapService<'_> {
        domainsrdap::DomainsrdapService::new(self)
    }
    /// Get playablelocations service handler
    pub fn playablelocations(&self) -> playablelocations::PlayablelocationsService<'_> {
        playablelocations::PlayablelocationsService::new(self)
    }
    /// Get groupsmigration service handler
    pub fn groupsmigration(&self) -> groupsmigration::GroupsmigrationService<'_> {
        groupsmigration::GroupsmigrationService::new(self)
    }
    /// Get androidmanagement service handler
    pub fn androidmanagement(&self) -> androidmanagement::AndroidmanagementService<'_> {
        androidmanagement::AndroidmanagementService::new(self)
    }
    /// Get run service handler
    pub fn run(&self) -> run::RunService<'_> {
        run::RunService::new(self)
    }
    /// Get run service handler
    pub fn run(&self) -> run::RunService<'_> {
        run::RunService::new(self)
    }
    /// Get run service handler
    pub fn run(&self) -> run::RunService<'_> {
        run::RunService::new(self)
    }
    /// Get run service handler
    pub fn run(&self) -> run::RunService<'_> {
        run::RunService::new(self)
    }
    /// Get cloudcommerceprocurement service handler
    pub fn cloudcommerceprocurement(&self) -> cloudcommerceprocurement::CloudcommerceprocurementService<'_> {
        cloudcommerceprocurement::CloudcommerceprocurementService::new(self)
    }
    /// Get jobs service handler
    pub fn jobs(&self) -> jobs::JobsService<'_> {
        jobs::JobsService::new(self)
    }
    /// Get jobs service handler
    pub fn jobs(&self) -> jobs::JobsService<'_> {
        jobs::JobsService::new(self)
    }
    /// Get jobs service handler
    pub fn jobs(&self) -> jobs::JobsService<'_> {
        jobs::JobsService::new(self)
    }
    /// Get jobs service handler
    pub fn jobs(&self) -> jobs::JobsService<'_> {
        jobs::JobsService::new(self)
    }
    /// Get networksecurity service handler
    pub fn networksecurity(&self) -> networksecurity::NetworksecurityService<'_> {
        networksecurity::NetworksecurityService::new(self)
    }
    /// Get networksecurity service handler
    pub fn networksecurity(&self) -> networksecurity::NetworksecurityService<'_> {
        networksecurity::NetworksecurityService::new(self)
    }
    /// Get clouddebugger service handler
    pub fn clouddebugger(&self) -> clouddebugger::ClouddebuggerService<'_> {
        clouddebugger::ClouddebuggerService::new(self)
    }
    /// Get monitoring service handler
    pub fn monitoring(&self) -> monitoring::MonitoringService<'_> {
        monitoring::MonitoringService::new(self)
    }
    /// Get monitoring service handler
    pub fn monitoring(&self) -> monitoring::MonitoringService<'_> {
        monitoring::MonitoringService::new(self)
    }
    /// Get youtubeanalytics service handler
    pub fn youtubeanalytics(&self) -> youtubeanalytics::YoutubeanalyticsService<'_> {
        youtubeanalytics::YoutubeanalyticsService::new(self)
    }
    /// Get youtubeanalytics service handler
    pub fn youtubeanalytics(&self) -> youtubeanalytics::YoutubeanalyticsService<'_> {
        youtubeanalytics::YoutubeanalyticsService::new(self)
    }
    /// Get youtubeanalytics service handler
    pub fn youtubeanalytics(&self) -> youtubeanalytics::YoutubeanalyticsService<'_> {
        youtubeanalytics::YoutubeanalyticsService::new(self)
    }
    /// Get policytroubleshooter service handler
    pub fn policytroubleshooter(&self) -> policytroubleshooter::PolicytroubleshooterService<'_> {
        policytroubleshooter::PolicytroubleshooterService::new(self)
    }
    /// Get policytroubleshooter service handler
    pub fn policytroubleshooter(&self) -> policytroubleshooter::PolicytroubleshooterService<'_> {
        policytroubleshooter::PolicytroubleshooterService::new(self)
    }
    /// Get assuredworkloads service handler
    pub fn assuredworkloads(&self) -> assuredworkloads::AssuredworkloadsService<'_> {
        assuredworkloads::AssuredworkloadsService::new(self)
    }
    /// Get assuredworkloads service handler
    pub fn assuredworkloads(&self) -> assuredworkloads::AssuredworkloadsService<'_> {
        assuredworkloads::AssuredworkloadsService::new(self)
    }
    /// Get versionhistory service handler
    pub fn versionhistory(&self) -> versionhistory::VersionhistoryService<'_> {
        versionhistory::VersionhistoryService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get merchantapi service handler
    pub fn merchantapi(&self) -> merchantapi::MerchantapiService<'_> {
        merchantapi::MerchantapiService::new(self)
    }
    /// Get gamesconfiguration service handler
    pub fn gamesconfiguration(&self) -> gamesconfiguration::GamesconfigurationService<'_> {
        gamesconfiguration::GamesconfigurationService::new(self)
    }
    /// Get cloudscheduler service handler
    pub fn cloudscheduler(&self) -> cloudscheduler::CloudschedulerService<'_> {
        cloudscheduler::CloudschedulerService::new(self)
    }
    /// Get cloudscheduler service handler
    pub fn cloudscheduler(&self) -> cloudscheduler::CloudschedulerService<'_> {
        cloudscheduler::CloudschedulerService::new(self)
    }
    /// Get vmwareengine service handler
    pub fn vmwareengine(&self) -> vmwareengine::VmwareengineService<'_> {
        vmwareengine::VmwareengineService::new(self)
    }
    /// Get drive service handler
    pub fn drive(&self) -> drive::DriveService<'_> {
        drive::DriveService::new(self)
    }
    /// Get drive service handler
    pub fn drive(&self) -> drive::DriveService<'_> {
        drive::DriveService::new(self)
    }
    /// Get dns service handler
    pub fn dns(&self) -> dns::DnsService<'_> {
        dns::DnsService::new(self)
    }
    /// Get dns service handler
    pub fn dns(&self) -> dns::DnsService<'_> {
        dns::DnsService::new(self)
    }
    /// Get dns service handler
    pub fn dns(&self) -> dns::DnsService<'_> {
        dns::DnsService::new(self)
    }
    /// Get dns service handler
    pub fn dns(&self) -> dns::DnsService<'_> {
        dns::DnsService::new(self)
    }
    /// Get analyticsreporting service handler
    pub fn analyticsreporting(&self) -> analyticsreporting::AnalyticsreportingService<'_> {
        analyticsreporting::AnalyticsreportingService::new(self)
    }
    /// Get blogger service handler
    pub fn blogger(&self) -> blogger::BloggerService<'_> {
        blogger::BloggerService::new(self)
    }
    /// Get blogger service handler
    pub fn blogger(&self) -> blogger::BloggerService<'_> {
        blogger::BloggerService::new(self)
    }
    /// Get replicapool service handler
    pub fn replicapool(&self) -> replicapool::ReplicapoolService<'_> {
        replicapool::ReplicapoolService::new(self)
    }
    /// Get mybusinessplaceactions service handler
    pub fn mybusinessplaceactions(&self) -> mybusinessplaceactions::MybusinessplaceactionsService<'_> {
        mybusinessplaceactions::MybusinessplaceactionsService::new(self)
    }
    /// Get consumersurveys service handler
    pub fn consumersurveys(&self) -> consumersurveys::ConsumersurveysService<'_> {
        consumersurveys::ConsumersurveysService::new(self)
    }
    /// Get games service handler
    pub fn games(&self) -> games::GamesService<'_> {
        games::GamesService::new(self)
    }
    /// Get gmailpostmastertools service handler
    pub fn gmailpostmastertools(&self) -> gmailpostmastertools::GmailpostmastertoolsService<'_> {
        gmailpostmastertools::GmailpostmastertoolsService::new(self)
    }
    /// Get gmailpostmastertools service handler
    pub fn gmailpostmastertools(&self) -> gmailpostmastertools::GmailpostmastertoolsService<'_> {
        gmailpostmastertools::GmailpostmastertoolsService::new(self)
    }
    /// Get oracledatabase service handler
    pub fn oracledatabase(&self) -> oracledatabase::OracledatabaseService<'_> {
        oracledatabase::OracledatabaseService::new(self)
    }
    /// Get dataform service handler
    pub fn dataform(&self) -> dataform::DataformService<'_> {
        dataform::DataformService::new(self)
    }
    /// Get dataform service handler
    pub fn dataform(&self) -> dataform::DataformService<'_> {
        dataform::DataformService::new(self)
    }
    /// Get authorizedbuyersmarketplace service handler
    pub fn authorizedbuyersmarketplace(&self) -> authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService<'_> {
        authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService::new(self)
    }
    /// Get authorizedbuyersmarketplace service handler
    pub fn authorizedbuyersmarketplace(&self) -> authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService<'_> {
        authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService::new(self)
    }
    /// Get authorizedbuyersmarketplace service handler
    pub fn authorizedbuyersmarketplace(&self) -> authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService<'_> {
        authorizedbuyersmarketplace::AuthorizedbuyersmarketplaceService::new(self)
    }
    /// Get serviceconsumermanagement service handler
    pub fn serviceconsumermanagement(&self) -> serviceconsumermanagement::ServiceconsumermanagementService<'_> {
        serviceconsumermanagement::ServiceconsumermanagementService::new(self)
    }
    /// Get serviceconsumermanagement service handler
    pub fn serviceconsumermanagement(&self) -> serviceconsumermanagement::ServiceconsumermanagementService<'_> {
        serviceconsumermanagement::ServiceconsumermanagementService::new(self)
    }
    /// Get notebooks service handler
    pub fn notebooks(&self) -> notebooks::NotebooksService<'_> {
        notebooks::NotebooksService::new(self)
    }
    /// Get notebooks service handler
    pub fn notebooks(&self) -> notebooks::NotebooksService<'_> {
        notebooks::NotebooksService::new(self)
    }
    /// Get parametermanager service handler
    pub fn parametermanager(&self) -> parametermanager::ParametermanagerService<'_> {
        parametermanager::ParametermanagerService::new(self)
    }
    /// Get logging service handler
    pub fn logging(&self) -> logging::LoggingService<'_> {
        logging::LoggingService::new(self)
    }
    /// Get logging service handler
    pub fn logging(&self) -> logging::LoggingService<'_> {
        logging::LoggingService::new(self)
    }
    /// Get adexperiencereport service handler
    pub fn adexperiencereport(&self) -> adexperiencereport::AdexperiencereportService<'_> {
        adexperiencereport::AdexperiencereportService::new(self)
    }
    /// Get bigquerydatatransfer service handler
    pub fn bigquerydatatransfer(&self) -> bigquerydatatransfer::BigquerydatatransferService<'_> {
        bigquerydatatransfer::BigquerydatatransferService::new(self)
    }
    /// Get datacatalog service handler
    pub fn datacatalog(&self) -> datacatalog::DatacatalogService<'_> {
        datacatalog::DatacatalogService::new(self)
    }
    /// Get datacatalog service handler
    pub fn datacatalog(&self) -> datacatalog::DatacatalogService<'_> {
        datacatalog::DatacatalogService::new(self)
    }
    /// Get cloudshell service handler
    pub fn cloudshell(&self) -> cloudshell::CloudshellService<'_> {
        cloudshell::CloudshellService::new(self)
    }
    /// Get cloudshell service handler
    pub fn cloudshell(&self) -> cloudshell::CloudshellService<'_> {
        cloudshell::CloudshellService::new(self)
    }
    /// Get cloudchannel service handler
    pub fn cloudchannel(&self) -> cloudchannel::CloudchannelService<'_> {
        cloudchannel::CloudchannelService::new(self)
    }
    /// Get osconfig service handler
    pub fn osconfig(&self) -> osconfig::OsconfigService<'_> {
        osconfig::OsconfigService::new(self)
    }
    /// Get osconfig service handler
    pub fn osconfig(&self) -> osconfig::OsconfigService<'_> {
        osconfig::OsconfigService::new(self)
    }
    /// Get osconfig service handler
    pub fn osconfig(&self) -> osconfig::OsconfigService<'_> {
        osconfig::OsconfigService::new(self)
    }
    /// Get osconfig service handler
    pub fn osconfig(&self) -> osconfig::OsconfigService<'_> {
        osconfig::OsconfigService::new(self)
    }
    /// Get osconfig service handler
    pub fn osconfig(&self) -> osconfig::OsconfigService<'_> {
        osconfig::OsconfigService::new(self)
    }
    /// Get migrationcenter service handler
    pub fn migrationcenter(&self) -> migrationcenter::MigrationcenterService<'_> {
        migrationcenter::MigrationcenterService::new(self)
    }
    /// Get migrationcenter service handler
    pub fn migrationcenter(&self) -> migrationcenter::MigrationcenterService<'_> {
        migrationcenter::MigrationcenterService::new(self)
    }
    /// Get gmail service handler
    pub fn gmail(&self) -> gmail::GmailService<'_> {
        gmail::GmailService::new(self)
    }
    /// Get workflowexecutions service handler
    pub fn workflowexecutions(&self) -> workflowexecutions::WorkflowexecutionsService<'_> {
        workflowexecutions::WorkflowexecutionsService::new(self)
    }
    /// Get workflowexecutions service handler
    pub fn workflowexecutions(&self) -> workflowexecutions::WorkflowexecutionsService<'_> {
        workflowexecutions::WorkflowexecutionsService::new(self)
    }
    /// Get apigateway service handler
    pub fn apigateway(&self) -> apigateway::ApigatewayService<'_> {
        apigateway::ApigatewayService::new(self)
    }
    /// Get apigateway service handler
    pub fn apigateway(&self) -> apigateway::ApigatewayService<'_> {
        apigateway::ApigatewayService::new(self)
    }
    /// Get apigateway service handler
    pub fn apigateway(&self) -> apigateway::ApigatewayService<'_> {
        apigateway::ApigatewayService::new(self)
    }
    /// Get apigateway service handler
    pub fn apigateway(&self) -> apigateway::ApigatewayService<'_> {
        apigateway::ApigatewayService::new(self)
    }
    /// Get businessprofileperformance service handler
    pub fn businessprofileperformance(&self) -> businessprofileperformance::BusinessprofileperformanceService<'_> {
        businessprofileperformance::BusinessprofileperformanceService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get metastore service handler
    pub fn metastore(&self) -> metastore::MetastoreService<'_> {
        metastore::MetastoreService::new(self)
    }
    /// Get pubsublite service handler
    pub fn pubsublite(&self) -> pubsublite::PubsubliteService<'_> {
        pubsublite::PubsubliteService::new(self)
    }
    /// Get vault service handler
    pub fn vault(&self) -> vault::VaultService<'_> {
        vault::VaultService::new(self)
    }
    /// Get cloudsupport service handler
    pub fn cloudsupport(&self) -> cloudsupport::CloudsupportService<'_> {
        cloudsupport::CloudsupportService::new(self)
    }
    /// Get cloudsupport service handler
    pub fn cloudsupport(&self) -> cloudsupport::CloudsupportService<'_> {
        cloudsupport::CloudsupportService::new(self)
    }
    /// Get analytics service handler
    pub fn analytics(&self) -> analytics::AnalyticsService<'_> {
        analytics::AnalyticsService::new(self)
    }
    /// Get analytics service handler
    pub fn analytics(&self) -> analytics::AnalyticsService<'_> {
        analytics::AnalyticsService::new(self)
    }
    /// Get sts service handler
    pub fn sts(&self) -> sts::StsService<'_> {
        sts::StsService::new(self)
    }
    /// Get sts service handler
    pub fn sts(&self) -> sts::StsService<'_> {
        sts::StsService::new(self)
    }
    /// Get certificatemanager service handler
    pub fn certificatemanager(&self) -> certificatemanager::CertificatemanagerService<'_> {
        certificatemanager::CertificatemanagerService::new(self)
    }
    /// Get poly service handler
    pub fn poly(&self) -> poly::PolyService<'_> {
        poly::PolyService::new(self)
    }
    /// Get firebaserules service handler
    pub fn firebaserules(&self) -> firebaserules::FirebaserulesService<'_> {
        firebaserules::FirebaserulesService::new(self)
    }
    /// Get plus service handler
    pub fn plus(&self) -> plus::PlusService<'_> {
        plus::PlusService::new(self)
    }
    /// Get dataproc service handler
    pub fn dataproc(&self) -> dataproc::DataprocService<'_> {
        dataproc::DataprocService::new(self)
    }
    /// Get dataproc service handler
    pub fn dataproc(&self) -> dataproc::DataprocService<'_> {
        dataproc::DataprocService::new(self)
    }
    /// Get mirror service handler
    pub fn mirror(&self) -> mirror::MirrorService<'_> {
        mirror::MirrorService::new(self)
    }
    /// Get baremetalsolution service handler
    pub fn baremetalsolution(&self) -> baremetalsolution::BaremetalsolutionService<'_> {
        baremetalsolution::BaremetalsolutionService::new(self)
    }
    /// Get baremetalsolution service handler
    pub fn baremetalsolution(&self) -> baremetalsolution::BaremetalsolutionService<'_> {
        baremetalsolution::BaremetalsolutionService::new(self)
    }
    /// Get baremetalsolution service handler
    pub fn baremetalsolution(&self) -> baremetalsolution::BaremetalsolutionService<'_> {
        baremetalsolution::BaremetalsolutionService::new(self)
    }
    /// Get managedidentities service handler
    pub fn managedidentities(&self) -> managedidentities::ManagedidentitiesService<'_> {
        managedidentities::ManagedidentitiesService::new(self)
    }
    /// Get managedidentities service handler
    pub fn managedidentities(&self) -> managedidentities::ManagedidentitiesService<'_> {
        managedidentities::ManagedidentitiesService::new(self)
    }
    /// Get managedidentities service handler
    pub fn managedidentities(&self) -> managedidentities::ManagedidentitiesService<'_> {
        managedidentities::ManagedidentitiesService::new(self)
    }
    /// Get sql service handler
    pub fn sql(&self) -> sql::SqlService<'_> {
        sql::SqlService::new(self)
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
