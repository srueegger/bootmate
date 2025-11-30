project(
  'bootmate',
  'rust',
  version: '1.1.0',
  license: 'GPL-2.0-only',
  meson_version: '>= 0.59.0',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'ch.srueegger.bootmate'

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.10')
dependency('libadwaita-1', version: '>= 1.5')

cargo = find_program('cargo', required: true)
glib_compile_resources = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
applicationsdir = datadir / 'applications'
metainfodir = datadir / 'metainfo'

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', meson.project_name())
conf.set_quoted('LOCALEDIR', localedir)
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('APP_ID', base_id)

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
