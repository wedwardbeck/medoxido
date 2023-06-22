DEFINE TABLE store SCHEMAFULL;

DEFINE FIELD user ON TABLE store TYPE record(user) ASSERT $value != NONE;
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

// Functions
DEFINE FUNCTION fn::list_stores_for_medication($id: string, $active: bool, $user: string) {LET $results = (SELECT id AS store_id,
medication AS medication_id, medication.name AS medication_name, quantity AS store_start_quantity,
production_date AS store_production_date, unit AS store_unit, created AS store_created, updated AS store_updated,
lot_number AS store_lot_number, expiration_date AS store_expiration_date, active AS store_active, user AS user
FROM store WHERE medication = type::thing('medication', $id) AND $active = active AND user = type::thing('user', $user)
ORDER BY store_created);
RETURN $results;};

DEFINE FUNCTION fn::list_all_stores_for_medication($id: string, $user: string) {let $results = (select id as store_id,
medication as medication_id, medication.name as medication_name, quantity as store_start_quantity,
production_date as store_production_date, unit as store_unit, created as store_created, updated as store_updated,
lot_number as store_lot_number, expiration_date as store_expiration_date, active as store_active, user as user
from store where medication = type::thing('medication', $id) AND user = type::thing('user', $user) ORDER BY store_created);
RETURN $results;};
