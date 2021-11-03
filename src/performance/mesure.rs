#![macro_use]
use std::fmt;
extern crate palaver;
use palaver::thread::gettid;
use std::time::{SystemTime, UNIX_EPOCH};
extern crate libc;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Sequence {
    POST,
    PRE,
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match *self {
            Sequence::POST => "POST",
            Sequence::PRE => "PRE",
        };
        write!(f, "{}", ret)
    }
}

#[derive(Clone, Copy, Hash, Eq)]
pub struct Source {
    pub column: u32,
    pub line: u32,
    pub file: &'static str,
    pub expression: &'static str,
}
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expression: {expression}\nfile: [{file} l{line}:c{column}]",
            expression = self.expression,
            file = self.file,
            line = self.line,
            column = self.column,
        )
    }
}
impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        let own_ex: String = self.expression.split_whitespace().collect();
        let other_ex: String = other.expression.split_whitespace().collect();
        own_ex == other_ex
    }
}
pub struct Mesurement {
    // System-Pid des Threads
    pub pid: u64,
    // Kern auf dem der Thread lief
    pub core: i32,
    // Laufzeit in ns
    pub time_ns: u128,
    // pre oder post der Ausfuehrung
    pub sequence: Sequence,
    // Inhalt des Kontextes
    pub source: Source,
    // eindeutige ID um pre post Paare zu finden
    pub pair_identifier: u128,
}
impl fmt::Display for Mesurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pid: {pid}\n\tcore: {core}\n\tsource: {source}\n\ttime_ns: {time_ns} [{sequence}]",
            pid = self.pid,
            core = self.core,
            source = self.source,
            time_ns = self.time_ns,
            sequence = self.sequence
        )
    }
}
impl Mesurement {
    pub fn new(source: Source, sequence: Sequence, pair_identifier: u128) -> Option<Mesurement> {
        let tid = gettid();
        let time_ns = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_nanos(),
            Err(_) => 666,
        };
        Some(Mesurement {
            pid: tid,
            core: unsafe { libc::sched_getcpu() },
            source: source,
            time_ns: time_ns,
            sequence: sequence,
            pair_identifier: pair_identifier,
        })
    }
    pub fn is_pair(&self, other: &Mesurement) -> bool {
        self.source == other.source
            && self.pair_identifier == other.pair_identifier
            && self.sequence != other.sequence
    }
}
#[macro_export]
macro_rules! mesure {
    ($c:ident, $e:expr) => {{
        use std::sync::mpsc::Sender;
        use std::time::{SystemTime, UNIX_EPOCH};
        fn _mesure(src: Source, sequence: Sequence, c: Sender<Mesurement>, pair_identifier: u128) {
            /*TODO
             *Erzeuge hier eine neue Mesurement und uebertrage diese
             *an den Analyzer-Thread via dem Channel, welcher
             *dem Macro uebergeben wurde. (siehe Zeitstempel $c)
             */
            let measurement = Mesurement::new(src, sequence, pair_identifier).unwrap();
            c.send(measurement).unwrap();
        }
        let pair_identifier = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_nanos(),
            Err(_) => 666,
        };
        let src = Source {
            file: file!(),
            line: line!(),
            column: column!(),
            expression: stringify!($e),
        };
        // Zeitstempel vor der Funktion
        _mesure(src, Sequence::PRE, $c.clone(), pair_identifier);
        // Ausfuehrung des gegeben Kontextes
        let ret_val = $e;
        // Zeitstempel nach der Funktion
        _mesure(src, Sequence::POST, $c.clone(), pair_identifier);
        ret_val
    }};
}
