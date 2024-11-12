pub mod moog;
pub mod roland;
pub mod le13700;
pub mod arp4075;

use arp4075::Arp4075;
use moog::MoogFilter;
use roland::RolandFilter;
use le13700::Le13700Filter;

pub enum FilterType {
    Moog,
    Roland,
    Le13700,
    ARP4075,
}

pub struct Filter {
    filter_type: FilterType,
    moog_filter: MoogFilter,
    roland_filter: RolandFilter,
    le13700_filter: Le13700Filter,
    arp4075: Arp4075,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            filter_type: FilterType::Moog,
            moog_filter: MoogFilter::new(),
            roland_filter: RolandFilter::new(),
            le13700_filter: Le13700Filter::new(),
            arp4075: Arp4075::new(),
        }
    }

    pub fn set_params(&mut self, cutoff: f32, resonance: f32) {
        self.moog_filter.set_params(cutoff, resonance);
        self.roland_filter.set_params(cutoff, resonance);
        self.le13700_filter.set_params(cutoff, resonance);
        self.arp4075.set_params(cutoff, resonance);

    }

    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        match self.filter_type {
            FilterType::Moog => self.moog_filter.process(input, sample_rate),
            FilterType::Roland => self.roland_filter.process(input, sample_rate),
            FilterType::Le13700 => self.le13700_filter.process(input, sample_rate),
            FilterType::ARP4075 => self.arp4075.process(input, sample_rate),
        }
    }
}
