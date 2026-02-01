use clap::Parser;
use std::path::PathBuf;

mod plugin_loader;
mod error;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    plugin: String,
    #[arg(long)]
    params: PathBuf,
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = image::open(&args.input)?.to_rgba8();
    let (width, height) = img.dimensions();
    let mut raw_pixels = img.into_raw();

    let lib_ext = if cfg!(target_os = "windows") {
        "dll"
    } else {
        "so"
    };
    let lib_name = if cfg!(target_os = "linux") {
        format!("lib{}.{}", args.plugin, lib_ext)
    } else {
        format!("{}.{}", args.plugin, lib_ext)
    };

    let full_plugin_path = args.plugin_path.join(lib_name);
    let plugin = plugin_loader::Plugin::load(&full_plugin_path)?;

    let params_content =
        std::fs::read_to_string(&args.params).expect("Could not read params file");
    let c_params = std::ffi::CString::new(params_content)?;

    println!("Process via {}...", args.plugin);
    plugin.execute(width, height, &mut raw_pixels, c_params.as_ptr());

    let output_img = image::RgbaImage::from_raw(width, height, raw_pixels)
        .ok_or("Create output image error")?;
    output_img.save(&args.output)?;

    println!("Done! The result is saved. {:?}", args.output);
    Ok(())
}
