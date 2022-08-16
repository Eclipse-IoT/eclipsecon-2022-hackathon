#!/usr/bin/env python3
import blemesh
try:
  from gi.repository import GLib
except ImportError:
  import glib as GLib
from dbus.mainloop.glib import DBusGMainLoop
import dbus
import dbus.service
import dbus.exceptions
import sys
import os

def main():
	blemesh.configure_logging("device")

	DBusGMainLoop(set_as_default=True)
	blemesh.bus = dbus.SystemBus()

	blemesh.mesh_net = dbus.Interface(blemesh.bus.get_object(blemesh.MESH_SERVICE_NAME,
						"/org/bluez/mesh"),
						blemesh.MESH_NETWORK_IFACE)

	blemesh.mesh_net.connect_to_signal('InterfacesRemoved', blemesh.interfaces_removed_cb)

	blemesh.app = blemesh.Application(blemesh.bus)

	# Provisioning agent
	blemesh.app.set_agent(blemesh.Agent(blemesh.bus))

	first_ele = blemesh.Element(blemesh.bus, 0x00)
	first_ele.add_model(blemesh.Model(0x1000))
	first_ele.add_model(blemesh.Model(0x100C))
	first_ele.add_model(blemesh.Model(0x1101))

	second_ele = blemesh.Element(blemesh.bus, 0x01)
	second_ele.add_model(blemesh.Model(0x1001))

	third_ele = blemesh.Element(blemesh.bus, 0x02)
	third_ele.add_model(blemesh.Model(0x1001))

	blemesh.app.add_element(first_ele)
	blemesh.app.add_element(second_ele)
	blemesh.app.add_element(third_ele)

	blemesh.mainloop = GLib.MainLoop()

	blemesh.join()
	blemesh.mainloop.run()


if __name__ == '__main__':
	main()
