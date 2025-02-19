# bananu7/Votol
Hello. This is my repository for all things related to the Votol controllers I was able to gather and put together.

## My use case and context

I'm converting a CRF250 MX motorcycle to electric power, using the Votol EM260SGTP controller. I want to have a bit more control over it that usual, so I decided to learn more about it.
Unless otherwise noted, all descriptions, labels etc. apply to _my specific controller_. Different models of the controller can differ; make sure you understand the cable functions
and model of yours.

## Project structure

* `desktop/` - early beginnings of a desktop app that were meant to replace the original desktop programmer
* `embassy/` - firmware of the display controller that goes on the bike and communicates with Votol EM
* `reference/` - technical details and descriptions of the controller protocol

## Log

### 2025-02-18

I did some more work on the miscellaneous features for the display. Got better frame decoding working, modularized things and importantly created a simulation mode that allows development without a controller connected (still on real hardware, though). I'm still not sure how I want to present all the available data or what peripherals to connect. Ideas list includes:

* all main controller parameters (works for now: battery voltage, temperatures)
* ride mode
* all controller errors in readable format
* temperature alerts
* speed based on the RPM counter
* trip computer: speed * time (distance traveled), current * time (amp-hours used), max/avg speed, max/avg current, moving time

Additional features based on peripheral hw:
* RTC - clock :), configuration (maybe I could use internal RTC)
* SD or flash - datalogger (trip computer but as series)
* IMU, magnetometer - compass, lean-aware TC
* front wheel speed sensor - TC, more accurate speed
* GPS - time, datalogging position
* BT - phone app, remote config and data reading

This is just a mental note to myself; it's unlikely I'll get to doing all of the above. Once I get the basic features working, I'll also upload the CAD files for the enclosure and print a prototype.

### 2025-02-13

I should've started this log immediately, but better late than never. I have already done quite a lot of work, but a lot still remains to be done.
Dealing with the controller can roughly be grouped into two categories:

1) Programming - setting up current and voltage limits, throttle rampup, engine phase config and hall config, as well as pin functions
2) Using - happens in the vehicle

#### Programming

Programming by itself is reasonably straighforward. I need to write more about the programming setup in the reference section, but in short,
the controller comes with a programming USB dongle. This dongle plugs into a two-pin port on the controller colored blue and orange. In my case,
the two-pin interface is a 1M CAN bus, xceived using TJA1050, connecting to an STM microcontroller, then exposed to USB via an USB-to-serial converter.

The controller also comes with a desktop app that looks like it was written with MFC. The app seems to have been made for older, smaller controllers,
and in fact doesn't even show the 260 on the list. To communicate, it opens a serial port at either 9600 or 115200 baud. There's also the CAN
switch that in my case needed to be... disabled. I believe that it's mostly a consequence of historical debt and reuse. The app originally
talked to the controller via serial; once they switched to CAN they were limited to 8-byte data frames, so they added the intermediate translator
chip to the dongle, so that the old app can still talk to the new controller. Incidentally I've noticed that the serial data going back to the app
is 24 bytes long, while the controller only responds with two can frames - so the translator chip must be doing some shuffling and filling in the 
blanks.

I have recorded some of the programming transactions, but haven't saved them. In order to properly reverse engineer the protocol, a whole
communication scheme needs to be observed and the parameters isolated. That being said, as the app seems to work fine for now, this is of low
priority. Perhaps one exciting possibility here would be to replace it with a web-based version talking serial over WebUSB. That would be a massive
usability boost, but getting all controllers working correctly could be a challenge.

#### Using the controller

Much more interesting is what happens during use. I don't want to use the dongle, and CAN is actually a great interface for the vehicle use.
The displays normally utilized with this controller use the separate LIN output, but I see zero reason to do that. CAN is more robust and offers
much more functionality.

The controller exposes a lot of frames on the bus, none of which I've decoded yet. There's been some mentions from SIA that they're their internal
protocol, but nothing concrete about it. I wouldd assume the usual parameters would be broadcasted continously, which would be great for display
purposes.

At the same time, when the desktop app wants live data from the controller, it sends a special query packet. The controller "responds" by sending
two frames containing the vitals; they are described in the reference folder. I was able to successfully read and decode those frames, and
that's the current state in the `embassy` folder; operating on an STM32F103C8 microcontroller, using its onboard CAN controller and TJA1050 
transceiver. On the other end, I plugged in a simple 8x8x4 LED matrix controlled by four MAX7219 chips in series, over SPI. Works great and 
the display of basic parameters has already been achieved.
