pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

blueprints = custom_target(
  'blueprints',
  input: files(
    'ui/shortcuts.blp',
    'ui/initial_setup.blp',
    'ui/views/main.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

gnome.compile_resources(
  'io.github.rdbende.Mona',
  'io.github.rdbende.Mona.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
  dependencies: blueprints,
)

desktop_file = i18n.merge_file(
        input: 'io.github.rdbende.Mona.desktop.in',
       output: 'io.github.rdbende.Mona.desktop',
         type: 'desktop',
       po_dir: '../po',
      install: true,
  install_dir: get_option('datadir') / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_file = i18n.merge_file(
        input: 'io.github.rdbende.Mona.metainfo.xml.in',
       output: 'io.github.rdbende.Mona.metainfo.xml',
       po_dir: '../po',
      install: true,
  install_dir: get_option('datadir') / 'metainfo'
)

appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test('Validate appstream file', appstreamcli,
     args: ['validate', '--no-net', '--explain', appstream_file])

install_data('io.github.rdbende.Mona.gschema.xml',
  install_dir: get_option('datadir') / 'glib-2.0' / 'schemas'
)

compile_schemas = find_program('glib-compile-schemas', required: false, disabler: true)
test('Validate schema file',
     compile_schemas,
     args: ['--strict', '--dry-run', meson.current_source_dir()])

subdir('icons')
