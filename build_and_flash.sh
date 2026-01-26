#!/bin/bash

cargo espflash flash --monitor --chip esp32 --release --target xtensa-esp32-espidf --ignore-app-descriptor