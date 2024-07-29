use plotly::common::{Marker, Mode, Title};
use plotly::{Bar, Layout, Plot, Scatter};
use plotly::layout::{Axis, GridPattern, GridYSide, LayoutGrid};

fn main() {
    // Define the data for the scatter plot
    let x_data = vec![1, 2, 3, 4, 5];
    let y_data = vec![2, 4, 1, 3, 5];

    // Create a Scatter trace
    let trace = Scatter::new(x_data, y_data)
        .mode(Mode::Markers)  // Set the marker mode
        .marker(Marker::new().size(10));  // Set the marker size

    // Create the plot layout
    let layout = Layout::new()
        .title(Title::with_text("Scatter plot example"))
        .x_axis(Axis::new().title(Title::with_text("X-axis")))
        .y_axis(Axis::new().title(Title::with_text("Y-axis")));

    // Create the plot and add the trace and layout
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    // Show the plot in the default browser
    // plot.show();

    // Define the data for the bar chart
    let x_data = vec!["Apples", "Oranges", "Bananas"];
    let y_data = vec![10, 20, 15];

    // Create a Bar trace
    let trace = Bar::new(x_data, y_data)
        .name("Fruit sales");

    // Create the plot layout
    let layout2 = Layout::new()
        .title(Title::with_text("Fruit Sales"))
        .x_axis(Axis::new().title(Title::with_text("Fruit")))
        .y_axis(Axis::new().title(Title::with_text("Sales")));

    // Create the plot and add the trace and layout
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout2);

    // Show the plot in the default browser
    // plot.show();

    let layout_grid = LayoutGrid::new()
        .rows(1)
        .columns(2)
        .sub_plots(vec!["x1", "x2"])
        .y_axes(vec!["y1", "y2"])
        .pattern(GridPattern::Independent)
        .y_side(GridYSide::Right);

    let layout = Layout::new()
        .title(Title::with_text("Subplots Example"))
        .grid(layout_grid).x_axis(Axis::new()
        .title(Title::with_text("X-axis")))
        .y_axis(Axis::new().title(Title::with_text("Y-axis")).overlaying("y1"))
        .x_axis2(Axis::new().title(Title::with_text("X-axis-2")).overlaying("x2"))
        .y_axis2(Axis::new().title(Title::with_text("Y-axis-2"))
            .anchor("x2")
            .overlaying("y2"));


    let mut plot = Plot::new();
    plot.add_traces(vec![
        Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("V1"),
    ]);

    plot.add_trace(Scatter::new(vec![1, 2, 3], vec![9, 10, 11]).name("V2"));


    plot.set_layout(layout);

    plot.add_trace(Bar::new(vec!["A", "B", "C"], vec![4, 5, 6])
        .name("Bars ").x_axis("x2").y_axis("y2"));


    plot.show();
    println!("Finish")

}
