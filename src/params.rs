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
}

#[derive(Enum, PartialEq, Clone)]
pub enum FilterType {
    Moog,
    Roland,
    Le13700,
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
        }
    }
}
