# Release & Publishing Guide

Diese Anleitung beschreibt den kompletten Prozess zum Veröffentlichen einer neuen Version von Boot Mate.

---

## 📋 Release-Prozess (Schritt für Schritt)

### 1. Versionsnummer aktualisieren

```bash
./update-version.sh 51.0
```

Das Script aktualisiert automatisch:
- `meson.build`
- `Cargo.toml` (als X.Y.0 für SemVer)
- `po/en.po` und `po/de.po`
- `data/me.rueegger.bootmate.metainfo.xml.in` (Version + Datum)

### 2. Changelog in metainfo.xml.in schreiben

Aktualisiere die Release-Beschreibung in `data/me.rueegger.bootmate.metainfo.xml.in`:

```xml
<release version="51.0" date="2026-XX-XX">
  <description>
    <p>Kurze Beschreibung</p>
    <ul>
      <li>Feature 1</li>
      <li>Bugfix 2</li>
    </ul>
  </description>
</release>
```

### 3. cargo-sources.json aktualisieren

Falls sich Rust-Abhängigkeiten geändert haben (in der Build-Umgebung ausführen,
auf dem Host ist kein Cargo installiert):

```bash
cargo update
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```

`flatpak-cargo-generator.py` stammt aus
[flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools) (Ordner `cargo/`).

### 4. Lokal testen

Linux (Distrobox):

```bash
scripts/devbox-build.sh
scripts/devbox-flatpak.sh
```

macOS (Multipass, erzeugt auf Apple Silicon ein ARM-Flatpak):

```bash
scripts/vm-test.sh
scripts/vm-flatpak.sh
```

### 5. Änderungen committen und pushen

```bash
git add .
git commit -m "Bump version to 51.0"
git push
```

### 6. Tag erstellen und pushen

```bash
git tag -a v51.0 -m "Release version 51.0"
git push origin v51.0
```

### 7. Flatpak veröffentlichen

Nur auf dem Ubuntu-Rechner (x86_64): Das Skript braucht den Signaturschlüssel
und baut in der Distrobox `bootmate-devsystem`.

```bash
~/Sites/flatpak.rueegger.dev/publish.sh bootmate
```

### 8. GitHub Release erstellen

1. Gehe zu: https://github.com/srueegger/bootmate/releases
2. Klicke auf **"Draft a new release"**
3. Wähle den Tag: `v51.0`
4. Release Title: `Boot Mate 51.0`
5. Beschreibung mit Features/Fixes
6. Klicke auf **"Publish release"**

---

## 🎯 Checkliste für Release

- [ ] Versionsnummer aktualisiert (`./update-version.sh X.Y`)
- [ ] Changelog in `metainfo.xml.in` geschrieben
- [ ] `cargo-sources.json` aktualisiert (falls Abhängigkeiten geändert)
- [ ] Lokaler Flatpak-Build erfolgreich
- [ ] Änderungen committed und gepusht
- [ ] Tag erstellt und gepusht
- [ ] Flatpak veröffentlicht (`publish.sh bootmate`)
- [ ] GitHub Release erstellt

---

## 🔗 Wichtige Links

- **GitHub Repository:** https://github.com/srueegger/bootmate
- **GitHub Releases:** https://github.com/srueegger/bootmate/releases
- **Flatpak Repository:** https://flatpak.rueegger.dev

---

### Versionierung

Boot Mate folgt dem GNOME-Versionsschema:
- **MAJOR.MINOR** (z.B. 51.0, 51.1)
- MAJOR: GNOME-Version (51 = GNOME 51)
- MINOR: Releases innerhalb des GNOME-Zyklus
