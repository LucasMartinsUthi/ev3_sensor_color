#!/usr/bin/env python3

from time import sleep

from ev3dev2.motor import LargeMotor, OUTPUT_C

m = LargeMotor(OUTPUT_C)
m.on(50)