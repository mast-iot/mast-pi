-- Your SQL goes here

CREATE TABLE "device" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" text NOT NULL,
  "desc" TEXT,
  "device_type" TEXT NOT NULL,
  "icon" TEXT,
  "room_id" INTEGER NOT NULL,
  "group_id" INTEGER
);



CREATE TABLE "group" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL
);
CREATE TABLE "param" (
  "id" INTEGER NOT NULL,
  "param_type" TEXT NOT NULL,
  "key" TEXT NOT NULL,
  "desc" TEXT,
  "options" TEXT NOT NULL,
  "value" TEXT NOT NULL,
  "usage" TEXT NOT NULL,
  "device_id" INTEGER NOT NULL,
  PRIMARY KEY ("id")
);


CREATE TABLE "room" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL,
  "desc" TEXT
);
INSERT INTO "room" VALUES (1, '客厅', '客厅某个插座');