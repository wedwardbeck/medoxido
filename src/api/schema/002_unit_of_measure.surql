
DEFINE table unit_of_measure SCHEMAFULL;
DEFINE FIELD name ON TABLE unit_of_measure TYPE string ASSERT $value != NONE;
DEFINE FIELD abbreviation ON TABLE unit_of_measure TYPE string ASSERT $value != NONE;
DEFINE FIELD active ON TABLE unit_of_measure TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON unit_of_measure VALUE $before OR time::now();
DEFINE FIELD updated ON unit_of_measure VALUE time::now();

//  Indexes
DEFINE INDEX unit_of_measure_index ON unit_of_measure FIELDS name, abbreviation UNIQUE;
