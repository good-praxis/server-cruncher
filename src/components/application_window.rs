use crate::utils::Application;
use egui::{Context, Window};

pub fn application_window(application: &Application, ctx: &Context) {
    let Application { name, status, .. } = application;
    let mut counter = 0;

    Window::new(name.clone().unwrap()).show(ctx, |ui| {
        ui.label(status.clone().unwrap());

        if let Some(servers) = &application.servers {
            for server in servers {
                ui.label(format!(
                    "IP: {}",
                    server.public_net.ipv4.as_ref().unwrap().ip.as_str()
                ));
                ui.label(format!(
                    "Datacenter: {}",
                    server.datacenter.description.as_str()
                ));
                ui.label(format!("Status: {:?}", server.status));
                ui.push_id(counter, |ui| {
                    ui.collapsing("details", |ui| ui.label(format!("{:?}", server)));
                });
                counter += 1;
                ui.separator();
            }
        }

        if let Some(images) = &application.images {
            for image in images {
                ui.label(image.description.clone());
                ui.label(format!("type: {:?}", image.r#type));
                ui.label(format!("Bound to: {:?}", image.bound_to));
                ui.label(format!("Status: {:?}", image.status));
                ui.push_id(counter, |ui| {
                    ui.collapsing("details", |ui| ui.label(format!("{:?}", image)));
                });
                counter += 1;
                ui.separator();
            }
        }
    });
}
