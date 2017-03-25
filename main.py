#!/usr/bin/env python

import os.path
import sys

import gi
gi.require_version('Gio', '2.0')
gi.require_version('Gdk', '3.0')
gi.require_version('Gtk', '3.0')
from gi.repository import Gio, Gdk, Gtk

class Application(Gtk.Application):

	def __init__(self):

		Gtk.Application.__init__(self)

		self.set_accels_for_action("win.open", ["<Ctrl>o"])
		self.set_accels_for_action("win.save", ["<Ctrl>s"])

		self.connect("activate", self.activate_action)

	def activate_action(self, app):

		win = ApplicationWindow()
		self.add_window(win)
		win.show_all()


class ApplicationWindow(Gtk.ApplicationWindow):

	def __init__(self):

		Gtk.ApplicationWindow.__init__(self)

		self.set_default_size(800, 600)

		box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)

		self.searchentry = Gtk.SearchEntry()
		self.searchentry.connect("search-changed", self.search_changed)
		self.searchentry.connect("stop-search", self.stop_search)

		self.searchbar = Gtk.SearchBar()
		self.searchbar.connect_entry(self.searchentry)
		self.searchbar.add(self.searchentry)

		box.pack_start(self.searchbar, False, False, 0)

		self.treeview = Gtk.TreeView()

		renderer = Gtk.CellRendererText()
		column = Gtk.TreeViewColumn("Text", renderer, text=0)
		self.treeview.append_column(column)

		scrolled = Gtk.ScrolledWindow()
		scrolled.add(self.treeview)
		box.pack_start(scrolled, True, True, 0)

		self.add(box)

		self.headerbar = Gtk.HeaderBar()
		self.headerbar.set_show_close_button(True)
		self.set_titlebar(self.headerbar)

		self.searchbutton = Gtk.ToggleButton()
		self.searchbutton.set_image(Gtk.Image.new_from_icon_name("edit-find-symbolic", Gtk.IconSize.BUTTON))
		self.searchbutton.set_tooltip_text("Search")
		self.searchbutton.connect("toggled", self.search_button_toggled)
		self.headerbar.pack_start(self.searchbutton)

		self.menubutton = Gtk.ToggleButton()
		self.menubutton.set_image(Gtk.Image.new_from_icon_name("open-menu-symbolic", Gtk.IconSize.BUTTON))
		self.menubutton.set_tooltip_text("Menu")
		self.menubutton.connect("toggled", self.menu_button_toggled)
		self.headerbar.pack_end(self.menubutton)

		menu = Gio.Menu()
		menu.append("Open", "win.open")
		menu.append("Save", "win.save")

		self.menupopover = Gtk.Popover.new_from_model(self.menubutton, menu)
		self.menupopover.connect("closed", self.menu_popover_closed)

		action = Gio.SimpleAction.new("open", None)
		action.connect("activate", self.open_action_activated)
		self.add_action(action)

		action = Gio.SimpleAction.new("save", None)
		action.connect("activate", self.save_action_activated)
		self.add_action(action)

		accel = Gtk.AccelGroup()
		accel.connect(Gdk.keyval_from_name('f'), Gdk.ModifierType.CONTROL_MASK, 0,
		              self.search_shortcut_activated)
		self.add_accel_group(accel)


	def set_filename(self, filename):

		self.filename = os.path.abspath(os.path.realpath(filename))

		self.headerbar.props.title = os.path.basename(self.filename)
		self.headerbar.props.subtitle = os.path.dirname(self.filename)

	def search(self, searchentry):

		filtered = Gtk.ListStore(str)
		needle = searchentry.get_text().lower()

		for item in self.data:
			if needle in item[0].lower():
				filtered.append([item[0]])

		self.treeview.set_model(filtered)

	def load_data(self):

		self.data = Gtk.ListStore(str)

		with open(self.filename, "rb") as f:
			for line in f:
				self.data.append([line.strip()])

		self.treeview.set_model(self.data)

	def save_data(self):

		with open(self.filename, 'wb') as f:
			for item in self.data:
				f.write(item[0] + "\n")


	# High-level actions

	def start_search_action(self):

		self.searchbar.set_search_mode(True)
		self.searchbutton.set_active(True)

	def stop_search_action(self):

		self.searchbutton.set_active(False)
		self.searchbar.set_search_mode(False)

	def start_menu_action(self):

		self.menupopover.popup()
		self.menubutton.set_active(True)

	def stop_menu_action(self):

		self.menubutton.set_active(False)
		self.menupopover.popdown()

	def open_action(self):

		dialog = Gtk.FileChooserDialog("Choose a file", self, Gtk.FileChooserAction.OPEN,
		                               (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
		                                Gtk.STOCK_OPEN, Gtk.ResponseType.OK))

		if dialog.run() == Gtk.ResponseType.OK:
			self.set_filename(dialog.get_filename())
			self.load_data()

		dialog.destroy()

	def save_action(self):

		self.save_data()


	# Signal handlers

	def search_button_toggled(self, button):
		if self.searchbutton.get_active():
			self.start_search_action()
		else:
			self.stop_search_action()

	def search_changed(self, entry):
		self.search()

	def stop_search(self, entry):
		self.stop_search_action()

	def menu_button_toggled(self, button):
		if self.menubutton.get_active():
			self.start_menu_action()
		else:
			self.stop_menu_action()

	def menu_popover_closed(self, popover):
		self.stop_menu_action()

	def search_shortcut_activated(self, accel, obj, key, mod):
		self.start_search_action()

	def open_action_activated(self, action, param):
		self.open_action()

	def save_action_activated(self, action, param):
		self.save_action()


if __name__ == '__main__':
	Application().run(sys.argv)
