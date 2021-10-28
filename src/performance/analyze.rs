use super::super::util::ui::*;
use super::mesure::*;
use std::collections::HashMap;
use std::fmt;
use std::sync::mpsc::Receiver;
use std::thread;
use tui::style::Color;
pub struct Analyze {
    pub pid: u64,
    // Source-Code, Startzeit und Laufzeit
    pub runtime_expression: HashMap<Source, Vec<(u128, u128)>>,
    // Kern Affinitaet zur Startzeit
    pub core_affinity: Vec<(u128, i32)>,
    raw_data: HashMap<Source, Vec<Mesurement>>,
    sequences: HashMap<Sequence, u64>,
}
impl Analyze {
    pub fn new(pid: u64) -> Analyze {
        Analyze {
            pid: pid,
            runtime_expression: HashMap::new(),
            core_affinity: Vec::new(),
            raw_data: HashMap::new(),
            sequences: HashMap::new(),
        }
    }
    pub fn add(&mut self, mesure: Mesurement) {
        match self.sequences.get_mut(&mesure.sequence) {
            Some(v) => *v += 1,
            _ => {
                self.sequences.insert(mesure.sequence, 1);
                ()
            }
        };
        self.core_affinity.push((mesure.time_ns, mesure.core));
        if !self.raw_data.contains_key(&mesure.source) {
            self.raw_data.insert(mesure.source, Vec::new());
        }
        self.raw_data.get_mut(&mesure.source).unwrap().push(mesure);
    }
}
pub struct Analyzer {
    // Zuordnung(Key) der Messungen via der Thread pid
    pub analyze: HashMap<u64, Analyze>,
    // dient zur einfachen Bestimmung der Zeitspanne
    first_time_stamp: u128,
    last_time_stamp: u128,
}
impl Analyzer {
    pub fn new(recv: Receiver<Mesurement>) -> thread::JoinHandle<Analyzer> {
        /*TODO
         *Erzeugen Sie einen Thread der ueber den Receiver die
         *Mesurements entgegen nimmt ggf. Analyze-Structs erstellt
         *und diese in die this.HashMap einordnet.
         *Verwenden Sie hierfuer new und add von dem Analyze-Struct.
         */
    }
    pub fn empty() -> Analyzer {
        Analyzer {
            analyze: HashMap::new(),
            first_time_stamp: 0,
            last_time_stamp: 0,
        }
    }
    /*TODO
     *pub fn print(mut self) -> String
     *Erstellen Sie eine Analyzer-Print-Funktion,
     *welche die erhaltenen Messungen mittels der
     *analyze-Funktion auswertet und diese
     *als eine Art Tabelle ausgibt.
     */
    fn analyze(&mut self) -> Result<Analyzer, String> {
        let mut aner = Analyzer::empty();

        for (pid, anna) in self.analyze.iter() {
            if anna.sequences.get(&Sequence::PRE) != anna.sequences.get(&Sequence::POST) {
                return Result::Err(format!("sequence missing for pid: {}", pid));
            }
            let mut new_anna = Analyze::new(*pid);
            for (src, mesure_vec) in anna.raw_data.iter() {
                for pre_m in mesure_vec {
                    if aner.first_time_stamp == 0 || aner.first_time_stamp > pre_m.time_ns {
                        aner.first_time_stamp = pre_m.time_ns;
                    }
                    if aner.last_time_stamp == 0 || aner.last_time_stamp < pre_m.time_ns {
                        aner.last_time_stamp = pre_m.time_ns;
                    }
                    if pre_m.sequence == Sequence::PRE {
                        for pos_m in mesure_vec {
                            if pre_m.is_pair(pos_m) {
                                let runtime_ns = pos_m.time_ns - pre_m.time_ns;
                                if !new_anna.runtime_expression.contains_key(src) {
                                    new_anna.runtime_expression.insert(*src, Vec::new());
                                }
                                new_anna
                                    .runtime_expression
                                    .get_mut(src)
                                    .unwrap()
                                    .push((pre_m.time_ns, runtime_ns));
                            }
                        }
                    }
                    new_anna.core_affinity.push((pre_m.time_ns, pre_m.core));
                }
            }
            aner.analyze.insert(*pid, new_anna);
        }
        Ok(aner)
    }
    pub fn plot(mut self) {
        let anner = self.analyze().unwrap();
        let mut pid_color_pick = 0;
        let pid_color: Vec<Color> = vec![
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::Gray,
            Color::DarkGray,
            Color::LightRed,
            Color::LightGreen,
            Color::LightYellow,
            Color::LightBlue,
            Color::LightMagenta,
            Color::LightCyan,
        ];
        let mut vec_gd_core_affinity: Vec<GraphData> = Vec::new();
        let mut vec_gd_runtime: Vec<GraphData> = Vec::new();

        for (pid, anna) in anner.analyze.iter() {
            let mut points: Vec<(f64, f64)> = Vec::new();
            for (time_ns, core) in anna.core_affinity.iter() {
                let time_diff = time_ns - anner.first_time_stamp;
                let xs = time_diff as f64;
                let ys = *core as f64;
                if points.len() > 0 && points[points.len() - 1].1 != ys {
                    points.push((xs, points[points.len() - 1].1));
                }
                points.push((xs, ys));
            }
            vec_gd_core_affinity.push(GraphData {
                label: pid.to_string(),
                color: pid_color[pid_color_pick],
                data: points.clone(),
            });
            for (src, time_vec) in anna.runtime_expression.iter() {
                let mut max_runtime_ns = 0.;
                let mut points: Vec<(f64, f64)> = Vec::new();
                for (start_time_ns, runtime_ns) in time_vec {
                    let time_diff = start_time_ns - anner.first_time_stamp;
                    let xs = time_diff as f64;
                    let ys = *runtime_ns as f64;
                    if ys > max_runtime_ns {
                        max_runtime_ns = ys;
                    }
                    points.push((xs, 0.));
                    points.push((xs, ys));
                    points.push((xs + ys, ys));
                    points.push((xs + ys, 0.));
                }
                vec_gd_runtime.push(GraphData {
                    label: pid.to_string() + ", " +&src.to_string(),
                    color: pid_color[pid_color_pick],
                    data: points.clone(),
                });
            }
            pid_color_pick += 1;
            if pid_color_pick == pid_color.len() {
                pid_color_pick = 0;
            }
        }
        let ui = UI::new();
        let core_plotdata = PlotData {
            chart_title: String::from("core_affinity"),
            chart_y_title: String::from("core"),
            chart_x_title: String::from("ns"),
            layout: ui.layout_core_affinity.clone(),
            graphdata: vec_gd_core_affinity,
            paragraph_title: String::from("pid"),
        };
        let runtime_plotdata = PlotData {
            chart_title: String::from("runtime"),
            chart_y_title: String::from("ns"),
            chart_x_title: String::from("ns"),
            layout: ui.layout_runtime.clone(),
            graphdata: vec_gd_runtime,
            paragraph_title: String::from("pid, src"),
        };
        ui.plot(vec![core_plotdata,runtime_plotdata]);
    }
}
impl fmt::Display for Analyzer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unique pids:{pids}", pids = self.analyze.len())
    }
}
