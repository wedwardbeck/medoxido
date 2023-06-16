DEFINE TABLE store SCHEMAFULL;

DEFINE FIELD user ON TABLE store TYPE string ASSERT $value != NONE;
DEFINE FIELD medication ON TABLE store TYPE record(medication) ASSERT $value != NONE;
DEFINE FIELD production_date ON TABLE store TYPE datetime;
DEFINE FIELD expiration_date ON TABLE store TYPE datetime;
DEFINE FIELD lot_number ON TABLE store TYPE string;
DEFINE FIELD quantity ON TABLE store TYPE float ASSERT $value != NONE AND $value > 0;
-- DEFINE FIELD remaining ON TABLE store TYPE float ASSERT $value != NONE AND $value >= 0;
DEFINE FIELD unit ON TABLE store TYPE string ASSERT $value != NONE;
DEFINE FIELD active ON TABLE store TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON store VALUE $before OR time::now();
DEFINE FIELD updated ON store VALUE time::now();

//  Indexes
-- DEFINE INDEX store_index ON TABLE store COLUMNS user, medication, start, active UNIQUE;
