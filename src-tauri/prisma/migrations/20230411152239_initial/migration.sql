-- CreateTable
CREATE TABLE "User" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "title" TEXT,
    "surname" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "other_names" TEXT,
    "gender" TEXT NOT NULL,
    "date_of_birth" DATETIME NOT NULL,
    "updated_at" DATETIME NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
