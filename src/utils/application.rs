use hcloud::models::{image::Type, Image, ListImagesResponse, ListServersResponse, Server};
use std::sync::atomic;

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Application {
    id: usize,
    pub name: Option<String>,
    pub status: Option<String>,
    pub images: Option<Vec<Image>>,
    pub servers: Option<Vec<Server>>,
}
impl Application {
    fn new() -> Self {
        Self {
            id: get_id(),
            name: None,
            status: None,
            images: None,
            servers: None,
        }
    }

    fn with_server(mut self, server: &Server) -> Self {
        self.name = Some(server.name.clone());
        self.status = Some(format!("{:?}", server.status));
        let servers = vec![server.clone()];
        self.servers = Some(servers);
        self
    }

    fn with_image(mut self, image: &Image) -> Self {
        let Image { name, status, .. } = image;

        match name {
            None => self.name = Some(format!("unnamed {}", self.id)),
            Some(name) => self.name = Some(name.clone()),
        }
        self.status = Some(format!("{:?}", status));

        let images = vec![image.clone()];
        self.images = Some(images);

        self
    }

    fn is_image_related(&self, image: &Image) -> bool {
        let Image { created_from, .. } = image;

        match (created_from.clone(), self.name.clone()) {
            (Some(source), Some(name)) if source.name == name => true,
            _ => false,
        }
    }

    fn add_image(&mut self, image: &Image) {
        match &self.images {
            None => {
                let vec = vec![image.clone()];
                self.images = Some(vec);
            }
            Some(_) => {
                self.images.as_mut().unwrap().push(image.clone());
            }
        }
    }
}

fn bump_counter() {
    COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
}

fn get_counter() -> usize {
    COUNTER.load(atomic::Ordering::SeqCst)
}

fn get_id() -> usize {
    let id = get_counter();
    bump_counter();
    id
}

pub fn generate_application_list(
    servers: &ListServersResponse,
    images: &ListImagesResponse,
) -> Vec<Application> {
    let mut vec = Vec::new();

    for server in &servers.servers {
        let app = Application::new().with_server(server);
        vec.push(app);
    }

    for image in &images.images {
        // Skip if this is anything but a snapshot
        if image.r#type != Type::Snapshot {
            continue;
        }

        let mut assinged = false;
        for app in &mut vec {
            if app.is_image_related(image) {
                app.add_image(image);
                assinged = true;
                break;
            }
        }
        // Continue if we assinged in inner loop
        if assinged {
            continue;
        }

        let app = Application::new().with_image(image);
        vec.push(app);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::Application;
    use hcloud::models::{
        image::{OsFlavor, Status, Type},
        CreatedFrom, Image, Protection,
    };
    use std::collections::HashMap;

    fn empty_snapshot() -> Image {
        Image {
            bound_to: None,
            created: String::new(),
            created_from: None,
            deleted: None,
            deprecated: None,
            description: "".to_string(),
            disk_size: 0.0,
            id: 0,
            image_size: None,
            labels: HashMap::new(),
            name: None,
            os_flavor: OsFlavor::Unknown,
            os_version: None,
            protection: Box::new(Protection { delete: false }),
            rapid_deploy: None,
            status: Status::Available,
            r#type: Type::Snapshot,
        }
    }

    mod application {
        use super::{empty_snapshot, Application, CreatedFrom};

        impl PartialEq for Application {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
                    && self.name == other.name
                    && self.status == other.status
                    && self.images == other.images
                    && self.servers == other.servers
            }
        }

        #[test]
        fn is_image_related() {
            let unrelated_image = empty_snapshot();
            let application = Application {
                id: 0,
                name: Some("unique".to_string()),
                status: None,
                images: None,
                servers: None,
            };

            let mut related_image = unrelated_image.clone();
            related_image.created_from = Some(Box::new(CreatedFrom {
                id: 0,
                name: "unique".to_string(),
            }));

            assert!(application.is_image_related(&related_image));
            assert!(!application.is_image_related(&unrelated_image));
        }

        #[test]
        fn with_image() {
            let mut image = empty_snapshot();
            image.name = Some("unique".to_string());

            let application = Application::new().with_image(&image);
            let control_application = Application {
                id: application.id,
                name: Some("unique".to_string()),
                status: Some("Available".to_string()),
                images: Some(vec![image]),
                servers: None,
            };

            assert_eq!(application, control_application);
        }

        #[test]
        fn with_image_unnamed() {
            let image = empty_snapshot();

            let application = Application::new().with_image(&image);
            let mut control_application = Application {
                id: application.id,
                name: Some(format!("unnamed {}", application.id)),
                status: Some("Available".to_string()),
                images: Some(vec![image.clone()]),
                servers: None,
            };

            assert_eq!(application, control_application);

            // Ensure that another unnamed application will have a new id
            let another_application = Application::new().with_image(&image);
            control_application.id += 1;
            control_application.name = Some(format!("unnamed {}", application.id + 1));
            assert_eq!(another_application, control_application)
        }
    }
}
