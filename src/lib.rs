use nih_plug::prelude::*;
use std::sync::Arc;

mod filter;
mod params;

use filter::{Filter, FilterType};
use params::{FilterType as ParamFilterType, VariableFilterParams};

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
    const URL: &'static str = "https://your-website.com";
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
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sample_rate = _context.transport().sample_rate;
        let smoothing_enabled = self.params.smoothing_enabled.value();
        let smoothing_time = self.params.smoothing_time.value();
    
        let filter_type = self.params.filter_type.value();
    
        // Calculate the smoothing coefficient
        let smoothing_coeff = if smoothing_enabled {
            (-2.0 * std::f32::consts::PI * (1.0 / (smoothing_time * 0.001 * sample_rate))).exp()
        } else {
            0.0 // No smoothing
        };
    
        // Use static variables to store the current smoothed values
        static mut CURRENT_CUTOFF: f32 = 1000.0;
        static mut CURRENT_RESONANCE: f32 = 0.0;
    
        for channel_samples in buffer.iter_samples() {
            let target_cutoff = self.params.cutoff.value();
            let target_resonance = self.params.resonance.value();
    
            // Apply smoothing
            unsafe {
                CURRENT_CUTOFF = CURRENT_CUTOFF * smoothing_coeff + target_cutoff * (1.0 - smoothing_coeff);
                CURRENT_RESONANCE = CURRENT_RESONANCE * smoothing_coeff + target_resonance * (1.0 - smoothing_coeff);
    
                self.filter.set_params(CURRENT_CUTOFF, CURRENT_RESONANCE);
            }
    
            self.filter.set_filter_type(match filter_type {
                ParamFilterType::Moog => FilterType::Moog,
                ParamFilterType::Roland => FilterType::Roland,
                ParamFilterType::Le13700 => FilterType::Le13700,
            });
    
            for sample in channel_samples {
                *sample = self.filter.process(*sample, sample_rate);
            }
        }
    
        ProcessStatus::Normal
    }
    
    
}

impl ClapPlugin for VariableFilter {
    const CLAP_ID: &'static str = "com.muzikar.variable-filter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A variable filter plugin");
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
