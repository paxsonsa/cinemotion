pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));
    frame.render_widget(block, area);
}
