
DEFINE TABLE dose SCHEMAFULL;

DEFINE FIELD user ON TABLE dose TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD store ON TABLE dose TYPE record(store) ASSERT $value != NONE;
DEFINE FIELD quantity ON TABLE dose TYPE float ASSERT $value != NONE AND $value > 0;
DEFINE FIELD unit ON TABLE dose TYPE string ASSERT $value != NONE;
DEFINE FIELD created ON dose VALUE $before OR time::now();
DEFINE FIELD updated ON dose VALUE time::now();

// Functions
DEFINE FUNCTION fn::list_doses_for_medication($id: string) {let $results = (select id, created, updated,
quantity as dose_quantity, unit as dose_unit, store as store_id,store.medication as  medication_id,
store.medication.name as medication_name, store.quantity as store_start_quantity,
store.production_date as store_production_date, store.unit as store_unit, store.created as store_created,
store.updated as store_updated, store.active as store_active, user as user
from dose where store.medication = type::thing('medication', $id) ORDER BY created);
RETURN $results;};

DEFINE FUNCTION fn::list_doses_for_store($id: string) {let $results = (select id, created, updated,
quantity as dose_quantity, unit as dose_unit, store as store_id,store.medication as  medication_id,
store.medication.name as medication_name, store.quantity as store_start_quantity,
store.production_date as store_production_date, store.unit as store_unit, store.created as store_created,
store.updated as store_updated, store.active as store_active, user as user
from dose where store = type::thing('store', $id) ORDER BY created);
RETURN $results;};

//TODO: Add user to the queries
