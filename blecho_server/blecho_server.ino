/** Tiny chat server that runs on the ESP32 microcontroller
 * Copyright © 2022 JonLiuFYI
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>

#define ESP32_SERVICE_UUID "4fafc201-1fb5-459e-8fcc-c5c9c331914b"
#define CHARACTERISTIC_UUID "12570b69-4f11-4b26-bbaa-54127cba01e9"

BLEServer *server = NULL;
bool connected = false;
bool once = false;

class ChatCallbacks : public BLECharacteristicCallbacks {
  void onWrite(BLECharacteristic *chara) {
    std::string recv = chara->getValue();

    if (recv.length() > 0) {
      Serial.print("PC: ");
      Serial.print(recv.c_str());
      Serial.println();
    }
  }
};

class SrvCallbacks : public BLEServerCallbacks {
  void onConnect(BLEServer *_) {
    connected = true;
    once = true;
    Serial.println("[Server] Client connected");
  };

  void onDisconnect(BLEServer *_) {
    connected = false;
  }
};

void setup() {
  Serial.begin(115200);
  Serial.println("[Server] Starting");

  BLEDevice::init("BLEcho");
  server = BLEDevice::createServer();
  server->setCallbacks(new SrvCallbacks());
  BLEService *service = server->createService(ESP32_SERVICE_UUID);

  BLECharacteristic *chara = service->createCharacteristic(
    CHARACTERISTIC_UUID,
    BLECharacteristic::PROPERTY_READ | BLECharacteristic::PROPERTY_WRITE);
  chara->setCallbacks(new ChatCallbacks());
  chara->setValue("Hey, write to this characteristic!");

  service->start();
  server->getAdvertising()->start();

  Serial.println("[Server] Ready and waiting for client");
}

void loop() {
  // disconnected
  if (!connected && once) {
    Serial.println("[Server] Client disconnected");
    delay(1000);
    server->startAdvertising();
    once = false;
    Serial.println("[Server] Ready and waiting for client");
  }
}
