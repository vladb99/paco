# Beschreibung des umgesetzten Algorithmus
Der Algorithmus besteht nun aus drei Teile. Das Laden der Frames, die Erkennung der Autos und die Zuordnung bzw. das Zählen der Autos in den jeweiligen Spuren. Jeden Teil habe ich versucht zu parallelisieren.

Das Laden der Frames wird durchgeführt mit einem frame-Skipping von 20 Frames. Dabei habe ich den parallelen Iterator aus rayon verwendet, um über das Video zu gehen. Dabei wird jeder Frame-Index auf einem Tuple aus dem Frame-Index selbst und das Bild in Graustufe gemappt. Diese Tupeln werden am Schluss als Liste zusammengefügt und mithilfe der **collect()** Methode. Ich habe mich für das entschieden, weil auf diese Weise braucht man die Liste wo die Frames abgespeichert werden kein Mutex, damit man aus mehreren Threads sicher dadrauf schreiben kann. Außerdem öffne ich in diesem Teil immer das Video neu, um ein Frame zu lesen. Das hat sich aus meine Versuche schneller ergeben, als das Video in einem **Arc::new(Mutex::new())** zu packen und daraus dann zu lesen.

Als nächstes kommt der Teil wo die Autos erkannt werden. Analog zum ersten Teil, verwende ich hier auch den parallelen Iterator. Am Schluss wird eine Liste aus Tupeln mit dem Frame-Index und die erkannten Autos zurückgegeben. Dabei läuft der parallele Iterator über jedes Frame und erkennt Objekte aus dem Frame. Dabei werden die Objekte gefiltert, die sich nicht im unteren Bereich des Frames befinden.

Um die Verwendung von Mutex auf die fünf Variablen für die Spurzählung zu vermeiden, habe ich hier versucht auch das gleiche Konzept wie bei der zwei anderen Teile anzuwenden. Dabei geht der parallele Iterator über die ganze Liste die aus dem zweiten Teil erstellt wurde.
Der Algorithmus den ich verwendet habe um die Autos in den Frames zuzuornden sieht folgerdermaße aus:
Jeder Task schleift durch die Liste von Objekte die auf einem Frame erkannt wurden. Dabei gibt es nochmal eine Schleife, die über die erkannten Autos aus dem nächstliegenden Frame geht (für ein Frame-Skipping von 20 Frames reicht es sich nur zwei Tupeln anzuschauen). D.h. die Differenz zum nächsten Frame soll ja dem Frame Skipping entsprechen. Dabei werden die Objekte aus dem ursprünglichen und nächstliegenden Frame auf die Position und in welche Spur sie sich befinden verglichen. Für die ersten zwei Spuren wird zum Beispiel auch auf das y geachtet, da man weiß dass dort die Autos auf jeden Fall von oben nach unten gehen. Das analog auch für die letzten drei
Spuren. Übereinstimmt diese Erkennungslogik, dann wird eine lokale Variable für die Spur inkrementiert. Zum Schluss liefert jeder Task ein Tupel mit allen Zählvariablen zurück. Dieses Tupel wird zusammen mit den aus den anderen Tasks vom parallelen Iterator zusammengefügt.

Am Schluss geht man sequentiell über alle Zählvariablen aus den unterschiedlichen Tasks und summiert diese zusammen, was dann als Text in die Konsole ausgeliefert wird.

## Zerlegungsmethode und Programmiermodelle
Dabei wurde eine statische Zerleung der Tasks verwendet, da schon am Anfang ein Frame-Skipping definiert wird. Das unterteilt das Problem schon davor in eine festgesetzte Anzahl an Tasks. Zum Beispiel das Detektieren der Objekte im Video wird durch den parallenen Iterator in einzelne Tasks zerlegt.
Die Programmieremodelle die ich verwendet habe, beziehen sich auf einem gemeinsamen Speicher. Dazu gehört zum Beispiel das Lesen einer gemeinsamen Liste bzw. das Schreiben zu einer geminsamen Liste.

## Wie sieht der Datenfluss aus?
Jeden von den drei Teile (Laden der Frames, Erkennung der Objekte und Zuweisung der Autos) die ich davor beschrieben habe, erstellt eine Liste als Produkt. Dabei ist jeder Teil des Algorithmus abhängig von einer Liste die von davorigen Teil erstellt wurde.

## Kategorisieren der Verteilung, arbeiten die Threads kooperativ?
Nein, sie arbeiten nicht kooperativ.

## Wie sieht die Thread Kommunikation aus? Ist sie nachrichtenbasiert?
Nein, die Kommunikation ist nicht nachrichtenbasiert. Dabei schreiben die Threads auf die gleiche Liste und somit ist keine Kommunikation notwendig. Ich war am Überlegen, ob eine Pipeline-Implentierung (send und receive) sinnvoll wäre. Ich habe aber aus der letzten Aufgabe (Damenproblem) rausgefunden, dass diese Umsetzung eigentlich langsamer ist, als wenn jeder Thread (Task) selber ein Ergebnis zurückliefert.

## Welcher Teil ist rein sequentiell?
Der Teil der die Zählvariablen aus dem unterschiedlichen Tasks aufsummiert, arbeitet rein sequentiell.

## Abschnitte im Code mit Mutex dass nur sequentiell laufen kann?
In dem Teil der sich mit der Zurordnung der Autos beschäftigt, musste ich ein Mutex verwenden. Dabei wurde die Liste mit den erkannten Objekte aus den Frames mit einem parallelen Iterator iteriert. Der Algorithmus den ich implementiert habe, musste dann die gleiche Liste nochmal iterieren. Um das in Rust zu compilen zu können, muss die Liste in einem Mutex umgewrappt werden. Aus diesem Grund kann nur ein Task zu einem Zeitpunkt durch diese Liste durchschleifen.

## Warum habe ich mich für Darstellung entschieden (Abstraktionsmodell)?
Ich habe mich für diesen Programmablauf entschieden, weil es eingentlich sehr wenig mit Synchronizationsvariablen (Mutex) zu tun hat. Dadurch dachte ich, dass sich auf diese Weise eine schnelle Zeit erzielen lässt, weil Threads an sich nicht durch das Mutex blockiert werden. Das lässt sich gut mit dem parallelen Iterator vom rayon umsetzen. Dabei habe ich die Elemente einer Liste mit dem parallelen Iterator auf ein Ergebnis gemappt und am Schluss alle Ergebnisse der Tasks zu einer Liste aufsummiert.

# Messung der Gesamtlaufzeiten
* Messung paralleles Programm:
	*  gesamtzeit = $T_4(n) = 32,83 s$
* Messung sequentielles Programm, wobei dafür das parallel Programm verwendet wurde, mit einer Threadseinschränkung von 1:
	* **set_var("RAYON_NUM_THREADS", "1");**
	*  gesamtzeit =  $T'(n) = 50,69 s$
* Zeitdifferenz = 17.86 Sekunden

# Berechnung des theoretischen Speedups
Mithilfe des Speedups wird die Reduktion der Laufzeit für das Gesamtproblem bei einer Parallelisierung angegeben. Der Speedup lässt sich als das Verhältnis zwischen die Laufzeit des schnellsten bekannten sequentiellen Algorithmus $T'(n)$ und die Laufzeit des parallen Programms auf $p$ Prozessore $T_p(n)$. Die Messungen wurden auf dem zur Verfügung gestellten Runner ausgeführt. Für dieser gilt $p = 4$, wobei diese Angabe auf die Anzahl der logischen Kerne bezogen ist.
$S_4(n) = \frac{T'(n)}{T_4(n)}$
$S_4(n) = \frac{50.69s}{32.83s} = 1.544$

# Berechnung der Overhead-Zeit
Zuerst müssen die Kosten des parallelen Programms berechnet werden, d.h. die Arbeit von allen Prozessoren bei der Durchführung der Problemlösung. Hier gilt auch die Angabe $p=4$ für 4 logische Prozessoren.
$C_p(n) = T_p(n)\cdot p$
$C_4(n) = T_4(n)\cdot 4 = 32,83s \cdot 4 = 131,32s$
Der Overhead gibt die Differenz zwischen den Kosten des parallelen Programms und des sequentiellen Programms an. Dieser wird wie folgend berechnet:
$H_4(n) = C_4(n) - T'(n) = 131,32s - 50,69s = 80,63s$

## Wodurch wird dieser Overhead verursacht?
Ein Grund das ich mir vorstellen könnte, wäre die Verwendung eines Mutex, für die Thread-Kommunikation. Bei meinem Algorithmus wurde für den Teil der die Autos zuordnet, ein Mutex verwendet, damit alle Threads eine Liste durchschleifen können. Um sicherzustellen, dass alle Threads dieselben Daten sehen, muss eine Art von Locking durchgeführt werden, was Kosten als Overhead verursacht.

Eine andere mögliche Ursache wäre die Erstellung und Terminierung von Threads, wobei durch die Verwendung von rayon und somit von einem Threadpool (Gruppe von Threads die vorinstanziert sind), werden diese Kosten schon reduziert.

Außerdem hätte ich gedacht, dass die Kontextwechsel auch zum Overhead etwas hinzufügen. Für die Messung des sequentiellen Programms wurde eine Anzahl an freiwillige Kontextwechsel von 145953 gemessen. Beim parallelen Programm ist diese Anzahl deutlich größer, mit 255789. Aus diesem Grund hätte ich gesagt, dass je größer die Anzahl an Kontextwechsel ist, desto größer auch der Overhead.

# Effizienz der Verarbeitungsgeschwindigkeit
Interessant zu wissen, wäre auch die Effizienz des parallelen Programms, da diese die Zusatzlast und Redundanz bewertet. Diese lässt sich mit dem Speedup berechnet werden, wobei der Speedup mit die Anzahl der Prozessoren normiert wird.
$E_4(n) = \frac{S_4(n)}{4} = \frac{1.544}{4} = 0,386$
In diesem Fall $E_4(n) < 1$ gilt die Effizienz des Algorithmus als suboptimal, was in der Praxis der Normalfall ist, wobei nähere Werte an 1 erwünscht sind. In meinem Fall hätte ich dieses Ergebnis so interpretiert, dass der oben beschriebene Overhead die Vorteile der Parallelität überwiegt, was zu einer suboptimalen Effizienz führt.

## Wäre es möglich bei meiner Implementierung die Laufzeiterhöhung, verursacht durch ein längeres Video mit mehr Prozessoren, zu vermeiden? (Gustafsons Gesetz)
Die Verringerung des sequentiellen Anteils bei größeren Problemgrößen und Erhöhung der Anzahl der Prozessoren 𝑝 lässt sich durch folgende Formel ausdrücken:
$f=\frac{f_1}{p\cdot(1-f_1) + f_1}$
Dabei ist $f_1$ der sequentielle Anteil innerhalb der Parallelisierung. Ich war mir nicht sicher wie ich diesen angeben soll, deshalb habe ich ihn auf $\frac{1}{3}$ festgelegt, weil in einem der drei Teile des parallelisierten Programms, ein Mutex verwendet wurde.
Wenn $f$ in die Formel des Speedups eingesetzt wird, dann ergibt sich die folgende Darstellung:
$S_p(n)=p\cdot(1-f_1)+f_1$
Dabei habe ich eine Funktion aufgestellt und einen Graph gezeichnet. Dieser kann als Bild im Ordner gefunden werden.
