# PPA Setup Guide für Boot Mate

## Übersicht

Wir erstellen ein **Personal Package Archive (PPA)** auf Launchpad, um Boot Mate einfach für Ubuntu-Nutzer verfügbar zu machen.

**Dein PPA:** https://launchpad.net/~rueegger/+archive/ubuntu/bootmate

### Ziel-Konfiguration
- **Ubuntu-Versionen:** 24.04 LTS (Noble), 25.10 STS (Oracular), 26.04 (Plucky - in Entwicklung)
- **Architekturen:** amd64, arm64, armhf
- **GPG-Schlüssel:** ✅ Bereits auf Launchpad hochgeladen
- **Maintainer:** samuel@rueegger.me

---

## Was ist ein PPA?

Ein PPA ist ein Repository auf Launchpad, das:
1. **Source-Pakete akzeptiert** (nicht binäre .deb-Dateien)
2. **Automatisch für alle Architekturen baut** (amd64, arm64, armhf)
3. **Für mehrere Ubuntu-Versionen baut** (Noble, Oracular, Plucky)
4. **Nutzer können einfach installieren:**
   ```bash
   sudo add-apt-repository ppa:rueegger/bootmate
   sudo apt update
   sudo apt install bootmate
   ```

---

## Wichtige Unterschiede zu GitHub Releases

| Aspekt | GitHub Releases | Launchpad PPA |
|--------|----------------|---------------|
| Upload | Binäre .deb-Dateien | Source-Pakete (.dsc, .tar.xz, .changes) |
| Build | Lokal (GitHub Actions) | Launchpad Build-Server |
| Architekturen | Manuell pro Architektur | Automatisch alle Architekturen |
| Ubuntu-Versionen | Eine Version pro Paket | Automatisch für alle Versionen |
| Installation | Manueller Download | `apt install` via PPA |

---

## Projektstruktur

```
bootmate/
├── debian/                    # Debian-Paket-Konfiguration (im Git)
│   ├── changelog
│   ├── control
│   ├── copyright
│   ├── rules
│   └── source/
│       └── format
├── build-ppa.sh              # Automatisiertes Build-Script (im Git)
├── ppa/                      # Build-Verzeichnis (gitignored!)
│   ├── bootmate_1.2.0.orig.tar.xz
│   ├── bootmate-1.2.0/       # Extrahierter Source
│   ├── *.dsc                 # Package descriptions
│   ├── *.changes             # Upload-Dateien
│   └── *.buildinfo           # Build-Informationen
└── .gitignore                # ppa/ ist hier drin!
```

**Wichtig:**
- Das `debian/` Verzeichnis ist Teil des Git-Repositories
- Das `ppa/` Verzeichnis wird von `build-ppa.sh` erstellt und ist in `.gitignore`
- Alle Build-Artefakte landen in `ppa/`, nicht im Hauptverzeichnis
- So bleibt das Git-Repository sauber und frei von Build-Artefakten

---

## Was wir erstellen müssen

### 1. debian/ Verzeichnis-Struktur

```
debian/
├── changelog          # Versionshistorie für jede Ubuntu-Version
├── control            # Paket-Metadaten, Dependencies (inkl. debhelper-compat)
├── rules              # Build-Anweisungen (Meson)
├── copyright          # Lizenz-Information
└── source/
    └── format         # Source-Paket-Format (3.0 quilt)
```

**Hinweis:** `debian/compat` wird nicht mehr benötigt - die debhelper-Version wird über `debhelper-compat (= 13)` in `debian/control` definiert.

### 2. debian/control

Definiert:
- Paketname, Beschreibung
- Build-Dependencies (meson, cargo, rustc, libgtk-4-dev, etc.)
- Runtime-Dependencies (libgtk-4-1, libadwaita-1-0, etc.)
- Architekturen (amd64, arm64, armhf)

### 3. debian/rules

Meson-Build-Anweisungen:
```makefile
#!/usr/bin/make -f

%:
	dh $@

override_dh_auto_configure:
	meson setup build --prefix=/usr

override_dh_auto_build:
	meson compile -C build

override_dh_auto_install:
	DESTDIR=$(CURDIR)/debian/bootmate meson install -C build
```

### 4. debian/changelog

**Kritisch für PPA!** Jede Ubuntu-Version braucht einen eigenen Changelog-Eintrag:

```
bootmate (1.2.0-0ubuntu1~noble1) noble; urgency=medium

  * New upstream release 1.2.0

 -- Samuel Rüegger <mail@samuelrueegger.ch>  Fri, 13 Dec 2025 22:00:00 +0100

bootmate (1.2.0-0ubuntu1~oracular1) oracular; urgency=medium

  * New upstream release 1.2.0

 -- Samuel Rüegger <mail@samuelrueegger.ch>  Fri, 13 Dec 2025 22:00:00 +0100
```

**Format:** `version-debianrev~ubuntucodename1`
- `1.2.0` = Upstream-Version
- `0ubuntu1` = Ubuntu-Paket-Revision
- `~noble1` = Für Ubuntu Noble (24.04)

---

## Workflow

### Schritt 1: debian/ Verzeichnis erstellen
Alle benötigten Dateien im `debian/` Verzeichnis erstellen.

### Schritt 2: Tools installieren
```bash
sudo apt install devscripts debhelper dput gnupg dpkg-dev
```

### Schritt 3: Build-Umgebung vorbereiten

```bash
# Build-Verzeichnis erstellen
mkdir -p ppa
cd ppa

# Source-Tarball aus Git erstellen
git archive --format=tar --prefix=bootmate-1.2.0/ HEAD | xz > bootmate_1.2.0.orig.tar.xz

# Source extrahieren
tar -xf bootmate_1.2.0.orig.tar.xz

# debian/ Verzeichnis in extrahierten Source kopieren
cp -r ../debian bootmate-1.2.0/
cd bootmate-1.2.0
```

### Schritt 4: Source-Pakete bauen

Für **jede Ubuntu-Version** separat:

```bash
# Für Noble (24.04)
dch -v 1.2.0-0ubuntu1~noble1 -D noble "New upstream release"
debuild -S -sa -d

# Für nächste Version: Neu extrahieren und debian/ kopieren
cd ..
rm -rf bootmate-1.2.0
tar -xf bootmate_1.2.0.orig.tar.xz
cp -r ../debian bootmate-1.2.0/
cd bootmate-1.2.0

# Für Oracular (25.10)
dch -v 1.2.0-0ubuntu1~oracular1 -D oracular "New upstream release"
debuild -S -sa -d

# Gleiches für Plucky...
```

**Output (im ppa/ Verzeichnis):**
- `bootmate_1.2.0-0ubuntu1~noble1.dsc` (Package description)
- `bootmate_1.2.0.orig.tar.xz` (Original source)
- `bootmate_1.2.0-0ubuntu1~noble1.debian.tar.xz` (Debian files)
- `bootmate_1.2.0-0ubuntu1~noble1_source.changes` (Upload instructions)
- `bootmate_1.2.0-0ubuntu1~noble1_source.buildinfo` (Build info)

**Hinweis:** Dieser manuelle Prozess ist komplex. Nutze stattdessen `./build-ppa.sh`, welches alles automatisch macht!

### Schritt 5: dput konfigurieren

`~/.dput.cf` erstellen:
```ini
[bootmate-ppa]
fqdn = ppa.launchpad.net
method = ftp
incoming = ~rueegger/ubuntu/bootmate/
login = anonymous
allow_unsigned_uploads = 0
```

### Schritt 6: Zu Launchpad hochladen

```bash
# Ins ppa/ Verzeichnis wechseln
cd ppa

# Hochladen
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~noble1_source.changes
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~oracular1_source.changes
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~plucky1_source.changes
```

### Schritt 7: Launchpad baut automatisch

Launchpad wird dann:
1. Source-Paket entpacken
2. Dependencies installieren
3. `debian/rules` ausführen
4. Für **jede Architektur** (amd64, arm64, armhf) bauen
5. Binärpakete im PPA veröffentlichen

---

## Wichtige Hinweise

### Build-Dependencies

Boot Mate benötigt:
- **Rust 1.70+** (verfügbar ab Ubuntu 22.04)
- **Meson 0.59+**
- **GTK 4.12+** (verfügbar ab Ubuntu 24.04)
- **libadwaita 1.5+** (verfügbar ab Ubuntu 24.04)

⚠️ **Problem:** Ubuntu 22.04 (Jammy) hat zu alte Versionen!
→ Wir konzentrieren uns auf **Ubuntu 24.04+**

### GPG-Signierung

Alle Source-Pakete müssen mit deinem GPG-Schlüssel signiert werden:
```bash
# GPG-Schlüssel-ID anzeigen
gpg --list-secret-keys --keyid-format LONG

# In debuild verwenden (automatisch)
debuild -S -sa
```

### Version-Nummern für PPA

Format: `upstream-version-debianrev~ubuntucodename+buildnumber`

Beispiele:
- `1.2.0-0ubuntu1~noble1` - Erste Noble-Version
- `1.2.0-0ubuntu1~noble2` - Rebuild für Noble (Bug-Fix)
- `1.2.1-0ubuntu1~noble1` - Neue Upstream-Version

---

## Testing nach Upload

### Build-Status prüfen
https://launchpad.net/~rueegger/+archive/ubuntu/bootmate/+packages

### Paket installieren und testen
```bash
sudo add-apt-repository ppa:rueegger/bootmate
sudo apt update
sudo apt install bootmate
```

---

## Troubleshooting

### "dpkg-source: error: unrepresentable changes to source"
→ Build-Artefakte (wie `.gresource`, `.rlib`, `.so`) oder IDE-Dateien im Source-Tree
→ Lösung:
  - Automatisch: `./build-ppa.sh` nutzt sauberen Git-Export
  - Manuell: Stelle sicher, dass du aus dem extrahierten Tarball baust, nicht aus dem Git-Repository
  - `ppa/` Verzeichnis ist in `.gitignore` und bleibt sauber

### "gpg: signing failed: No such file or directory"
→ GPG-Schlüssel nicht gefunden
→ Lösung: `gpg --list-secret-keys` und Schlüssel prüfen

### "Upload rejected: Already uploaded"
→ Versionsnummer bereits hochgeladen
→ Lösung: Versionsnummer erhöhen (z.B. `~noble2`)

### Build schlägt fehl auf Launchpad
→ Dependencies fehlen oder falsche Versionen
→ Lösung: `debian/control` Build-Dependencies prüfen

### Binäre Dateien im ppa/ Verzeichnis
→ Normal! Das `ppa/` Verzeichnis enthält Build-Artefakte
→ Lösung: `ppa/` ist in `.gitignore`, einfach ignorieren oder mit `rm -rf ppa/` löschen

---

## Nächste Schritte

1. ✅ debian/ Verzeichnis erstellen
2. ✅ Tools installieren
3. ✅ Source-Pakete bauen
4. ✅ Zu Launchpad hochladen
5. ⏳ Builds auf Launchpad beobachten
6. ✅ Testen

---

---

## Quick Start (EINFACHER WEG)

Wir haben ein automatisiertes Script erstellt, das alles für dich macht!

### Schritt 1: Tools installieren

```bash
sudo apt install devscripts debhelper dput gnupg dpkg-dev
```

### Schritt 2: Build-Script ausführen

```bash
./build-ppa.sh
```

Das Script wird:
- ✓ Alle benötigten Tools prüfen
- ✓ `ppa/` Verzeichnis erstellen (gitignored)
- ✓ `~/.dput.cf` konfigurieren
- ✓ Source-Tarball aus Git erstellen
- ✓ Source-Pakete für Noble, Oracular und Plucky bauen
- ✓ Changelog für jede Version aktualisieren
- ✓ Alle Build-Artefakte in `ppa/` ablegen

**Wichtig:** Alle Build-Artefakte landen im `ppa/` Verzeichnis, welches in `.gitignore` ist. So bleibt dein Git-Repository sauber!

### Schritt 3: Zu Launchpad hochladen

Nach dem Build zeigt das Script die Upload-Befehle:

```bash
# Alle auf einmal hochladen (empfohlen)
cd ppa && for file in bootmate_1.2.0-*.changes; do dput ppa:rueegger/bootmate $file; done
```

Oder einzeln:

```bash
cd ppa
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~noble1_source.changes
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~oracular1_source.changes
dput ppa:rueegger/bootmate bootmate_1.2.0-0ubuntu1~plucky1_source.changes
```

### Schritt 4: Builds beobachten

https://launchpad.net/~rueegger/+archive/ubuntu/bootmate/+packages

Fertig!

---

**Erstellt:** 2025-12-13
**PPA:** https://launchpad.net/~rueegger/+archive/ubuntu/bootmate
