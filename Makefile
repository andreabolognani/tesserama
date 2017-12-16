NULL=

builddir=flatpak/build
repodir=flatpak/repo

ifneq ($(ARCH),)
arch= \
	--arch=$(ARCH) \
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
	--filesystem=home \
	$(NULL)

all:
	flatpak build-init $(arch) $(sdkextensions) $(builddir) org.kiyuko.Tesserama org.gnome.Sdk org.gnome.Platform 3.26
	flatpak build $(builddir) sh -c 'source /usr/lib/sdk/rust-stable/enable.sh && cargo build --release'
	flatpak build $(builddir) install -m 0755 -d /app/bin
	flatpak build $(builddir) install -m 0755 -d /app/share/applications
	flatpak build $(builddir) install -m 0755 target/release/tesserama /app/bin
	flatpak build $(builddir) install -m 0644 tesserama.desktop /app/share/applications/org.kiyuko.Tesserama.desktop
	flatpak build-finish $(builddir) $(sockets) $(shares) $(filesystems) --command=tesserama
	flatpak build-export $(repodir) $(builddir)
	rm -rf $(builddir)

clean:
	rm -rf $(builddir)
