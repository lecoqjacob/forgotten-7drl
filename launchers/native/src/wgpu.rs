use clap::Args;

#[derive(Default, Args)]
pub struct Wgpu {
    #[clap(short, long, action)]
    force_opengl: bool,
}

impl Wgpu {
    pub fn run(&self, app: gridbugs::chargrid::control_flow::App) {
        use gridbugs::chargrid_wgpu::*;

        const CELL_SIZE_PX: f64 = 16.;

        let ctx = Context::new(Config {
            resizable: true,
            force_secondary_adapter: false,
            underline_width_cell_ratio: 0.1,
            underline_top_offset_cell_ratio: 0.8,
            title: forgotten_app::LAUNCHER_TITLE.to_string(),
            window_dimensions_px: Dimensions { width: 960., height: 720. },
            font_scale: Dimensions { width: CELL_SIZE_PX, height: CELL_SIZE_PX },
            cell_dimensions_px: Dimensions { width: CELL_SIZE_PX, height: CELL_SIZE_PX },
            font_bytes: FontBytes {
                normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin-custom.ttf").to_vec(),
                bold: include_bytes!("./fonts/PxPlus_IBM_CGA-custom.ttf").to_vec(),
            },
        });

        ctx.run(app);
    }
}
