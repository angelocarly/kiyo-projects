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
            ImageConfig {
                clear: ClearConfig::None
            },
            ImageConfig {
                clear: ClearConfig::None
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "feedback/shaders/setup.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
            Pass {
                shader: "feedback/shaders/blur.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 0 ]),
                output_resources: Vec::from([ 1 ]),
            },
            Pass {
                shader: "feedback/shaders/sharpen.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 1 ]),
                output_resources: Vec::from([ 0 ]),
            },
        ])
    };

    app.run(config, None);
}