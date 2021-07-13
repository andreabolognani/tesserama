appname = tesserama
appid = org.kiyuko.Tesserama

bindir = ${HOME}/.local/bin
datadir = ${HOME}/.local/share

outdir = target/debug

all: binary data

binary:
	cargo build

data:
	sed -E \
		's,@bindir@,$(bindir),g' \
		$(appid).desktop.in \
		>$(outdir)/$(appid).desktop

install: binary data
	mkdir -p \
		$(bindir)/ \
		$(datadir)/applications/
	install -m 0755 \
		$(outdir)/$(appname) \
		$(bindir)/
	install -m 0644 \
		$(outdir)/$(appid).desktop \
		$(datadir)/applications/

clean:
	rm -rf $(outdir)/

.PHONY: all binary data install clean
