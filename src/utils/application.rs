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

    fn is_image_related(&mut self, image: &Image) -> bool {
        let Image { created_from, .. } = image;

        match (created_from.clone(), self.name.clone()) {
            (None, _) => false,
            (Some(_), None) => false,
            (Some(source), Some(name)) if source.name == name => {
                self.add_image(image);
                true
            }
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
                assinged = true;
                continue;
            }
        }
        // Break if we assinged in inner loop
        if assinged {
            continue;
        }

        let app = Application::new().with_image(image);
        vec.push(app);
    }

    vec
}
