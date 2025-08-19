use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

mod audio_processor;
mod ui;

use audio_processor::{AudioProcessor, ProcessingProgress, ProcessingResult};

#[derive(Default)]
pub struct AutotuneApp {
    // File paths
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,

    // Audio processing
    processor: AudioProcessor,

    // UI state
    is_processing: bool,
    processing_progress: f32,
    processing_status: String,
    processing_result: Option<ProcessingResult>,

    // Communication with processing thread
    progress_receiver: Option<mpsc::Receiver<ProcessingProgress>>,
    result_receiver: Option<mpsc::Receiver<ProcessingResult>>,

    // Musical settings
    selected_key: usize,
    selected_note: i32,
    octave: i32,
    formant_shift: i32,

    // Autotune parameters
    pitch_correction_strength: f32,
    transition_speed: f32,

    // Audio info
    sample_rate: Option<f32>,
    duration: Option<f32>,
    channels: Option<u16>,
}

impl AutotuneApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_key: 0,  // C Major
            selected_note: 0, // Auto mode
            octave: 2,
            formant_shift: 0,
            pitch_correction_strength: 0.8,
            transition_speed: 0.1,
            processing_status: "Ready".to_string(),
            ..Default::default()
        }
    }

    fn select_input_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("WAV Audio", &["wav"])
            .set_title("Select Input WAV File")
            .pick_file()
        {
            self.input_file = Some(path.clone());
            self.load_audio_info(&path);
            self.processing_result = None;
            log::info!("Selected input file: {:?}", path);
        }
    }

    fn select_output_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("WAV Audio", &["wav"])
            .set_title("Save Autotuned WAV File")
            .set_file_name("autotuned.wav")
            .save_file()
        {
            self.output_file = Some(path);
            log::info!("Selected output file: {:?}", self.output_file);
        }
    }

    fn load_audio_info(&mut self, path: &PathBuf) {
        match hound::WavReader::open(path) {
            Ok(reader) => {
                let spec = reader.spec();
                self.sample_rate = Some(spec.sample_rate as f32);
                self.channels = Some(spec.channels);
                self.duration = Some(reader.duration() as f32 / spec.sample_rate as f32);
                self.processing_status = format!(
                    "Loaded: {:.1}s, {}Hz, {} ch",
                    self.duration.unwrap(),
                    spec.sample_rate,
                    spec.channels
                );
            }
            Err(e) => {
                self.processing_status = format!("Error loading file: {}", e);
                self.sample_rate = None;
                self.duration = None;
                self.channels = None;
            }
        }
    }

    fn start_processing(&mut self) {
        if self.input_file.is_none() || self.output_file.is_none() {
            self.processing_status = "Please select input and output files".to_string();
            return;
        }

        let input_path = self.input_file.as_ref().unwrap().clone();
        let output_path = self.output_file.as_ref().unwrap().clone();

        // Create channels for progress updates
        let (progress_tx, progress_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();

        self.progress_receiver = Some(progress_rx);
        self.result_receiver = Some(result_rx);

        // Configure autotune settings
        let autotune_config = synthphone_vocals::AutotuneConfig {
            fft_size: 1024,
            hop_size: 256,
            sample_rate: self.sample_rate.unwrap_or(44100.0),
            pitch_correction_strength: self.pitch_correction_strength,
            transition_speed: self.transition_speed,
            ..Default::default()
        };

        let musical_settings = synthphone_vocals::MusicalSettings {
            key: self.selected_key as i32,
            note: self.selected_note,
            octave: self.octave,
            formant: self.formant_shift,
        };

        // Start processing in background thread
        thread::spawn(move || {
            let result = AudioProcessor::process_file(
                &input_path,
                &output_path,
                autotune_config,
                musical_settings,
                progress_tx,
            );

            let _ = result_tx.send(result);
        });

        self.is_processing = true;
        self.processing_progress = 0.0;
        self.processing_status = "Processing...".to_string();
        self.processing_result = None;
    }

    fn update_processing_status(&mut self) {
        // Check for progress updates
        if let Some(ref receiver) = self.progress_receiver {
            while let Ok(progress) = receiver.try_recv() {
                match progress {
                    ProcessingProgress::Progress(percent) => {
                        self.processing_progress = percent;
                        self.processing_status = format!("Processing... {:.1}%", percent * 100.0);
                    }
                    ProcessingProgress::Status(status) => {
                        self.processing_status = status;
                    }
                }
            }
        }

        // Check for completion
        if let Some(ref receiver) = self.result_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.is_processing = false;
                self.processing_result = Some(result.clone());

                match result {
                    ProcessingResult::Success { duration_ms, .. } => {
                        self.processing_status =
                            format!("Completed in {:.2}s!", duration_ms / 1000.0);
                    }
                    ProcessingResult::Error(err) => {
                        self.processing_status = format!("Error: {}", err);
                    }
                }

                // Clean up channels
                self.progress_receiver = None;
                self.result_receiver = None;
            }
        }
    }
}

impl eframe::App for AutotuneApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_processing_status();

        // Request repaint if processing
        if self.is_processing {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Desktop Autotune Application");
            ui.separator();

            // File selection section
            ui.group(|ui| {
                ui.label("📁 File Selection");

                ui.horizontal(|ui| {
                    if ui.button("Select Input WAV").clicked() {
                        self.select_input_file();
                    }

                    if let Some(ref path) = self.input_file {
                        ui.label(format!("📄 {}", path.file_name().unwrap().to_string_lossy()));
                    } else {
                        ui.label("No file selected");
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Select Output Path").clicked() {
                        self.select_output_file();
                    }

                    if let Some(ref path) = self.output_file {
                        ui.label(format!("💾 {}", path.file_name().unwrap().to_string_lossy()));
                    } else {
                        ui.label("No output path selected");
                    }
                });

                // Audio file info
                if let (Some(duration), Some(sample_rate), Some(channels)) =
                    (self.duration, self.sample_rate, self.channels)
                {
                    ui.label(format!(
                        "📊 Duration: {:.2}s | Sample Rate: {}Hz | Channels: {}",
                        duration, sample_rate as i32, channels
                    ));
                }
            });

            ui.separator();

            // Musical settings section
            ui.group(|ui| {
                ui.label("🎼 Musical Settings");

                ui.horizontal(|ui| {
                    ui.label("Key:");
                    egui::ComboBox::from_id_source("key_selector")
                        .selected_text(ui::get_key_name(self.selected_key))
                        .show_ui(ui, |ui| {
                            for (i, name) in ui::KEY_NAMES.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_key, i, *name);
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Note Mode:");
                    egui::ComboBox::from_id_source("note_selector")
                        .selected_text(if self.selected_note == 0 {
                            "Auto"
                        } else {
                            "Manual"
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_note, 0, "Auto (snap to key)");
                            // Could add manual note selection here
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Octave:");
                    ui.add(egui::Slider::new(&mut self.octave, 0..=4).text("octave"));
                });

                ui.horizontal(|ui| {
                    ui.label("Formant Shift:");
                    ui.add(egui::Slider::new(&mut self.formant_shift, -12..=12).text("semitones"));
                });
            });

            ui.separator();

            // Autotune parameters section
            ui.group(|ui| {
                ui.label("⚙️ Autotune Parameters");

                ui.horizontal(|ui| {
                    ui.label("Pitch Correction:");
                    ui.add(
                        egui::Slider::new(&mut self.pitch_correction_strength, 0.0..=1.0)
                            .text("strength"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Transition Speed:");
                    ui.add(egui::Slider::new(&mut self.transition_speed, 0.01..=1.0).text("speed"));
                });
            });

            ui.separator();

            // Processing section
            ui.group(|ui| {
                ui.label("🔄 Processing");

                ui.horizontal(|ui| {
                    let process_button = ui.add_enabled(
                        !self.is_processing
                            && self.input_file.is_some()
                            && self.output_file.is_some(),
                        egui::Button::new("🚀 Start Processing"),
                    );

                    if process_button.clicked() {
                        self.start_processing();
                    }

                    if self.is_processing {
                        ui.spinner();
                    }
                });

                // Progress bar
                if self.is_processing {
                    ui.add(
                        egui::ProgressBar::new(self.processing_progress)
                            .text(format!("{:.1}%", self.processing_progress * 100.0)),
                    );
                }

                // Status
                ui.label(format!("Status: {}", self.processing_status));

                // Results
                if let Some(ref result) = self.processing_result {
                    match result {
                        ProcessingResult::Success { samples_processed, duration_ms } => {
                            ui.label(format!(
                                "✅ Success: Processed {} samples in {:.2}s",
                                samples_processed,
                                duration_ms / 1000.0
                            ));
                        }
                        ProcessingResult::Error(err) => {
                            ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", err));
                        }
                    }
                }
            });

            ui.separator();

            // Info section
            ui.collapsing("ℹ️ About", |ui| {
                ui.label("Desktop Autotune Application");
                ui.label("Built with the synthphone_vocals real-time autotune library");
                ui.label("Supports WAV files with various sample rates");
                ui.label("Uses FFT-based pitch detection and correction");
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Desktop Autotune Application",
        options,
        Box::new(|cc| Box::new(AutotuneApp::new(cc))),
    )
}
