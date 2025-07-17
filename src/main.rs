use std::{fmt::Display, str::FromStr};

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

trait SupportedWidth<T> {
    fn from_hex(input: String) -> Option<T>;
}

impl SupportedWidth<u32> for u32 {
    fn from_hex(input: String) -> Option<u32> {
        u32::from_str_radix(&input.trim_start_matches("0x"), 16).ok()
    }
}
impl SupportedWidth<u64> for u64 {
    fn from_hex(input: String) -> Option<u64> {
        u64::from_str_radix(&input.trim_start_matches("0x"), 16).ok()
    }
}

#[derive(Clone)]
enum Mode {
    RV32,
    RV64,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::RV32 => "RV32",
            Mode::RV64 => "RV64",
        })
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RV32" => Ok(Mode::RV32),
            "RV64" => Ok(Mode::RV64),
            _ => Err(()),
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let (pmp_addr_input, pmp_addr_input_set) = signal("0x0".to_string());
    let (pmp_addr, pmp_addr_set) = signal(0u64);
    let (mode, mode_set) = signal(Mode::RV32);

    view! {
      <select
        on:change:target=move |ev| {
          mode_set.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || mode.get().to_string()
      >
        <option value="RV32">"RV32"</option>
        <option value="RV64">"RV64"</option>
      </select>
        <input type="text"
            on:input:target=move |ev| {
                let input_addr = ev.target().value();
                if let Some(calculated_addr) = match *mode.read() {
                    Mode::RV32 => convert_addr::<u32>(input_addr.clone()).map(|v| v as u64),
                    Mode::RV64 => convert_addr::<u64>(input_addr.clone()),
                } {
                    pmp_addr_set.set(calculated_addr);
                }
                pmp_addr_input_set.set(input_addr);
            }

            prop:value=pmp_addr_input
        />
        <p>"Name is: " {pmp_addr}</p>
    }
}

fn convert_addr<T>(input: String) -> Option<T>
where
    T: SupportedWidth<T> + FromStr,
{
    match input.starts_with("0x") {
        true => T::from_hex(input),
        false => input.parse().ok(),
    }
}
