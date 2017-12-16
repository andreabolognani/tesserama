NULL=

builddir=flatpak/build
repodir=flatpak/repo

FLATPAK=flatpak
CARGO=cargo

ifneq ($(ARCH),)
arch= \
	--arch=$(ARCH) \
	$(NULL)
endif
SDK_EXTENSIONS= \
	--sdk-extension=org.freedesktop.Sdk.Extension.rust-stable \
	$(NULL)
SOCKETS= \
	--socket=session-bus \
	--socket=wayland \
	--socket=x11 \
	$(NULL)
SHARES= \
	--share=ipc \
	$(NULL)
FILESYSTEMS= \
	--filesystem=home \
	$(NULL)

all:
	$(FLATPAK) build-init $(arch) $(SDK_EXTENSIONS) $(builddir) org.kiyuko.Tesserama org.gnome.Sdk org.gnome.Platform 3.26
	$(FLATPAK) build $(builddir) sh -c 'source /usr/lib/sdk/rust-stable/enable.sh && $(CARGO) build --release'
	$(FLATPAK) build $(builddir) install -m 0755 -d /app/bin
	$(FLATPAK) build $(builddir) install -m 0755 -d /app/share/applications
	$(FLATPAK) build $(builddir) install -m 0755 target/release/tesserama /app/bin
	$(FLATPAK) build $(builddir) install -m 0644 tesserama.desktop /app/share/applications/org.kiyuko.Tesserama.desktop
	$(FLATPAK) build-finish $(builddir) $(SOCKETS) $(SHARES) $(FILESYSTEMS) --command=tesserama
	$(FLATPAK) build-export $(repodir) $(builddir)
	rm -rf $(builddir)

clean:
	rm -rf $(builddir)
