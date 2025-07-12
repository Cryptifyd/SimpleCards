# 🧠 Projektplan: Task-Management-Tool (Trello-Alternative)

## 📘 1. Projektübersicht

### 🎯 Ziel
Wir entwickeln ein benutzerfreundliches, visuell ansprechendes Task-Management-Tool mit anpassbaren Boards, granularer Rechteverwaltung und praktischen Darstellungen wie Roadmaps. Es soll Teams ermöglichen, Projekte effektiv zu strukturieren, Aufgaben flexibel zwischen Teams zu organisieren und beliebige Konzepte wie Kanban, Roadmaps oder eigene Workflows umzusetzen.

### 👥 Zielnutzer
Das Tool wird primär für den Eigenbedarf entwickelt, um unabhängig von kommerziellen Tools zu sein. Perspektivisch soll es auch für kleine bis mittlere Teams geeignet sein (3–30 Personen), mit der Option, später für größere Organisationen skalierbar zu werden.

### 🆚 Abgrenzung & Vision
Aktuell gibt es keine klar definierten Einschränkungen beim Funktionsumfang. Langfristig ist das System offen für Erweiterungen wie integrierte Wikis, Dokumentationen oder öffentliche Projekt-Landingpages.

### 🚀 Unterschiede zu Trello, Jira & Co.
- Frei gestaltbare Boards (z. B. nach Status, Person, Label, Epics)
- Vordefinierte Board-Typen wie Roadmap oder Kanban
- Granulare Rechteverwaltung auf Aufgaben- und Projektebene
- Keine Cloud-Abhängigkeit, volle Datenhoheit
- Modernste Technologien (Next.js, Rust, PostgreSQL, WebSockets, K8s)

---

## 👤 2. Stakeholder & Rollen

### 👨‍💻 Projektteam
- Der Entwickler arbeitet allein, unterstützt von ChatGPT und Claude zur Planung, Ideengenerierung und Entwicklung.
- Weitere Teammitglieder sind derzeit nicht vorgesehen, aber möglich.

### 🔐 Nutzerrollen

| Rolle   | Berechtigungen |
|---------|----------------|
| Admin   | Vollzugriff auf Projekte, Boards, Aufgaben und Rechteverwaltung |
| Mitglied | Aufgaben erstellen, bearbeiten, verschieben, kommentieren |
| Gast    | Nur lesender Zugriff |

---

## 🧩 3. Nutzeraktionen & Datenmodell

### 🔄 Häufige Nutzeraktionen

1. Aufgaben erstellen
2. Aufgaben bearbeiten (Titel, Beschreibung, Status usw.)
3. Aufgaben verschieben (mit Status-Änderung oder Gruppierung)
4. Boards anzeigen (flexibel konfigurierbar)
5. Boards konfigurieren (Spalten nach Status, Person etc.)
6. Kommentare schreiben
7. Aufgaben zuweisen
8. Rechte verwalten
9. Projekte und Teams strukturieren
10. Live-Updates in Echtzeit empfangen

### 📦 Datenstruktur: Aufgabe (Card)

| Feld         | Beschreibung |
|--------------|--------------|
| id           | Eindeutige ID |
| titel        | Titel der Aufgabe |
| beschreibung | Inhalt/Beschreibung |
| status       | Status (z. B. Offen, Erledigt) |
| assignedTo   | Zuständige Person(en) |
| authorId     | Ersteller der Aufgabe |
| createdAt    | Erstellungsdatum |
| updatedAt    | Letzte Bearbeitung |
| labels       | Tags/Labels |
| dueDate      | Fälligkeitsdatum |
| attachments  | Dateianhänge |
| comments     | Kommentare (separates Objekt) |
| customFields | Freie Zusatzfelder |
| epicId       | Zugeordneter Epic |
| projektId    | Projektzugehörigkeit |
| teamId       | Teamzugehörigkeit |

---

## 🗺️ 4. Epics & Roadmaps

### Epics
- Eigenständige Objekte (Titel, Beschreibung, Zeitraum)
- Container für Aufgaben
- Aufgaben referenzieren `epicId`
- Können in Boards angezeigt oder gruppiert werden

### Roadmaps
- Visualisierungen über Zeitachse (startDate, endDate)
- Gruppierung z. B. nach Epic, Team oder Projekt
- Darstellung als Balken pro Task/Epic
- Drag & Drop zur Zeitplanung
- Kein eigener Datentyp – basiert auf Board-Logik

---

## 🔄 5. Realtime-Verhalten

### Realtime-Änderungen im Board:
- Karten erstellen, löschen, verschieben
- Änderungen an Titel, Status, Zuweisungen
- Labels und Fälligkeitsdaten

### Realtime bei geöffneter Karte:
- Beschreibung bearbeiten
- Kommentare in Echtzeit
- Alle Metafelder live aktualisieren

### Technisches Realtime-Modell:
- WebSocket-Broadcasts
- Trennung in Board-Feed (leichtgewichtig) und Card-Feed (detailliert)

---

## 🧱 6. Technologie-Stack

| Bereich     | Technologie           |
|-------------|------------------------|
| Frontend    | Next.js (React)        |
| Backend     | Rust + axum + tokio    |
| Datenbank   | PostgreSQL             |
| Realtime    | WebSockets (tokio-tungstenite) |
| Skalierung  | Redis PubSub (optional) |
| Deployment  | Docker + Kubernetes    |

---

## 🚀 7. MVP – Erste Version (Produktionsbereit intern)

| Funktion                                      | Status |
|----------------------------------------------|--------|
| Projekte & Teams anlegen                     | ✅     |
| Aufgaben erstellen/bearbeiten                | ✅     |
| Boards anzeigen (flexibel konfigurierbar)    | ✅     |
| Aufgaben verschieben (Drag & Drop)           | ✅     |
| Realtime-Updates                              | ✅     |
| Kommentare                                    | ✅     |
| Zuweisungen                                   | ✅     |
| Epics + Roadmap                               | ✅     |
| Rechteverwaltung                              | ✅     |
| Authentifizierung                             | ✅     |
| Dark Mode                                     | ❌     (später) |

---

## 📆 8. Projektphasen & Roadmap

| Phase | Ziel | Inhalt |
|-------|------|--------|
| 1. Setup & Architektur | Technisches Fundament | Repo, CI/CD, Auth, DB |
| 2. Board-System | Visualisierung & Modell | Projekte, Aufgabenmodell, flexible Boards |
| 3. Interaktion | Task-Handling | Aufgaben erstellen, bearbeiten, verschieben |
| 4. Realtime | Synchronisation | WebSockets, Board- & Card-Feeds |
| 5. Epics & Roadmap | Höhere Ebene | Epics, Zuordnung, Roadmap-Ansicht |
| 6. Rechteverwaltung | Zugriffskontrolle | Rollen, Projektrechte |
| 7. UI-Feinschliff | Design & UX | Erste Designelemente, Responsiveness |
| 8. Deployment | Staging für Nutzung | Hosting, Monitoring, Release

---
