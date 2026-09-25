//! Modul msa_parser dla skrzynki m2-client-granny
//! Parser skryptow animacji Metin2 .msa (czasy uderzen StartingTime)
//! Odczyt MotionDuration, Accumulation, MotionEventData, dokladnego czasu ciosu StartingTime

#[derive(Debug, Clone, PartialEq)]
pub struct MotionEventData {
    pub starting_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsaData {
    pub motion_duration: f32,
    pub accumulation: (f32, f32, f32),
    pub motion_event_data: Vec<MotionEventData>,
}

#[derive(Debug, Clone, Default)]
pub struct MsaParser {
    pub initialized: bool,
}

#[derive(Debug, PartialEq)]
pub enum MsaError {
    InvalidFormat,
    ParseFloatError,
    MissingField,
}

impl MsaParser {
    /// Tworzy nowa instancje parsera.
    pub fn new() -> Self {
        Self { initialized: true }
    }

    /// Sprawdza czy parser jest gotowy do uzycia.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Parsuje zawartosc pliku .msa i zwraca dane w strukturze MsaData.
    pub fn parse(&self, content: &str) -> Result<MsaData, MsaError> {
        let mut motion_duration = None;
        let mut accumulation = None;
        let mut motion_event_data = Vec::new();

        let mut lines = content.lines();

        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap_or("");

            match key {
                "MotionDuration" => {
                    let val = parts.next().ok_or(MsaError::InvalidFormat)?;
                    let parsed: f32 = val.parse().map_err(|_| MsaError::ParseFloatError)?;
                    motion_duration = Some(parsed);
                }
                "Accumulation" => {
                    let x_str = parts.next().ok_or(MsaError::InvalidFormat)?;
                    let y_str = parts.next().ok_or(MsaError::InvalidFormat)?;
                    let z_str = parts.next().ok_or(MsaError::InvalidFormat)?;
                    let x: f32 = x_str.parse().map_err(|_| MsaError::ParseFloatError)?;
                    let y: f32 = y_str.parse().map_err(|_| MsaError::ParseFloatError)?;
                    let z: f32 = z_str.parse().map_err(|_| MsaError::ParseFloatError)?;
                    accumulation = Some((x, y, z));
                }
                "StartingTime" => {
                    let val = parts.next().ok_or(MsaError::InvalidFormat)?;
                    let parsed: f32 = val.parse().map_err(|_| MsaError::ParseFloatError)?;
                    motion_event_data.push(MotionEventData {
                        starting_time: parsed,
                    });
                }
                _ => {}
            }
        }

        Ok(MsaData {
            motion_duration: motion_duration.ok_or(MsaError::MissingField)?,
            accumulation: accumulation.ok_or(MsaError::MissingField)?,
            motion_event_data,
        })
    }
}
