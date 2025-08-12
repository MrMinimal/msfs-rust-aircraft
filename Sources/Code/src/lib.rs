use msfs::{
    legacy::{AircraftVariable, NamedVariable},
    MSFSEvent,
    nvg,
};

#[msfs::gauge(name=Demo)]
async fn demo(mut gauge: msfs::Gauge) -> Result<(), Box<dyn std::error::Error>> {
    let nvg = gauge.create_nanovg().unwrap();

    let red = nvg::Style::default().fill(nvg::Color::from_rgb(255, 0, 0));

    while let Some(event) = gauge.next_event().await {
        match event {
            MSFSEvent::PreDraw(draw_data) => {
                let delta_time = draw_data.dt;
                let time_elapsed = draw_data.t;
                
                nvg.draw_frame(d.width(), d.height(), |f| {
                    // draw square
                    f.draw_path(&red, |p| {
                        p.rect(500.0, 200.0, 200.0, 200.0);

                        Ok(())
                    })?;

                    Ok(())
                });

                let custom_variable = NamedVariable::from("YOUR_CUSTOM_NAME");
                println!("My custom variable was: {}", custom_variable.get_value::<f64>());
                custom_variable.set_value(custom_variable.get_value::<f64>() + 0.1);
                println!("My custom variable is now: {}", custom_variable.get_value::<f64>());
            }
            _ => {
                //dbg!("{}", event);
            }
        }
    }

    Ok(())
}
