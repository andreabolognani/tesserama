Tesserama
=========

Membership cards manager.

It was written with a very specific use case in mind, so it will probably
not be very useful to anyone else.


Requirements
------------

Python 2 and GTK+ 3. Any relatively recent GNU/Linux distro, eg. Fedora 24
or Ubuntu 16.04 LTS, should be able to run it just fine out of the box.


Installation
------------

For all users: copy `tesserama.py` to `/usr/bin` and `tesserama.desktop` to
`/usr/share/application`.

For your user only: copy `tesserama` anywhere (`~/bin` is a good candidate),
then adjust `Exec` and `TryExec` accordingly before copying it to
`~/.local/share/applications`.

Or just run it straight from the source directory.


Limitations
-----------

Although fully working, this is basically a prototype: more specifically,
error checking is not performed nearly as extensively as it should, so for
example trying to load an invalid file will trash the application.

Some features are not discoverable, only partially by design.


Future work
-----------

Remove limitations listed above, provide a Flatpak, rewrite in Rust.


Resources
---------

The canonical Git repository can be found at

  https://git.kiyuko.org/tesserama


License
-------

This program is free software; you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation; either version 2 of the License, or (at your option) any later
version.
