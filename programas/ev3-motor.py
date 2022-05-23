#!/usr/bin/env python3
from time import sleep

from ev3dev2.sensor.lego import GyroSensor

gyro = GyroSensor()
gyro.mode = gyro.MODE_TILT  

filteredAngle = 0
while True:
    print(gyro.angle)

    # (gyroAngle, rate) = gyro.angle_and_rate 

    # filteredAngle = 0.98 * (filteredAngle + rate) + 0.02 * gyroAngle

    # angle = 0.98*gyrAngle + 0.02*rate
    
    # print(filteredAngle)
    sleep(0.1)
