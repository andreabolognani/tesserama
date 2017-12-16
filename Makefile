NULL=

builddir=flatpak/build
repodir=flatpak/repo

FLATPAK=flatpak
CARGO=~/.cargo/bin/cargo

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
	$(FLATPAK) build-init $(builddir) org.kiyuko.Tesserama org.gnome.Sdk org.gnome.Platform 3.26
	$(FLATPAK) build $(builddir) $(CARGO) build --release
	$(FLATPAK) build $(builddir) install -m 0755 -d /app/bin
	$(FLATPAK) build $(builddir) install -m 0755 -d /app/share/applications
	$(FLATPAK) build $(builddir) install -m 0755 target/release/tesserama /app/bin
	$(FLATPAK) build $(builddir) install -m 0644 tesserama.desktop /app/share/applications/org.kiyuko.Tesserama.desktop
	$(FLATPAK) build-finish $(builddir) $(SOCKETS) $(SHARES) $(FILESYSTEMS) --command=tesserama
	$(FLATPAK) build-export $(repodir) $(builddir)
	rm -rf $(builddir)

clean:
	rm -rf $(builddir) $(repodir)
