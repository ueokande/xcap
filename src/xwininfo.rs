use std::error::Error;
use std::fmt;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

pub fn run() -> Result<WindowInfo, Box<Error>> {
    let out = Command::new("xwininfo").output()?;
    parse_xwininfo(String::from_utf8(out.stdout)?)
}

const FIELD_X: &'static str = "Absolute upper-left X:";
const FIELD_Y: &'static str = "Absolute upper-left Y:";
const FIELD_WIDTH: &'static str = "Width:";
const FIELD_HEIGHT: &'static str = "Height:";

// xwininfo: Please select the window about which you
//           would like information by clicking the
//           mouse in that window.
//
// xwininfo: Window id: 0xe00021 "Firefox Developer Edition"
//
//   Absolute upper-left X:  3841
//   Absolute upper-left Y:  21
//   Relative upper-left X:  0
//   Relative upper-left Y:  0
//   Width: 958
//   Height: 1178
//   Depth: 24
//   Visual: 0x2b
//   Visual Class: TrueColor
//   Border width: 0
//   Class: InputOutput
//   Colormap: 0xe00002 (not installed)
//   Bit Gravity State: NorthWestGravity
//   Window Gravity State: NorthWestGravity
//   Backing Store State: NotUseful
//   Save Under State: no
//   Map State: IsViewable
//   Override Redirect State: no
//   Corners:  +3841+21  -961+21  -961-961  +3841-961
//   -geometry 958x1178+3840+20
pub fn parse_xwininfo(source: String) -> Result<WindowInfo, Box<Error>> {
    let mut x: Option<i64> = None;
    let mut y: Option<i64> = None;
    let mut w: Option<i64> = None;
    let mut h: Option<i64> = None;

    for line in source.split("\n") {
        let line = line.trim();

        if line.starts_with(FIELD_X) {
            x = Some(line.trim_left_matches(FIELD_X).trim().parse::<i64>()?);
        } else if line.starts_with(FIELD_Y) {
            y = Some(line.trim_left_matches(FIELD_Y).trim().parse::<i64>()?);
        } else if line.starts_with(FIELD_WIDTH) {
            w = Some(line.trim_left_matches(FIELD_WIDTH).trim().parse::<i64>()?);
        } else if line.starts_with(FIELD_HEIGHT) {
            h = Some(line.trim_left_matches(FIELD_HEIGHT).trim().parse::<i64>()?);
        }
    }
    if x.is_none() {
        return Err(Box::new(FieldError::new(format!(
            "lack of `{}' field",
            FIELD_X
        ))));
    }
    if y.is_none() {
        return Err(Box::new(FieldError::new(format!(
            "lack of `{}' field",
            FIELD_Y
        ))));
    }
    if w.is_none() {
        return Err(Box::new(FieldError::new(format!(
            "lack of `{}' field",
            FIELD_WIDTH
        ))));
    }
    if h.is_none() {
        return Err(Box::new(FieldError::new(format!(
            "lack of `{}' field",
            FIELD_HEIGHT,
        ))));
    }
    return Ok(WindowInfo {
        x: x.unwrap(),
        y: y.unwrap(),
        width: w.unwrap(),
        height: h.unwrap(),
    });
}

#[derive(Debug, Clone)]
pub struct FieldError {
    message: String,
}

impl FieldError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for FieldError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "FieldError: {}", self.message)
    }
}

#[test]
fn test_parse_xwininfo_empty() {
    let src = "";
    let r = parse_xwininfo(src.to_string());
    assert!(r.is_err());
}

#[test]
fn test_parse_xwininfo() {
    let src = r#"
xwininfo: Please select the window about which you
          would like information by clicking the
          mouse in that window.

xwininfo: Window id: 0xe00021 "Firefox Developer Edition"

  Relative upper-left X:  0
  Relative upper-left Y:  0
  Width: 958
  Height: 1178
  Depth: 24
  Visual: 0x2b
  Visual Class: TrueColor
  Border width: 0
  Class: InputOutput
  Colormap: 0xe00002 (not installed)
  Bit Gravity State: NorthWestGravity
  Window Gravity State: NorthWestGravity
  Backing Store State: NotUseful
  Save Under State: no
  Map State: IsViewable
  Override Redirect State: no
  Corners:  +3841+21  -961+21  -961-961  +3841-961
  -geometry 958x1178+3840+20
"#;
    let r = parse_xwininfo(src.to_string());
    assert!(r.is_err());
}

#[test]
fn test_parse_xwininfo_ok() {
    let src = r#"
xwininfo: Please select the window about which you
          would like information by clicking the
          mouse in that window.

xwininfo: Window id: 0xe00021 "Firefox Developer Edition"

  Absolute upper-left X:  3841
  Absolute upper-left Y:  21
  Relative upper-left X:  0
  Relative upper-left Y:  0
  Width: 958
  Height: 1178
  Depth: 24
  Visual: 0x2b
  Visual Class: TrueColor
  Border width: 0
  Class: InputOutput
  Colormap: 0xe00002 (not installed)
  Bit Gravity State: NorthWestGravity
  Window Gravity State: NorthWestGravity
  Backing Store State: NotUseful
  Save Under State: no
  Map State: IsViewable
  Override Redirect State: no
  Corners:  +3841+21  -961+21  -961-961  +3841-961
  -geometry 958x1178+3840+20
"#;
    let r = parse_xwininfo(src.to_string());
    assert!(r.is_ok());

    let info = r.unwrap();
    assert_eq!(info.x, 3841);
    assert_eq!(info.y, 21);
    assert_eq!(info.width, 958);
    assert_eq!(info.height, 1178);
}
