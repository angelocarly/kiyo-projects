use kiyo::app::app::{App, AppConfig};
use kiyo::app::draw_orch::{DispatchConfig, DrawConfig, Pass};

fn main() {

    let app_config = AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: true,
    };
    let app = App::new(app_config);

    let mut draw_config = DrawConfig::new();
    draw_config.passes = Vec::from([
        Pass {
            shader: "fractal-folding/shaders/fractal.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from( [] ),
            output_resources: Vec::from([ 0 ]),
        },
        Pass {
            shader: "fractal-folding/shaders/overlay.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from( []),
            output_resources: Vec::from([ 0 ]),
        },
    ]);

    app.run(draw_config);
}