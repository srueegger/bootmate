cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'src']

if get_option('profile') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'bootmate',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    cargo,
    'build',
    cargo_options,
    '&&',
    'cp',
    meson.project_build_root() / 'src' / rust_target / 'bootmate',
    '@OUTPUT@',
  ],
)
