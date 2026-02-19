Rpi based rust server that does stream processing and serving of car's telemetry from the obd-2 port, obd-2 translation reverse-engineered and not universal, the code universal, of course needs follow up on the pi.

OBD CAN hat is needed for the pi, the obd-2 protocol consists of 11bit packets, it's sent to the hat which is registered as a can interface with
#### sudo ip link set up vcan0
then as the logs are streaming, the obd are processed with socketcan library as much in-place as possible, then those logs are parsed into internal data types and then streamed via webSockets to a connected client.
The actual values in those incoming 11-bit packets have to be reverse engineered, but it's not that difficult, i used savvyCan open tool for that, which is nice for visualization of the data, also in the project quite a lot of linux config is needed, for example native services setup and other immediate config, also tools for collecting logs with a usb-stick, for analysis. The software part is complete, but there should also be a correct hardware setup, i didn't print a case for the pi and thought through the connections. 
