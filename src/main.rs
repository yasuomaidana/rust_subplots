use plotly::common::{Marker, Mode, Title};
use plotly::{Bar, Layout, Plot, Scatter};
use plotly::layout::Axis;

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
    plot.show();

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
    plot.show();


}
