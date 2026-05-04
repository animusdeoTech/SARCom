# Dragino LoRa GPS HAT — User Manual v1.0

Source: `LoRa_GPS_HAT_UserManual_v1.0.pdf` (Dragino, 2019-03-06). PDF not in repo; this markdown is the preserved content.

---

## SARCOM bring-up notes (added 2026-05-04)

Critical translations from WiringPi → BCM numbering, relevant for Rust/linux-embedded-hal:

| Function | WiringPi (doc) | BCM (actual) | Notes |
|---|---|---|---|
| LoRa DIO0 | GPIO7 | BCM 4 | interrupt |
| LoRa NSS (CS) | GPIO6 | BCM 25 | **not hardware CE0** — must be software CS in spidev |
| LoRa MISO | GPIO13 | BCM 9 | SPI0 |
| LoRa MOSI | GPIO12 | BCM 10 | SPI0 |
| LoRa SCK | GPIO14 | BCM 11 | SPI0 |
| LoRa RESET | GPIO0 | BCM 17 | |
| LoRa DIO1 | GPIO4 | BCM 23 | added v1.3 |
| LoRa DIO2 | GPIO5 | BCM 24 | added v1.3 |
| GPS TX → Pi RX | GPIO16/RX | BCM 15 | UART RX on Pi |
| GPS RX → Pi TX | GPIO15/TX | BCM 14 | UART TX on Pi |
| GPS PPS | GPIO1 | BCM 18 | **v1.4**: explicitly wired to BCM 18 |

**SPI chip select:** LoRa_NSS is BCM 25, not the hardware CE0 (BCM 8). Configure spidev with `CS_HIGH` or drive BCM 25 manually as GPIO. `linux-embedded-hal` handles this fine.

**GPS UART:** GPS_TX → BCM 15 (Pi RX), GPS_RX → BCM 14 (Pi TX). This is UART0 on Pi 4. On Pi 5 the device node differs — verify with `ls /dev/ttyAMA*` and check `/boot/firmware/config.txt`. Serial console must be disabled on this UART before gpsd can use it.

**GPS PPS:** BCM 18 (v1.4 confirmed). Use as `refclock PPS /dev/pps0` in chrony after loading `pps-gpio` kernel module with `gpiopin=18`.

---

## 1. Introduction

### 1.1 What is LoRa GPS HAT

LoRa GPS HAT is an expansion module for LoRaWAN and GPS for use with the Raspberry Pi. This product is intended for those interested in developing LoRaWAN solutions.

LoRa GPS HAT is based on the SX1276/SX1278 transceiver. The add-on L80 GPS (based on MTK MT3339) is designed for applications that use a GPS connected via the serial ports to the Raspberry Pi such as timing applications or general applications that require GPS information.

The transceivers of the HAT feature the LoRa long range modem that provides ultra-long range spread spectrum communication and high interference immunity whilst minimizing current consumption. The LoRa/GPS HAT can achieve a sensitivity of over -148 dBm using a low cost crystal and bill of materials. The high sensitivity combined with the integrated +20 dBm power amplifier yields industry leading link budget making it optimal for any application requiring range or robustness. LoRa also provides significant advantages in both blocking and selectivity over conventional modulation techniques, solving the traditional design compromise between range, interference immunity and energy consumption.

The L80 GPS module can calculate and predict orbits automatically using the ephemeris data (up to 3 days) stored in internal flash memory, so the HAT can fix position quickly even at indoor signal levels with low power consumption. With AlwaysLocate technology, the LoRa/GPS HAT can adaptively adjust the on/off time to achieve balance between positioning accuracy and power consumption according to the environmental and motion conditions. The GPS also supports automatic antenna switching function. It can achieve the switching between internal patch antenna and external active antenna. Moreover, it keeps positioning during the switching process.

---

### 1.2 Specifications

#### LoRa Spec

- 168 dB maximum link budget
- +20 dBm — 100 mW constant RF output vs. +14 dBm high efficiency PA
- Programmable bit rate up to 300 kbps
- High sensitivity: down to -148 dBm
- Bullet-proof front end: IIP3 = -12.5 dBm
- Excellent blocking immunity
- Low RX current of 10.3 mA, 200 nA register retention
- Fully integrated synthesizer with a resolution of 61 Hz
- FSK, GFSK, MSK, GMSK, LoRa and OOK modulation
- Built-in bit synchronizer for clock recovery
- Preamble detection
- 127 dB Dynamic Range RSSI
- Automatic RF Sense and CAD with ultra-fast AFC
- Packet engine up to 256 bytes with CRC
- Built-in temperature sensor and low battery indicator

#### GPS Spec

- Based on MT3339 (Quectel L80 module)
- Power: Acquisition 25 mA, Tracking 20 mA
- Compliant with GPS, SBAS
- Serial Interface UART: Adjustable 4800–115200 bps, Default: 9600 bps
- Update rate: 1 Hz (default), up to 10 Hz
- I/O Voltage: 2.7 V – 2.9 V
- Protocols: NMEA 0183, PMTK
- Horizontal Position Accuracy: Autonomous < 2.5 m CEP
- TTFF @ -130 dBm with EASY: Cold Start < 15 s, Warm Start < 5 s, Hot Start < 1 s
- TTFF @ -130 dBm without EASY: Cold Start < 35 s, Warm Start < 30 s, Hot Start < 1 s
- Timing Accuracy: 1PPS out 10 ns, Reacquisition Time < 1 s
- Velocity Accuracy without aid: < 0.1 m/s; Acceleration Accuracy without aid: 0.1 m/s²
- Sensitivity: Acquisition -148 dBm, Tracking -165 dBm, Reacquisition -160 dBm
- Environmental: Operating Temperature -40 °C to 85 °C, Storage Temperature -45 °C to 125 °C
- Dynamic Performance: Altitude max 18000 m, Velocity max 515 m/s, Acceleration max 4G
- L1 Band Receiver (1575.42 MHz), Channel 22 (Tracking) / 66 (Acquisition)

---

### 1.3 Features

- Frequency Band: 868 MHz / 433 MHz / 915 MHz (pre-configured in factory)
- Low power consumption
- Compatible with Raspberry Pi 2 Model B / Raspberry Pi 3 Model B/B+
- LoRa Modem
- FSK, GFSK, MSK, GMSK, LoRa and OOK modulation
- Preamble detection
- Baud rate configurable
- Built-in temperature sensor and low battery indicator
- Excellent blocking immunity
- Automatic RF Sense and CAD with ultra-fast AFC
- Support DGPS, SBAS (WAAS/EGNOS/MSAS/GAGAN)
- GPS automatic switching between internal patch antenna and external active antenna
- PPS vs. NMEA can be used in time service
- Support SDK command
- Built-in LNA for better sensitivity
- EASY, advanced AGPS technology without external memory
- AlwaysLocate, an intelligent controller of periodic mode
- GPS FLP mode, about 50% power consumption of normal mode
- GPS support short circuit protection and antenna detection

### 1.4 Applications

- Smart Buildings & Home Automation
- Logistics and Supply Chain Management
- Smart Metering
- Smart Agriculture
- Smart Cities
- Smart Factory

---

### 1.5 Pin Definition

#### Pin Mapping

| LoRa GPS HAT | RaspberryPi Wiring PI IO |
|---|---|
| 3.3v | 3.3v |
| 5v | 5v |
| GND | GND |
| DIO0 | GPIO7 |
| GPS_RX | GPIO15/TX |
| GPS_TX | GPIO16/RX |
| RESET | GPIO0 |
| LoRa_NSS | GPIO6 |
| LoRa_MISO | GPIO13/MISO |
| LoRa_MOSI | GPIO12/MOSI |
| SCK | GPIO14/SCLK |
| DIO1 | GPIO4 |
| DIO2 | GPIO5 |
| 1PPS | GPIO1 |

> Note: All GPIO numbers above are WiringPi numbering, not BCM. See SARCOM bring-up notes at the top of this file for BCM equivalents.

---

### 1.6 Hardware Change Log

- **LoRa/GPS_HAT v1.0:** First hardware release.
- **LoRa/GPS_HAT v1.3:**
  - Add trace from LoRa DIO1 to RPi GPIO4 (WiringPi). Required by LMIC library on RPi.
  - Add trace from LoRa DIO2 to RPi GPIO5 (WiringPi). Required by LMIC library on RPi.
- **LoRa/GPS HAT v1.4:**
  - Change SMA connector to support active antenna
  - Add AADET_N LED to show if external antenna is active
  - **Connect GPS PPS pin to RPi BCM pin 18**
  - Modify silkscreen for GPS TXD/RXD

---

### 1.7 LEDs

- **PWR:** Power indicator. Turns on once there is power.
- **LoRa-RX:** Indicates a wireless packet received in the LoRa module.
- **3D_FIX:** Blinks every 100 ms after GPS position fix.
- **EXT_ANT:** Indicates an external GPS antenna is connected.

### 1.8 Dimension & Weight

- Size: 60 mm × 53 mm × 25 mm
- Net weight: 30 g
- Package Size: 98 mm × 81 mm × 32 mm

---

## 2. Example 1: Set up as a Single Channel LoRaWAN gateway with Raspbian OS

> SARCOM note: This section describes TTN packet forwarding with Raspbian — not relevant to SARCOM's Yocto/Rust stack. Preserved for SPI enable procedure and pin confirmation only.

### 2.1 Configure LoRa GPS HAT

#### 2.1.1 Install packet forwarder software

a) Install git:
```
sudo apt-get install git
```

b) Enable SPI on the Raspberry Pi:
```
sudo raspi-config
```
Navigate to: Interfacing Options → SPI → Enable

c) Install wiringpi (GPIO access library):
```
sudo apt-get install wiringpi
```

d) Install packet_forwarder:
```
cd /home/pi
git clone https://github.com/dragino/dual_chan_pkt_fwd
make
```

Run to start and check gateway ID:
```
sudo ./dual_chan_pkt_fwd
```

Example output:
```
Trying to detect module CE0 with NSS=6 DIO0=7 Reset=3 Led1=unused
SX1276 detected on CE0, starting.
Trying to detect module CE1 with NSS=6 DIO0=7 Reset=3 Led1=unused
SX1276 detected on CE1, starting.
Gateway ID: b8:27:eb:ff:ff:78:3b:7f
Listening at SF7 on 868.100000 Mhz.
```

Install as system service:
```
sudo make install
```

Service management:
```
systemctl start dual_chan_pkt_fwd
systemctl stop dual_chan_pkt_fwd
systemctl status dual_chan_pkt_fwd
```

View real-time log:
```
sudo journalctl -f -u dual_chan_pkt_fwd
```

#### 2.1.2 Configure Frequency and server

For a single channel gateway, set one frequency in `global_conf.json`:

```json
{
  "SX127x_conf": {
    "freq": 868100000,
    "freq_2": 868100000,
    "spread_factor": 7,
    "pin_nss": 6,
    "pin_dio0": 7,
    "pin_nss_2": 6,
    "pin_dio0_2": 7,
    "pin_rst": 3,
    "pin_led1": 4,
    "pin_NetworkLED": 22,
    "pin_InternetLED": 23,
    "pin_ActivityLED_0": 21,
    "pin_ActivityLED_1": 29
  },
  "gateway_conf": {
    "ref_latitude": 0.0,
    "ref_longitude": 0.0,
    "ref_altitude": 10,
    "name": "your name",
    "email": "a@b.c",
    "desc": "Dual channel pkt forwarder",
    "interface": "eth0",
    "servers": [
      {
        "address": "router.eu.staging.thethings.network",
        "port": 1700,
        "enabled": true
      },
      {
        "address": "router.eu.thethings.network",
        "port": 1700,
        "enabled": false
      }
    ]
  }
}
```

Stop/start the service after making changes.

### 2.2 Create a gateway in TTN Server

1. Sign up at The Things Network
2. Create a Gateway with:
   - Gateway EUI: read from the running `dual_chan_pkt_fwd` output (e.g. `b827ebffff783b7f`)
   - Check "I'm using the legacy packet forwarder"
   - Frequency Plan: Europe 868MHz
   - Router: ttn-router-eu

---

## 3. Example 2: Single Channel LoRaWAN gateway with RPI and Windows 10 IoT Core

This example (by Mattias Larsson) describes building a LoRaWAN "The Things Network" packet-forwarding gateway on Windows 10 IoT Core in native .NET code.

> SARCOM note: Not applicable. Preserved for completeness.

---

## 4. Example 3: Two RPIs use LoRa to transmit to each other

Download lora transmit code:
```
wget https://codeload.github.com/dragino/rpi-lora-tranceiver/zip/master
```

Unzip and build:
```
unzip master
cd rpi-lora-tranceiver-master/dragino_lora_app
make
```

Send:
```
sudo ./dragino_lora_app sender
```

Receive (on second RPI):
```
sudo ./dragino_lora_app rec
```

Example receive output:
```
SX1276 detected, starting.
Listening at SF7 on 868.100000 Mhz.
Packet RSSI: -52, RSSI: -138, SNR: 9, Length: 5
Payload: HELLO
```

Data can also be received by Arduino using the `arduino-LoRa` library:

```cpp
#include <SPI.h>
#include <LoRa.h>

void setup() {
  Serial.begin(9600);
  while (!Serial);
  Serial.println("LoRa Receiver");
  if (!LoRa.begin(868100000)) {
    Serial.println("Starting LoRa failed!");
    while (1);
  }
  LoRa.setSpreadingFactor(7);
}

void loop() {
  int packetSize = LoRa.parsePacket();
  if (packetSize) {
    Serial.print("Received packet '");
    while (LoRa.available()) {
      Serial.print((char)LoRa.read());
    }
    Serial.print("' with RSSI ");
    Serial.println(LoRa.packetRssi());
  }
}
```

---

## 5. FAQ

### 5.1 Why is there a 433/868/915 version?

Different countries have different ISM band rules for LoRa. Although the LoRa chip supports a wide frequency range, Dragino provides different versions tuned for regional bands.

### 5.2 What is the frequency range of the LoRa GPS HAT?

| Version | LoRa IC | Working Frequency | Best Tune | Recommended Bands |
|---|---|---|---|---|
| 433 | SX1278 | 410–525 MHz | 433 MHz | CN470/EU433 |
| 868 | SX1276 | 862–1020 MHz | 868 MHz | EU868 |
| 915 | SX1276 | 862–1020 MHz | 915 MHz | AS923/AU915/KR920/US915 |
| JP | SX1276 | 862–1020 MHz | 915 MHz | AS923/AU915/KR920/US915 |

> SARCOM uses the **868 MHz variant** (EU868, SX1276). Operating frequency: 868.1 MHz (sub-band M). See ADR-010 and ADR-013.

---

## 6. Order Info

Part Number: `LoRa-GPS-HAT-XXX`

- `433`: Best tuned 433 MHz
- `868`: Best tuned 868 MHz
- `915`: Best tuned 915 MHz

---

## 7. Packing Info

Package includes:
- 1× LoRa/GPS HAT
- 4× Brass cylinders
- 4× Screws
- 4× Nuts
- 1× Glue stick antenna (868/433/915 MHz depending on order)

Dimensions:
- Device Size: 10 × 8 × 3 mm/pcs
- Device Weight: G.W. 72 g/pcs

---

## 8. Support

Support: Monday–Friday, 09:00–18:00 GMT+8.

Email: support@dragino.com
