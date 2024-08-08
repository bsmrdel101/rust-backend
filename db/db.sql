CREATE TABLE "5e_characters" (
  "id" SERIAL PRIMARY KEY,
  "user_id" INTEGER REFERENCES "users",
  "name" TEXT,
  "classes" TEXT DEFAULT '[]' NOT NULL,
  "race_id" INTEGER REFERENCES "5e_races",
  "background_id" INTEGER REFERENCES "5e_backgrounds",
  "characteristics_id" INTEGER REFERENCES "5e_characteristics",
  "asset_id" INTEGER REFERENCES "assets",
  "backstory" TEXT,
  "level" INTEGER DEFAULT 1 NOT NULL,
  "xp" INTEGER DEFAULT 0 NOT NULL,
  "ac" INTEGER DEFAULT 10 NOT NULL,
  "maxHp" INTEGER DEFAULT 0 NOT NULL,
  "currentHp" INTEGER DEFAULT 0 NOT NULL,
  "tempHp" INTEGER DEFAULT 0 NOT NULL,
  "inspiration" BOOLEAN DEFAULT false,
  "str" INTEGER DEFAULT 10 NOT NULL,
  "dex" INTEGER DEFAULT 10 NOT NULL,
  "con" INTEGER DEFAULT 10 NOT NULL,
  "int" INTEGER DEFAULT 10 NOT NULL,
  "wis" INTEGER DEFAULT 10 NOT NULL,
  "char" INTEGER DEFAULT 10 NOT NULL,
  "speeds" TEXT DEFAULT '[]' NOT NULL,
  "skills" TEXT DEFAULT '[]' NOT NULL,
  "senses" TEXT DEFAULT '[]' NOT NULL,
  "languages" TEXT DEFAULT '[]' NOT NULL,
  "resistances" TEXT DEFAULT '[]' NOT NULL,
  "vulnerabilities" TEXT DEFAULT '[]' NOT NULL,
  "immunities" TEXT DEFAULT '[]' NOT NULL,
  "inventory" TEXT DEFAULT '[]' NOT NULL,
  "currency" TEXT DEFAULT '[]' NOT NULL,
  "effects" TEXT DEFAULT '[]' NOT NULL,
  "variables" TEXT DEFAULT '[]' NOT NULL
);
