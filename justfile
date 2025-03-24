name := 'frostty'
appid := 'dev.bance.frostty'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))

bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name

desktop := name + '.desktop'
desktop-src := 'extra' / 'linux' / desktop
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop

appdata := appid + '.appdata.xml'
appdata-src := 'extra' / 'linux' / appdata
appdata-dst := clean(rootdir / prefix) / 'share' / 'appdata' / appdata

icons-src := 'extra' / 'icons' / 'hicolor' / 'frostty.png'
icons-dst := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

fonts-src := 'assets' / 'fonts' / 'JetBrainsMono'
fonts-dst := clean(rootdir / prefix) / 'share' / 'fonts'

info := 'extra' / 'frostty.info'

default: build-release

clean:
    cargo clean

build-debug *args:
    cargo build {{args}}

build-release *args: (build-debug '--release' args)

run *args:
    env RUST_BACKTRACE=full cargo run --release {{args}}

install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}
    install -Dm0644 {{appdata-src}} {{appdata-dst}}
    install -Dm0644 {{icons-src}} {{icons-dst}}
    sudo cp -r {{fonts-src}} {{fonts-dst}}
    tic -x {{info}}
    fc-cache -fv

update:
    install -Dm0755 {{bin-src}} {{bin-dst}}

uninstall:
    rm {{bin-dst}} {{desktop-dst}} {{icons-dst}}
