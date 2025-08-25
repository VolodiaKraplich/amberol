pkgname=amberol
pkgver=$(grep '^version' Cargo.toml | head -n 1 | cut -d '"' -f 2)
pkgrel=1
pkgdesc='Small and simple sound and music player that is well integrated with GNOME'
arch=('x86_64')
license=('GPL-3.0-or-later')
options=('!debug')
makedepends=(
    'rust'
    'cargo'
)
depends=(
    'dconf'
    'gcc-libs'
    'gdk-pixbuf2'
    'glib2'
    'glibc'
    'graphene'
    'gst-plugins-bad-libs'
    'gst-plugins-base'
    'gst-plugins-base-libs'
    'gst-plugins-good'
    'gstreamer'
    'gtk4'
    'hicolor-icon-theme'
    'libadwaita'
    'pango'
)
optdepends=(
    'gst-libav: Extra media codecs'
    'gst-plugins-bad: Extra media codecs'
    'gst-plugins-ugly: Extra media codecs'
)

build() {
    cargo build --release --locked
}

package() {
    cd ..

    # Create necessary directories
    install -d "${pkgdir}/usr/bin"
    install -d "${pkgdir}/usr/share/applications"
    install -d "${pkgdir}/usr/share/glib-2.0/schemas"

    # Install the application binary
    install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"

    # Install GSettings schema
    install -Dm644 "data/io.bassi.Amberol.gschema.xml" \
        "${pkgdir}/usr/share/glib-2.0/schemas/io.bassi.Amberol.gschema.xml"

    # Install application icons
    if [ -d "data/icons" ]; then
        cp -r data/icons/* "${pkgdir}/usr/share/icons/"
        echo "Installed all icons from data/icons/"
    fi

    # Create and install the .desktop file
    cat <<EOF > "${pkgdir}/usr/share/applications/${pkgname}.desktop"
[Desktop Entry]
Name=Amberol
GenericName=Music Player
TryExec=amberol
Exec=amberol %U
Icon=io.bassi.Amberol
Terminal=false
Type=Application
Categories=GNOME;GTK;Music;Audio;AudioVideo;
# Translators: Search terms to find this application. Do NOT translate or localize the semicolons! The list MUST also end with a semicolon!
Keywords=music;player;media;audio;playlist;
StartupNotify=true
X-SingleMainWindow=true
X-Purism-FormFactor=Workstation;Mobile;
DBusActivatable=true
StartupWMClass=io.bassi.Amberol
MimeType=audio/mpeg;audio/wav;audio/x-aac;audio/x-aiff;audio/x-ape;audio/x-flac;audio/x-m4a;audio/x-m4b;audio/x-mp1;audio/x-mp2;audio/x-mp3;audio/x-mpg;audio/x-mpeg;audio/x-mpegurl;audio/x-opus+ogg;audio/x-pn-aiff;audio/x-pn-au;audio/x-pn-wav;audio/x-speex;audio/x-vorbis;audio/x-vorbis+ogg;audio/x-wavpack;inode/directory;
EOF
}

# Post-install hook to compile GSettings schemas
post_install() {
    glib-compile-schemas /usr/share/glib-2.0/schemas
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor
    update-desktop-database -q /usr/share/applications
}

# Post-upgrade hook
post_upgrade() {
    post_install
}

# Post-remove hook to clean up
post_remove() {
    glib-compile-schemas /usr/share/glib-2.0/schemas
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor
    update-desktop-database -q /usr/share/applications
}
