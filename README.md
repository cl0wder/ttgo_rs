This is an ESP32 embedded Rust project that drives an ST7789 LCD display. 


- ESP32-based embedded system using no_std
- Controls a 135x240 ST7789 LCD display via SPI
- Currently displays static text 1234567890123456789012345"
Key Files:
src/bin/main.rs: Main application with display initialization and text rendering
- src/lib.rs: Minimal library file
- Cargo. toml: Dependencies including esp-hal, embedded-graphics, mipidsi
- blueprint.md: Detailed implementation plan for adding WiFi, Bluetooth, and buttons
Current Hardware Setup:
- Display: ST7789 LCD (135x240, rotated 90º)
- SPI: 26MHz on GPIO 18/19 (SCLK/MOSI)
- Control: GPIO 5 (CS), 16 (DC), 23 (RST), 4 (Backlight)
Planned Features (from blueprint):
- WiFi connectivity with esp-wifi
- Two GPIO buttons (GPIOO, GPIO2) with debouncing
- Bluetooth/BLE support
- Status display improvements
