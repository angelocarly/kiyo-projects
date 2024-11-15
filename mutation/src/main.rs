use kiyo::app::app::AppConfig;
use kiyo::app::App;
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};

fn main() {
    let app_config = AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: true,
    };
    let app = App::new(app_config);

    let draw_config = DrawConfig {
        passes: Vec::from([
            Pass {
                shader: "mutation/shaders/decrease.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([0]),
                output_resources: Vec::from([0]),
            },
            Pass {
                shader: "mutation/shaders/shader.comp".to_string(),
                dispatches: DispatchConfig::Count(100, 1, 1),
                input_resources: Vec::from([]),
                output_resources: Vec::from([0]),
            },
        ]),
        images: vec![
            ImageConfig {
                clear: ClearConfig::None,
            },
        ],
    };

    app.run(draw_config, None);
}