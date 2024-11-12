use nih_plug::prelude::*;

#[derive(Params)]
pub struct VariableFilterParams {
    #[id = "cutoff"]
    pub cutoff: FloatParam,

    #[id = "resonance"]
    pub resonance: FloatParam,

    #[id = "filter_type"]
    pub filter_type: EnumParam<FilterType>,

    #[id = "smoothing_enabled"]
    pub smoothing_enabled: BoolParam,

    #[id = "smoothing_time"]
    pub smoothing_time: FloatParam,

    #[id = "sync_mode"]
    pub sync_mode: EnumParam<SyncMode>,

    #[id = "sync_value"]
    pub sync_value: EnumParam<SyncValue>,
}

#[derive(Enum, PartialEq, Clone)]
pub enum FilterType {
    Moog,
    Roland,
    Le13700,
    ARP4075,
}

#[derive(Enum, PartialEq, Clone)]
pub enum SyncMode {
    Milliseconds,
    MidiSync,
}

#[derive(Enum, PartialEq, Clone)]
pub enum SyncValue {
    Bars4,
    Bars3_5,
    Bars3,
    Bars2_5,
    Bars2,
    Bars1_5,
    Bar1,
    Note3_4,
    Note1_2,
    Note3_8,
    Note1_4,
    Note3_16,
    Note1_6,
    Note1_8,
    Note1_12,
    Note1_16,
    Note1_24,
    Note1_32,
    Note1_48,
    Note1_64,
}

impl VariableFilterParams {
    pub fn new() -> Self {
        Self {
            cutoff: FloatParam::new(
                "Cutoff",
                1000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz"),

            resonance: FloatParam::new(
                "Resonance",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            filter_type: EnumParam::new("Filter Type", FilterType::Moog),

            smoothing_enabled: BoolParam::new("Smoothing Enabled", false),

            smoothing_time: FloatParam::new(
                "Smoothing Time",
                50.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1000.0,
                },
            )
            .with_unit(" ms"),

            sync_mode: EnumParam::new("Sync Mode", SyncMode::Milliseconds),

            sync_value: EnumParam::new("Sync Value", SyncValue::Note1_4),
        }
    }
}
