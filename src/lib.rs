use nih_plug::prelude::*;
use std::sync::Arc;

mod filter;
mod params;

use filter::{Filter, FilterType};
use params::{FilterType as ParamFilterType, VariableFilterParams, SyncMode, SyncValue};

struct VariableFilter {
    params: Arc<VariableFilterParams>,
    filter: Filter,
}

impl Default for VariableFilter {
    fn default() -> Self {
        Self {
            params: Arc::new(VariableFilterParams::new()),
            filter: Filter::new(),
        }
    }
}

impl Plugin for VariableFilter {
    const NAME: &'static str = "Variable Filter";
    const VENDOR: &'static str = "The Muzikar";
    const URL: &'static str = "";
    const EMAIL: &'static str = "your-email@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sample_rate = context.transport().sample_rate;
        
        // Holen Sie sich das aktuelle Tempo von der DAW
        let tempo = context.transport().tempo.map(|t| t as f32).unwrap_or(120.0);

        
        let smoothing_enabled = self.params.smoothing_enabled.value();
        let smoothing_time = match self.params.sync_mode.value() {
            SyncMode::Milliseconds => self.params.smoothing_time.value(),
            SyncMode::MidiSync => self.get_sync_time_ms(self.params.sync_value.value(), tempo),
        };
    
        let filter_type = self.params.filter_type.value();
    
        // Berechne den Smoothing-Koeffizienten
        let smoothing_coeff = if smoothing_enabled {
            (-2.0 * std::f32::consts::PI * (1.0 / (smoothing_time * 0.001 * sample_rate))).exp()
        } else {
            0.0 // Kein Smoothing
        };
    
        // Verwende statische Variablen, um die aktuellen geglätteten Werte zu speichern
        static mut CURRENT_CUTOFF: f32 = 1000.0;
        static mut CURRENT_RESONANCE: f32 = 0.0;
    
        for channel_samples in buffer.iter_samples() {
            let target_cutoff = self.params.cutoff.value();
            let target_resonance = self.params.resonance.value();
    
            // Wende Smoothing an
            unsafe {
                CURRENT_CUTOFF = CURRENT_CUTOFF * smoothing_coeff + target_cutoff * (1.0 - smoothing_coeff);
                CURRENT_RESONANCE = CURRENT_RESONANCE * smoothing_coeff + target_resonance * (1.0 - smoothing_coeff);
    
                self.filter.set_params(CURRENT_CUTOFF, CURRENT_RESONANCE);
            }
    
            self.filter.set_filter_type(match filter_type {
                ParamFilterType::Moog => FilterType::Moog,
                ParamFilterType::Roland => FilterType::Roland,
                ParamFilterType::Le13700 => FilterType::Le13700,
                ParamFilterType::ARP4075 => FilterType::ARP4075,
            });
    
            for sample in channel_samples {
                *sample = self.filter.process(*sample, sample_rate);
            }
        }
    
        ProcessStatus::Normal
    }
    
}

impl VariableFilter {
    fn get_sync_time_ms(&self, sync_value: SyncValue, tempo: f32) -> f32 {
        let beats_per_minute = tempo;
        let ms_per_beat = 60000.0 / beats_per_minute;

        match sync_value {
            SyncValue::Bars4 => ms_per_beat * 16.0,
            SyncValue::Bars3_5 => ms_per_beat * 14.0,
            SyncValue::Bars3 => ms_per_beat * 12.0,
            SyncValue::Bars2_5 => ms_per_beat * 10.0,
            SyncValue::Bars2 => ms_per_beat * 8.0,
            SyncValue::Bars1_5 => ms_per_beat * 6.0,
            SyncValue::Bar1 => ms_per_beat * 4.0,
            SyncValue::Note3_4 => ms_per_beat * 3.0,
            SyncValue::Note1_2 => ms_per_beat * 2.0,
            SyncValue::Note3_8 => ms_per_beat * 1.5,
            SyncValue::Note1_4 => ms_per_beat,
            SyncValue::Note3_16 => ms_per_beat * 0.75,
            SyncValue::Note1_6 => ms_per_beat * 2.0 / 3.0,
            SyncValue::Note1_8 => ms_per_beat * 0.5,
            SyncValue::Note1_12 => ms_per_beat / 3.0,
            SyncValue::Note1_16 => ms_per_beat * 0.25,
            SyncValue::Note1_24 => ms_per_beat / 6.0,
            SyncValue::Note1_32 => ms_per_beat * 0.125,
            SyncValue::Note1_48 => ms_per_beat / 12.0,
            SyncValue::Note1_64 => ms_per_beat * 0.0625,
        }
    }
}

impl ClapPlugin for VariableFilter {
    const CLAP_ID: &'static str = "com.muzikar.variable-filter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Ein variables Filter-Plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Filter,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for VariableFilter {
    const VST3_CLASS_ID: [u8; 16] = *b"VariableFilterMZ";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Filter];
}

nih_export_clap!(VariableFilter);
nih_export_vst3!(VariableFilter);
