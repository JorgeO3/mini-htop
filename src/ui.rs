// TODO [x] draw_cpu_graph
// TODO [-] draw_disk_usage
// TODO [-] draw_memory_usage
// TODO [-] draw_components_temps
// TODO [-] draw_network_usage
// TODO [-] draw_list_process

use ratatui::{prelude::*, text::Spans, widgets::*};

use crate::system_info::{DiskUsageData, SystemInfo, SystemResources};

pub fn draw<B: Backend>(f: &mut Frame<B>, sys: &SystemInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_cpu_graph(f, chunks[0], sys);
    draw_system_stats(f, chunks[1], sys);
    draw_network_and_processes(f, chunks[2], sys);
}

// first block
pub fn draw_cpu_graph<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
    let cpu = sys.sys_resources.cpu_usage.get_values();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" Cpu Usage ".white().bold())
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::LightBlue))
        .data(&cpu)
        .bar_set(symbols::bar::NINE_LEVELS);

    f.render_widget(sparkline, area);
}

// second block
pub fn draw_system_stats<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
        .split(area);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(chunks[0]);

    draw_memory_usage(f, chunks[1], sys);
    draw_disk_usage(f, vertical_chunks[0], sys);
    draw_components_temps(f, vertical_chunks[1], sys);
}

pub fn draw_memory_usage<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
    let SystemResources {
        ram_memory_usage,
        swap_memory_usage,
        ..
    } = &sys.sys_resources;

    let ram = ram_memory_usage.get_values_with_index();
    let swap = swap_memory_usage.get_values_with_index();

    let datasets = vec![
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .graph_type(GraphType::Line)
            .data(ram.as_slice()),
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Blue))
            .graph_type(GraphType::Line)
            .data(swap.as_slice()),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(" Memory ".white().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 200.0]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(chart, area);

    const LEGEND_HEIGHT: u16 = 4;
    const LEGEND_WIDTH: u16 = 14;
    if area.height >= LEGEND_HEIGHT && area.width >= LEGEND_WIDTH {
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: LEGEND_WIDTH,
            y: area.y,
            x: area.left() + 2,
        };

        let ram_percentaje = ram.first().unwrap_or(&(0.0, 0.0)).1;
        let swap_percentaje = swap.first().unwrap_or(&(0.0, 0.0)).1;

        draw_legend(f, legend_area, ram_percentaje, swap_percentaje);
    }
}

pub fn draw_legend<B>(f: &mut Frame<B>, area: Rect, ram: f64, swap: f64)
where
    B: Backend,
{
    let text = vec![
        Line::from(Span::styled(
            format!("RAM: {:.2}%", ram),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("SWAP: {:.2}%", swap),
            Style::default().fg(Color::Blue),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().padding(Padding::new(1, 1, 1, 0)))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}

pub fn draw_disk_usage<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
    let empty_disk = DiskUsageData {
        free_space: 0.0,
        used_space: 0.0,
        name: "".to_string(),
    };
    let disk = sys.sys_resources.disk_usage.last().unwrap_or(&empty_disk);

    let text = vec![
        Line::from(Span::styled(
            format!("{}: {:.2}GB", disk.name, disk.free_space),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            format!("{}: {:.2}%", disk.name, disk.used_space),
            Style::default().fg(Color::Blue),
        )),
    ];

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

pub fn draw_components_temps<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
    let compts_temps = &sys.sys_resources.component_temperature;

    let text: Vec<Line> = compts_temps
        .iter()
        .filter(|(name, _)| name.contains("Package id 0") || name.contains("Composite"))
        .map(|(name, temp)| {
            let names: Vec<_> = name.split_whitespace().collect();
            let short_name = names[0];
            let temp_str = format!("{}°C", temp);
            let available_width = area.width as usize - short_name.len() - temp_str.len() - 2; // -2 for the colons
            let space = " ".repeat(available_width);

            let white_style = Style::default().fg(Color::White);
            let yello_style = Style::default().fg(Color::Yellow);

            Line::from(vec![
                Span::styled(format!("{}:", short_name), white_style),
                Span::styled(space, white_style),
                Span::styled(temp_str, yello_style),
            ])
        })
        .collect();

    let block = Block::default()
        .title(Span::styled(
            " Temperatures ",
            Style::default().fg(Color::White).bold(),
        ))
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

// network and process
pub fn draw_network_usage<B>()
where
    B: Backend,
{
}

pub fn draw_list_process<B>()
where
    B: Backend,
{
}

// network and process
pub fn draw_network_and_processes<B>(f: &mut Frame<B>, area: Rect, sys: &SystemInfo)
where
    B: Backend,
{
}
