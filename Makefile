NULL=

builddir=flatpak/build
repodir=flatpak/repo

src=src/main.rs
bin=$(builddir)/files/bin/tesserama

ifneq ($(ARCH),)
arch= \
	--arch=$(ARCH) \
	$(NULL)
endif
ifeq ($(DEBUG),1)
release=
else
release= \
	--release \
	$(NULL)
endif
ifeq ($(GPGSIGN),0)
gpgsign=
else
gpgsign= \
	--gpg-sign=$(KEYID) \
	$(NULL)
endif
sdkextensions= \
	--sdk-extension=org.freedesktop.Sdk.Extension.rust-stable \
	$(NULL)
sockets= \
	--socket=session-bus \
	--socket=wayland \
	--socket=x11 \
	$(NULL)
shares= \
	--share=ipc \
	$(NULL)
filesystems= \
	--filesystem=host \
	$(NULL)

all: $(bin)

$(builddir):
	flatpak build-init $(arch) $(sdkextensions) $(builddir) org.kiyuko.Tesserama org.gnome.Sdk org.gnome.Platform 3.26

$(bin): $(builddir) $(src)
	flatpak build $(builddir) sh -c 'source /usr/lib/sdk/rust-stable/enable.sh && cargo build $(release)'
	flatpak build $(builddir) install -m 0755 -d /app/bin
	flatpak build $(builddir) install -m 0755 -d /app/share/applications
	flatpak build $(builddir) install -m 0755 target/release/tesserama /app/bin
	flatpak build $(builddir) install -m 0644 tesserama.desktop /app/share/applications/org.kiyuko.Tesserama.desktop

run: $(bin)
	flatpak build $(sockets) $(shares) $(filesystems) $(builddir) sh -c 'RUST_BACKTRACE=1 tesserama'

publish: $(bin)
	flatpak build-finish $(sockets) $(shares) $(filesystems) --command=tesserama $(builddir)
	flatpak build-export $(gpgsign) $(repodir) $(builddir)
	rm -rf $(builddir)

clean:
	rm -rf $(builddir)

.PHONY: all run publish clean
