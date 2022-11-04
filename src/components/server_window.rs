use egui::{Context, InnerResponse, Window};
use hcloud::models::Server;

type Component = Option<InnerResponse<Option<()>>>;

pub fn server_window(server: &Server, ctx: &Context) -> Component {
    Window::new(server.name.as_str()).show(ctx, |ui| {
        ui.label(format!(
            "IP: {}",
            server.public_net.ipv4.as_ref().unwrap().ip.as_str()
        ));
        ui.label(format!(
            "Datacenter: {}",
            server.datacenter.description.as_str()
        ));
        ui.label(format!("Status: {:?}", server.status));
        ui.collapsing("details", |ui| ui.label(format!("{:?}", server)));
    })
}
