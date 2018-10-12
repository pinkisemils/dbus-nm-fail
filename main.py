import dbus

bus = dbus.SystemBus()

nm = bus.get_object('org.freedesktop.NetworkManager', '/org/freedesktop/NetworkManager')
nmi = dbus.Interface(nm, dbus_interface='org.freedesktop.NetworkManager')
path = nmi.GetDeviceByIpIface('enp0s31f6') # insert the iface name of your choice
print('path {}'.format(path))

dev = bus.get_object('org.freedesktop.NetworkManager', path)
devi = dbus.Interface(dev, dbus_interface='org.freedesktop.NetworkManager.Device')
conf, version = devi.GetAppliedConnection(0)
result = devi.Reapply(conf, version, 0)
print(result)

