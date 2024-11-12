# Variable Filter Plugin
[![Build](https://github.com/OseMine/variable-filter/actions/workflows/automated-prerelease.yml)]
[![Latest Release](https://img.shields.io/github/v/release/OseMine/variable-filter?style=flat)]


A versatile audio filter plugin that implements various filter types and allows for dynamic parameter smoothing.

## Project Structure

- `src/`
  - `lib.rs`: Main plugin file, contains the plugin implementation and audio processing logic.
  - `params.rs`: Defines the plugin parameters.
  - `filter/`
    - `mod.rs`: Manages the different filter types and their selection.
    - `moog.rs`: Implementation of the Moog filter.
    - `roland.rs`: Implementation of the Roland filter.
    - `le13700.rs`: Implementation of the Le13700 filter.

## Adding a New Filter

1. Create a new file in `src/filter/`, e.g., `new_filter.rs`.
2. Implement the filter in this file:

```rust
pub struct NewFilter {
    // Filter-specific fields
}

impl NewFilter {
    pub fn new() -> Self {
        // Initialization logic
    }

    pub fn set_params(&mut self, cutoff: f32, resonance: f32) {
        // Parameter setting logic
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Audio processing logic
    }
}
```

3. Add the new filter in `src/filter/mod.rs`:

```rust
pub mod new_filter;
use new_filter::NewFilter;

// Add the new filter to the FilterType enumeration
pub enum FilterType {
    // ...
    NewFilter,
}

// Update the Filter structure
pub struct Filter {
    // ...
    new_filter: NewFilter,
}

// Update the implementation of the Filter structure
impl Filter {
    // ...
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        match self.filter_type {
            // ...
            FilterType::NewFilter => self.new_filter.process(input, sample_rate),
        }
    }
}
```

4. Update `src/params.rs` to add the new filter type as an option:

```rust
#[derive(Enum, PartialEq, Clone)]
pub enum FilterType {
    // ...
    NewFilter,
}
```

5. In `src/lib.rs`, update the `process` function to consider the new filter type:

```rust
self.filter.set_filter_type(match filter_type {
    // ...
    ParamFilterType::NewFilter => FilterType::NewFilter,
});
```

## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## Building the Project

1. Ensure Rust and Cargo are installed.
2. Clone the repository: `git clone https://github.com/OseMine/variable-filter.git`
3. Navigate to the project directory: `cd variable-filter`
4. Install all the necessary dependencies: `cargo fetch`
5. Build the project: `cargo xtask bundle variable-filter --release`
6. Find the plugin files in the `target/release` directory.

## Using the Plugin

- Copy the created plugin files to your VST3/CLAP plugin directory.
- Load the plugin in your preferred DAW.

## License

[MIT](https://choosealicense.com/licenses/mit/)
