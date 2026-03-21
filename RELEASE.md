# Release & Publishing Guide

Diese Anleitung beschreibt den kompletten Prozess zum Veröffentlichen einer neuen Version von Boot Mate.

---

## 📋 Release-Prozess (Schritt für Schritt)

### 1. Versionsnummer aktualisieren

```bash
./update-version.sh 50.1
```

Das Script aktualisiert automatisch:
- `meson.build`
- `Cargo.toml` (als X.Y.0 für SemVer)
- `po/en.po` und `po/de.po`
- `data/me.rueegger.bootmate.metainfo.xml.in` (Version + Datum)

### 2. Changelog in metainfo.xml.in schreiben

Aktualisiere die Release-Beschreibung in `data/me.rueegger.bootmate.metainfo.xml.in`:

```xml
<release version="50.1" date="2026-XX-XX">
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

Falls sich Rust-Abhängigkeiten geändert haben:

```bash
cargo update
python3 ~/Projects/me.rueegger.cargo/flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```

### 4. Lokal testen (Flatpak)

```bash
flatpak-builder --user --install --force-clean --disable-rofiles-fuse _flatpak me.rueegger.bootmate.yml
flatpak run me.rueegger.bootmate
```

### 5. Änderungen committen und pushen

```bash
git add .
git commit -m "Bump version to 50.1"
git push
```

### 6. Tag erstellen und pushen

```bash
git tag -a v50.1 -m "Release version 50.1"
git push origin v50.1
```

### 7. Flatpak veröffentlichen

```bash
~/Projects/flatpak.rueegger.dev/publish.sh bootmate
```

### 8. GitHub Release erstellen

1. Gehe zu: https://github.com/srueegger/bootmate/releases
2. Klicke auf **"Draft a new release"**
3. Wähle den Tag: `v50.1`
4. Release Title: `Boot Mate 50.1`
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
- **MAJOR.MINOR** (z.B. 50.0, 50.1)
- MAJOR: GNOME-Version (50 = GNOME 50)
- MINOR: Releases innerhalb des GNOME-Zyklus

---

**Letzte Aktualisierung:** 2026-03-21
**Aktuelle Version:** 50.0
