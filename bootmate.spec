Name:           bootmate
Version:        1.2.0
Release:        1%{?dist}
Summary:        A modern GNOME application for managing autostart entries

License:        GPL-2.0-only
URL:            https://github.com/srueegger/bootmate
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  meson >= 0.59.0
BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  pkgconfig(glib-2.0) >= 2.66
BuildRequires:  pkgconfig(gio-2.0) >= 2.66
BuildRequires:  pkgconfig(gtk4) >= 4.10
BuildRequires:  pkgconfig(libadwaita-1) >= 1.5
BuildRequires:  gettext
BuildRequires:  desktop-file-utils
BuildRequires:  libappstream-glib

Requires:       gtk4 >= 4.10
Requires:       libadwaita >= 1.5
Requires:       glib2 >= 2.66

%description
Boot Mate is a modern GNOME application for managing autostart entries,
built with Rust and libadwaita. It allows you to view, edit, and manage
autostart entries from both user and system directories.

Features:
- View all autostart entries from user and system directories
- Edit autostart entry commands and parameters
- Delete or disable autostart entries
- Multi-language support (English and German)
- Follows GNOME Human Interface Guidelines
- Fast and lightweight, built with Rust

%prep
%autosetup

%build
%meson -Dprofile=release
%meson_build

%install
%meson_install

%check
desktop-file-validate %{buildroot}%{_datadir}/applications/ch.srueegger.bootmate.desktop
appstream-util validate-relax --nonet %{buildroot}%{_datadir}/metainfo/ch.srueegger.bootmate.metainfo.xml

%files
%license LICENSE
%doc README.md
%{_bindir}/bootmate
%{_datadir}/applications/ch.srueegger.bootmate.desktop
%{_datadir}/metainfo/ch.srueegger.bootmate.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/ch.srueegger.bootmate.svg
%{_datadir}/icons/hicolor/symbolic/apps/ch.srueegger.bootmate-symbolic.svg
%{_datadir}/bootmate/
%{_datadir}/locale/de/LC_MESSAGES/bootmate.mo
%{_datadir}/locale/en/LC_MESSAGES/bootmate.mo
%{_datadir}/glib-2.0/schemas/ch.srueegger.bootmate.gschema.xml

%changelog
* Sat Jan 04 2026 Samuel Rüegger <samuel@rueegger.me> - 1.2.0-1
- Initial RPM package for Fedora 43
