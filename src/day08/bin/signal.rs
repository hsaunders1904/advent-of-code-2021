#[derive(Debug)]
pub struct Signal {
    pub signals: [String; 10],
    pub outputs: [String; 4],
}

impl Signal {
    pub fn new(str_rep: &str) -> Result<Signal, String> {
        let mut new_signal = Signal {
            signals: Default::default(),
            outputs: Default::default(),
        };

        let mut signal_types = str_rep.split(" | ");

        let signal_patterns: Vec<&str> = signal_types.next().unwrap().split(" ").collect();
        if signal_patterns.len() != 10 {
            return Err(format!(
                "10 signal patterns required. Found '{}'",
                signal_patterns.len()
            ));
        }
        for (idx, pattern) in signal_patterns.iter().enumerate() {
            new_signal.signals[idx] = String::from(*pattern);
        }

        match signal_types.next() {
            None => {
                return Err(String::from(
                    "No ' | ' found dividing signal patterns and outputs.",
                ))
            }
            Some(st) => {
                let signal_outputs: Vec<&str> = st.split(" ").collect();
                if signal_outputs.len() != 4 {
                    return Err(format!(
                        "4 signal outputs required. Found '{}'",
                        signal_outputs.len()
                    ));
                }
                for (idx, pattern) in signal_outputs.iter().enumerate() {
                    new_signal.outputs[idx] = String::from(*pattern);
                }
            }
        }
        return Ok(new_signal);
    }
}
