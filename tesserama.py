#!/usr/bin/env python

# Tesserama - Simple membership cards manager
# Copyright (C) 2017  Andrea Bolognani <eof@kiyuko.org>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

import csv
import datetime
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

		GLib.set_application_name("Tesserama")

		self.set_accels_for_action("win.search", ["<Ctrl>f"])
		self.set_accels_for_action("win.insert", ["<Ctrl>i"])
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
	COLUMN_SIGNATURE = 3

	def __init__(self):

		Gtk.ApplicationWindow.__init__(self)

		# Though formally deprecated, set_wmclass() seems to be the only way
		# to reliably convince GNOME Shell to display the proper application
		# name in the topbar instead of "tesserama.py", so it's staying :)
		self.set_wmclass("Tesserama", "Tesserama")
		self.set_title("Tesserama")

		self.set_default_size(800, 600)
		self.connect("delete-event", self.delete_event)

		# Internal state

		self.source_filename = ""
		self.source_uri = ""
		self.dirty = False

		# Actions

		self.searchaction = Gio.SimpleAction.new_stateful("search", None, GLib.Variant.new_boolean(False))
		self.searchaction.connect("activate", self.search_action_activated)
		self.searchaction.set_enabled(False)
		self.add_action(self.searchaction)

		self.insertaction = Gio.SimpleAction.new("insert", None)
		self.insertaction.connect("activate", self.insert_action_activated)
		self.insertaction.set_enabled(False)
		self.add_action(self.insertaction)

		self.menuaction = Gio.SimpleAction.new_stateful("menu", None, GLib.Variant.new_boolean(False))
		self.menuaction.connect("activate", self.menu_action_activated)
		self.add_action(self.menuaction)

		action = Gio.SimpleAction.new("open", None)
		action.connect("activate", self.open_action_activated)
		self.add_action(action)

		self.saveaction = Gio.SimpleAction.new("save", None)
		self.saveaction.connect("activate", self.save_action_activated)
		self.saveaction.set_enabled(False)
		self.add_action(self.saveaction)

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

		signature_renderer = Gtk.CellRendererText()
		signature_renderer.set_property("editable", True)
		signature_renderer.connect("edited", self.signature_cell_edited)
		column = Gtk.TreeViewColumn("Signature", signature_renderer, text=self.COLUMN_SIGNATURE)
		column.set_sizing(Gtk.TreeViewColumnSizing.GROW_ONLY)
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

		self.insertbutton = Gtk.Button("Insert")
		self.insertbutton.set_action_name("win.insert")
		self.headerbar.pack_start(self.insertbutton)

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

	def update_title(self):

		if self.is_dirty():
				self.headerbar.props.title = "*" + os.path.basename(self.source_filename)
		else:
				self.headerbar.props.title = os.path.basename(self.source_filename)

		self.headerbar.props.subtitle = os.path.dirname(self.source_filename)

	def set_data_source(self, filename, uri):

		self.source_filename = os.path.abspath(os.path.realpath(filename))
		self.source_uri = uri

		self.update_title()

	def set_dirty(self, dirty):

		self.dirty = dirty
		self.saveaction.set_enabled(dirty)

		self.update_title()

	def is_dirty(self):

		return self.dirty

	def search(self):

		self.filter_needle = self.searchentry.get_text().lower()
		self.filtered_data.refilter()

	def load_data(self):

		self.data = Gtk.ListStore(str, str, str, str)

		with open(self.source_filename, "rb") as f:
			reader = csv.reader(f)
			for row in reader:
				if row[self.COLUMN_PEOPLE] != "":
					self.data.append(row[0:4])

		self.set_dirty(False)
		self.searchaction.set_enabled(True)
		self.insertaction.set_enabled(True)

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
				if item[self.COLUMN_PEOPLE] != "":
					writer.writerow([item[0], item[1], item[2], item[3]])

		self.set_dirty(False)

	def filter_func(self, model, iter, data):

		try:
			# If the needle can be converted to a number, we look up
			# the corresponding record
			int(self.filter_needle)
			if self.filter_needle == self.data[iter][self.COLUMN_NUMBER]:
				return True
		except ValueError:
			# In all other cases, we perform a case-insensitive substring
			# search among people's names
			if self.filter_needle in self.data[iter][self.COLUMN_PEOPLE].lower():
				return True

		return False

	# Returns True if it's okay to discard changes in the current
	# document, either because the user has confirmed by clicking
	# the relative button or because there are none
	def discard_changes_okay(self):

		ret = False

		if self.is_dirty():
			dialog = Gtk.MessageDialog(self, 0, Gtk.MessageType.QUESTION,
			                           Gtk.ButtonsType.OK_CANCEL, "Discard unsaved changes?")

			# Only proceed if the user has explicitly selected the
			# corresponding option; pressing Cancel or dismissing
			# the dialog by pressing ESC cancel the close operation
			if dialog.run() == Gtk.ResponseType.OK:
				ret = True

			dialog.destroy()

		else:
			# No unsaved changes
			ret = True

		return ret

	def convert_path(self, path):

		# Since we use filtering on the data displayed in the
		# treeview, we have to convert paths from the filtered
		# model to the actual model before using them
		tmp = Gtk.TreePath.new_from_string(path)
		tmp = self.filtered_data.convert_path_to_child_path(tmp)
		path = str(tmp)

		return path

	def update_number(self, path, text):

		path = self.convert_path(path)

		if self.data[path][self.COLUMN_NUMBER] != text:
			self.data[path][self.COLUMN_NUMBER] = text
			self.set_dirty(True)

	def update_people(self, path, text):

		path = self.convert_path(path)

		if self.data[path][self.COLUMN_PEOPLE] != text:
			self.data[path][self.COLUMN_PEOPLE] = text
			self.set_dirty(True)

	def update_signature(self, path, text):

		path = self.convert_path(path)

		if self.data[path][self.COLUMN_SIGNATURE] != text:
			self.data[path][self.COLUMN_SIGNATURE] = text
			self.set_dirty(True)

	def update_date(self, path, text):

		path = self.convert_path(path)

		if self.data[path][self.COLUMN_DATE] != text:
			self.data[path][self.COLUMN_DATE] = text
			self.set_dirty(True)


	# High-level actions

	def start_search_action(self):

		self.searchbar.set_search_mode(True)
		self.insertaction.set_enabled(False)

	def stop_search_action(self):

		self.searchbar.set_search_mode(False)
		self.insertaction.set_enabled(True)

	def insert_action(self):

		number = 1
		for item in self.data:
			try:
				number = max(number, int(item[self.COLUMN_NUMBER]) + 1)
			except ValueError:
				# Invalid values are ignored
				pass

		# Fill in some sensible data: the next number in the
		# sequence and today's date
		fresh = ["", "", "", ""]
		fresh[self.COLUMN_NUMBER] = str(number)
		fresh[self.COLUMN_DATE] = datetime.date.today().strftime("%d/%m/%y")

		# Insert the fresh data and scroll to it
		cell = self.data.append(fresh)
		self.treeview.scroll_to_cell(self.data.get_path(cell), None, False, 0.0, 0.0)

	def start_menu_action(self):

		self.menupopover.show()

	def stop_menu_action(self):

		self.menupopover.hide()

	def open_action(self):

		# Don't overwrite changes unless the user is okay with that
		if not self.discard_changes_okay():
			return

		dialog = Gtk.FileChooserDialog("Choose a file", self, Gtk.FileChooserAction.OPEN,
		                               (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
		                                Gtk.STOCK_OPEN, Gtk.ResponseType.OK))

		if dialog.run() == Gtk.ResponseType.OK:
			self.set_data_source(dialog.get_filename(), dialog.get_uri())
			self.load_data()

		dialog.destroy()

	def save_action(self):

		self.save_data()

	def close_action(self):

		# False means we want to close the window, True means
		# we don't, so we have to flip the result here
		return not self.discard_changes_okay()


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
		# A bit redundant, but guarantees we perform the same teardown
		# steps regardless of how the search has been interrupted
		self.stop_search_action()

	def insert_action_activated(self, action, param):
		self.insert_action()

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

	def signature_cell_edited(self, renderer, path, text):
		self.update_signature(path, text)

	def date_cell_edited(self, renderer, path, text):
		self.update_date(path, text)

	def delete_event(self, widget, event):
		return self.close_action()


if __name__ == '__main__':
	Application().run(sys.argv)