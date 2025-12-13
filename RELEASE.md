# Release & Publishing Guide

Diese Anleitung beschreibt den kompletten Prozess zum Veröffentlichen einer neuen Version von Boot Mate.

## Übersicht

Bei jedem Release werden automatisch über GitHub Actions DEB-Pakete gebaut:
- **DEB-Paket** (`bootmate_X.Y.Z_amd64.deb`)
- **DEB-Paket ARM64** (`bootmate_X.Y.Z_arm64.deb`)

Diese werden als Release Assets auf GitHub bereitgestellt.

---

## 📋 Release-Prozess (Schritt für Schritt)

### 1. Versionsnummer aktualisieren

Die Version muss in folgenden Dateien angepasst werden:

- `Cargo.toml` (Zeile 3)
- `meson.build` (Zeile 4)
- `po/en.po` (Zeile 8)
- `po/de.po` (Zeile 8)
- `data/ch.srueegger.bootmate.metainfo.xml.in` (Zeile 28 - Version UND Datum)

**Beispiel für Version 1.1.0:**

```bash
# In allen Dateien 1.0.0 → 1.1.0 ersetzen
# Datum in metainfo.xml.in aktualisieren
```

### 2. Release-Notizen in metainfo.xml.in

Aktualisiere die Release-Beschreibung in `data/ch.srueegger.bootmate.metainfo.xml.in`:

```xml
<release version="1.1.0" date="2025-12-15">
  <description>
    <p>Neue Features und Verbesserungen</p>
    <ul>
      <li>Feature 1</li>
      <li>Feature 2</li>
      <li>Bugfix 3</li>
    </ul>
  </description>
</release>
```

### 3. Änderungen committen

```bash
git add .
git commit -m "Bump version to X.Y.Z"
```

### 4. Release-Branch erstellen (optional)

Für größere Releases empfiehlt sich ein Release-Branch:

```bash
git checkout -b release-X.Y.Z
git push -u origin release-X.Y.Z
```

Dann Pull Request erstellen und nach Review in `main` mergen.

### 5. Tag erstellen und pushen

```bash
# Auf main branch wechseln
git checkout main
git pull

# Tag erstellen
git tag -a vX.Y.Z -m "Release version X.Y.Z"

# Tag pushen
git push origin vX.Y.Z
```

### 6. GitHub Release erstellen

1. Gehe zu: https://github.com/srueegger/bootmate/releases
2. Klicke auf **"Draft a new release"**
3. Wähle den Tag: `vX.Y.Z`
4. Release Title: `Boot Mate X.Y.Z`
5. Beschreibung mit Features/Fixes
6. Klicke auf **"Publish release"**

### 7. GitHub Actions läuft automatisch

Nach dem Veröffentlichen des Releases:

1. GitHub Actions startet automatisch (`.github/workflows/release.yml`)
2. Baut DEB-Pakete für AMD64 und ARM64 mit `cargo deb`
3. Lädt die Pakete als Release Assets hoch

**Status überprüfen:**
- Gehe zu **Actions** Tab auf GitHub
- Sieh dir den "Build and Release" Workflow an

**Nach erfolgreichem Build:**
- Die Pakete erscheinen automatisch unter dem Release

---

## 🔧 Troubleshooting

### GitHub Actions Build schlägt fehl

**DEB Build Fehler:**
- Überprüfe `Cargo.toml` `[package.metadata.deb]` Sektion
- Stelle sicher, dass `build-release` Verzeichnis korrekt ist
- Überprüfe, ob alle Build-Dependencies installiert sind

---

## 📊 Status überprüfen

### GitHub Release Assets

Gehe zu: https://github.com/srueegger/bootmate/releases/tag/vX.Y.Z

Erwartete Assets:
- ✅ `bootmate_X.Y.Z_amd64.deb`
- ✅ `bootmate_X.Y.Z_arm64.deb`

---

## 🎯 Checkliste für Release

- [ ] Version in allen Dateien aktualisiert
- [ ] Release-Notizen in metainfo.xml.in geschrieben
- [ ] Änderungen committed
- [ ] Tag erstellt und gepusht
- [ ] GitHub Release veröffentlicht
- [ ] GitHub Actions erfolgreich durchgelaufen
- [ ] Release Assets vorhanden (DEB für AMD64 und ARM64)

---

## 📝 Wichtige Notizen

### Automatische Builds

- **DEB**: Wird mit `cargo deb` gebaut (nutzt Cargo.toml Konfiguration)
- **Architekturen**: AMD64 und ARM64
- **Trigger**: Automatisch bei Release-Veröffentlichung ODER manuell via "Actions" Tab

### Sandbox Permissions

Die App zeigt ein Banner an, wenn Flatpak-Berechtigungen nicht gesetzt sind:

- **Flatpak**: Zeigt `flatpak override` Befehle

### Versionierung

Boot Mate folgt Semantic Versioning:
- **MAJOR.MINOR.PATCH** (z.B. 1.2.3)
- MAJOR: Breaking Changes
- MINOR: Neue Features (backwards compatible)
- PATCH: Bugfixes

---

## 🔗 Wichtige Links

- **GitHub Repository:** https://github.com/srueegger/bootmate
- **GitHub Releases:** https://github.com/srueegger/bootmate/releases
- **GitHub Actions:** https://github.com/srueegger/bootmate/actions

---

**Letzte Aktualisierung:** 2025-11-30
**Aktuelle Version:** 1.0.0
