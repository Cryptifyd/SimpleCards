
# 🏠 Einstiegspunkt: Dashboard (Projektübersicht)

## 🎯 Zweck

Das Dashboard ist die erste Ansicht nach dem Login. Es zeigt dem Nutzer alle Projekte, auf die er Zugriff hat, gegliedert nach Teams. Es dient als zentraler Einstiegspunkt, um schnell zu einem Board, Projekt oder Task zu springen.

## 📦 Inhalt & Komponenten

| Element               | Beschreibung                                                                                                   |
| --------------------- | -------------------------------------------------------------------------------------------------------------- |
| **Teamüberschriften** | Gruppieren die Projekte logisch                                                                                |
| **Projektkarten**     | Jede Karte zeigt ein Projekt mit Titel, Kurzbeschreibung, Teamzugehörigkeit, Projektfarbe, letzten Aktivitäten |
| **Schnellaktionen**   | Buttons für "Neues Projekt", "Neues Team" (abhängig von Rolle)                                                 |
| **Suchleiste**        | Volltextsuche über alle sichtbaren Projekte                                                                    |
| **Projektfavoriten**  | Optional oben als kleine Kacheln fixiert                                                                       |
| **Responsives Grid**  | Auf Desktop 3–4 Spalten, auf Mobil 1–2                                                                         |

## 🖱️ Interaktionen

* Klick auf eine Projektkarte → Öffnet letztes Board oder die Board-Auswahl
* Hover auf Projekt → zeigt Kontextmenü: Umbenennen, Löschen (nur mit Rechten)
* Suchleiste filtert dynamisch
* Rechteabhängige Sichtbarkeit von Buttons

## 🎨 Styling & Anmutung

* **Modern, luftig**, ähnlich Linear, mit Tailwind oder vergleichbarem System
* **Projektkarten**: Abgerundete Kacheln, sanfte Schatten, Primärfarbe pro Projekt
* **Teamüberschriften** sticky am oberen Rand beim Scrollen
* Schriftgröße: Headline `xl`, Projektname `base` bis `lg`, subtile Meta-Infos `sm` mit geringer Deckkraft
* Farben orientieren sich an Projektfarben & UI-Tokens
* **Dark Mode ready**

## 🔐 Rechtesystem

* Nur sichtbare Projekte und Teams werden gezeigt
* Nur Admins sehen "Neues Team" und "Projekt löschen"
* Gäste sehen nur lesbare Projekte


---

# 🧩 Modul: Projekte & Teams verwalten

## 🎯 Zweck

Strukturierung der Arbeit in logischen Einheiten:

* **Teams** gruppieren Nutzer & Projekte
* **Projekte** enthalten Aufgaben, Boards, Epics
* Berechtigungen werden auf Team- oder Projektebene vergeben

---

## 📦 Inhalt & Komponenten

### 🔹 Teamverwaltung (optional sichtbar, z. B. für Admins)

| Element                         | Beschreibung                                  |
| ------------------------------- | --------------------------------------------- |
| **Teamliste**                   | Alle Teams, bei denen der Nutzer Mitglied ist |
| **Teamdetails (Sidebar/Modal)** | Name, Beschreibung, Mitglieder, Rollen        |
| **Mitgliederliste**             | Tabelle mit Name, Rolle, Einladung/Entfernung |
| **Button: Team erstellen**      | Sichtbar für Admins                           |

### 🔹 Projektverwaltung

| Element                             | Beschreibung                                                          |
| ----------------------------------- | --------------------------------------------------------------------- |
| **Projektkarte (wie im Dashboard)** | Titel, Beschreibung, Teamzugehörigkeit, Ersteller                     |
| **Projektdetails (Modal/Sidebar)**  | Umbenennen, Farbe ändern, Projekt löschen                             |
| **Projekt erstellen (Modal)**       | Titel, Beschreibung, Teamzuordnung, Standard-Board-Template auswählen |
| **Projektrechte**                   | Rollen pro Nutzer: Admin, Mitglied, Gast (nur lesend)                 |

---

## 🖱️ Interaktionen

* **Team erstellen** → öffnet Modal: Name, Beschreibung
* **Nutzer einladen** → per E-Mail oder Benutzersuche
* **Rollen ändern/löschen** → direkt in der Mitgliederliste
* **Projekt erstellen** → bei Auswahl eines Teams oder global
* **Rechte ändern** → im Projekt-Kontextmenü oder Modal
* **Filter**: Meine Projekte / Alle Projekte / Nach Team filtern

---

## 🎨 Styling & Anmutung

* **Einheitliches Design mit dem Dashboard**
* Modale Fenster mit klarer Trennung in Sektionen: Titel, Metadaten, Aktionen
* Farbige Team-Icons oder Initialen
* Hover-Interaktionen für Aktionen (z. B. Benutzer entfernen)
* Drop-down oder Dialoge für Rollenwechsel
* Visuelles Feedback bei Einladungen, z. B. via Toaster oben rechts

---

## 🔐 Rechtesystem

| Berechtigung                             | Admin | Mitglied | Bearbeiter | Gast |
| ---------------------------------------- | :---: | :------: | :--------: | :--: |
| Projekt erstellen                        |   ✅   |     ✅    |      ❌     |   ❌  |
| Projekt bearbeiten / löschen             |   ✅   |     ❌    |      ❌     |   ❌  |
| Board konfigurieren (groupBy, Status, …) |   ✅   |     ❌    |      ❌     |   ❌  |
| Aufgabe erstellen                        |   ✅   |     ✅    |      ✅     |   ❌  |
| Aufgabe bearbeiten                       |   ✅   |     ✅    |      ✅     |   ❌  |
| Aufgabe archivieren                      |   ✅   |     ✅    |      ✅     |   ❌  |
| Aufgabe löschen                          |   ✅   |     ✅    |      ❌     |   ❌  |
| Kommentare schreiben                     |   ✅   |     ✅    |      ✅     |   ❌  |
| Epics erstellen                          |   ✅   |     ✅    |      ❌     |   ❌  |
| Rechtemanagement                         |   ✅   |     ❌    |      ❌     |   ❌  |
| Projekte ansehen                         |   ✅   |     ✅    |      ✅     |   ✅  |
| Boards & Aufgaben ansehen                |   ✅   |     ✅    |      ✅     |   ✅  |


---

## 📌 Neue Architektur-Konzepte

### 1. 🔭 **Boards = Ansichten (nicht Daten!)**

Boards sind rein visuelle Konfigurationen:

* Filter: Welche Aufgaben?
* Gruppierung: z. B. `status`, `assignedTo`, `label`, `epicId`
* Sortierung: z. B. `dueDate`, manuell
* Spaltenreihenfolge

→ Aufgaben liegen **nicht** in Boards – sie werden **nur dargestellt**.

---

### 2. 🎛️ **Zwei Board-Typen:**

| Typ                          | Sichtbarkeit                              | Änderbar von                       |
| ---------------------------- | ----------------------------------------- | ---------------------------------- |
| 🧩 **Team-Boards**           | für alle Mitglieder des Projekts sichtbar | nur Admins & Mitglieder            |
| 🙋‍♂️ **Persönliche Boards** | nur für den Nutzer selbst                 | Bearbeiter dürfen eigene erstellen |

---

### 3. 🧱 **Status = Projektweite Konfiguration**

* Die möglichen Status (z. B. „Open“, „In Progress“, „Done“) werden **pro Projekt** definiert
* Diese Status können **nur Admins oder Mitglieder** bearbeiten
* Jeder Board-Typ, der nach `status` gruppiert ist, verwendet diese zentrale Liste

→ Vergleichbar mit „Spaltennamen“ in Trello – aber global je Projekt.

---

## 🔐 Aktualisierte Rechte im Detail

| Aktion                                          | Admin | Mitglied | Bearbeiter | Gast |
| ----------------------------------------------- | :---: | :------: | :--------: | :--: |
| Team-Boards erstellen/bearbeiten                |   ✅   |     ✅    |      ❌     |   ❌  |
| Persönliche Boards erstellen/bearbeiten         |   ✅   |     ✅    |      ✅     |   ❌  |
| Statusliste des Projekts bearbeiten             |   ✅   |     ✅    |      ❌     |   ❌  |
| Aufgaben in allen Boards sehen/bearbeiten       |   ✅   |     ✅    |      ✅     |   ✅  |
| Boards nach eigenen Vorlieben filtern/sortieren |   ✅   |     ✅    |      ✅     |   ❌  |

---

## 📎 Fazit für Designer & Entwickler

* **Boards = Views** mit speicherbarer Konfiguration
* **Datenmodell trennt klar**: Aufgabenlogik (Status, Zuordnung) vs. Anzeige (Boards)
* **Status sind projektspezifisch** und global
* **Bearbeiter haben maximale operative Freiheit**, aber **keine strukturelle Kontrolle**

---

# 📋 Modul: **Board-Ansicht (Anzeige & Interaktion)**

## 🎯 Zweck

Boards sind **konfigurierbare visuelle Ansichten auf Aufgaben**, die nach bestimmten Kriterien gruppiert (Spalten) und gefiltert werden. Sie ermöglichen schnelles Arbeiten mit Aufgaben per Drag & Drop und sind das Herzstück der täglichen Nutzung.

---

## 📦 Inhalt & Komponenten

| Komponente          | Beschreibung                                                                   |
| ------------------- | ------------------------------------------------------------------------------ |
| **Board-Kopfzeile** | Titel, Projektreferenz, persönliches oder Team-Board, Einstellungen            |
| **Filterleiste**    | Optional: Filter für Label, Zuständigkeit, Status, Zeitraum                    |
| **Spalten**         | Eine Spalte pro Gruppierung (z. B. Status: „Open“, „Done“)                     |
| **Karten (Tasks)**  | Aufgaben, die zur Gruppierung passen                                           |
| **Spaltenkopf**     | Titel des Gruppierungswertes (z. B. Statusname), Anzahl der Tasks              |
| **Neue Karte**      | „+“ Button oder Inline-Feld zur schnellen Erstellung                           |
| **Spaltenmenü**     | Nur für Mitglieder/Admins sichtbar: Status umbenennen, löschen, neu hinzufügen |
| **Board-Menü**      | Board umbenennen, teilen, löschen, als Standard setzen                         |

---

## 🖱️ Interaktionen & Verhalten

| Aktion                         | Beschreibung                                                                                                                |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| **Drag & Drop**                | Aufgaben innerhalb der Spalte oder in andere Spalten ziehen → aktualisiert automatisch das zugehörige Feld (z. B. `status`) |
| **Karte klicken**              | Öffnet Detailansicht mit vollständigen Infos und Kommentaren                                                                |
| **Spalte umbenennen**          | Nur für Mitglieder/Admins, wirkt auf das Feld `status` im gesamten Projekt                                                  |
| **Filter anwenden**            | Reduziert angezeigte Aufgaben dynamisch                                                                                     |
| **Board-Einstellungen öffnen** | Gruppierung, Filter, Sortierung ändern                                                                                      |
| **Spalten manuell sortieren**  | Per Drag & Drop (nur für eigene Boards)                                                                                     |
| **Board speichern**            | Änderungen an Gruppierung/Filter speichern (wenn berechtigt)                                                                |

---

## ⚙️ Technische Logik

* **groupBy**: Gibt an, welches Feld zur Spaltengenerierung verwendet wird (z. B. `status`, `assignedTo`)
* **filter**: Backend-gesteuerte Filterlogik auf Aufgabenebene
* **sortBy**: Reihenfolge innerhalb einer Spalte (z. B. `dueDate`, `manualOrder`)
* **columnOrder**: Reihenfolge der Werte (z. B. `["Open", "In Progress", "Done"]`)
* **manualCardOrder**: Task-Reihenfolge in jeder Spalte, speicherbar

---

## 🎨 Styling & Anmutung

* **Kanban-Optik**, ähnlich Linear oder Trello, aber moderner
* **Spaltenlayout**: Flexibles Scrollen horizontal bei vielen Spalten
* **Karten**: abgerundete Rechtecke mit sanftem Shadow, Hover-Effekt
* **Drag-Feedback**: Live-Highlighting der Drop-Ziele, sanfte Animation
* **Responsiv**: Spalten scrollen auf Mobil, Karten werden komprimiert dargestellt
* **Farben**: Spalten können leicht farblich hinterlegt sein (z. B. je Statusfarbe)

---

## 🔐 Rechteabhängigkeiten

| Aktion                        | Admin | Mitglied | Bearbeiter | Gast |
| ----------------------------- | :---: | :------: | :--------: | :--: |
| Board anzeigen                |   ✅   |     ✅    |      ✅     |   ✅  |
| Persönliches Board erstellen  |   ✅   |     ✅    |      ✅     |   ❌  |
| Team-Board erstellen          |   ✅   |     ✅    |      ❌     |   ❌  |
| Boardeinstellungen ändern     |   ✅   |     ✅    |      ❌     |   ❌  |
| Spaltennamen ändern (Status)  |   ✅   |     ✅    |      ❌     |   ❌  |
| Aufgaben per Drag verschieben |   ✅   |     ✅    |      ✅     |   ❌  |


---

# 🧾 Modul: **Task-Karte – Vorschau & Detailansicht**

## 🎯 Zweck

Die Karte repräsentiert eine einzelne Aufgabe. Sie erscheint in Boards als kompakte Vorschau und lässt sich in einer Detailansicht öffnen. Nutzer interagieren mit der Karte am häufigsten: Inhalte ansehen, bearbeiten, kommentieren, verschieben.

---

## 📦 Komponentenübersicht

### 🔹 Karten-Vorschau (z. B. im Board)

| Element                    | Beschreibung                                        |
| -------------------------- | --------------------------------------------------- |
| **Titel**                  | Kurzer Name der Aufgabe (1–2 Zeilen)                |
| **Labels**                 | Tags als kleine farbige Chips                       |
| **Zugewiesene Person(en)** | Rund-Avatare oder Initialen                         |
| **Fälligkeitsdatum**       | Klein & dezent, z. B. rotes Icon bei Überfälligkeit |
| **Epic (optional)**        | Verweis als kleiner Tag, wenn verknüpft             |
| **Drag Handle**            | Bereich zum Ziehen der Karte                        |
| **Statusfarbe**            | (Optional) linke farbige Linie entsprechend Status  |

---

### 🔹 Karten-Detailansicht (Modal oder Seite)

| Element               | Beschreibung                             |
| --------------------- | ---------------------------------------- |
| **Titel (editable)**  | Als große Überschrift mit Inline-Edit    |
| **Beschreibung**      | Markdown-unterstützter Textbereich       |
| **Zuweisung**         | Dropdown für User-Auswahl                |
| **Status**            | Dropdown aus projektweiter Statusliste   |
| **Epic-Zuordnung**    | Optionaler Selector für zugehörigen Epic |
| **Labels**            | Multi-Select (farbig)                    |
| **Fälligkeitsdatum**  | Datepicker                               |
| **Custom Fields**     | (Optional) weitere definierbare Felder   |
| **Kommentare**        | Sortiert nach Zeit, mit Markdown         |
| **Aktivitätsverlauf** | Optional, z. B. "X hat Status geändert"  |

---

## 🖱️ Interaktionen & Verhalten

* Klick auf Karte → öffnet Detailansicht
* **Inline-Editing** für Titel, Beschreibung
* **Dropdowns** mit Suchfunktion (z. B. Nutzerzuweisung)
* **Kommentar hinzufügen** via Textfeld unten mit Absenden per `Enter` oder Button
* **Markdown-Unterstützung** in Beschreibung und Kommentaren
* **Archivieren** statt Löschen (außer Admin/Mitglied)
* Änderungen triggern Live-Update via WebSocket
* **@Mentions** in Kommentaren (optional in späterer Version)

---

## 🎨 Styling & Anmutung

* Vorschau: Clean, nicht überladen – Fokus auf Titel + Key-Infos
* Detail: Dialog mit zwei Spalten (links Inhalt, rechts Metadaten & Aktionen)
* Animiertes Öffnen/Schließen der Detailansicht
* **Kommentarbereich**: klar getrennt, Chat-ähnlich oder Threaded-Design
* **Dark Mode ready**

---

## 🔐 Rechteabhängigkeiten

| Aktion              | Admin | Mitglied | Bearbeiter | Gast |
| ------------------- | :---: | :------: | :--------: | :--: |
| Karte sehen         |   ✅   |     ✅    |      ✅     |   ✅  |
| Karte bearbeiten    |   ✅   |     ✅    |      ✅     |   ❌  |
| Karte löschen       |   ✅   |     ✅    |      ❌     |   ❌  |
| Karte archivieren   |   ✅   |     ✅    |      ✅     |   ❌  |
| Kommentar schreiben |   ✅   |     ✅    |      ✅     |   ❌  |
| Beschreibung ändern |   ✅   |     ✅    |      ✅     |   ❌  |


---

# 💬 Modul: **Kommentare & Aktivitäten**

## 🎯 Zweck

Kommentare ermöglichen inhaltlichen Austausch direkt auf Aufgabenebene.
Der Aktivitätsbereich zeigt chronologisch alle Änderungen an einer Aufgabe und erhöht Transparenz.

---

## 📦 Komponentenübersicht

### 🔹 Kommentarbereich

| Element                  | Beschreibung                                                  |
| ------------------------ | ------------------------------------------------------------- |
| **Kommentar-Editor**     | Textfeld mit Markdown-Unterstützung                           |
| **Abschicken-Button**    | Optional, oder per `Enter` abschicken                         |
| **Kommentarliste**       | Chronologisch sortiert (neu unten oder oben – konfigurierbar) |
| **Avatar + Name + Zeit** | Für jeden Kommentar sichtbar                                  |
| **Bearbeiten/Löschen**   | Für eigene Kommentare (Bearbeiter & höher)                    |
| **@Mentions (später)**   | Nutzer erwähnen mit Autovervollständigung                     |

---

### 🔹 Aktivitätsprotokoll

| Ereignisse                    | Beispiele                              |
| ----------------------------- | -------------------------------------- |
| **Statusänderung**            | „Status geändert von In Review → Done“ |
| **Zuweisung geändert**        | „Zugewiesen an Alena“                  |
| **Beschreibung aktualisiert** | „Beschreibung bearbeitet“              |
| **Epic-Zuordnung**            | „Hängt jetzt an Epic XY“               |
| **Label hinzugefügt**         | „Label 'UX' hinzugefügt“               |

* Diese Events sind **nur lesbar**, nicht löschbar.
* Optional ein-/ausblendbar (Collapse unter „Aktivität anzeigen“).

---

## 🖱️ Interaktionen

* Kommentare lassen sich direkt inline bearbeiten
* Lösch-Icon erscheint bei Hover über eigenem Kommentar
* Automatische Scroll-to-Last bei neuen Kommentaren
* „@“ schreibt Autovervollständigungsbox (optional für später)
* Live-Aktualisierung über WebSocket bei neuen Kommentaren oder Events

---

## 🎨 Styling & UX

* **Kommentare**: Bubbles oder klassische Listenform (je nach Modus)
* Inline-Editor mit minimaler Toolbar (`bold`, `italic`, `code`, `link`)
* Aktivitätsfeed im helleren Stil, optisch vom Kommentarteil getrennt
* Kommentare nutzen `sm` bis `base` Schriftgröße, Avatare `xs`

---

## 🔐 Rechteabhängigkeiten

| Aktion               | Admin | Mitglied |   Bearbeiter   | Gast |
| -------------------- | :---: | :------: | :------------: | :--: |
| Kommentar schreiben  |   ✅   |     ✅    |        ✅       |   ❌  |
| Kommentar bearbeiten |   ✅   |     ✅    | ✅ (nur eigene) |   ❌  |
| Kommentar löschen    |   ✅   |     ✅    | ✅ (nur eigene) |   ❌  |
| Aktivitäten sehen    |   ✅   |     ✅    |        ✅       |   ✅  |


---

# 🧱 Modul: **Epics (übergeordnete Aufgabenblöcke)**

## 🎯 Zweck

Epics sind größere Arbeitseinheiten, die mehrere Aufgaben logisch zusammenfassen. Sie geben Orientierung, helfen bei der Planung und ermöglichen eine Gruppierung in Boards oder Roadmaps.

---

## 📦 Komponentenübersicht

### 🔹 Epic-Datenmodell (Backend)

| Feld                      | Beschreibung                                                 |
| ------------------------- | ------------------------------------------------------------ |
| `id`                      | Eindeutige ID                                                |
| `titel`                   | Name des Epics                                               |
| `beschreibung`            | Ausführliche Beschreibung (Markdown möglich)                 |
| `projektId`               | Zugehöriges Projekt                                          |
| `teamId`                  | Zugehöriges Team (optional)                                  |
| `authorId`                | Ersteller                                                    |
| `assignedTo[]`            | Zuständige Person(en) (optional)                             |
| `startDate`               | Optionaler Startzeitpunkt                                    |
| `endDate`                 | Optionaler Endzeitpunkt                                      |
| `labels[]`                | Kategorisierung                                              |
| `status`                  | Optionaler Zustand („geplant“, „in Arbeit“, „abgeschlossen“) |
| `createdAt` / `updatedAt` | Metafelder                                                   |

---

### 🔹 Epic-UI-Komponenten

| Komponente             | Beschreibung                                                                         |
| ---------------------- | ------------------------------------------------------------------------------------ |
| **Epic-Übersicht**     | Liste aller Epics eines Projekts, sortier-/filterbar                                 |
| **Epic-Karte**         | Ähnlich einer Task-Karte: Titel, Zeitraum, Fortschritt, Anzahl zugeordneter Aufgaben |
| **Epic-Detailansicht** | Große Ansicht mit Beschreibung, Aufgabenliste, Metadaten                             |
| **Epic-Zuweisung**     | In der Aufgabenkarte wählbar über Dropdown                                           |
| **Filter „nach Epic“** | In Board- oder Roadmap-Ansicht möglich                                               |

---

## 🖱️ Interaktionen

* Epic erstellen über „+ Epic“ in der Projektansicht oder Roadmap
* Epic bearbeiten (Titel, Beschreibung, Zeitraum, Status)
* Aufgaben zu einem Epic zuordnen (im Task-Dropdown)
* Anzeige des Fortschritts: % erledigter Aufgaben (z. B. 7/10)
* Epic löschen oder archivieren (nur mit Rechten)
* In Boards und Roadmaps nach Epic gruppieren oder filtern

---

## 🎨 Styling & UX

* **Epic-Karte** ähnlich einer Task-Karte, aber breiter + prominenter Fortschrittsbalken
* **Farblich neutral**, damit sie nicht mit Labels kollidieren
* In Roadmaps als farbige Zeitblöcke darstellbar
* Optionales Icon (z. B. „⛰️“, „📦“) je Epic-Typ (visuelle Kategorisierung)

---

## 🔐 Rechteabhängigkeiten

| Aktion                     | Admin | Mitglied | Bearbeiter | Gast |
| -------------------------- | :---: | :------: | :--------: | :--: |
| Epic erstellen             |   ✅   |     ✅    |      ❌     |   ❌  |
| Epic bearbeiten            |   ✅   |     ✅    |      ❌     |   ❌  |
| Epic zu Aufgaben zuordnen  |   ✅   |     ✅    |      ✅     |   ❌  |
| Epic löschen / archivieren |   ✅   |     ✅    |      ❌     |   ❌  |
| Epic sehen & filtern       |   ✅   |     ✅    |      ✅     |   ✅  |

---

# 🗺️ Modul: **Roadmap-Ansicht**

## 🎯 Zweck

Die Roadmap bietet eine **zeitbasierte, visuelle Übersicht über Epics** und deren zugehörige Aufgaben. Sie hilft Teams dabei, langfristige Planung, Deadlines und Abhängigkeiten zu verstehen und zu steuern.

---

## 📦 Komponentenübersicht

### 🔹 Roadmap-Datenmodell (konzeptionell)

Die Roadmap ist **eine Ansicht**, keine eigene Entität. Sie basiert auf:

* `Epics` mit Start- und Enddatum
* Den `Tasks`, die einem Epic zugeordnet sind
* Zeitlicher Darstellung (z. B. Wochen, Monate)

---

### 🔹 UI-Komponenten

| Komponente             | Beschreibung                                                                       |
| ---------------------- | ---------------------------------------------------------------------------------- |
| **Zeitleiste**         | Horizontal scrollbare Wochen-/Monatsleiste                                         |
| **Epic-Balken**        | Farbige Balken pro Epic (mit Titel), basierend auf Start/Enddatum                  |
| **Task-Markierungen**  | Optional kleine Icons oder Punkte im Balken, z. B. für Meilensteine oder Deadlines |
| **Tooltip/Popover**    | Bei Hover auf Balken → Details zum Epic                                            |
| **Epic-Detail öffnen** | Klick auf Balken → öffnet Epic-Detailansicht                                       |
| **Filterleiste**       | Nach Team, Status, Label, Epic-Typ, Zeitraum etc.                                  |
| **Zoom-Level**         | Woche, Monat, Quartal (umschaltbar)                                                |
| **„Heute“-Marker**     | Rote Linie für aktuellen Tag                                                       |

---

### 🖱️ Interaktionen (aktualisiert)

* **Editiermodus aktivieren**: Nur wenn der Nutzer aktiv den *Bearbeiten-Modus* (z. B. Button „✏️ Bearbeiten“) einschaltet, wird das **Drag & Drop von Epics** freigegeben.
* Im Normalmodus sind alle Balken **fixiert** (nur Lesemodus)
* Beim Verlassen des Editiermodus: automatische Abfrage, ob Änderungen gespeichert oder verworfen werden sollen (z. B. Modal)
* Visuelles Feedback (z. B. gelber Balken oben „Bearbeiten aktiv“)

---

## 🎨 Styling & UX

| Element      | Stil                                                            |
| ------------ | --------------------------------------------------------------- |
| Zeitleiste   | Sticky Header mit hellgrauem Hintergrund                        |
| Balkenfarben | Automatisch aus Label, Epic-Typ oder manuell wählbar            |
| Balkenform   | Runde Ecken, sanfte Schatten                                    |
| Hintergrund  | Gitterlinien je Zeitintervall, klare Trennung                   |
| Tooltip      | Mini-Popup mit Epic-Titel, Zeitraum, Status, Fortschritt        |
| Farben       | Kontrastreich, aber barrierefrei (Farbenblinde berücksichtigen) |

---


### 🔐 Rechteabhängigkeiten (angepasst)

| Aktion                    | Admin | Mitglied | Bearbeiter | Gast |
| ------------------------- | :---: | :------: | :--------: | :--: |
| Editiermodus aktivieren   |   ✅   |     ✅    |      ❌     |   ❌  |
| Epic per Drag verschieben |   ✅   |     ✅    |      ❌     |   ❌  |

---

## 🧠 Technische Hinweise

* Roadmap wird rein clientseitig gerendert auf Basis von API-Daten (`epics` + `tasks`)
* Eventuell geeignet für [virtual scrolling](https://react.dev/learn/rendering-lists#optimizing-performance-with-key) bei vielen Epics
* Rendering-Logik für horizontale Zeitachse ähnlich Gantt, aber einfacher
