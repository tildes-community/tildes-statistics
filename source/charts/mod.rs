//! All code for drawing the [`plotters`] charts.

use {
  async_std::{fs::create_dir_all, path::PathBuf},
  color_eyre::Result,
  plotters::prelude::*,
};

use crate::group_data::GroupDataModel;

const BACKGROUND_1: RGBColor = RGBColor(31, 23, 49);
const BACKGROUND_2: RGBColor = RGBColor(42, 32, 65);
const BACKGROUND_3: RGBColor = RGBColor(10, 8, 16);
const FOREGROUND: RGBColor = RGBColor(242, 239, 255);
const ACCENT_1: RGBColor = RGBColor(210, 184, 58);

/// The chart for the user count.
#[derive(Debug)]
pub struct UserCountChart {
  /// The groups to use for user counts.
  pub groups: Vec<GroupDataModel>,
}

impl UserCountChart {
  /// Render the chart and write it to file.
  pub async fn render(
    &self,
    parent: &PathBuf,
    group_name: &str,
    render_point_circles: bool,
    truncate: bool,
    output_dir: &str,
  ) -> Result<PathBuf> {
    let parent = parent.join(format!("{}/user-count", output_dir));
    create_dir_all(&parent).await?;

    let (mut datapoints, mut min_count, mut max_count) = (vec![], i64::MAX, 0);

    for (index, group) in self.groups.iter().enumerate() {
      datapoints.push(((index + 1) as isize, group.subscribers));

      if group.subscribers > max_count {
        max_count = group.subscribers;
      }

      if group.subscribers < min_count {
        min_count = group.subscribers;
      }
    }

    let datapoints_len = datapoints.len() as isize;
    let min_count = if truncate { min_count - 10 } else { 0 };
    let max_count = max_count + 10;

    let path = parent.join(format!("{group_name}.svg"));
    let output_path = path.clone();
    let chart_root = SVGBackend::new(&path, (1280, 720)).into_drawing_area();
    chart_root.fill(&BACKGROUND_2)?;

    let text_style =
      |font_size: i32| ("sans-serif", font_size).into_font().color(&FOREGROUND);

    let chart_root = chart_root
      .margin(20, 20, 20, 20)
      .titled("User Count", text_style(30))?;

    chart_root.fill(&BACKGROUND_2)?;

    let mut chart = ChartBuilder::on(&chart_root)
      .caption(
        format!("Using the {group_name} subscriber count."),
        text_style(20),
      )
      .x_label_area_size(50)
      .y_label_area_size(50)
      .margin(10)
      .build_cartesian_2d(0..(datapoints_len + 1), min_count..max_count)?;

    chart
      .configure_mesh()
      .x_labels(datapoints.len() + 2)
      .x_label_formatter(&|x| {
        if (x - 1) % (datapoints_len / 20).max(1) != 0 {
          String::new()
        } else {
          format!("{:0}", datapoints_len - x)
        }
      })
      .x_desc("N days ago")
      .y_labels(5)
      .y_label_formatter(&|y| format!("{y:0}"))
      .label_style(text_style(20))
      .axis_style(&BACKGROUND_1)
      .light_line_style(&BACKGROUND_1)
      .bold_line_style(&BACKGROUND_3)
      .draw()?;

    chart
      .draw_series(LineSeries::new(
        datapoints.clone(),
        ACCENT_1.stroke_width(2),
      ))?
      .label("User Count")
      .legend(|(x, y)| {
        PathElement::new(vec![(x, y), (x + 20, y)], ACCENT_1.stroke_width(4))
      });

    chart.draw_series(PointSeries::of_element(
      datapoints,
      5,
      &ACCENT_1,
      &|(x, y), size, style| {
        EmptyElement::at((x, y))
          + Circle::new(
            (0, 0),
            size,
            if render_point_circles {
              style.filled()
            } else {
              TRANSPARENT.filled()
            },
          )
          + Text::new(
            {
              if (x - 1) % (datapoints_len / 10).max(1) != 0 {
                String::new()
              } else {
                format!("{:0}", y)
              }
            },
            (-20, -25),
            text_style(20),
          )
      },
    ))?;

    Ok(output_path)
  }
}
