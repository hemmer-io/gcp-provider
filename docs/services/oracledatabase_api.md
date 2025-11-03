# Oracledatabase_api Service



**Resources**: 24

---

## Overview

The oracledatabase_api service provides access to 24 resource types:

- [Odb_network](#odb_network) [CRD]
- [Db_server](#db_server) [R]
- [Cloud_exadata_infrastructure](#cloud_exadata_infrastructure) [CRD]
- [Database](#database) [R]
- [Cloud_vm_cluster](#cloud_vm_cluster) [CRD]
- [Autonomous_database_backup](#autonomous_database_backup) [R]
- [Odb_subnet](#odb_subnet) [CRD]
- [Db_system_initial_storage_size](#db_system_initial_storage_size) [R]
- [Db_node](#db_node) [R]
- [Db_system](#db_system) [CRD]
- [Autonomous_database_character_set](#autonomous_database_character_set) [R]
- [Db_system_shape](#db_system_shape) [R]
- [Operation](#operation) [CRD]
- [Db_version](#db_version) [R]
- [Database_character_set](#database_character_set) [R]
- [Autonomous_database](#autonomous_database) [CRUD]
- [Pluggable_database](#pluggable_database) [R]
- [Autonomous_db_version](#autonomous_db_version) [R]
- [Exascale_db_storage_vault](#exascale_db_storage_vault) [CRD]
- [Minor_version](#minor_version) [R]
- [Gi_version](#gi_version) [R]
- [Location](#location) [R]
- [Entitlement](#entitlement) [R]
- [Exadb_vm_cluster](#exadb_vm_cluster) [CRUD]

---

## Resources


### Odb_network

Creates a new ODB Network in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entitlement_id` | String |  | Output only. The ID of the subscription entitlement associated with the OdbNetwork. |
| `labels` | HashMap<String, String> |  | Optional. Labels or tags associated with the resource. |
| `gcp_oracle_zone` | String |  | Optional. The GCP Oracle zone where OdbNetwork is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `name` | String |  | Identifier. The name of the OdbNetwork resource in the following format: projects/{project}/locations/{region}/odbNetworks/{odb_network} |
| `network` | String |  | Required. The name of the VPC network in the following format: projects/{project}/global/networks/{network} |
| `create_time` | String |  | Output only. The date and time that the OdbNetwork was created. |
| `state` | String |  | Output only. State of the ODB Network. |
| `parent` | String | ✅ | Required. The parent value for the OdbNetwork in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entitlement_id` | String | Output only. The ID of the subscription entitlement associated with the OdbNetwork. |
| `labels` | HashMap<String, String> | Optional. Labels or tags associated with the resource. |
| `gcp_oracle_zone` | String | Optional. The GCP Oracle zone where OdbNetwork is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `name` | String | Identifier. The name of the OdbNetwork resource in the following format: projects/{project}/locations/{region}/odbNetworks/{odb_network} |
| `network` | String | Required. The name of the VPC network in the following format: projects/{project}/global/networks/{network} |
| `create_time` | String | Output only. The date and time that the OdbNetwork was created. |
| `state` | String | Output only. State of the ODB Network. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create odb_network
odb_network = provider.oracledatabase_api.Odb_network {
    parent = "value"  # Required. The parent value for the OdbNetwork in the following format: projects/{project}/locations/{location}.
}

# Access odb_network outputs
odb_network_id = odb_network.id
odb_network_entitlement_id = odb_network.entitlement_id
odb_network_labels = odb_network.labels
odb_network_gcp_oracle_zone = odb_network.gcp_oracle_zone
odb_network_name = odb_network.name
odb_network_network = odb_network.network
odb_network_create_time = odb_network.create_time
odb_network_state = odb_network.state
```

---


### Db_server

Lists the database servers of an Exadata Infrastructure instance.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_servers` | Vec<String> | The list of database servers. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access db_server outputs
db_server_id = db_server.id
db_server_db_servers = db_server.db_servers
db_server_next_page_token = db_server.next_page_token
```

---


### Cloud_exadata_infrastructure

Creates a new Exadata Infrastructure in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Labels or tags associated with the resource. |
| `display_name` | String |  | Optional. User friendly name for this resource. |
| `entitlement_id` | String |  | Output only. Entitlement ID of the private offer against which this infrastructure resource is provisioned. |
| `name` | String |  | Identifier. The name of the Exadata Infrastructure resource with the format: projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure} |
| `create_time` | String |  | Output only. The date and time that the Exadata Infrastructure was created. |
| `gcp_oracle_zone` | String |  | Optional. The GCP Oracle zone where Oracle Exadata Infrastructure is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `properties` | String |  | Optional. Various properties of the infra. |
| `parent` | String | ✅ | Required. The parent value for CloudExadataInfrastructure in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Labels or tags associated with the resource. |
| `display_name` | String | Optional. User friendly name for this resource. |
| `entitlement_id` | String | Output only. Entitlement ID of the private offer against which this infrastructure resource is provisioned. |
| `name` | String | Identifier. The name of the Exadata Infrastructure resource with the format: projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure} |
| `create_time` | String | Output only. The date and time that the Exadata Infrastructure was created. |
| `gcp_oracle_zone` | String | Optional. The GCP Oracle zone where Oracle Exadata Infrastructure is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `properties` | String | Optional. Various properties of the infra. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloud_exadata_infrastructure
cloud_exadata_infrastructure = provider.oracledatabase_api.Cloud_exadata_infrastructure {
    parent = "value"  # Required. The parent value for CloudExadataInfrastructure in the following format: projects/{project}/locations/{location}.
}

# Access cloud_exadata_infrastructure outputs
cloud_exadata_infrastructure_id = cloud_exadata_infrastructure.id
cloud_exadata_infrastructure_labels = cloud_exadata_infrastructure.labels
cloud_exadata_infrastructure_display_name = cloud_exadata_infrastructure.display_name
cloud_exadata_infrastructure_entitlement_id = cloud_exadata_infrastructure.entitlement_id
cloud_exadata_infrastructure_name = cloud_exadata_infrastructure.name
cloud_exadata_infrastructure_create_time = cloud_exadata_infrastructure.create_time
cloud_exadata_infrastructure_gcp_oracle_zone = cloud_exadata_infrastructure.gcp_oracle_zone
cloud_exadata_infrastructure_properties = cloud_exadata_infrastructure.properties
```

---


### Database

Gets details of a single Database.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ops_insights_status` | String | Output only. The Status of Operations Insights for this Database. |
| `db_name` | String | Optional. The database name. The name must begin with an alphabetic character and can contain a maximum of eight alphanumeric characters. Special characters are not permitted. |
| `admin_password` | String | Required. The password for the default ADMIN user. |
| `ncharacter_set` | String | Optional. The national character set for the database. The default is AL16UTF16. |
| `name` | String | Identifier. The name of the Database resource in the following format: projects/{project}/locations/{region}/databases/{database} |
| `properties` | String | Optional. The properties of the Database. |
| `tde_wallet_password` | String | Optional. The TDE wallet password for the database. |
| `db_home_name` | String | Optional. The name of the DbHome resource associated with the Database. |
| `database_id` | String | Optional. The database ID of the Database. |
| `db_unique_name` | String | Optional. The DB_UNIQUE_NAME of the Oracle Database being backed up. |
| `character_set` | String | Optional. The character set for the database. The default is AL32UTF8. |
| `oci_url` | String | Output only. HTTPS link to OCI resources exposed to Customer via UI Interface. |
| `create_time` | String | Output only. The date and time that the Database was created. |
| `gcp_oracle_zone` | String | Output only. The GCP Oracle zone where the Database is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access database outputs
database_id = database.id
database_ops_insights_status = database.ops_insights_status
database_db_name = database.db_name
database_admin_password = database.admin_password
database_ncharacter_set = database.ncharacter_set
database_name = database.name
database_properties = database.properties
database_tde_wallet_password = database.tde_wallet_password
database_db_home_name = database.db_home_name
database_database_id = database.database_id
database_db_unique_name = database.db_unique_name
database_character_set = database.character_set
database_oci_url = database.oci_url
database_create_time = database.create_time
database_gcp_oracle_zone = database.gcp_oracle_zone
```

---


### Cloud_vm_cluster

Creates a new VM Cluster in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Optional. The name of the VPC network. Format: projects/{project}/global/networks/{network} |
| `odb_network` | String |  | Optional. The name of the OdbNetwork associated with the VM Cluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the odb_subnet and backup_odb_subnet. |
| `properties` | String |  | Optional. Various properties of the VM Cluster. |
| `backup_subnet_cidr` | String |  | Optional. CIDR range of the backup subnet. |
| `exadata_infrastructure` | String |  | Required. The name of the Exadata Infrastructure resource on which VM cluster resource is created, in the following format: projects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure} |
| `cidr` | String |  | Optional. Network settings. CIDR to use for cluster IP allocation. |
| `create_time` | String |  | Output only. The date and time that the VM cluster was created. |
| `backup_odb_subnet` | String |  | Optional. The name of the backup OdbSubnet associated with the VM Cluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `labels` | HashMap<String, String> |  | Optional. Labels or tags associated with the VM Cluster. |
| `name` | String |  | Identifier. The name of the VM Cluster resource with the format: projects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster} |
| `odb_subnet` | String |  | Optional. The name of the OdbSubnet associated with the VM Cluster for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `display_name` | String |  | Optional. User friendly name for this resource. |
| `gcp_oracle_zone` | String |  | Output only. The GCP Oracle zone where Oracle CloudVmCluster is hosted. This will be the same as the gcp_oracle_zone of the CloudExadataInfrastructure. Example: us-east4-b-r2. |
| `identity_connector` | String |  | Output only. The identity connector details which will allow OCI to securely access the resources in the customer project. |
| `parent` | String | ✅ | Required. The name of the parent in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | Optional. The name of the VPC network. Format: projects/{project}/global/networks/{network} |
| `odb_network` | String | Optional. The name of the OdbNetwork associated with the VM Cluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the odb_subnet and backup_odb_subnet. |
| `properties` | String | Optional. Various properties of the VM Cluster. |
| `backup_subnet_cidr` | String | Optional. CIDR range of the backup subnet. |
| `exadata_infrastructure` | String | Required. The name of the Exadata Infrastructure resource on which VM cluster resource is created, in the following format: projects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure} |
| `cidr` | String | Optional. Network settings. CIDR to use for cluster IP allocation. |
| `create_time` | String | Output only. The date and time that the VM cluster was created. |
| `backup_odb_subnet` | String | Optional. The name of the backup OdbSubnet associated with the VM Cluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `labels` | HashMap<String, String> | Optional. Labels or tags associated with the VM Cluster. |
| `name` | String | Identifier. The name of the VM Cluster resource with the format: projects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster} |
| `odb_subnet` | String | Optional. The name of the OdbSubnet associated with the VM Cluster for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `display_name` | String | Optional. User friendly name for this resource. |
| `gcp_oracle_zone` | String | Output only. The GCP Oracle zone where Oracle CloudVmCluster is hosted. This will be the same as the gcp_oracle_zone of the CloudExadataInfrastructure. Example: us-east4-b-r2. |
| `identity_connector` | String | Output only. The identity connector details which will allow OCI to securely access the resources in the customer project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloud_vm_cluster
cloud_vm_cluster = provider.oracledatabase_api.Cloud_vm_cluster {
    parent = "value"  # Required. The name of the parent in the following format: projects/{project}/locations/{location}.
}

# Access cloud_vm_cluster outputs
cloud_vm_cluster_id = cloud_vm_cluster.id
cloud_vm_cluster_network = cloud_vm_cluster.network
cloud_vm_cluster_odb_network = cloud_vm_cluster.odb_network
cloud_vm_cluster_properties = cloud_vm_cluster.properties
cloud_vm_cluster_backup_subnet_cidr = cloud_vm_cluster.backup_subnet_cidr
cloud_vm_cluster_exadata_infrastructure = cloud_vm_cluster.exadata_infrastructure
cloud_vm_cluster_cidr = cloud_vm_cluster.cidr
cloud_vm_cluster_create_time = cloud_vm_cluster.create_time
cloud_vm_cluster_backup_odb_subnet = cloud_vm_cluster.backup_odb_subnet
cloud_vm_cluster_labels = cloud_vm_cluster.labels
cloud_vm_cluster_name = cloud_vm_cluster.name
cloud_vm_cluster_odb_subnet = cloud_vm_cluster.odb_subnet
cloud_vm_cluster_display_name = cloud_vm_cluster.display_name
cloud_vm_cluster_gcp_oracle_zone = cloud_vm_cluster.gcp_oracle_zone
cloud_vm_cluster_identity_connector = cloud_vm_cluster.identity_connector
```

---


### Autonomous_database_backup

Lists the long-term and automatic backups of an Autonomous Database.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `autonomous_database_backups` | Vec<String> | The list of Autonomous Database Backups. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autonomous_database_backup outputs
autonomous_database_backup_id = autonomous_database_backup.id
autonomous_database_backup_autonomous_database_backups = autonomous_database_backup.autonomous_database_backups
autonomous_database_backup_next_page_token = autonomous_database_backup.next_page_token
```

---


### Odb_subnet

Creates a new ODB Subnet in a given ODB Network.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `purpose` | String |  | Required. Purpose of the subnet. |
| `labels` | HashMap<String, String> |  | Optional. Labels or tags associated with the resource. |
| `cidr_range` | String |  | Required. The CIDR range of the subnet. |
| `create_time` | String |  | Output only. The date and time that the OdbNetwork was created. |
| `name` | String |  | Identifier. The name of the OdbSubnet resource in the following format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `state` | String |  | Output only. State of the ODB Subnet. |
| `parent` | String | ✅ | Required. The parent value for the OdbSubnet in the following format: projects/{project}/locations/{location}/odbNetworks/{odb_network}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purpose` | String | Required. Purpose of the subnet. |
| `labels` | HashMap<String, String> | Optional. Labels or tags associated with the resource. |
| `cidr_range` | String | Required. The CIDR range of the subnet. |
| `create_time` | String | Output only. The date and time that the OdbNetwork was created. |
| `name` | String | Identifier. The name of the OdbSubnet resource in the following format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `state` | String | Output only. State of the ODB Subnet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create odb_subnet
odb_subnet = provider.oracledatabase_api.Odb_subnet {
    parent = "value"  # Required. The parent value for the OdbSubnet in the following format: projects/{project}/locations/{location}/odbNetworks/{odb_network}.
}

# Access odb_subnet outputs
odb_subnet_id = odb_subnet.id
odb_subnet_purpose = odb_subnet.purpose
odb_subnet_labels = odb_subnet.labels
odb_subnet_cidr_range = odb_subnet.cidr_range
odb_subnet_create_time = odb_subnet.create_time
odb_subnet_name = odb_subnet.name
odb_subnet_state = odb_subnet.state
```

---


### Db_system_initial_storage_size

Lists all the DbSystemInitialStorageSizes for the given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `db_system_initial_storage_sizes` | Vec<String> | The list of DbSystemInitialStorageSizes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access db_system_initial_storage_size outputs
db_system_initial_storage_size_id = db_system_initial_storage_size.id
db_system_initial_storage_size_next_page_token = db_system_initial_storage_size.next_page_token
db_system_initial_storage_size_db_system_initial_storage_sizes = db_system_initial_storage_size.db_system_initial_storage_sizes
```

---


### Db_node

Lists the database nodes of a VM Cluster.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_nodes` | Vec<String> | The list of DB Nodes |
| `next_page_token` | String | A token identifying a page of results the node should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access db_node outputs
db_node_id = db_node.id
db_node_db_nodes = db_node.db_nodes
db_node_next_page_token = db_node.next_page_token
```

---


### Db_system

Creates a new DbSystem in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `properties` | String |  | Optional. The properties of the DbSystem. |
| `oci_url` | String |  | Output only. HTTPS link to OCI resources exposed to Customer via UI Interface. |
| `create_time` | String |  | Output only. The date and time that the DbSystem was created. |
| `gcp_oracle_zone` | String |  | Optional. The GCP Oracle zone where Oracle DbSystem is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `odb_network` | String |  | Optional. The name of the OdbNetwork associated with the DbSystem. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `labels` | HashMap<String, String> |  | Optional. The labels or tags associated with the DbSystem. |
| `entitlement_id` | String |  | Output only. The ID of the subscription entitlement associated with the DbSystem |
| `display_name` | String |  | Required. The display name for the System db. The name does not have to be unique within your project. |
| `name` | String |  | Identifier. The name of the DbSystem resource in the following format: projects/{project}/locations/{region}/dbSystems/{db_system} |
| `odb_subnet` | String |  | Required. The name of the OdbSubnet associated with the DbSystem for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `parent` | String | ✅ | Required. The value for parent of the DbSystem in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `properties` | String | Optional. The properties of the DbSystem. |
| `oci_url` | String | Output only. HTTPS link to OCI resources exposed to Customer via UI Interface. |
| `create_time` | String | Output only. The date and time that the DbSystem was created. |
| `gcp_oracle_zone` | String | Optional. The GCP Oracle zone where Oracle DbSystem is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `odb_network` | String | Optional. The name of the OdbNetwork associated with the DbSystem. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `labels` | HashMap<String, String> | Optional. The labels or tags associated with the DbSystem. |
| `entitlement_id` | String | Output only. The ID of the subscription entitlement associated with the DbSystem |
| `display_name` | String | Required. The display name for the System db. The name does not have to be unique within your project. |
| `name` | String | Identifier. The name of the DbSystem resource in the following format: projects/{project}/locations/{region}/dbSystems/{db_system} |
| `odb_subnet` | String | Required. The name of the OdbSubnet associated with the DbSystem for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create db_system
db_system = provider.oracledatabase_api.Db_system {
    parent = "value"  # Required. The value for parent of the DbSystem in the following format: projects/{project}/locations/{location}.
}

# Access db_system outputs
db_system_id = db_system.id
db_system_properties = db_system.properties
db_system_oci_url = db_system.oci_url
db_system_create_time = db_system.create_time
db_system_gcp_oracle_zone = db_system.gcp_oracle_zone
db_system_odb_network = db_system.odb_network
db_system_labels = db_system.labels
db_system_entitlement_id = db_system.entitlement_id
db_system_display_name = db_system.display_name
db_system_name = db_system.name
db_system_odb_subnet = db_system.odb_subnet
```

---


### Autonomous_database_character_set

Lists Autonomous Database Character Sets in a given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `autonomous_database_character_sets` | Vec<String> | The list of Autonomous Database Character Sets. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autonomous_database_character_set outputs
autonomous_database_character_set_id = autonomous_database_character_set.id
autonomous_database_character_set_autonomous_database_character_sets = autonomous_database_character_set.autonomous_database_character_sets
autonomous_database_character_set_next_page_token = autonomous_database_character_set.next_page_token
```

---


### Db_system_shape

Lists the database system shapes available for the project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_system_shapes` | Vec<String> | The list of Database System shapes. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access db_system_shape outputs
db_system_shape_id = db_system_shape.id
db_system_shape_db_system_shapes = db_system_shape.db_system_shapes
db_system_shape_next_page_token = db_system_shape.next_page_token
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.oracledatabase_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
```

---


### Db_version

List DbVersions for the given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `db_versions` | Vec<String> | The list of DbVersions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access db_version outputs
db_version_id = db_version.id
db_version_next_page_token = db_version.next_page_token
db_version_db_versions = db_version.db_versions
```

---


### Database_character_set

List DatabaseCharacterSets for the given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `database_character_sets` | Vec<String> | The list of DatabaseCharacterSets. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access database_character_set outputs
database_character_set_id = database_character_set.id
database_character_set_next_page_token = database_character_set.next_page_token
database_character_set_database_character_sets = database_character_set.database_character_sets
```

---


### Autonomous_database

Creates a new Autonomous Database in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_config` | String |  | Optional. The source Autonomous Database configuration for the standby Autonomous Database. The source Autonomous Database is configured while creating the Peer Autonomous Database and can't be updated after creation. |
| `properties` | String |  | Optional. The properties of the Autonomous Database. |
| `entitlement_id` | String |  | Output only. The ID of the subscription entitlement associated with the Autonomous Database. |
| `create_time` | String |  | Output only. The date and time that the Autonomous Database was created. |
| `display_name` | String |  | Optional. The display name for the Autonomous Database. The name does not have to be unique within your project. |
| `network` | String |  | Optional. The name of the VPC network used by the Autonomous Database in the following format: projects/{project}/global/networks/{network} |
| `labels` | HashMap<String, String> |  | Optional. The labels or tags associated with the Autonomous Database. |
| `name` | String |  | Identifier. The name of the Autonomous Database resource in the following format: projects/{project}/locations/{region}/autonomousDatabases/{autonomous_database} |
| `odb_subnet` | String |  | Optional. The name of the OdbSubnet associated with the Autonomous Database. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `disaster_recovery_supported_locations` | Vec<String> |  | Output only. List of supported GCP region to clone the Autonomous Database for disaster recovery. Format: `project/{project}/locations/{location}`. |
| `peer_autonomous_databases` | Vec<String> |  | Output only. The peer Autonomous Database names of the given Autonomous Database. |
| `admin_password` | String |  | Optional. The password for the default ADMIN user. |
| `odb_network` | String |  | Optional. The name of the OdbNetwork associated with the Autonomous Database. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `cidr` | String |  | Optional. The subnet CIDR range for the Autonomous Database. |
| `database` | String |  | Optional. The name of the Autonomous Database. The database name must be unique in the project. The name must begin with a letter and can contain a maximum of 30 alphanumeric characters. |
| `parent` | String | ✅ | Required. The name of the parent in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_config` | String | Optional. The source Autonomous Database configuration for the standby Autonomous Database. The source Autonomous Database is configured while creating the Peer Autonomous Database and can't be updated after creation. |
| `properties` | String | Optional. The properties of the Autonomous Database. |
| `entitlement_id` | String | Output only. The ID of the subscription entitlement associated with the Autonomous Database. |
| `create_time` | String | Output only. The date and time that the Autonomous Database was created. |
| `display_name` | String | Optional. The display name for the Autonomous Database. The name does not have to be unique within your project. |
| `network` | String | Optional. The name of the VPC network used by the Autonomous Database in the following format: projects/{project}/global/networks/{network} |
| `labels` | HashMap<String, String> | Optional. The labels or tags associated with the Autonomous Database. |
| `name` | String | Identifier. The name of the Autonomous Database resource in the following format: projects/{project}/locations/{region}/autonomousDatabases/{autonomous_database} |
| `odb_subnet` | String | Optional. The name of the OdbSubnet associated with the Autonomous Database. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `disaster_recovery_supported_locations` | Vec<String> | Output only. List of supported GCP region to clone the Autonomous Database for disaster recovery. Format: `project/{project}/locations/{location}`. |
| `peer_autonomous_databases` | Vec<String> | Output only. The peer Autonomous Database names of the given Autonomous Database. |
| `admin_password` | String | Optional. The password for the default ADMIN user. |
| `odb_network` | String | Optional. The name of the OdbNetwork associated with the Autonomous Database. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `cidr` | String | Optional. The subnet CIDR range for the Autonomous Database. |
| `database` | String | Optional. The name of the Autonomous Database. The database name must be unique in the project. The name must begin with a letter and can contain a maximum of 30 alphanumeric characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create autonomous_database
autonomous_database = provider.oracledatabase_api.Autonomous_database {
    parent = "value"  # Required. The name of the parent in the following format: projects/{project}/locations/{location}.
}

# Access autonomous_database outputs
autonomous_database_id = autonomous_database.id
autonomous_database_source_config = autonomous_database.source_config
autonomous_database_properties = autonomous_database.properties
autonomous_database_entitlement_id = autonomous_database.entitlement_id
autonomous_database_create_time = autonomous_database.create_time
autonomous_database_display_name = autonomous_database.display_name
autonomous_database_network = autonomous_database.network
autonomous_database_labels = autonomous_database.labels
autonomous_database_name = autonomous_database.name
autonomous_database_odb_subnet = autonomous_database.odb_subnet
autonomous_database_disaster_recovery_supported_locations = autonomous_database.disaster_recovery_supported_locations
autonomous_database_peer_autonomous_databases = autonomous_database.peer_autonomous_databases
autonomous_database_admin_password = autonomous_database.admin_password
autonomous_database_odb_network = autonomous_database.odb_network
autonomous_database_cidr = autonomous_database.cidr
autonomous_database_database = autonomous_database.database
```

---


### Pluggable_database

Gets details of a single PluggableDatabase.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `properties` | String | Optional. The properties of the PluggableDatabase. |
| `oci_url` | String | Output only. HTTPS link to OCI resources exposed to Customer via UI Interface. |
| `name` | String | Identifier. The name of the PluggableDatabase resource in the following format: projects/{project}/locations/{region}/pluggableDatabases/{pluggable_database} |
| `create_time` | String | Output only. The date and time that the PluggableDatabase was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pluggable_database outputs
pluggable_database_id = pluggable_database.id
pluggable_database_properties = pluggable_database.properties
pluggable_database_oci_url = pluggable_database.oci_url
pluggable_database_name = pluggable_database.name
pluggable_database_create_time = pluggable_database.create_time
```

---


### Autonomous_db_version

Lists all the available Autonomous Database versions for a project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `autonomous_db_versions` | Vec<String> | The list of Autonomous Database versions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autonomous_db_version outputs
autonomous_db_version_id = autonomous_db_version.id
autonomous_db_version_next_page_token = autonomous_db_version.next_page_token
autonomous_db_version_autonomous_db_versions = autonomous_db_version.autonomous_db_versions
```

---


### Exascale_db_storage_vault

Creates a new ExascaleDB Storage Vault resource.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the ExascaleDbStorageVault. Format: projects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault} |
| `create_time` | String |  | Output only. The date and time when the ExascaleDbStorageVault was created. |
| `entitlement_id` | String |  | Output only. The ID of the subscription entitlement associated with the ExascaleDbStorageVault. |
| `display_name` | String |  | Required. The display name for the ExascaleDbStorageVault. The name does not have to be unique within your project. The name must be 1-255 characters long and can only contain alphanumeric characters. |
| `gcp_oracle_zone` | String |  | Optional. The GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `labels` | HashMap<String, String> |  | Optional. The labels or tags associated with the ExascaleDbStorageVault. |
| `properties` | String |  | Required. The properties of the ExascaleDbStorageVault. |
| `parent` | String | ✅ | Required. The value for parent of the ExascaleDbStorageVault in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the ExascaleDbStorageVault. Format: projects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault} |
| `create_time` | String | Output only. The date and time when the ExascaleDbStorageVault was created. |
| `entitlement_id` | String | Output only. The ID of the subscription entitlement associated with the ExascaleDbStorageVault. |
| `display_name` | String | Required. The display name for the ExascaleDbStorageVault. The name does not have to be unique within your project. The name must be 1-255 characters long and can only contain alphanumeric characters. |
| `gcp_oracle_zone` | String | Optional. The GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted. Example: us-east4-b-r2. If not specified, the system will pick a zone based on availability. |
| `labels` | HashMap<String, String> | Optional. The labels or tags associated with the ExascaleDbStorageVault. |
| `properties` | String | Required. The properties of the ExascaleDbStorageVault. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create exascale_db_storage_vault
exascale_db_storage_vault = provider.oracledatabase_api.Exascale_db_storage_vault {
    parent = "value"  # Required. The value for parent of the ExascaleDbStorageVault in the following format: projects/{project}/locations/{location}.
}

# Access exascale_db_storage_vault outputs
exascale_db_storage_vault_id = exascale_db_storage_vault.id
exascale_db_storage_vault_name = exascale_db_storage_vault.name
exascale_db_storage_vault_create_time = exascale_db_storage_vault.create_time
exascale_db_storage_vault_entitlement_id = exascale_db_storage_vault.entitlement_id
exascale_db_storage_vault_display_name = exascale_db_storage_vault.display_name
exascale_db_storage_vault_gcp_oracle_zone = exascale_db_storage_vault.gcp_oracle_zone
exascale_db_storage_vault_labels = exascale_db_storage_vault.labels
exascale_db_storage_vault_properties = exascale_db_storage_vault.properties
```

---


### Minor_version

Lists all the valid minor versions for the given project, location, gi version and shape family.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `minor_versions` | Vec<String> | The list of MinorVersions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access minor_version outputs
minor_version_id = minor_version.id
minor_version_next_page_token = minor_version.next_page_token
minor_version_minor_versions = minor_version.minor_versions
```

---


### Gi_version

Lists all the valid Oracle Grid Infrastructure (GI) versions for the given project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gi_versions` | Vec<String> | The list of Oracle Grid Infrastructure (GI) versions. |
| `next_page_token` | String | A token identifying a page of results the server should return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access gi_version outputs
gi_version_id = gi_version.id
gi_version_gi_versions = gi_version.gi_versions
gi_version_next_page_token = gi_version.next_page_token
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_location_id = location.location_id
location_display_name = location.display_name
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
```

---


### Entitlement

Lists the entitlements in a given project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token identifying a page of results the server should return. |
| `entitlements` | Vec<String> | The list of Entitlements |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entitlement outputs
entitlement_id = entitlement.id
entitlement_next_page_token = entitlement.next_page_token
entitlement_entitlements = entitlement.entitlements
```

---


### Exadb_vm_cluster

Creates a new Exadb (Exascale) VM Cluster resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Immutable. The display name for the ExadbVmCluster. The name does not have to be unique within your project. The name must be 1-255 characters long and can only contain alphanumeric characters. |
| `odb_network` | String |  | Optional. Immutable. The name of the OdbNetwork associated with the ExadbVmCluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `name` | String |  | Identifier. The name of the ExadbVmCluster resource in the following format: projects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster} |
| `labels` | HashMap<String, String> |  | Optional. The labels or tags associated with the ExadbVmCluster. |
| `backup_odb_subnet` | String |  | Required. Immutable. The name of the backup OdbSubnet associated with the ExadbVmCluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `odb_subnet` | String |  | Required. Immutable. The name of the OdbSubnet associated with the ExadbVmCluster for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `properties` | String |  | Required. The properties of the ExadbVmCluster. |
| `entitlement_id` | String |  | Output only. The ID of the subscription entitlement associated with the ExadbVmCluster. |
| `create_time` | String |  | Output only. The date and time that the ExadbVmCluster was created. |
| `gcp_oracle_zone` | String |  | Output only. Immutable. The GCP Oracle zone where Oracle ExadbVmCluster is hosted. Example: us-east4-b-r2. During creation, the system will pick the zone assigned to the ExascaleDbStorageVault. |
| `parent` | String | ✅ | Required. The value for parent of the ExadbVmCluster in the following format: projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Immutable. The display name for the ExadbVmCluster. The name does not have to be unique within your project. The name must be 1-255 characters long and can only contain alphanumeric characters. |
| `odb_network` | String | Optional. Immutable. The name of the OdbNetwork associated with the ExadbVmCluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network} It is optional but if specified, this should match the parent ODBNetwork of the OdbSubnet. |
| `name` | String | Identifier. The name of the ExadbVmCluster resource in the following format: projects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster} |
| `labels` | HashMap<String, String> | Optional. The labels or tags associated with the ExadbVmCluster. |
| `backup_odb_subnet` | String | Required. Immutable. The name of the backup OdbSubnet associated with the ExadbVmCluster. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `odb_subnet` | String | Required. Immutable. The name of the OdbSubnet associated with the ExadbVmCluster for IP allocation. Format: projects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet} |
| `properties` | String | Required. The properties of the ExadbVmCluster. |
| `entitlement_id` | String | Output only. The ID of the subscription entitlement associated with the ExadbVmCluster. |
| `create_time` | String | Output only. The date and time that the ExadbVmCluster was created. |
| `gcp_oracle_zone` | String | Output only. Immutable. The GCP Oracle zone where Oracle ExadbVmCluster is hosted. Example: us-east4-b-r2. During creation, the system will pick the zone assigned to the ExascaleDbStorageVault. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create exadb_vm_cluster
exadb_vm_cluster = provider.oracledatabase_api.Exadb_vm_cluster {
    parent = "value"  # Required. The value for parent of the ExadbVmCluster in the following format: projects/{project}/locations/{location}.
}

# Access exadb_vm_cluster outputs
exadb_vm_cluster_id = exadb_vm_cluster.id
exadb_vm_cluster_display_name = exadb_vm_cluster.display_name
exadb_vm_cluster_odb_network = exadb_vm_cluster.odb_network
exadb_vm_cluster_name = exadb_vm_cluster.name
exadb_vm_cluster_labels = exadb_vm_cluster.labels
exadb_vm_cluster_backup_odb_subnet = exadb_vm_cluster.backup_odb_subnet
exadb_vm_cluster_odb_subnet = exadb_vm_cluster.odb_subnet
exadb_vm_cluster_properties = exadb_vm_cluster.properties
exadb_vm_cluster_entitlement_id = exadb_vm_cluster.entitlement_id
exadb_vm_cluster_create_time = exadb_vm_cluster.create_time
exadb_vm_cluster_gcp_oracle_zone = exadb_vm_cluster.gcp_oracle_zone
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple odb_network resources
odb_network_0 = provider.oracledatabase_api.Odb_network {
    parent = "value-0"
}
odb_network_1 = provider.oracledatabase_api.Odb_network {
    parent = "value-1"
}
odb_network_2 = provider.oracledatabase_api.Odb_network {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    odb_network = provider.oracledatabase_api.Odb_network {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Oracledatabase_api Documentation](https://cloud.google.com/oracledatabase_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
