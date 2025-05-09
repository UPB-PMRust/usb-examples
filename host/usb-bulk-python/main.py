import usb.core
import usb.util
import time

while True:
    # find our device
    #   VendorID:  0xc0de
    #   ProductID: 0xcafe
    dev = usb.core.find(idVendor=0xc0de, idProduct=0xcafe)

    # was it found?
    if dev is None:
        print ("Device not found, retying in 1s")
        time.sleep(1)

    else:
        try:
            print("Device connected")
            # set the active configuration. With no arguments, the first
            # configuration will be the active one
            dev.set_configuration()

            # get an endpoint instance
            cfg = dev.get_active_configuration()
            intf = cfg[(0,0)]

            ep_out = usb.util.find_descriptor(
                intf,
                # match the first OUT endpoint
                custom_match = \
                lambda e: \
                    usb.util.endpoint_direction(e.bEndpointAddress) == \
                    usb.util.ENDPOINT_OUT)

            ep_in = usb.util.find_descriptor(
                intf,
                # match the first IN endpoint
                custom_match = \
                lambda e: \
                    usb.util.endpoint_direction(e.bEndpointAddress) == \
                    usb.util.ENDPOINT_IN)

            assert ep_out is not None
            assert ep_in is not None

            # This example writes data and waits to read data
            #   Reading amn writing can be done independently, most probably in
            #   separate threads.

            # write the data
            for i in range(0, 1000000):
                ep_out.write(f'test {i}')
                value = ep_in.read(100)
                print(str(value, 'ascii'))
                time.sleep(1)

        except Exception as error:
            print (f'Error {error}')
