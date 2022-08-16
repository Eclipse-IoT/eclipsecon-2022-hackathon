import sys
import struct
import fcntl
import os
import numpy
import random
import dbus
import dbus.service
import dbus.exceptions

from threading import Timer
import time
import uuid
import logging
import logging.handlers
from random import randrange, uniform

try:
  from gi.repository import GLib
except ImportError:
  import glib as GLib
from dbus.mainloop.glib import DBusGMainLoop

# Provisioning agent
try:
  import agent
except ImportError:
  agent = None

MESH_SERVICE_NAME = 'org.bluez.mesh'
DBUS_PROP_IFACE = 'org.freedesktop.DBus.Properties'
DBUS_OM_IFACE = 'org.freedesktop.DBus.ObjectManager'

MESH_MGR_IFACE = 'org.bluez.mesh.Management1'
MESH_NETWORK_IFACE = 'org.bluez.mesh.Network1'
MESH_NODE_IFACE = 'org.bluez.mesh.Node1'
MESH_APPLICATION_IFACE = 'org.bluez.mesh.Application1'
MESH_ELEMENT_IFACE = 'org.bluez.mesh.Element1'

AGENT_IFACE = 'org.bluez.mesh.ProvisionAgent1'
AGENT_PATH = "/mesh/test/agent"

APP_COMPANY_ID = 0x05f1
APP_PRODUCT_ID = 0x0001
APP_VERSION_ID = 0x0001

VENDOR_ID_NONE = 0xffff

TRANSACTION_TIMEOUT = 6

app = None
bus = None
mainloop = None
node = None
node_mgr = None
mesh_net = None
log = None

dst_addr = 0x0000
app_idx = 0

# Node token housekeeping
token = None
have_token = False
attached = False

# Remote device UUID
have_uuid = False
remote_uuid = None

# Menu housekeeping
MAIN_MENU = 0
ON_OFF_CLIENT_MENU = 1

INPUT_NONE = 0
INPUT_TOKEN = 1
INPUT_DEST_ADDRESS = 2
INPUT_APP_KEY_INDEX = 3
INPUT_MESSAGE_PAYLOAD = 4
INPUT_UUID = 5

menus = []
current_menu = None

user_input = 0
input_error = False

send_opts = dbus.Dictionary(signature='sv')
send_opts = {'ForceSegmented' : dbus.Boolean(True)}

def raise_error(str_value):
	global input_error

	input_error = True
	print(set_error(str_value))

def clear_error():
	global input_error
	input_error = False

def is_error():
	return input_error

def app_exit():
	global mainloop
	global app

	for el in app.elements:
		for model in el.models:
			if model.timer != None:
				model.timer.cancel()
	mainloop.quit()

def configure_logging(name):
	global log
	# Change root logger level from WARNING (default) to NOTSET in order for all messages to be delegated.
	logging.getLogger().setLevel(logging.NOTSET)

	# Add stdout handler, with level INFO
	console = logging.StreamHandler(sys.stdout)
	console.setLevel(logging.INFO)
	formater = logging.Formatter('%(asctime)s %(name)-13s: %(levelname)-8s %(message)s')
	console.setFormatter(formater)
	logging.getLogger().addHandler(console)

	# Add file rotating handler, with level DEBUG
	rotatingHandler = logging.handlers.RotatingFileHandler(filename='logs/' + name + '.log', maxBytes=100000, backupCount=5)
	rotatingHandler.setLevel(logging.DEBUG)
	formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
	rotatingHandler.setFormatter(formatter)
	logging.getLogger().addHandler(rotatingHandler)

	log = logging.getLogger("app." + name)

def set_token(str_value):
	global token
	global have_token

	if len(str_value) != 16:
		raise_error('Expected 16 digits')
		return

	try:
		input_number = int(str_value, 16)
	except ValueError:
		log.error('Not a valid hexadecimal number')
		return

	token = numpy.uint64(input_number)
	have_token = True

def set_uuid(str_value):
	global remote_uuid
	global have_uuid

	if len(str_value) != 32:
		raise_error('Expected 32 digits')
		return

	remote_uuid = bytearray.fromhex(str_value)
	have_uuid = True

def array_to_string(b_array):
	str_value = ""
	for b in b_array:
		str_value += "%02x" % b
	return str_value

def generic_error_cb(error):
	log.error('D-Bus call failed: ' + str(error))

def generic_reply_cb():
	return

def attach_app_error_cb(error):
	log.error('Failed to register application: ' + str(error))

def attach(token):
	log.info('Attach mesh node to bluetooth-meshd daemon')

	mesh_net.Attach(app.get_path(), token,
					reply_handler=attach_app_cb,
					error_handler=attach_app_error_cb)

def join():
	uuid_bytes = uuid.uuid4().bytes
	uuid_str = array_to_string(uuid_bytes)

	log.info('Joining with UUID:  ' + uuid_str)
	mesh_net.Join(app.get_path(), uuid_bytes,
		reply_handler=join_cb,
		error_handler=join_error_cb)

def join_cb():
	log.info('Join procedure started')

def join_error_cb(reason):
	log.info('Join procedure failed: %s', reason)

def remove_node_cb():
	global attached
	global have_token

	log.info('Node removed')
	attached = False
	have_token = False

def unwrap(item):
	if isinstance(item, dbus.Boolean):
		return bool(item)
	if isinstance(item, (dbus.UInt16, dbus.Int16, dbus.UInt32, dbus.Int32,
						dbus.UInt64, dbus.Int64)):
		return int(item)
	if isinstance(item, dbus.Byte):
		return bytes([int(item)])
	if isinstance(item, dbus.String):
			return item
	if isinstance(item, (dbus.Array, list, tuple)):
		return [unwrap(x) for x in item]
	if isinstance(item, (dbus.Dictionary, dict)):
		return dict([(unwrap(x), unwrap(y)) for x, y in item.items()])

	log.error('Dictionary item not handled: ' + type(item))

	return item

def attach_app_cb(node_path, dict_array):
	global attached

	attached = True

	log.info('Mesh app registered: ' + node_path)

	obj = bus.get_object(MESH_SERVICE_NAME, node_path)

	global node_mgr
	node_mgr = dbus.Interface(obj, MESH_MGR_IFACE)

	global node
	node = dbus.Interface(obj, MESH_NODE_IFACE)

	els = unwrap(dict_array)

	for el in els:
		idx = struct.unpack('b', el[0])[0]

		models = el[1]
		element = app.get_element(idx)
		element.set_model_config(models)

def interfaces_removed_cb(object_path, interfaces):
	log.info('Removed')
	if not mesh_net:
		return

	print(object_path)
	if object_path == mesh_net[2]:
		log.info('Service was removed')
		app_exit()

def get_state_str(state):
	if state == 0:
		return 'OFF'
	elif state == 1:
		return 'ON'
	else:
		return 'UNKNOWN'

def print_state(state):
	print('State is ', end='')
	print(get_state_str(state))

class ModTimer():
	def __init__(self):
		self.seconds = None
		self.func = None
		self.thread = None
		self.busy = False

	def _timeout_cb(self):
		self.func()
		self.busy = True
		self._schedule_timer()
		self.busy =False

	def _schedule_timer(self):
		self.thread = Timer(self.seconds, self._timeout_cb)
		self.thread.start()

	def start(self, seconds, func):
		self.func = func
		self.seconds = seconds
		if not self.busy:
			self._schedule_timer()

	def cancel(self):
		if self.thread is not None:
			self.thread.cancel()
			self.thread = None

class Agent(dbus.service.Object):
	def __init__(self, bus):
		self.path = AGENT_PATH
		self.bus = bus
		dbus.service.Object.__init__(self, bus, self.path)

	def get_properties(self):
		caps = []
		oob = []
		caps.append('out-numeric')
		caps.append('static-oob')
		oob.append('other')
		return {
			AGENT_IFACE: {
				'Capabilities': dbus.Array(caps, 's'),
				'OutOfBandInfo': dbus.Array(oob, 's')
			}
		}

	def get_path(self):
		return dbus.ObjectPath(self.path)

	@dbus.service.method(AGENT_IFACE, in_signature="", out_signature="")
	def Cancel(self):
		log("Cancel")

	@dbus.service.method(AGENT_IFACE, in_signature="su", out_signature="")
	def DisplayNumeric(self, type, value):
		print('DisplayNumeric (', type,') number =', value)

	@dbus.service.method(AGENT_IFACE, in_signature="s", out_signature="ay")
	def PromptStatic(self, type):
		static_key = numpy.random.randint(0, 255, 16)
		key_str = array_to_string(static_key)
		print('PromptStatic (', type, ')')
		print('Enter 16 octet key on remote device: ',key_str);
		return dbus.Array(static_key, signature='y')

class Application(dbus.service.Object):

	def __init__(self, bus):
		self.path = '/simulator/application'
		self.agent = None
		self.elements = []
		dbus.service.Object.__init__(self, bus, self.path)

	def set_agent(self, agent):
		self.agent = agent

	def get_path(self):
		return dbus.ObjectPath(self.path)

	def add_element(self, element):
		self.elements.append(element)

	def get_element(self, idx):
		for ele in self.elements:
			if ele.get_index() == idx:
				return ele

	def get_properties(self):
		return {
			MESH_APPLICATION_IFACE: {
				'CompanyID': dbus.UInt16(APP_COMPANY_ID),
				'ProductID': dbus.UInt16(APP_PRODUCT_ID),
				'VersionID': dbus.UInt16(APP_VERSION_ID)
			}
		}

	@dbus.service.method(DBUS_OM_IFACE, out_signature='a{oa{sa{sv}}}')
	def GetManagedObjects(self):
		response = {}
		response[self.path] = self.get_properties()
		response[self.agent.get_path()] = self.agent.get_properties()
		for element in self.elements:
			response[element.get_path()] = element.get_properties()
		return response

	@dbus.service.method(MESH_APPLICATION_IFACE,
					in_signature="t", out_signature="")
	def JoinComplete(self, value):
		global token
		global have_token
		global attach

		log.info('Joined mesh network with token: ' + format(value, '016x'))

		token = value
		have_token = True

	@dbus.service.method(MESH_APPLICATION_IFACE,
					in_signature="s", out_signature="")
	def JoinFailed(self, value):
		log.error('JoinFailed: %s', value)


class Element(dbus.service.Object):
	PATH_BASE = '/simulator/ele'

	def __init__(self, bus, index, location):
		self.path = self.PATH_BASE + format(index, '02x')
		self.models = []
		self.bus = bus
		self.index = index
		self.location = location
		dbus.service.Object.__init__(self, bus, self.path)

	def _get_sig_models(self):
		mods = []
		for model in self.models:
			opts = []
			id = model.get_id()
			vendor = model.get_vendor()
			if vendor == VENDOR_ID_NONE:
				mod = (id, opts)
				mods.append(mod)
		return mods

	def _get_v_models(self):
		mods = []
		for model in self.models:
			opts = []
			id = model.get_id()
			v = model.get_vendor()
			if v != VENDOR_ID_NONE:
				mod = (v, id, opts)
				mods.append(mod)
		return mods

	def get_properties(self):
		vendor_models = self._get_v_models()
		sig_models = self._get_sig_models()

		props = {'Index' : dbus.Byte(self.index)}
		props['Models'] = dbus.Array(sig_models, signature='(qa{sv})')
		props['VendorModels'] = dbus.Array(vendor_models,
							signature='(qqa{sv})')
		props['Location'] = dbus.UInt16(self.location)
		return { MESH_ELEMENT_IFACE: props }

	def add_model(self, model):
		model.set_path(self.path)
		self.models.append(model)

	def get_index(self):
		return self.index

	def set_model_config(self, configs):
		for config in configs:
			mod_id = config[0]
			self.update_model_config(mod_id, config[1])

	@dbus.service.method(MESH_ELEMENT_IFACE,
					in_signature="qqvay", out_signature="")
	def MessageReceived(self, source, key, dest, data):
		if isinstance(dest, int):
			dst_str = '%04x' % dest
		elif isinstance(dest, dbus.Array):
			dst_str = array_to_string(dest)

		log.info(('Message Received on Element %02x') % self.index + ', dst=' + dst_str)

		for model in self.models:
			model.process_message(source, dest, key, data)

	@dbus.service.method(MESH_ELEMENT_IFACE,
					in_signature="qa{sv}", out_signature="")

	def UpdateModelConfiguration(self, model_id, config):
		cfg = unwrap(config)
		self.update_model_config(model_id, cfg)

	def update_model_config(self, model_id, config):
		log.info('Update Model Config ' + format(model_id, '04x'))
		for model in self.models:
			if model_id == model.get_id():
				model.set_config(config)
				return

	@dbus.service.method(MESH_ELEMENT_IFACE,
					in_signature="", out_signature="")

	def get_path(self):
		return dbus.ObjectPath(self.path)

class Model():
	def __init__(self, model_id):
		self.cmd_ops = []
		self.model_id = model_id
		self.vendor = VENDOR_ID_NONE
		self.bindings = []
		self.pub_period = 0
		self.pub_id = 0
		self.path = None
		self.timer = None

	def set_path(self, path):
		self.path = path

	def get_id(self):
		return self.model_id

	def get_vendor(self):
		return self.vendor

	def process_message(self, source, dest, key, data):
		return

	def set_publication(self, period):
		self.pub_period = period

	def send_publication(self, data):
		pub_opts = dbus.Dictionary(signature='sv')
		node.Publish(self.path, self.model_id, pub_opts, data,
						reply_handler=generic_reply_cb,
						error_handler=generic_error_cb)

	def send_message(self, dest, key, data):
		global send_opts

		node.Send(self.path, dest, key, send_opts, data,
						reply_handler=generic_reply_cb,
						error_handler=generic_error_cb)

	def set_config(self, config):
		if 'Bindings' in config:
			self.bindings = config.get('Bindings')
			log.info('Bindings: ' + str(self.bindings))
		if 'PublicationPeriod' in config:
			self.set_publication(config.get('PublicationPeriod'))
			log.info('Model publication period: ' + str(self.pub_period) + ' ms')
		if 'Subscriptions' in config:
			log.info('Model subscriptions: ' + self.subscriptions_to_str(config.get('Subscriptions')))

	def subscriptions_to_str(self, subscriptions):
		ret = ''
		for sub in subscriptions:
			if isinstance(sub, int):
				ret += ('%04x,' % sub) + ' '

			if isinstance(sub, list):
				label = uuid.UUID(bytes=b''.join(sub))
				ret += str(label) + ', '
		return ret

class ServerModel(Model):
	def t_track(self):
			self.t_timer.cancel()
			self.tid = None
			self.last_src = 0x0000
			self.last_dst = 0x0000

	def set_publication(self, period):

		self.pub_period = period
		if period == 0:
			self.pub_timer.cancel()
			return

		# We do not handle ms in this example
		if period < 1000:
			return

		self.pub_timer.start(period/1000, self.publish)

########################
# On Off Server Model
########################
class OnOffServer(ServerModel):
	def __init__(self, model_id):
		Model.__init__(self, model_id)
		self.tid = None
		self.last_src = 0x0000
		self.last_dst = 0x0000
		self.cmd_ops = { 0x8201,  # get
				 0x8202,  # set
				 0x8203,  # set unacknowledged
				 0x8204 } # status

		self.state = 0
		log.info("OnOff Server: " + get_state_str(self.state))
		self.pub_timer = ModTimer()
		self.t_timer = ModTimer()

	def process_message(self, source, dest, key, data):
		datalen = len(data)

		if datalen != 2 and datalen != 4:
			# The opcode is not recognized by this model
			return

		if datalen == 2:
			op_tuple=struct.unpack('>H',bytes(data))
			opcode = op_tuple[0]

			if opcode != 0x8201:
				# The opcode is not recognized by this model
				return
			log.info('Get state')
		elif datalen == 4:
			opcode,self.state, tid = struct.unpack('>HBB',
							       bytes(data))

			if opcode != 0x8202 and opcode != 0x8203:
				# The opcode is not recognized by this model
				return

			log.info("Set state: " + get_state_str(self.state))

			if (self.tid != None and self.tid == tid and
						self.last_src == source and
						self.last_dst == dest):
				# Ignore duplicate transaction
				return

			self.t_timer.cancel()
			self.tid = tid
			self.last_src = source
			self.last_dst = dest
			self.t_timer.start(TRANSACTION_TIMEOUT, self.t_track)

			# Unacknowledged "set"
			if opcode == 0x8203:
				return

		rsp_data = struct.pack('>HB', 0x8204, self.state)
		self.send_message(source, key, rsp_data)

	def publish(self):
		log.info('Publish: state=' + get_state_str(self.state))
		data = struct.pack('>HB', 0x8204, self.state)
		self.send_publication(data)

########################
# On Off Client Model
########################
class OnOffClient(Model):
	def __init__(self, model_id):
		Model.__init__(self, model_id)
		self.tid = 0
		self.data = None
		self.cmd_ops = { 0x8201,  # get
				 0x8202,  # set
				 0x8203,  # set unacknowledged
				 0x8204 } # status
		log.info('OnOff Client')

	def _send_message(self, dest, key, data):
		log.info('OnOffClient send command')
		self.send_message(dest, key, data)

	def get_state(self, dest, key):
		opcode = 0x8201
		self.data = struct.pack('>H', opcode)
		self._send_message(dest, key, self.data)

	def set_state(self, dest, key, state):
		opcode = 0x8202
		log.info('Set state: ' + str(state))
		self.data = struct.pack('>HBB', opcode, state, self.tid)
		self.tid = (self.tid + 1) % 255
		self._send_message(dest, key, self.data)

	def repeat(self, dest, key):
		if self.data != None:
			self._send_message(dest, key, self.data)
		else:
			log.info('No previous command stored')

	def process_message(self, source, dest, key, data):
		datalen = len(data)
		log.info('OnOffClient process message len = ' + datalen)

		if datalen != 3:
			# The opcode is not recognized by this model
			return

		opcode, state = struct.unpack('>HB',bytes(data))

		if opcode != 0x8204 :
			# The opcode is not recognized by this model
			return

		log.info('Got state ' + get_state_str(state) + ' from ' + ('%04x' % source))

class Sensor(Model):
	def pack_property(self, format, length, property):
		length_bits = '{:04b}'.format(length)
		property_bits = '{:011b}'.format(property)

		data = ''.join([str(format), length_bits, property_bits])
		return int(data, 2)

	def unpack_property(self, data):
		format_length_byte = bytes(data[0:1])[0]
		sensor_data_format = (format_length_byte >> 7)
		# only Format A is supported
		if (sensor_data_format == 0):
			sensor_value_length = ((format_length_byte & 0b01111000) >> 3)

			id1 = format_length_byte & 0b00000111
			id2 = bytes(data[1:2])[0]
			property_id = (id1 << 8) | id2
			return (property_id, sensor_value_length)
		else:
			raise Exception("Unsupported format")

	def create_sensor_data(self):
		temp = uniform(18.0, 23.0)
		log.info('Publish: temperature=' + str(int(temp*2)/2))
		property = self.pack_property(0, 1, 0x004f)
		return struct.pack('>BHB', 0x52, property, int(temp*2))

	def parse_sensor_data(self, data):
		datalen = len(data)
		opcode = bytes(data[0:1])[0]
		if (opcode == 0x52):
			index = 1
			property, length = self.unpack_property(bytes(data[index:]))
			if (property == 0x004F):
				sensor_value = bytes(data[3:4])[0]
				sensor_value = sensor_value * 0.5
				return sensor_value
		return None

########################
# Sensor Client Model
########################
class SensorClient(Sensor):
	def __init__(self, model_id):
		Model.__init__(self, model_id)
		self.tid = 0
		self.data = None
		self.cmd_ops = { 0x52 } # status

	def process_message(self, source, dest, key, data):
		sensor_value = self.parse_sensor_data(data)
		if sensor_value != None:
			log.info('Sensor value=' + str(sensor_value))

########################
# Sensor Server Model
########################
class SensorServer(ServerModel, Sensor):
	def __init__(self, model_id):
		Model.__init__(self, model_id)
		self.tid = None
		self.last_src = 0x0000
		self.last_dst = 0x0000
		self.cmd_ops = { 0x8201,  # get
				 0x8202,  # set
				 0x8203,  # set unacknowledged
				 0x8204 } # status

		self.state = 0
		self.pub_timer = ModTimer()
		self.t_timer = ModTimer()

	def process_message(self, source, dest, key, data):
		sensor_value = self.parse_sensor_data(data)
		if sensor_value != None:
			log.info('Sensor value=' + str(sensor_value))

	def publish(self):
		data = self.create_sensor_data()
		self.send_publication(data)

class BurrBoardSensorServer(SensorServer):
	def create_sensor_data(self):
		log.info('Publishing sensor data')
		#TODO randomize values

		#opcode
		data = struct.pack(">B", 0x52)
		#buttons_leds_state
		data += struct.pack(">HB", self.pack_property(0, 1, 0x0001), 0x1A)
		#counter1
		data += struct.pack(">H", self.pack_property(0, 2, 0x0007))
		data += (25).to_bytes(2, byteorder="little")
		#counter2
		data += struct.pack(">H", self.pack_property(0, 2, 0x0008))
		data += (32).to_bytes(2, byteorder="little")
		#temp
		data += struct.pack(">H", self.pack_property(0, 2, 0x0009))
		data += (19).to_bytes(2, byteorder="little", signed=True)
		#brightness
		data += struct.pack(">H", self.pack_property(0, 2, 0x000A))
		data += (75).to_bytes(2, byteorder="little")
		#accel
		x=numpy.float32(0.33)
		y=numpy.float32(0.55)
		z=numpy.float32(0.66)
		data += struct.pack(">H", self.pack_property(0, 12, 0x000B))
		data+=x.tobytes()
		data+=y.tobytes()
		data+=z.tobytes()
		#battery
		data += struct.pack(">HB", self.pack_property(0, 1, 0x000C), 0x23)
		return data

	def get_bytes(self, data, index, length):
		return bytes(data[index+2:index+2+length])

	def get_byte(self, data, index, length):
		return bytes(data[index+2:index+2+length])[0]

	def parse_sensor_data(self, data):
		datalen = len(data)
		opcode = bytes(data[0:1])[0]
		if (opcode == 0x52):
			index = 1
			sensor_data = {}
			while index < datalen:
				property, length = self.unpack_property(bytes(data[index:]))
				if (property == 0x004F):
					temp8 = self.get_byte(data, index, length)
					sensor_data['temp8'] = temp8 * 0.5
				elif (property == 0x0001):
					led_data = self.get_byte(data, index, length)
					sensor_data['button_1'] = (led_data & 0x1) != 0
					sensor_data['button_2'] = (led_data & 0x2) != 0
					sensor_data['led_1'] = (led_data & 0x4) != 0
					sensor_data['led_2'] = (led_data & 0x8) != 0
					sensor_data['led_3'] = (led_data & 0x10) != 0
					sensor_data['led_4'] = (led_data & 0x20) != 0
				elif (property == 0x0007):
					sensor_data['counter_1'] = int.from_bytes(self.get_bytes(data, index, length), byteorder='little')
				elif (property == 0x0008):
					sensor_data['counter_2'] = int.from_bytes(self.get_bytes(data, index, length), byteorder='little')
				elif (property == 0x0009):
					sensor_data['temperature'] = int.from_bytes(self.get_bytes(data, index, length), byteorder='little', signed=True)
				elif (property == 0x000A):
					sensor_data['brightness'] = int.from_bytes(self.get_bytes(data, index, length), byteorder='little')
				elif (property == 0x000B):
					acc_data = numpy.frombuffer(self.get_bytes(data, index, length), dtype=numpy.float32)
					acc = {}
					acc['x'] = str(acc_data[0])
					acc['y'] = str(acc_data[1])
					acc['z'] = str(acc_data[2])
					sensor_data['accelerometer'] = acc
				elif (property == 0x000C):
					sensor_data['battery'] = self.get_byte(data, index, length)
				else:
					log.warn("Unkown property: " + str(property))
				index += 2+length

			return sensor_data
		return None

########################
# Sample Vendor Model
########################
class SampleVendor(Model):
	def __init__(self, model_id):
		Model.__init__(self, model_id)
		self.vendor = 0x05F1 # Linux Foundation Company ID
