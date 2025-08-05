use msfs::{nvg, MSFSEvent};

#[msfs::gauge(name=Demo)]
async fn demo(mut gauge: msfs::Gauge) -> Result<(), Box<dyn std::error::Error>> {
    let nvg = gauge.create_nanovg().unwrap();

    let red = nvg::Style::default().fill(nvg::Color::from_rgb(255, 0, 0));

    while let Some(event) = gauge.next_event().await {
        match event {
            MSFSEvent::PreDraw(d) => {
                nvg.draw_frame(d.width(), d.height(), |f| {
                    // draw square
                    f.draw_path(&red, |p| {
                        p.rect(500.0, 200.0, 200.0, 200.0);

                        Ok(())
                    })?;

                    Ok(())
                });
            }
            MSFSEvent::PreUpdate => {
                println!("Hello rusty world!");
            }
            _ => {
                //dbg!("{}", event);
            }
        }
    }

    Ok(())
}
