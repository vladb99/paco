cargo run -- mandel.png 1000x750 -1.20,0.35 -1,0.20

# single-threaded
## Einsatz des Macros
*mesure!{sender, {
*   render(&mut pixels, bounds, upper_left, lower_right);
*}} 
## Laufzeit-Ausgaben
PID | Core | Runtime | Source
20494 3 4424682494 { render(&mut pixels, bounds, upper_left, lower_right); }
## Kurze Interpretation 
Es wurde nur eine Messung durchgefuehrt, weil diese Implementierung die gesamte Arbeit auf dem Hauptthread ausfuerht.
Das ist an der Funktion 'render' zu sehen, wo die ganze Berechnung der Mandelbrot-Menge stattfindet.
Hier sind zwei geschlachteten for-Schleifen, die auf dem gleichen Thread ausgefuehrt werden und nicht Multithreading ausnutzen.
D.h. das Hauptthread wurde nur auf einem Core ausgefuehrt (aus vier verfuegbaren Cores), was auch zu einer langsamen Zeit fuerht (4.4 Sekuden).

# bands
## Einsatz des Macros
183         mesure!{sender, {
184             let bands: Vec<&mut [u8]> =
185                 pixels.chunks_mut(rows_per_band * bounds.0).collect();
186             crossbeam::scope(|spawner| {
187                 for (i, band) in bands.into_iter().enumerate() {
188                     let top = rows_per_band * i;
189                     let height = band.len() / bounds.0;
190                     let band_bounds = (bounds.0, height);
191                     let band_upper_left =
192                         pixel_to_point(bounds, (0, top), upper_left, lower_right);
193                     let band_lower_right =
194                         pixel_to_point(bounds, (bounds.0, top + height),
195                                     upper_left, lower_right);
196                     let s = sender.clone();
197                     spawner.spawn(move |_| {
198                         mesure!{s, {
199                             render(band, band_bounds, band_upper_left, band_lower_right);
200                         }}
201                      });
202                 }
203             }).unwrap();
204         }}
## Laufzeit-Ausgaben
PID | Core | Runtime | Source
23602 2 1459873102 {
23710 3 1453444067 { render(band, band_bounds, band_upper_left, band_lower_right); }
23708 3 719127765 { render(band, band_bounds, band_upper_left, band_lower_right); }
23707 2 270627326 { render(band, band_bounds, band_upper_left, band_lower_right); }
23709 0 1156488360 { render(band, band_bounds, band_upper_left, band_lower_right); }
23711 1 1336902647 { render(band, band_bounds, band_upper_left, band_lower_right); }
23704 0 179303765 { render(band, band_bounds, band_upper_left, band_lower_right); }
23705 1 247746678 { render(band, band_bounds, band_upper_left, band_lower_right); }
23706 1 294877588 { render(band, band_bounds, band_upper_left, band_lower_right); }
## Kurze Interpretation
Es wurde einmal eine Messung um die ganze Schleife gemacht und eine Messung jeweils fuer einen Thread.
Die Bands-Implementierung verteilt die Arbeit unter 8 Threads.
Die Zeit mit ca. 1,5 Sekunden ist dabei deutlich schneller als bei der single-thread-Implementierung.
Aus der Laufzeit-Ausgaben kann man sehen, dass die Zeit jedes Threads nicht wirklich gleichmaessig verteilt ist.
Spaeter aufgetretene Threads wie 23709, 23710 und 23711 brauchen deutlich mehr Zeit, da sie auf die juengeren Threads noch warten muessen.
Bei mehreren Cores wuerde es wahrscheinlich besser aussehen, aber auf wenigere Cores wird die Zeit fuer die Arbeit nicht gleichmaessig aufgeteilt.

# task-queue
## Einsatz des Macros
183     //let (sender, receiver) = sync_channel(1);
184     let (sender, receiver) = channel();
185     let anna = Analyzer::new(receiver);
186     {
187         let bands = Mutex::new(pixels.chunks_mut(band_rows * bounds.0).enumerate());
188         crossbeam::scope(|scope| {
189             for _ in 0..threads {
190                 let s = sender.clone();
191                 //struct Wrap(RefCell<Sender<Mesurement>>);
192                 //unsafe impl Sync for Wrap{};
193                 //let s = Wrap(RefCell::new(sender.clone()));
194                 //let b = bands.clone();
195                 //let mut s = RefCell::new(sender.clone());
196                 //let s2 = &mut s;
197                 mesure!{s,{
198                 scope.spawn(|_| {
199                     loop {
200                         match {
201                             let mut guard = bands.lock().unwrap();
202                             guard.next()
203                         }
204                         {
205                             None => { return; }
206                             Some((i, band)) => {
207                                 let top = band_rows * i;
208                                 let height = band.len() / bounds.0;
209                                 let band_bounds = (bounds.0, height);
210                                 let band_upper_left = pixel_to_point(bounds, (0, top),
211                                                                      upper_left, lower_right);
212                                 let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
213                                                                       upper_left, lower_right);
214                                 //let test = &*s.0.borrow();
215                                 //mesure!{s, {
216                                 render(band, band_bounds, band_upper_left, band_lower_right);
217                                 //}}
218                             }
219                         }
220                     }
221                 });
222                }}
223             }
224         }).unwrap();
225     }
## Laufzeit-Ausgaben
PID | Core | Runtime | Source
28954 2 269895 {
## Kurze Interpretation
Die Laufzeit-Ausgabe bezieht sich auf dem mesure-macro aus Zeile 197.
Die Idee der task-queue-Implementierung ist, dasss nachdem ein Thread seine Aufgabe ausgefuehrt hat, soll er danach schauen ob er noch etwas machen kann. D.h. im Thread wird eine Schleife eingebaut.
Die Laufzeit die alle Threads zusammen gemacht ist deutlich schneller als die von den ersten Implementierungen. Das macht auch Sinn, da die Threads die Schneller fertig werden, auch weiter 'mithelfen' und nicht gleich beendet werden.

Den macro wollte ich eigentlich wieder um der render Methode rum haben, um genau zu sehen wie jeder Thread Aufgaben nimmt, diese ausfuehrt und dann sich eine neue Aufgabe holt.
Ich konnte den macro aber nicht an dieser Stelle machen, da ich es einfach nicht geschafft habe es zum Laufen zu bringen. Dabei hatte ich den folgenden Fehler nicht loesen koennen:
error[E0277]: `std::sync::mpsc::Sender<measurement::performance::mesure::Mesurement>` cannot be shared between threads safely
   --> src/main.rs:198:23
    |
198 |                 scope.spawn(|_| {
    |                       ^^^^^ `std::sync::mpsc::Sender<measurement::performance::mesure::Mesurement>` cannot be shared between threads safely
    |
    = help: the trait `Sync` is not implemented for `std::sync::mpsc::Sender<measurement::performance::mesure::Mesurement>`
    = note: required because of the requirements on the impl of `Send` for `&std::sync::mpsc::Sender<measurement::performance::mesure::Mesurement>`
    = note: required because it appears within the type `[closure@src/main.rs:198:29: 221:18]`

Aus der Fehleranzeige kann man sehen, dass der Sender nicht die Trait Sync implementiert. Ich glaube es braucht die Sync Trait, um versichern zu koennen dass eine Referenz auf Sender auf sichere Weise in mehreren Threads gleichzeitig verwendet werden kann.
In Zeile 198 wird das Thread erstellt und das Closure des Threads wird nicht gezwungen Ownership ueber die 'eingefangenen' Variablen zu nehmen, dass heisst die Closure arbeitet mit Referenzen.

Ich habe dabei mehrere Sachen ausprobiert um dieses Problem zu loesen, war aber nicht erfolgreich. In dem obigen Code kann man in den Kommentaren sehen, was ich so ausprobiert habe. Darunter zaehlt:
- die Closure des Threads mit 'move' zu zwingen Ownership zu nehmen, also 'scope.spawn(move |_| {'. Das hat nicht funktoniert, da die 'bands' Variable im Programm so erstellt wurde, dass mehrere Threads dadrauf zugreifen und es wuerde nicht funktionieren jedem Thread die Ownership von 'bands' zu geben. 
- Selber eine Dummy-unsafe Implementierung von Sync mit Wrap Struct und RefCell fuer Sender zu implementieren und versuchen den Rust Compiler zu 'tauschen', dass Sender doch Sync implementiert und sicher ist.
- Habe gelesen, dass es auch die Methode 'sync_channel' gibt, die dabei einen normalen Receiver und dabei einen SyncSender erstellt. Der SyncSender implementiert die Sync Trait, aber arbeitet dann mit einem Buffer, der kontrolliert wann gesendet werden kann. Habe versucht auch das zu implementieren, aber kam dann auf andere Probleme mit der schon implementieren analyze.rs und mesure.rs

# lockfree
## Einsatz des Macros
181     let (sender, receiver) = channel();
182     let analyzer = Analyzer::new(receiver);
183     {
184         let bands = AtomicChunksMut::new(&mut pixels, rows_per_band * bounds.0);
185         crossbeam::scope(|scope| {
186             let s = sender.clone();
187             mesure!{s, {
188             for _ in 0..threads {
189                 scope.spawn(|| {
190                         for (i, band) in &bands {
191                             let top = i * rows_per_band;
192                             let height = band.len() / bounds.0;
193                             let band_bounds = (bounds.0, height);
194                             let band_upper_left = pixel_to_point(bounds, (0, top),
195                                                                  upper_left, lower_right);
196                             let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
197                                                                   upper_left, lower_right);
198                             //mesure!{s, {
199                             render(band, band_bounds, band_upper_left, band_lower_right);
200                         //}}
201                         }
202                 });     
203             }   
204             }}
205         }); 
206     }   

## Laufzeit
PID | Core | Runtime | Source
29295 2 340121 {
## Kurze Interpretation
Aehnlich wie bei der task-queue-Implementierung, wollte ich wieder den mesure! macro um dem Render Befehl rum machen, aber habe dabei den gleichen Fehler bekommen und konnte ihn nicht loesen.

Die Implementierung an sich ist aehnlich wie bei task-queue. Nachdem ein Thread mit einer render-Aufgabe fertig ist, schaut es ob es mehr render-Aufgabe verfuegbar sind, und fuehrt diese eventuell aus.
Anhand der Laufzeit, kann man auch sehen dass die Leistung auch auehnlich wie bei task-queue ist.
Aus der Gesamtlaufzeit koennte man sagen, dass es nicht wirklich ein grosser Unterschied zu task-queue gibt.
Im Prinzip liegt der Unterschied nur daran, wie es mit dem 'bands' umgegangen wird.

# rayon
## Einsatz des Macros
182     {
183         let bands: Vec<(usize, &mut [u8])> = pixels
184             .chunks_mut(bounds.0)
185             .enumerate()
186             .collect();
187 
188         let s = sender.clone();
189         mesure!{s, {
190         bands.into_par_iter()
191             .for_each(|(i, band)| {
192                 let top = i;
193                 let band_bounds = (bounds.0, 1);
194                 let band_upper_left = pixel_to_point(bounds, (0, top),
195                                                      upper_left, lower_right); 
196                 let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1),
197                                                       upper_left, lower_right);
198                 //let s = sender.clone();
199                 //mesure!{s, {
200                     render(band, band_bounds, band_upper_left, band_lower_right);
201                 //}}
202             });
203         }}
204     }
## Laufzeit
PID | Core | Runtime | Source
31454 2 1200443696 {
## Kurze Interpretation
Aehnlich wie bei den letzten zwei Implementierungen, wusste ich nicht wie ich diesen Fehler loesen koennte.

Ich vermute dass die Threadlogik sich im Iterator 'versteckt' befindet.
Beurteilend auf der Laufzeit, sieht es ziemlich langsam aus, im Vergleich mit den anderen Implementierungen. Ich haette erwartet, dass es gleich schnell oder schneller waere, da es nach der gleichen Idee mit mehreren Bands arbeitet und das Multithreading auch sinnvoll einsetzt.
Im Vergleich mit den anderen Implementierungen, sieht die Rust-Umsetzung aber sehr einfach aus.
