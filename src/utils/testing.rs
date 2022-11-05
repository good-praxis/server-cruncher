use hcloud::models::{
    image::{self, OsFlavor, Type},
    server,
    server_type::{CpuType, StorageType},
    Datacenter, DatacenterServerTypes, Image, Location, Protection, Server, ServerProtection,
    ServerPublicNet, ServerType,
};
use std::collections::HashMap;

pub fn empty_snapshot() -> Image {
    Image {
        bound_to: None,
        created: String::new(),
        created_from: None,
        deleted: None,
        deprecated: None,
        description: String::new(),
        disk_size: 0.0,
        id: 0,
        image_size: None,
        labels: HashMap::new(),
        name: None,
        os_flavor: OsFlavor::Unknown,
        os_version: None,
        protection: Box::new(Protection { delete: false }),
        rapid_deploy: None,
        status: image::Status::Available,
        r#type: Type::Snapshot,
    }
}

pub fn empty_server() -> Server {
    Server {
        backup_window: None,
        created: String::new(),
        datacenter: Box::new(Datacenter {
            description: String::new(),
            id: 0,
            location: Box::new(Location {
                city: String::new(),
                country: String::new(),
                description: String::new(),
                id: 0,
                latitude: 0.0,
                longitude: 0.0,
                name: String::new(),
                network_zone: String::new(),
            }),
            name: String::new(),
            server_types: Box::new(DatacenterServerTypes {
                available: Vec::new(),
                available_for_migration: Vec::new(),
                supported: Vec::new(),
            }),
        }),
        id: 0,
        image: None,
        included_traffic: None,
        ingoing_traffic: None,
        iso: None,
        labels: HashMap::new(),
        load_balancers: None,
        locked: false,
        name: String::new(),
        outgoing_traffic: None,
        placement_group: None,
        primary_disk_size: 0,
        private_net: Vec::new(),
        protection: Box::new(ServerProtection {
            delete: false,
            rebuild: false,
        }),
        public_net: Box::new(ServerPublicNet {
            firewalls: None,
            floating_ips: Vec::new(),
            ipv4: None,
            ipv6: None,
        }),
        rescue_enabled: false,
        server_type: Box::new(ServerType {
            cores: 0,
            cpu_type: CpuType::Dedicated,
            deprecated: None,
            description: String::new(),
            disk: 0.0,
            id: 0,
            memory: 0.0,
            name: String::new(),
            prices: Vec::new(),
            storage_type: StorageType::Local,
        }),
        status: server::Status::Running,
        volumes: None,
    }
}
