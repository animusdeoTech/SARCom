---
title: "Zephyr study (ARCHIVED)"
status: superseded
type: archive
tags: [archive, zephyr, superseded]
superseded_by: "decisions/ADR-001-firmware-language.md"
archived_date: 2026-04-22
---

> **ARCHIVED — DO NOT USE AS CURRENT REFERENCE.**
>
> This document is a research study about putting Zephyr RTOS on the Heltec Wireless Tracker V1.1. It was useful at the time for understanding the Zephyr-on-ESP32-S3 landscape.
>
> It is **superseded by [ADR-001: Firmware language — Rust everywhere](../decisions/ADR-001-firmware-language.md)** (accepted 2026-04-22). The decision is:
>
> - No Zephyr. No C. No C++. Rust everywhere.
> - MCU side: `esp-hal` + Embassy + `lora-phy`, not Zephyr.
> - Also: the hardware pick is **Wireless Tracker V2**, not V1.1 (see [ADR-002](../decisions/ADR-002-tag-hardware.md)). Any V1.1-specific detail below (pin map, VEXT on GPIO3, upstream board definition) does not apply to V2.
>
> The content below is preserved only for historical context. Nothing here should drive a current decision.

---

# Zephyr 4.4 is ready for your Heltec distress beacon — with caveats

**The Heltec Wireless Tracker V1.1 already has an official upstream Zephyr board definition, and every major subsystem you need (SX1262 LoRa, UC6580 GNSS via generic NMEA, ESP32-S3 deep sleep, MCUboot OTA via sysbuild) is in-tree and working as of April 2026.** You do not need to port a board from scratch. The board ships in `boards/heltec/heltec_wireless_tracker/` as of Zephyr 4.4 (released April 14, 2026), with a complete devicetree covering the SX1262, UC6580 on UART2, ST7735 TFT, battery ADC, user button, and the V1.1-specific active-HIGH VEXT power rail on GPIO3. The raw (non-LoRaWAN) LoRa API has been stable since Zephyr 2.4 (2020), cross-chip interoperability with a Linux-side SX1276 gateway works out of the box via `public_network = false`, and the new GNSS subsystem (added in Zephyr 3.6, Feb 2024) handles the UC6580's standard NMEA 0183 output without any custom driver.

That said, the honest assessment is that **Zephyr + ESP32-S3 + SX1262 is technically production-ready but not the path of least resistance**. The ecosystem around Heltec hardware is overwhelmingly Arduino/ESP-IDF/Meshtastic, Zephyr suffers release-to-release regressions on ESP32-S3 (Issue #86192 broke boot on three S3 boards going from 4.0 to 4.1), Wi-Fi+BLE coexistence is weaker than on ESP-IDF, and no prominent commercial product publicly ships Zephyr on this silicon combo. The strongest argument for Zephyr here is strategic: clean DT/Kconfig/sysbuild, MCUboot OTA, a single codebase that can later migrate to nRF52840 or STM32WL, and a cleaner safety/certification story for a distress beacon. The Raspberry Pi gateway should **stay on Linux** — Zephyr's RPi support is minimal and the Dragino HAT has mature Linux userspace drivers.

## ESP32-S3 and the Heltec board are already upstream

Zephyr 4.4.0 is the current release, and the ESP32-S3 is a first-class supported SoC with hardware model v2 (HWMv2) fully in force. Every ESP32-S3 board targets split into `<board>/esp32s3/procpu` and `<board>/esp32s3/appcpu` variants, and you build with `west build -b heltec_wireless_tracker/esp32s3/procpu`. The `hal_espressif` module is actively maintained by Espressif (notably committer `sylvioalves`), and all core peripherals work: CPU (Xtensa LX7 dual-core), 3× UART, 2× SPI (SPI2/SPI3), 2× I2C, GPIO, Wi-Fi, BLE, USB-OTG, ADC, TWAI, I2S, LEDC PWM, GDMA, SDHC, and the LCD-CAM interface.

**The authoritative pin map for the Wireless Tracker V1.1, from the upstream board documentation**, is: SX1262 on SPI2 with NSS=GPIO8, SCK=GPIO9, MOSI=GPIO10, MISO=GPIO11, RST=GPIO12, BUSY=GPIO13, DIO1=GPIO14; UC6580 GNSS on UART2 with RX=GPIO33, TX=GPIO34; ST7735 TFT on SPI3 using GPIO38–42 plus backlight on GPIO21; user button on GPIO0; and — critically — **VEXT on GPIO3 as active-HIGH in V1.1** (reversed from V1.0), gating both the GNSS and TFT power rails. The upstream `board_init.c` forces GPIO3 HIGH at boot via `SYS_INIT`. A sister board `heltec_wifi_lora32_v3` (PR #101598, merged Feb 14 2026) and `heltec_wireless_stick_lite_v3` provide near-identical reference implementations if you need to extend the port.

The known gotchas are worth internalizing before you start. **GPIO pin numbering splits at 32**: pins 0–31 use `&gpio0`, pins 32+ use `&gpio1` with the index rebased (GPIO35 = `&gpio1 3`). **APPCPU has no Zephyr-managed serial output** — `printk`, logging, and console UART only run on PROCPU; APPCPU must use ROM `ets_printf()` or IPM/mbox for inter-core comms. **SPI DMA is unreliable** on some S3 boards (Issue #87127 on xiao_esp32s3). **Hardware chip-select on SPI2 does not work reliably with the SX1262** according to VynDragon's testing during PR #101598 — keep `cs-gpios = <&gpio0 8 GPIO_ACTIVE_LOW>` instead of HW-CS via pinctrl. The board is marked *"Not actively maintained"* in upstream, so budget time to fix regressions yourself.

## The SX1262 driver is mature; syncword compatibility is automatic

Zephyr has had a native SX126x driver since Zephyr 2.4 (mid-2020), living at `drivers/lora/loramac-node/sx126x.c` and built on Semtech's LoRaMac-node HAL. As of Zephyr 4.4 (April 2026), BayLibre contributed a brand-new **native SX126x driver** at `drivers/lora/native/sx126x/` that drops the loramac-node dependency — enabled via `CONFIG_LORA_MODULE_BACKEND_NATIVE` (or `BACKEND_NONE` depending on the exact merge revision), but marked experimental. For a safety-critical distress beacon, **stick with the loramac-node-backed driver** until the native one has a release cycle of field validation.

The `semtech,sx1262` devicetree binding supports every signal you need on Heltec-class hardware: `reset-gpios`, `busy-gpios`, `dio1-gpios`, plus the crucial optional properties `tx-enable-gpios`/`rx-enable-gpios` (for GPIO-driven RF switches on E22/Heltec modules), `dio2-tx-enable` (DIO2-driven switch), and `dio3-tcxo-voltage` with constants from `include/zephyr/dt-bindings/lora/sx126x.h` (`SX126X_DIO3_TCXO_1V8` through `_3V3`). A typical node for the Heltec Wireless Tracker looks like:

```dts
&spi2 {
    status = "okay";
    pinctrl-0 = <&spim2_default>;
    pinctrl-names = "default";
    cs-gpios = <&gpio0 8 GPIO_ACTIVE_LOW>;

    lora: sx1262@0 {
        compatible = "semtech,sx1262";
        reg = <0>;
        spi-max-frequency = <DT_FREQ_M(8)>;
        reset-gpios = <&gpio0 12 GPIO_ACTIVE_LOW>;
        busy-gpios  = <&gpio0 13 GPIO_ACTIVE_HIGH>;
        dio1-gpios  = <&gpio0 14 GPIO_ACTIVE_HIGH>;
        dio2-tx-enable;
    };
};
```

**Raw LoRa (not LoRaWAN) is a first-class Zephyr API** exposed via `include/zephyr/drivers/lora.h` — separate from the optional `subsys/lorawan/` stack. The core functions are `lora_config()`, `lora_send()`, `lora_send_async()`, `lora_recv()`, `lora_recv_async()`, and `lora_test_cw()` for regulatory CW testing, with Channel Activity Detection (`lora_cad()`) and duty-cycle RX added in Zephyr 3.7 and 4.4 respectively. The `lora_modem_config` struct carries frequency (Hz), bandwidth (`BW_125/250/500_KHZ`), datarate (`SF_6` through `SF_12`), coding rate (`CR_4_5` through `CR_4_8`), preamble length, `tx_power` in dBm, `iq_inverted`, `tx` direction, and `public_network` — **but notably no CRC-on/off flag** (CRC is always on in the loramac-node backend) and **no arbitrary syncword API** (GitHub Discussion #78995 documents this gap; it only matters if you need a non-standard value like Meshtastic's 0x2B).

**Syncword cross-compatibility between your SX1262 beacon and SX1276 gateway is automatic**, which is fortunate given that the chips use different byte-width syncwords at the register level (SX1276: 1-byte, 0x12 private / 0x34 public; SX1262: 2-byte, 0x1424 private / 0x3444 public). Zephyr's `public_network = false` on the SX1262 programs `0x1424`, and any correctly-configured SX1276 Linux driver (e.g., `pyLoRa`, `rpsreal/pySX127x`, RadioHead) programs `0x12` for private — **these are on-air equivalent**, confirmed by the MicroPython LoRa documentation. Just ensure every PHY parameter (frequency 868.1/868.3/868.5 MHz, bandwidth 125 kHz, spreading factor SF10–SF12 for maximum link budget, coding rate, preamble, IQ inversion = false, CRC on) matches exactly on both ends, and respect ETSI EN 300 220 limits (≤14 dBm / 25 mW ERP on h1.6 sub-band, ≤27 dBm only at 869.525 MHz with ≤10% duty cycle).

## Keep the RPi gateway on Linux, not Zephyr

Zephyr does have `rpi_4b` and `rpi_3b` board definitions (ARM64 Cortex-A72/A53), but support is essentially UART console and basic GPIO — there is no SX1276 driver wired up to any `rpi_*` board, no mature SPI glue, no Wi-Fi or Ethernet. The Raspberry Pi 5 (BCM2712) is not a first-class Zephyr board as of April 2026. **For the Dragino LoRa/GPS HAT (SX1276 + MTK3339 GNSS), use Raspberry Pi OS with userspace drivers**: for LoRaWAN concentrator HATs use the Semtech packet forwarder or ChirpStack's `chirpstack-concentratord`, and for raw SX1276 point-to-point use `/dev/spidev0.0` with `pyLoRa`, `rpsreal/pySX127x`, or the SX127x portions of RadioHead, with DIO0 wired as a gpiod interrupt for RX-done. Zephyr offers zero advantage for a mains-powered gateway.

If you wanted parity at the driver level, Zephyr's SX1276 support (`drivers/lora/loramac-node/sx127x.c`, binding `semtech,sx1276`) has been upstream since Zephyr 2.2 (2020) and is where the Zephyr LoRa API originated. The Dragino/RFM95W uses PA_BOOST, so the DT node would include `power-amplifier-output = "pa-boost"`. But you'll never reach that code path on an RPi running Linux.

## GNSS, power management, and the sleep flow

The Zephyr GNSS subsystem landed in **Zephyr 3.6 (February 2024)** at `drivers/gnss/` with an API header at `include/zephyr/drivers/gnss.h`, built on top of the Zephyr modem_chat infrastructure. There is **no dedicated UC6580 driver**, but the generic `gnss-nmea-generic` driver handles it correctly — this is exactly what the upstream Heltec Wireless Tracker board definition uses. The UC6580 emits standard multi-constellation NMEA 0183 sentences (`$GP/GL/GA/GB/GN` prefixes) at 115200 baud, which the wildcard `$??GGA/RMC/GSV` matchers parse into a `struct gnss_data` with nav data (latitude/longitude in nanodegrees, altitude in millimeters, speed, bearing), fix info (satellite count, HDOP, fix status/quality), and UTC time. GSA sentences are not parsed, but HDOP arrives via GGA. You register callbacks at build time via the iterable-section macros `GNSS_DATA_CALLBACK_DEFINE(dev, cb)` and `GNSS_SATELLITES_CALLBACK_DEFINE(dev, cb)` — there is no runtime subscribe API.

The one real limitation is that `gnss-nmea-generic` has no `set_fix_rate`, `set_navigation_mode`, or `set_enabled_systems` implementation (all return `-ENOTSUP`), so you cannot reconfigure the UC6580's default NMEA output rate or constellation selection without sending proprietary `$CFGSYS/$CFGPRT` commands via raw UART before the driver starts, or writing a thin UC6580-specific driver that wraps `gnss-nmea-generic` with an init block. For a distress beacon where a 1 Hz fix cadence is sufficient, this is irrelevant.

**ESP32-S3 power management in Zephyr routes through hal_espressif** into the same `esp_sleep.h` / `esp_pm.h` APIs as ESP-IDF — no native Zephyr reimplementation. Deep sleep is triggered via `sys_poweroff()` which calls `esp_deep_sleep_start()`; the samples live at `samples/boards/espressif/deep_sleep` and `light_sleep`. Wake sources include RTC timer (`esp_sleep_enable_timer_wakeup`), **EXT0** (single RTC GPIO at configured level — requires RTC peripherals kept on), **EXT1** (multiple RTC GPIOs with ANY_HIGH or ANY_LOW, ANY_LOW being S3/C6/H2-only), touch, ULP, and UART. **Only GPIOs 0–21 are RTC-capable on ESP32-S3**, which thankfully includes both the Heltec user button on GPIO0 and the SX1262 DIO1 on GPIO14 — so both the SOS button wake and wake-on-radio from the SX1262 will work from deep sleep. Compare this to boards like the Seeed XIAO ESP32-S3 + Wio-SX1262 shield where DIO1 lands on a non-RTC GPIO, making wake-on-radio only possible from light sleep.

**The critical power gotcha on the Heltec V1.1** is that VEXT on GPIO3 powers both the UC6580 (~25 mA when on) and the TFT — you must drive GPIO3 LOW before sleep, then call `rtc_gpio_hold_en(3)` to latch the pin level across the deep-sleep transition. Without the hold call, the pin goes Hi-Z at the instant of sleep and peripherals may stay powered through leakage. The SX1262 itself sleeps at ~600 nA if placed in `STDBY_RC` + `SetSleep` before the MCU powers down, but Zephyr's LoRa API does not expose a `lora_sleep()` — you'll need to call the underlying Semtech `Radio.Sleep()` directly or rely on the driver's `PM_DEVICE` hook. Community measurements suggest **typical achievable deep sleep current of 40–150 µA** on S3-WROOM modules (datasheet floor ~8 µA), limited by module LDO quiescent current and pull-up leakage, not by Zephyr-vs-ESP-IDF differences.

One **open bug worth knowing**: Issue #96639 reports that `CONFIG_PM=y` for automatic light-sleep on ESP32-S3 can hang because Zephyr's `pm.c` drives the core into light sleep without programming `esp_sleep_enable_timer_wakeup()`, and system time isn't restored across wake properly. Status as of April 2026: still open, low priority. Workaround: skip automatic tickless light-sleep on S3 and invoke deep sleep explicitly from the application.

## Workspace structure for the tag + relay multi-firmware project

The idiomatic Zephyr layout for this project is **T2 topology (workspace application)** — your project repo *is* the west manifest repo, importing Zephyr and `hal_espressif` as pinned dependencies. This mirrors the official `zephyrproject-rtos/example-application` template and is what Golioth, Memfault, Nordic's nRF Connect SDK, and virtually every commercial Zephyr SDK uses:

```
distress-beacon-workspace/
├── zephyr/                       # populated by `west update`
├── modules/hal/espressif/
└── distress-beacon/              # manifest repo (self-path)
    ├── west.yml
    ├── zephyr/module.yml         # makes this repo a module
    ├── boards/heltec/heltec_wireless_tracker/  # optional overrides
    ├── dts/bindings/
    ├── lib/common/               # shared protocol/crypto/power code
    ├── include/distress_beacon/
    ├── snippets/lora-eu868/
    └── apps/
        ├── tag/                  # one Zephyr app
        │   ├── CMakeLists.txt, prj.conf, sysbuild.conf
        │   ├── app.overlay
        │   └── src/main.c
        └── relay/                # another Zephyr app
```

The `zephyr/module.yml` declaration is what makes shared code work cleanly — it lets your `boards/`, `dts/bindings/`, and `lib/common/` be auto-discovered by every app in the workspace without `BOARD_ROOT` gymnastics. You build each app separately: `west build -b heltec_wireless_tracker/esp32s3/procpu distress-beacon/apps/tag -d build/tag --sysbuild`. **Use sysbuild + MCUboot from day one** — sysbuild has been stable since Zephyr 3.4 and is now the default recommended way to produce multi-image firmware with OTA support. A `sysbuild.conf` with `SB_CONFIG_BOOTLOADER_MCUBOOT=y` automatically builds and signs the MCUboot bootloader plus your app, and `west flash` programs both.

The `west.yml` should pin Zephyr to a specific commit hash rather than a tag — **this is Espressif's own explicit recommendation** ("version tags are weak indicators of software status" per their Zephyr Support Status page). Use `name-allowlist` in the manifest import to whitelist only the modules you need (cmsis, hal_espressif, mbedtls, mcuboot, picolibc, segger, zcbor) to keep `west update` fast.

Don't forget **`west blobs fetch hal_espressif`** before the first build — Wi-Fi, BLE, and PHY binary blobs are mandatory, and the build will link-fail without them.

## The honest verdict and the alternatives

Zephyr + ESP32-S3 + SX1262 + UC6580 is **technically ready** — Espressif officially calls ESP32-S3 production-ready in Zephyr, the Wireless Tracker V1.1 is upstream with working LoRa/GNSS/TFT/ADC/button support, the raw LoRa API is battle-tested, cross-chip syncword compatibility is automatic, the GNSS subsystem handles UC6580 via generic NMEA, and deep sleep with GPIO + RTC + wake-on-radio is achievable. The pain points are real but manageable: pin Zephyr to a known-good commit hash and run your own regression tests before bumping, use GPIO chip-select not HW-CS on the SX1262, respect the GPIO3/VEXT active-HIGH hold-before-sleep sequence, avoid automatic light-sleep on S3, and skip Zephyr entirely for the RPi gateway.

The **strongest counter-argument** is ecosystem weight. Meshtastic (github.com/meshtastic/firmware, 7.3k stars) already supports the Heltec Wireless Tracker V1.1 out of the box with identical conceptual design — LoRa mesh, GNSS broadcast, BLE config, deep-sleep tracker role. It's Arduino/ESP-IDF-based, not Zephyr. For a proof-of-concept or low-volume deployment, **flashing Meshtastic onto both devices to validate RF range, GNSS performance, and power budget first** is strictly lower risk than building a Zephyr port upfront. Alternatively, ESP-IDF + RadioLib + TinyGPS gives you first-class Wi-Fi+BLE coex and the full Espressif tooling ecosystem, at the cost of Zephyr's cleaner DT/Kconfig/sysbuild/MCUboot story.

A pragmatic hybrid worth considering: **Zephyr on the relay/gateway where its multi-SoC portability and MCUboot OTA shine, and Meshtastic or ESP-IDF+RadioLib on the ESP32-S3 tag where the community momentum is**. If a single unified Zephyr codebase across tag and relay is a hard requirement — for audit trails, certification, or team skill consolidation — proceed with the T2 layout above, pin to a specific Zephyr commit, test every upgrade in a regression suite, and keep Meshtastic in your pocket as a reference for the power-management and GNSS-cadence tuning you'll eventually need to match.

## Key links to bookmark

**Upstream Zephyr documentation and source**
- Heltec Wireless Tracker V1.1 board doc: https://docs.zephyrproject.org/latest/boards/heltec/heltec_wireless_tracker/doc/index.html
- Heltec WiFi LoRa 32 V3 (sister board reference): https://docs.zephyrproject.org/latest/boards/heltec/heltec_wifi_lora32_v3/doc/index.html
- ESP32-S3 SoC features: https://docs.zephyrproject.org/latest/boards/espressif/common/soc-esp32s3-features.html
- LoRa/LoRaWAN subsystem: https://docs.zephyrproject.org/latest/connectivity/lora_lorawan/index.html
- SX1262 DT binding: https://docs.zephyrproject.org/latest/build/dts/api/bindings/lora/semtech,sx1262.html
- SX1276 DT binding: https://docs.zephyrproject.org/latest/build/dts/api/bindings/lora/semtech,sx1276.html
- LoRa send sample: https://github.com/zephyrproject-rtos/zephyr/tree/main/samples/drivers/lora
- GNSS subsystem: https://docs.zephyrproject.org/latest/hardware/peripherals/gnss.html
- `gnss-nmea-generic` binding: https://docs.zephyrproject.org/latest/build/dts/api/bindings/gnss/gnss-nmea-generic.html
- GNSS sample: https://docs.zephyrproject.org/latest/samples/drivers/gnss/README.html
- ESP32 deep sleep sample: https://docs.zephyrproject.org/latest/samples/boards/espressif/deep_sleep/README.html
- Workspaces/west: https://docs.zephyrproject.org/latest/develop/west/workspaces.html
- Sysbuild: https://docs.zephyrproject.org/latest/build/sysbuild/index.html

**Relevant GitHub PRs and issues**
- PR #101598 (heltec_wifi_lora32_v3, HW-CS discussion): https://github.com/zephyrproject-rtos/zephyr/pull/101598
- Issue #86192 (ESP32-S3 boot regression 4.0→4.1): https://github.com/zephyrproject-rtos/zephyr/issues/86192
- Issue #87127 (XIAO ESP32-S3 SPI DMA): https://github.com/zephyrproject-rtos/zephyr/issues/87127
- Issue #86255 (SX1262 RX freeze): https://github.com/zephyrproject-rtos/zephyr/issues/86255
- Issue #96639 (ESP32-S3 light-sleep hang): https://github.com/zephyrproject-rtos/zephyr/issues/96639
- Discussion #78995 (LoRa syncword override): https://github.com/zephyrproject-rtos/zephyr/discussions/78995
- Generic NMEA driver PR #65422: https://github.com/zephyrproject-rtos/zephyr/pull/65422

**Espressif and tooling**
- Espressif Zephyr Support Status: https://developer.espressif.com/software/zephyr-support-status
- hal_espressif module: https://github.com/zephyrproject-rtos/hal_espressif
- OpenOCD ESP32 fork: https://github.com/espressif/openocd-esp32
- MCUboot Espressif readme: https://docs.mcuboot.com/readme-espressif.html

**Templates and reference projects**
- Official example-application (T2 template): https://github.com/zephyrproject-rtos/example-application
- Golioth Firmware SDK: https://github.com/golioth/golioth-firmware-sdk
- Interrupt "Practical Zephyr — West Workspaces": https://interrupt.memfault.com/blog/practical_zephyr_west
- Golioth blog on helper-code modules: https://blog.golioth.io/how-to-turn-helper-code-into-a-zephyr-module/

**Hardware references**
- Heltec product page: https://heltec.org/project/wireless-tracker/
- V1.1 schematic: https://resource.heltec.cn/download/Wireless_Tracker/Wireless_Tacker1.1/HTIT-Tracker_V0.5.pdf
- Dragino LoRa/GPS HAT wiki: https://wiki1.dragino.com/index.php/Lora/GPS_HAT
- Meshtastic firmware (alternative reference implementation): https://github.com/meshtastic/firmware
- LoRaMac-node (driver backend): https://github.com/Lora-net/LoRaMac-node
