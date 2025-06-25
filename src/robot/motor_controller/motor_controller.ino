#include <AccelStepper.h>

#define xStepPin 2
#define xDirPin 5
#define yStepPin 3
#define yDirPin 6
#define zStepPin 4
#define zDirPin 7
#define enablePin 8

#define motorInterfaceType 1

#define outerPos 1.5

#define rev 200 * 2
#define vel rev * 2
#define acc vel * 100.

int left_pos = 0;
int right_pos = 0;

AccelStepper sideL(motorInterfaceType, xStepPin, xDirPin, 4, 5, false); // x Axis is left Side
AccelStepper sideR(motorInterfaceType, yStepPin, yDirPin, 4, 5, false); // y Axis is right Side
AccelStepper slice(motorInterfaceType, zStepPin, zDirPin, 4, 5, false); // z Axis is Slice

void setup() {
  sideL.setMaxSpeed(vel * 1.5);
  sideL.setAcceleration(acc);
  sideR.setMaxSpeed(vel * 1.5);
  sideR.setAcceleration(acc);
  slice.setMaxSpeed(vel);
  slice.setAcceleration(acc);

  pinMode(enablePin, OUTPUT);
  digitalWrite(enablePin, HIGH);

  Serial.begin(115200);
  Serial.write((uint8_t) 0x00);
}

/* codes:

1111xxxx : other

01xx : turn on
00 : -2
01 : -1
10 : 1
11 : 2

0000 : turn off

10xx : move to slice pos
00 : -2
01 : -1
10 : 1
11 : 2

11xx adjust lr
lr

----

llllrrrr : lr turn

*/

void loop() {
  if (sideL.isRunning() || sideR.isRunning()) {
    // runs motors
    bool left_running = sideL.run();
    bool right_running = sideR.run();
    // checks if motors are done
    if (!left_running && !right_running) {
      // bases motors position
      if (left_pos > 12) {
        left_pos -= 12;
        sideL.setCurrentPosition((left_pos * rev) / 12);
      }
      else if (left_pos < -12) {
        left_pos += 12;
        sideL.setCurrentPosition((left_pos * rev) / 12);
      }
      if (right_pos > 12) {
        right_pos -= 12;
        sideR.setCurrentPosition((right_pos * rev) / 12);
      }
      else if (right_pos < -12) {
        right_pos += 12;
        sideR.setCurrentPosition((right_pos * rev) / 12);
      }
      // sends done
      Serial.write(0x00);
    }
  } else if (Serial.available() > 0) {
    int command = Serial.read();
    if ((command & 0b11110000) == 0b11110000) {
      // other command
      if (command & 0b1000) {
        if (command & 0b0100) {
          // 11xx
          // set speed mode
          if (command & 0b0001) {
            sideL.setMaxSpeed(vel * 1.75);
            sideR.setMaxSpeed(vel * 1.75);
            slice.setMaxSpeed(vel * 1.25);
          } else {
            sideL.setMaxSpeed(vel * 1.5);
            sideR.setMaxSpeed(vel * 1.5);
            slice.setMaxSpeed(vel);
          }
          Serial.write(0x00);
        } else {
          // 10xx
          // moves to slice position
          switch (command & 0b0011) {
            case 0:
              slice.runToNewPosition((long) (-(outerPos + 0.1) * rev / 4));
              slice.setCurrentPosition(-outerPos * rev / 4);
              break;
            case 1:
              slice.runToNewPosition(-1 * rev / 4);
              break;
            case 2:
              slice.runToNewPosition(1 * rev / 4);
              break;
            case 3:
              slice.runToNewPosition((long) ((outerPos + 0.1) * rev / 4));
              slice.setCurrentPosition(outerPos * rev / 4);
              break;
          }
          Serial.write(0x00);
        }
      } else {
        if (command & 0b0100) {
          // 01xx
          // turns on motors
          sideL.enableOutputs();
          sideR.enableOutputs();
          slice.enableOutputs();
          digitalWrite(enablePin, LOW);
          float slice_pos = 0;
          switch (command & 0b0011) {
            case 0:
              slice_pos = -outerPos;
              break;
            case 1:
              slice_pos = -1;
              break;
            case 2:
              slice_pos = 1;
              break;
            case 3:
              slice_pos = outerPos;
              break;
          }
          slice.setCurrentPosition(slice_pos * rev / 4);
          Serial.write(0x00);
        } else {
          // 0000
          // turns off motors
          digitalWrite(enablePin, HIGH);
          sideL.disableOutputs();
          sideR.disableOutputs();
          slice.disableOutputs();
          Serial.write(0x00);
        }
      }
    } else {
      // llllrrrr
      // executes layer turn
      int left = (command & 0b11110000) >> 4;
      int right = command & 0b00001111;
      if (left > 6)
        left -= 12;
      if (right > 6)
        right -= 12;
      if (left == 0 && right == 0)
        Serial.write(0x00);
      else {
        left_pos += left;
        right_pos += right;
        sideL.moveTo((left_pos * rev) / 12);
        sideR.moveTo((right_pos * rev) / 12);
      }
    }
  }
}
