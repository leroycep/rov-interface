
#define RESPONSE_MOTOR 0x10
#define RESPONSE_COMPASS_ORIENTATION 0x20
#define RESPONSE_COMPASS_DISABLED 0x21
#define RESPONSE_LIGHTS_ON 0x31
#define RESPONSE_LIGHTS_OFF 0x30
#define RESPONSE_MASTER_ON 0x40
#define RESPONSE_MASTER_OFF 0x43
#define RESPONSE_SERVO 0x66

#define TWO_BYTES_TO_ARRAY(name) ((uint8_t)((name>>8)&0xff)),((uint8_t)((name)&0xff))

void say_motor(uint8_t id, int16_t thrust) {
    uint8_t buf[] = {
        RESPONSE_MOTOR,
        id,
        (uint8_t)((thrust>>8)&0xff),
        (uint8_t)((thrust)&0xff)
        };
    Serial.write(buf, 4);
}

void say_compass_orientation(int16_t x, int16_t y, int16_t z) {
    uint8_t buf[] = {
      RESPONSE_COMPASS_ORIENTATION,
      TWO_BYTES_TO_ARRAY(x),
      TWO_BYTES_TO_ARRAY(y),
      TWO_BYTES_TO_ARRAY(z)
    };
    Serial.write(buf, 3 * 2 + 1); // 7
}

void say_compass_disabled() {
    Serial.write(RESPONSE_COMPASS_DISABLED);
}

void say_lights_on() {
    Serial.write(RESPONSE_LIGHTS_ON);
}

void say_lights_off() {
    Serial.write(RESPONSE_LIGHTS_OFF);
}

void say_master_on() {
    Serial.write(RESPONSE_MASTER_ON);
}

void say_master_off() {
    Serial.write(RESPONSE_MASTER_OFF);
}

void say_servo(uint8_t id, int16_t microseconds) {
    uint8_t buf[] = {
        RESPONSE_SERVO,
        id,
        (uint8_t)((microseconds>>8)&0xff),
        (uint8_t)((microseconds)&0xff)
        };
    Serial.write(buf, 4);
}

