# Demo App for RTC HAL

A simple demo that shows how RTC HAL works. We will use the RTC chips with ESP32. This demo works with both DS1307 and DS3231 RTC chips using the same code.

## What This Demo Does
The demo sets the date and time on your RTC chip, makes an LED blink using square wave output, and reads the current time every minute. It works with both DS1307 and DS3231 chips without changing any code.

## What You Need

You'll need an ESP32 DevKit v1 board, a DS1307 or DS3231 RTC module with backup battery, an LED, a resistor for the LED, and some breadboard with jumper wires.

## Circuit Connection Table

| RTC Module Pin | ESP32 DevKit v1 Pin | Purpose |
|----------------|---------------------|---------|
| VCC | 3.3V | Power supply |
| GND | GND | Ground |
| SDA | GPIO 21 | I2C Data line |
| SCL | GPIO 22 | I2C Clock line |
| SQW/INT | LED Anode (+) | Square wave output |

**Note:** The LED will blink at 1Hz when you run `app.start_square_wave()` in the demo code. It will stop blinking when you run `app.stop_square_wave()`. This gives you a visual way to see the square wave output working!

## How to Run

For DS3231 chip, run `cargo run --features ds3231`. For DS1307 chip, just run `cargo run` since it's the default.

The program will set the time on your RTC, make the LED blink for 1 minute, stop the LED blinking, then show the time every minute in the logs.

## What You'll See

You'll see the LED blinking to show the square wave output is working. The serial output will show the current date and time every minute.
