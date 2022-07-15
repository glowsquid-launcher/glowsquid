/*
  Warnings:

  - The primary key for the `ModsInModpack` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_ModsInModpack" (
    "modId" TEXT NOT NULL,
    "modAdapter" TEXT NOT NULL,
    "modpackId" TEXT NOT NULL,
    "modpackAdapter" TEXT NOT NULL,

    PRIMARY KEY ("modpackId", "modId", "modpackAdapter", "modAdapter"),
    CONSTRAINT "ModsInModpack_modpackId_modpackAdapter_fkey" FOREIGN KEY ("modpackId", "modpackAdapter") REFERENCES "Modpack" ("modpackId", "adapter") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "ModsInModpack_modId_modAdapter_fkey" FOREIGN KEY ("modId", "modAdapter") REFERENCES "Mod" ("modid", "adapter") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_ModsInModpack" ("modAdapter", "modId", "modpackAdapter", "modpackId") SELECT "modAdapter", "modId", "modpackAdapter", "modpackId" FROM "ModsInModpack";
DROP TABLE "ModsInModpack";
ALTER TABLE "new_ModsInModpack" RENAME TO "ModsInModpack";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
