-- CreateTable
CREATE TABLE "Account" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "username" TEXT NOT NULL,
    "accessToken" TEXT NOT NULL,
    "refreshToken" TEXT NOT NULL,
    "expiresAt" DATETIME NOT NULL,
    "lastRefreshed" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "Modpack" (
    "modpackId" TEXT NOT NULL,
    "adapter" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "iconUrl" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "descriptionFormat" TEXT NOT NULL,
    "shortDescription" TEXT NOT NULL,
    "author" TEXT NOT NULL,

    PRIMARY KEY ("modpackId", "adapter")
);

-- CreateTable
CREATE TABLE "Mod" (
    "modid" TEXT NOT NULL,
    "adapter" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "file" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "descriptionFormat" TEXT NOT NULL,
    "shortDescription" TEXT NOT NULL,
    "author" TEXT NOT NULL,

    PRIMARY KEY ("modid", "adapter")
);

-- CreateTable
CREATE TABLE "ModsInModpack" (
    "modId" TEXT NOT NULL,
    "modAdapter" TEXT NOT NULL,
    "modpackId" TEXT NOT NULL,
    "modpackAdapter" TEXT NOT NULL,

    PRIMARY KEY ("modpackId", "modId"),
    CONSTRAINT "ModsInModpack_modpackId_modpackAdapter_fkey" FOREIGN KEY ("modpackId", "modpackAdapter") REFERENCES "Modpack" ("modpackId", "adapter") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "ModsInModpack_modId_modAdapter_fkey" FOREIGN KEY ("modId", "modAdapter") REFERENCES "Mod" ("modid", "adapter") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "Account_username_key" ON "Account"("username");
