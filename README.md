Tesserama
=========

Membership cards manager.

It was written with a very specific use case in mind, so it will
probably not be very useful to anyone else.


Requirements
------------

GTK 3 is the only runtime requirement.

The target OS is Debian 11, but Tesserama will probably work just
fine on other Linux distributions of the same vintage.


Building
--------

Tesserama can be built using `cargo`, just like you'd expect for a
Rust project. Running `make` also works.


Installing
----------

Running `make install` will install the application, as well as the
corresponding `.desktop` file, in the user's home directory.

Note that installation support is currently pretty rough, so there's
a fair chance you'll have to resort to copying files around manually.


Limitations
-----------

Error checking is not performed nearly as extensively as it should,
so for example trying to load an invalid file will trash the
application.

Some features are not discoverable, only partially by design.


Future work
-----------

Address the limitations listed above.


Resources
---------

The canonical Git repository can be found at

  https://git.kiyuko.org/tesserama


License
-------

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or (at
your option) any later version.
