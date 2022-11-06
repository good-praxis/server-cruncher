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
        matches!((created_from.clone(), self.name.clone()), (Some(source), Some(name)) if source.name == name)
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
    use crate::utils::{empty_server, empty_snapshot};
    use hcloud::models::{CreatedFrom, Image, ListImagesResponse, ListServersResponse, Server};

    mod application {
        use super::{empty_server, empty_snapshot, Application};
        use hcloud::models::CreatedFrom;

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
            let control_application = Application {
                id: application.id,
                name: Some(format!("unnamed {}", application.id)),
                status: Some("Available".to_string()),
                images: Some(vec![image.clone()]),
                servers: None,
            };

            assert_eq!(application, control_application);
        }

        #[test]
        fn with_image_multiple_unamed() {
            let image = empty_snapshot();
            let application = Application::new().with_image(&image);
            let another_application = Application::new().with_image(&image);
            assert_ne!(application.id, another_application.id);
            assert_ne!(application.name, another_application.name);
        }

        #[test]
        fn with_server() {
            let mut server = empty_server();
            server.name = "unique".to_string();
            let application = Application::new().with_server(&server);
            let mut control_application = Application::new();
            control_application.id = application.id;
            control_application.name = Some("unique".to_string());
            control_application.status = Some("Running".to_string());
            control_application.servers = Some(vec![server.clone()]);
            assert_eq!(application, control_application);
        }

        #[test]
        fn add_image() {
            let image = empty_snapshot();
            let mut application = Application::new();

            assert_eq!(application.images, None);
            application.add_image(&image);
            assert_eq!(application.images, Some(vec![image.clone()]));
            application.add_image(&image);
            assert_eq!(application.images, Some(vec![image.clone(), image.clone()]));
        }
    }

    #[test]
    fn generate_application_list() {
        let base_server = empty_server();
        let server1 = Server {
            name: "Amogus".to_string(),
            ..base_server.clone()
        };
        let server2 = Server {
            name: "In Our Midst".to_string(),
            ..base_server.clone()
        };
        let server3 = Server {
            name: "Amid Us".to_string(),
            ..base_server.clone()
        };
        let base_image = empty_snapshot();
        let image1 = Image {
            created_from: Some(Box::new(CreatedFrom {
                id: 0,
                name: "Amogus".to_string(),
            })),
            ..base_image.clone()
        };
        let image2 = Image {
            name: Some("In Our Group".to_string()),
            ..base_image.clone()
        };
        let invalid_image = Image {
            r#type: hcloud::models::image::Type::App,
            ..base_image.clone()
        };
        let server_list = ListServersResponse {
            meta: None,
            servers: vec![server1, server2, server3],
        };
        let image_list = ListImagesResponse {
            meta: None,
            images: vec![image1, image2, invalid_image],
        };
        let applications = super::generate_application_list(&server_list, &image_list);
        assert_eq!(applications.len(), 4);
        let first = applications.get(0).unwrap();
        let last = applications.get(3).unwrap();
        assert_eq!(first.name.clone().unwrap(), "Amogus".to_string());
        assert_eq!(first.images.clone().unwrap().len(), 1);
        assert_eq!(last.name.clone().unwrap(), "In Our Group".to_string());
    }
}
