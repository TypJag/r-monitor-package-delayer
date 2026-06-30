/// Parses a `$F,...` CSV line from the Scorit scoring system.
/// Returns the raw lap field (index 1) if the message type is `$F`,
/// otherwise returns `None`.
pub fn parse_lap_field(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.splitn(3, ',');
    if parts.next()? != "$F" {
        return None;
    }
    parts.next().map(|s| s.trim().to_owned())
}

/// Formats a message for the PixelCom display.
/// Sends the same line 5 times, matching the original Python `sendToPixel` behaviour.
pub fn format_pixel_message(laps: i32) -> String {
    let line = format!(
        "$F,{},\"00:00:00\",\"00:00:00\",\"00:00:00\",\"      \"\r\n",
        laps
    );
    line.repeat(5)
}
