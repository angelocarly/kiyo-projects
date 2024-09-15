use kiyo::app::app::{App, AppConfig};
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};

fn main() {

    let app = App::new(AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: false,
    });

    let config = DrawConfig {
        images: Vec::from([
            // Color
            ImageConfig {
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
            // Pos
            ImageConfig {
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
            // Normal
            ImageConfig {
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "prison/shaders/color.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 0 ]),
                output_resources: Vec::from([ 0 ]),
            },
        ])
    };

    app.run(config, None);
}