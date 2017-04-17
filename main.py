#!/usr/bin/env python

import csv
import os.path
import sys

import gi
gi.require_version('GLib', '2.0')
gi.require_version('Gio', '2.0')
gi.require_version('Gdk', '3.0')
gi.require_version('Gtk', '3.0')
gi.require_version('Pango', '1.0')
from gi.repository import GLib, Gio, Gdk, Gtk, Pango

class Application(Gtk.Application):

	def __init__(self):

		Gtk.Application.__init__(self)

		self.set_accels_for_action("win.search", ["<Ctrl>f"])
		self.set_accels_for_action("win.open", ["<Ctrl>o"])
		self.set_accels_for_action("win.save", ["<Ctrl>s"])

		self.connect("activate", self.activate_action)

	def activate_action(self, app):

		win = ApplicationWindow()
		self.add_window(win)
		win.show_all()


class ApplicationWindow(Gtk.ApplicationWindow):

	COLUMN_DATE = 0
	COLUMN_NUMBER = 1
	COLUMN_PEOPLE = 2

	def __init__(self):

		Gtk.ApplicationWindow.__init__(self)

		self.set_default_size(800, 600)

		# An empty label will be displayed before a file has been loaded
		empty = Gtk.Label()

		# The contents will be displayed once a file has been loaded
		contents = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)

		self.searchentry = Gtk.SearchEntry()
		self.searchentry.connect("search-changed", self.search_changed)
		self.searchentry.connect("stop-search", self.stop_search)

		self.searchbar = Gtk.SearchBar()
		self.searchbar.connect_entry(self.searchentry)
		self.searchbar.add(self.searchentry)

		contents.pack_start(self.searchbar, False, False, 0)

		self.treeview = Gtk.TreeView()
		self.treeview.set_enable_search(False)

		number_renderer = Gtk.CellRendererText()
		number_renderer.set_alignment(1.0, 0.5)
		number_renderer.set_property("editable", True)
		number_renderer.connect("edited", self.number_cell_edited)
		column = Gtk.TreeViewColumn("", number_renderer, text=self.COLUMN_NUMBER)
		self.treeview.append_column(column)

		people_renderer = Gtk.CellRendererText()
		people_renderer.set_property("ellipsize", Pango.EllipsizeMode.END)
		people_renderer.set_property("editable", True)
		people_renderer.connect("edited", self.people_cell_edited)
		column = Gtk.TreeViewColumn("People", people_renderer, text=self.COLUMN_PEOPLE)
		column.set_expand(True)
		self.treeview.append_column(column)

		date_renderer = Gtk.CellRendererText()
		date_renderer.set_property("editable", True)
		date_renderer.connect("edited", self.date_cell_edited)
		column = Gtk.TreeViewColumn("Date", date_renderer, text=self.COLUMN_DATE)
		self.treeview.append_column(column)

		scrolled = Gtk.ScrolledWindow()
		scrolled.add(self.treeview)
		contents.pack_start(scrolled, True, True, 0)

		# The stack allows us to switch between application states
		self.stack = Gtk.Stack()
		self.stack.add_named(empty, "empty")
		self.stack.add_named(contents, "contents")
		self.add(self.stack)

		# The header bar will be displayed at all times
		self.headerbar = Gtk.HeaderBar()
		self.headerbar.set_show_close_button(True)
		self.set_titlebar(self.headerbar)

		self.searchbutton = Gtk.ToggleButton()
		self.searchbutton.set_image(Gtk.Image.new_from_icon_name("edit-find-symbolic", Gtk.IconSize.BUTTON))
		self.searchbutton.set_tooltip_text("Search")
		self.searchbutton.set_action_name("win.search")
		self.headerbar.pack_start(self.searchbutton)

		self.menubutton = Gtk.ToggleButton()
		self.menubutton.set_image(Gtk.Image.new_from_icon_name("open-menu-symbolic", Gtk.IconSize.BUTTON))
		self.menubutton.set_tooltip_text("Menu")
		self.menubutton.set_action_name("win.menu")
		self.headerbar.pack_end(self.menubutton)

		menu = Gio.Menu()
		menu.append("Open", "win.open")
		menu.append("Save", "win.save")

		self.menupopover = Gtk.Popover.new_from_model(self.menubutton, menu)
		self.menupopover.connect("closed", self.menu_popover_closed)

		self.searchaction = Gio.SimpleAction.new_stateful("search", None, GLib.Variant.new_boolean(False))
		self.searchaction.connect("activate", self.search_action_activated)
		self.add_action(self.searchaction)

		self.menuaction = Gio.SimpleAction.new_stateful("menu", None, GLib.Variant.new_boolean(False))
		self.menuaction.connect("activate", self.menu_action_activated)
		self.add_action(self.menuaction)

		action = Gio.SimpleAction.new("open", None)
		action.connect("activate", self.open_action_activated)
		self.add_action(action)

		action = Gio.SimpleAction.new("save", None)
		action.connect("activate", self.save_action_activated)
		self.add_action(action)

	def set_data_source(self, filename, uri):

		self.source_filename = os.path.abspath(os.path.realpath(filename))
		self.source_uri = uri

		self.headerbar.props.title = os.path.basename(self.source_filename)
		self.headerbar.props.subtitle = os.path.dirname(self.source_filename)

	def search(self):

		self.filter_needle = self.searchentry.get_text().lower()
		self.filtered_data.refilter()

	def load_data(self):

		self.data = Gtk.ListStore(str, str, str)

		with open(self.source_filename, "rb") as f:
			reader = csv.reader(f)
			for row in reader:
				self.data.append(row[0:3])

		self.filtered_data = self.data.filter_new()
		self.filter_needle = ""
		self.filtered_data.set_visible_func(self.filter_func)
		self.treeview.set_model(self.filtered_data)

		self.stack.set_visible_child_name("contents")

		recents = Gtk.RecentManager.get_default()
		recents.add_item(self.source_uri)

	def save_data(self):

		with open(self.source_filename, 'wb') as f:
			writer = csv.writer(f)
			for item in self.data:
				writer.writerow([item[0], item[1], item[2]])

	def filter_func(self, model, iter, data):

		if self.filter_needle in self.data[iter][self.COLUMN_PEOPLE].lower():
			return True

		return False


	# High-level actions

	def start_search_action(self):

		self.searchbar.set_search_mode(True)

	def stop_search_action(self):

		self.searchbar.set_search_mode(False)

	def start_menu_action(self):

		self.menupopover.popup()

	def stop_menu_action(self):

		self.menupopover.popdown()

	def open_action(self):

		dialog = Gtk.FileChooserDialog("Choose a file", self, Gtk.FileChooserAction.OPEN,
		                               (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
		                                Gtk.STOCK_OPEN, Gtk.ResponseType.OK))

		if dialog.run() == Gtk.ResponseType.OK:
			self.set_data_source(dialog.get_filename(), dialog.get_uri())
			self.load_data()

		dialog.destroy()

	def save_action(self):

		self.save_data()

	def update_number(self, path, text):

		self.data[path][self.COLUMN_NUMBER] = text

	def update_people(self, path, text):

		self.data[path][self.COLUMN_PEOPLE] = text

	def update_date(self, path, text):

		self.data[path][self.COLUMN_DATE] = text


	# Signal handlers

	def search_action_activated(self, action, param):
		state = not self.searchaction.get_state().get_boolean()
		self.searchaction.set_state(GLib.Variant.new_boolean(state))
		if state:
			self.start_search_action()
		else:
			self.stop_search_action()

	def search_changed(self, entry):
		self.search()

	def stop_search(self, entry):
		self.searchaction.set_state(GLib.Variant.new_boolean(False))

	def menu_action_activated(self, action, param):
		state = not self.menuaction.get_state().get_boolean()
		self.menuaction.set_state(GLib.Variant.new_boolean(state))
		if state:
			self.start_menu_action()
		else:
			self.stop_menu_action()

	def menu_popover_closed(self, popover):
		self.menuaction.set_state(GLib.Variant.new_boolean(False))

	def open_action_activated(self, action, param):
		self.open_action()

	def save_action_activated(self, action, param):
		self.save_action()

	def number_cell_edited(self, renderer, path, text):
		self.update_number(path, text)

	def people_cell_edited(self, renderer, path, text):
		self.update_people(path, text)

	def date_cell_edited(self, renderer, path, text):
		self.update_date(path, text)


if __name__ == '__main__':
	Application().run(sys.argv)
