#!/bin/bash 

cross build --target arm-unknown-linux-gnueabi


readonly TARGET_HOST=pi@192.168.0.10
readonly TARGET_PATH=/home/pi/lights
readonly SOURCE_PATH=./target/arm-unknown-linux-gnueabi/debug/rust_leds

rsync $(SOURCE_PATH) $(TARGET_HOST):$(TARGET_PATH)
ssh -t $(TARGET_HOST) $(TARGET_PATH)
# ok I need to send this over to the pi if it is on the network 
# Test for it being on 192.168.0.10??
