# GGTO RS Project Blueprint

## Current State Overview

This is an ESP32-based embedded Rust project that currently drives an ST7789 LCD display (135x240, rotated 90 degrees). The project uses `no_std` and is built for embedded systems deployment.

## Current Architecture

### Hardware Configuration
- **MCU**: ESP32
- **Display**: ST7789 LCD (135x240)
- **Communication**: SPI interface

### GPIO Pin Assignments
```
SCLK: GPIO18  (SPI Clock)
MOSI: GPIO19  (SPI Master Out)
CS:   GPIO5   (Chip Select)
DC:   GPIO16  (Data/Command)
RST:  GPIO23  (Reset)
Backlight: GPIO4
```

### Software Stack
- **Core**: `esp-hal` v0.23.1 with ESP32 support
- **Display**: `embedded-graphics`, `mipidsi`, `display-interface-spi`
- **Bus**: `embedded-hal-bus` for SPI device management
- **SPI Config**: 26MHz, Mode 0

### Current Functionality
1. Initialize ESP32 peripherals
2. Set up SPI communication for display
3. Configure and initialize ST7789 display
4. Display static text "ST7789 ONLINE" in white on black background
5. Enter infinite loop with 1-second delays

## Code Structure

### Files
- `src/bin/main.rs`: Main application entry point (83 lines)
- `src/lib.rs`: Minimal library file (currently just `#![no_std]`)
- `Cargo.toml`: Project configuration and dependencies
- `build.rs`: Build script with linker error handling

### Key Components
1. **Panic Handler**: Simple infinite loop panic handler
2. **Display Initialization**: Complete ST7789 setup with orientation and color inversion
3. **Text Rendering**: Using embedded-graphics with 9x18 ASCII font

## Missing Features for TODO Plan

### Currently NOT Implemented:
- ❌ Wi-Fi connectivity
- ❌ Bluetooth support
- ❌ GPIO input handling (buttons)
- ❌ User interaction logic
- ❌ Network communication protocols
- ❌ State management
- ❌ Error handling beyond basic panic
- ❌ Configuration management

## Hardware Requirements for New Features

### Wi-Fi Support
- ESP32 has built-in Wi-Fi hardware
- Need antenna connection on ESP32 board
- Requires `esp-hal` Wi-Fi features to be enabled

### Bluetooth Support  
- ESP32 has built-in Bluetooth hardware
- Need `esp-hal` Bluetooth features to be enabled
- Classic Bluetooth and/or BLE support needed

### 2 Onboard Buttons
- Need 2 GPIO pins configured as inputs
- Should include pull-up/pull-down resistors
- Debouncing logic required
- Need to decide on specific GPIO pins

## Development Environment Setup

### Build Requirements
- Rust 1.88+ with embedded targets
- `esp32` target installed
- `cargo-espflash` or similar for flashing

### Current Build Configuration
- Optimized for size (`opt-level = "s"`)
- Fat LTO enabled in release
- Debug info included in release builds

## Detailed TODO Plan for Wi-Fi, Buttons, and Bluetooth

### 🔥 HIGH PRIORITY

#### 1. Wi-Fi Implementation
**Dependencies to Add:**
```toml
esp-wifi = { version = "0.11", features = ["esp32", "wifi"] }
smoltcp = { version = "0.11", default-features = false, features = ["proto-igmp", "proto-ipv4", "socket-tcp", "socket-icmp", "socket-udp", "medium-ethernet", "proto-dhcpv4", "socket-raw"] }
heapless = "0.8"
```

**Implementation Steps:**
1. Initialize WiFi using `esp_wifi::init()` with timer and RNG
2. Create WiFi configuration with SSID/password storage
3. Implement connection status monitoring
4. Add IP handling and network stack initialization
5. Display WiFi status on LCD (connecting/connected/error)
6. Add error handling for connection failures

**Code Changes Needed:**
- Update `main.rs` to include WiFi initialization after display setup
- Add WiFi state management
- Integrate status updates into display loop

#### 2. Two Onboard Buttons Implementation
**GPIO Pin Selection:**
- Button 1: GPIO0 (commonly used, has pull-up)
- Button 2: GPIO2 (available, alternative: GPIO15)

**Dependencies to Add:**
```toml
# No additional dependencies needed - using esp-hal
```

**Implementation Steps:**
1. Configure GPIO pins as inputs with pull-up resistors
2. Implement button debouncing (software or hardware)
3. Add interrupt handling or polling in main loop
4. Create button state management
5. Add visual feedback on display for button presses
6. Implement button actions (WiFi toggle, menu navigation, etc.)

**Code Changes Needed:**
- Add button initialization in `main.rs`
- Implement button reading logic with debouncing
- Update display based on button interactions

### 🔶 MEDIUM PRIORITY

#### 3. Bluetooth Support Implementation
**Dependencies to Add:**
```toml
esp32-nimble = { version = "0.8", features = ["ble"] }
# OR
bleps = { version = "0.1", features = ["esp32"] }
```

**Implementation Steps:**
1. Choose between esp32-nimble or bleps crate
2. Initialize BLE stack
3. Implement basic BLE peripheral (advertising)
4. Add basic GATT service for configuration
5. Add Bluetooth status display on LCD
6. Implement simple protocol for remote control

**Code Changes Needed:**
- Add BLE initialization after WiFi setup
- Create BLE task/loop alongside main display loop
- Add status indicators to display

### 📋 LOW PRIORITY

#### 4. System Integration and Polish
**Tasks:**
1. Improve error handling throughout application
2. Add configuration management (WiFi credentials, device settings)
3. Implement power management for battery operation
4. Add OTA update support via WiFi
5. Create comprehensive status display system
6. Add logging/debugging capabilities

### 🔧 Technical Implementation Details

#### Button Implementation Code Pattern:
```rust
use esp_hal::gpio::{Input, Pull};

let button1 = Input::new(peripherals.GPIO0, Pull::Up);
let button2 = Input::new(peripherals.GPIO2, Pull::Up);

// In main loop with debouncing:
if button1.is_low() {
    // Button 1 pressed (active-low due to pull-up)
    delay.delay_millis(50); // Debounce
    if button1.is_low() {
        // Confirmed press - handle action
    }
}
```

#### WiFi Implementation Code Pattern:
```rust
use esp_wifi::{initialize, EspWifiInitFor, wifi::WifiController};

let timg0 = esp_hal::timer::TimerGroup::new(peripherals.TIMG0);
let init = esp_wifi::init(
    EspWifiInitFor::Wifi,
    timg0.timer0,
    Rng::new(peripherals.RNG),
    peripherals.RADIO_CLK,
).unwrap();

let wifi = wifi::WifiController::new(init, peripherals.WIFI)?;
// Configure and connect...
```

#### BLE Implementation Code Pattern:
```rust
use esp32_nimble::{BLEDevice, utilities::BleUtilities};

let ble_device = BLEDevice::take();
let ble_advertising = ble_device.get_advertising();

// Set up advertising data and start...
```

### 📊 Resource Requirements

#### Memory Usage Estimates:
- **Current**: ~50KB flash, ~8KB RAM
- **With WiFi**: +150KB flash, +30KB RAM  
- **With Buttons**: +5KB flash, +1KB RAM
- **With BLE**: +100KB flash, +20KB RAM
- **Total**: ~305KB flash, ~59KB RAM

#### GPIO Pin Usage:
```
Current:
- GPIO4:  Backlight control
- GPIO5:  SPI CS
- GPIO16: SPI DC
- GPIO18: SPI SCLK  
- GPIO19: SPI MOSI
- GPIO23: Display Reset

Adding:
- GPIO0:  Button 1 (input, pull-up)
- GPIO2:  Button 2 (input, pull-up)

Available for future:
- GPIO12,13,14,15,17,21,22,25,26,27,32,33,34,35,36,39
```

## Next Steps Priority

1. **HIGH**: Implement WiFi connectivity with status display
2. **HIGH**: Add 2 onboard buttons with debouncing and actions
3. **MEDIUM**: Implement Bluetooth support for remote control
4. **LOW**: System integration, error handling, and polish

## Technical Considerations

### Memory Constraints
- Embedded system with limited RAM
- Need to optimize for code size
- Consider stack usage for networking stacks

### Real-time Requirements
- Display updates need to be responsive
- Button inputs require timely processing
- Network operations may need async handling

### Power Management
- Consider sleep modes for battery operation
- Display backlight power control
- Wi-Fi/Bluetooth power optimization