#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use eframe::egui;

mod functions;
fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "SimImg - Image Similarity Calculator",
        options,
        Box::new(|_cc| Box::new(SimImg::default())),
    )
}

struct SimImg {
    path1: Option<String>,
    path2: Option<String>,
    psnr: Option<f64>,
    rase: Option<f64>,
    rme: Option<f64>,
    sam: Option<f64>,
    scc: Option<f64>,
    ssim: Option<f64>,
    uqi: Option<f64>,
    vif: Option<f64>,
}

impl Default for SimImg {
    fn default() -> Self {
        Self {
            path1: Some("None".into()),
            path2: Some("None".into()),
            psnr: Some(0.into()),
            rase: Some(0.into()),
            rme: Some(0.into()),
            sam: Some(0.into()),
            scc: Some(0.into()),
            ssim: Some(0.into()),
            uqi: Some(0.into()),
            vif: Some(0.into()),
        }
    }
}

struct Args {
    path1: String,
    path2: String,
}


impl eframe::App for SimImg {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SimImg");
            ui.separator();
            ui.label("Image 1:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.path1.clone().unwrap());
                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", &["png", "jpeg", "jpg"])
                        .pick_file()
                    {
                        self.path1 = Some(path.display().to_string());
                    }
                }
            });
            ui.separator();
            ui.label("Image 2:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.path2.clone().unwrap());
                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", &["png", "jpeg", "jpg"])
                        .pick_file()
                    {
                        self.path2 = Some(path.display().to_string());
                    }
                }
            });
            ui.separator();
            if ui.button("Compare").clicked() {
                println!("Comparing");
                // Do this in a separate thread

                let path1 = &self.path1.clone().unwrap();
                let path2 = &self.path2.clone().unwrap();
                let args = Args {
                    path1: path1.clone(),
                    path2: path2.clone(),
                };
                let handle = thread::spawn(move || {
                    compare_images(&args.path1, &args.path2)
                });
                let (psnr, rase, rme, sam, scc, ssim, uqi, vif) = handle.join().unwrap();
                self.psnr = Some(psnr);
                self.rase = Some(rase);
                self.rme = Some(rme);
                self.sam = Some(sam);
                self.scc = Some(scc);
                self.ssim = Some(ssim);
                self.uqi = Some(uqi);
                self.vif = Some(vif);
            }

            ui.separator();
            ui.label("Results:");
            ui.horizontal(|ui| {
                ui.label("PSNR:");
                ui.label(self.psnr.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("RASE:");
                ui.label(self.rase.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("RME:");
                ui.label(self.rme.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("SAM:");
                ui.label(self.sam.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("SCC:");
                ui.label(self.scc.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("SSIM:");
                ui.label(self.ssim.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("UQI:");
                ui.label(self.uqi.clone().unwrap().to_string());
            });
            ui.horizontal(|ui| {
                ui.label("VIF:");
                ui.label(self.vif.clone().unwrap().to_string());
            });
        });
    }
}

fn compare_images(path1: &str, path2: &str) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let mut img1 = image::open(path1).expect("Failed to Open File");
    let mut img2 = image::open(path2).expect("Failed to Open File");
    img1 = img1.resize_exact(500, 500, image::imageops::FilterType::Triangle);
    img2 = img2.resize_exact(500, 500, image::imageops::FilterType::Triangle);
    let pixels1 = img1
        .to_rgba32f()
        .into_iter()
        .map(|x| x.to_owned() as f64)
        .collect::<Vec<f64>>();
    let pixels2 = img2
        .to_rgba32f()
        .into_iter()
        .map(|x| x.to_owned() as f64)
        .collect::<Vec<f64>>();
    let psnr = functions::psnr::peak_signal_to_noise_ratio(&pixels1, &pixels2);
    let rase = functions::rase::relative_average_spectral_error(&pixels1, &pixels2);
    let rme = functions::rme::rme(&pixels1, &pixels2);
    let sam = functions::sam::spectral_angle_mapper(&pixels1, &pixels2);
    let scc = functions::scc::spatial_correlation_coefficient(&pixels1, &pixels2);
    let ssim = functions::ssim::structural_similarity_index(&pixels1, &pixels2);
    let uqi = functions::uqi::universal_quality_image_index(&pixels1, &pixels2);
    let vif = functions::vif::visual_information_fidelity(&pixels1, &pixels2);
    (psnr, rase, rme, sam, scc, ssim, uqi, vif)
}
