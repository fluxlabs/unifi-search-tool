
use crate::{
    gui::{
        popup::{GuiError, ModalMeta, PopupModal},
        {ChannelsGuiThread, ChannelsSearchThread},
    },
    mac_address::{validation::text_is_valid_mac, MacAddress},
    unifi::search::{find_unifi_device, UnifiSearchInfo},
};
use std::thread;
use zeroize::Zeroize;

#[derive(Debug, Clone, PartialEq)]
enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Default, Debug, Clone)]
struct UserInputs {
    username: String,
    password: String,
    server_url: String,
    mac_address: String,
    allow_invalid_certs: bool,
    remember_password: bool,
}

pub(crate) struct GuiApp<'a> {
    font_size: FontSize,
    inputs: UserInputs,
    gui_channels: ChannelsGuiThread,
    popup: Option<PopupModal<'a>>,
}

impl<'a> GuiApp<'a> {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            font_size: FontSize::Medium,
            inputs: Default::default(),
            gui_channels: ChannelsGuiThread::default(),
            popup: None,
        }
    }

    fn apply_ui_scaling(&self, ctx: &egui::Context) {
        let scale = match self.font_size {
            FontSize::Small => 1.25,
            FontSize::Medium => 1.5,
            FontSize::Large => 1.75,
            FontSize::ExtraLarge => 2.0,
        };
        if (ctx.pixels_per_point() - scale).abs() > f32::EPSILON {
            ctx.set_pixels_per_point(scale);
        }
    }

    fn render_menu_bar(&self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("menu_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Font Size:");
                for variant in [
                    FontSize::Small,
                    FontSize::Medium,
                    FontSize::Large,
                    FontSize::ExtraLarge,
                ] {
                    if ui
                        .selectable_label(self.font_size == variant, format!("{:?}", variant))
                        .clicked()
                    {
                        // Handle font size change externally
                    }
                }
            });
        });
    }

    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Unifi Device Search");
        ui.separator();
        ui.label("Enter MAC address to search across sites.");
        // Inputs and Search Button Logic goes here
    }
}

impl eframe::App for GuiApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_ui_scaling(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_menu_bar(ui);
            self.render_main_ui(ui);
        });

        if let Some(popup) = &mut self.popup {
            popup.show(ctx);
        }
    }
}
