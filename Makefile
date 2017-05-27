builddir=flatpak/build
repodir=flatpak/repo

FLATPAK=flatpak
CARGO=~/.cargo/bin/cargo

all:
	$(FLATPAK) build-init $(builddir) org.kiyuko.Tesserama org.gnome.Sdk org.gnome.Platform 3.22
	$(FLATPAK) build $(builddir) $(CARGO) build
	$(FLATPAK) build $(builddir) install -m 0755 -d /app/bin
	$(FLATPAK) build $(builddir) install -m 0755 target/debug/tesserama /app/bin
	$(FLATPAK) build-finish $(builddir) --socket=wayland --socket=x11 --share=ipc --command=tesserama
	$(FLATPAK) build-export $(repodir) $(builddir)

clean:
	rm -rf $(builddir) $(repodir)
