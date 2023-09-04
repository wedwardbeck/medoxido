DEFINE TABLE note SCHEMAFULL;

DEFINE FIELD user ON TABLE note TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD note_table ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD note_thing ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD content ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD created ON note VALUE $before OR time::now();
DEFINE FIELD updated ON note VALUE time::now();

// Functions
DEFINE FUNCTION fn::list_all_dose_notes(

) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as dose_id,
type::thing(note_table,note_thing).quantity as dose_quantity,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).store.id as store_id,
type::thing(note_table,note_thing).store.quantity as store_start_quantity,
type::thing(note_table,note_thing).store.production_date as store_production_date,
type::thing(note_table,note_thing).created as dose_created,
type::thing(note_table,note_thing).updated as dose_updated,
type::thing(note_table,note_thing).store.medication as medication_id,
type::thing(note_table,note_thing).store.medication.name as medication_name,
type::thing(note_table,note_thing).user as user
from note where note_table = "dose" ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_notes_for_dose(
    $id: string
) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as dose_id,
type::thing(note_table,note_thing).quantity as dose_quantity,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).store.id as store_id,
type::thing(note_table,note_thing).store.quantity as store_start_quantity,
type::thing(note_table,note_thing).store.production_date as store_production_date,
type::thing(note_table,note_thing).created as dose_created,
type::thing(note_table,note_thing).updated as dose_updated,
type::thing(note_table,note_thing).store.medication as medication_id,
type::thing(note_table,note_thing).store.medication.name as medication_name,
type::thing(note_table,note_thing).user as user
from note where note_table = "dose" and note_thing = $id ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_notes_for_store(
    $id: string
) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as store_id,
type::thing(note_table,note_thing).medication as medication_id,
type::thing(note_table,note_thing).medication.name as medication_name,
type::thing(note_table,note_thing).quantity as store_start_quantity,
type::thing(note_table,note_thing).production_date as store_production_date,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).created as store_created,
type::thing(note_table,note_thing).updated as store_updated,
type::thing(note_table,note_thing).active as store_active,
type::thing(note_table,note_thing).user as user
from note where note_table = "store" and note_thing = $id ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_all_store_notes(

) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as store_id,
type::thing(note_table,note_thing).medication as medication_id,
type::thing(note_table,note_thing).medication.name as medication_name,
type::thing(note_table,note_thing).quantity as store_start_quantity,
type::thing(note_table,note_thing).production_date as store_production_date,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).created as store_created,
type::thing(note_table,note_thing).updated as store_updated,
type::thing(note_table,note_thing).active as store_active,
type::thing(note_table,note_thing).user as user
from note where note_table = "store" ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_notes_for_medication(
    $id: string, $user: string
) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as medication_id,
type::thing(note_table,note_thing).name as medication_name,
type::thing(note_table,note_thing).created as medication_created,
type::thing(note_table,note_thing).updated as medication_updated,
type::thing(note_table,note_thing).active as medication_active,
type::thing(note_table,note_thing).user as user
from note where note_table = "medication" AND note_thing = $id AND user = type::thing('user', $user) ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_all_medication_notes($user: string
) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as medication_id,
type::thing(note_table,note_thing).name as medication_name,
type::thing(note_table,note_thing).created as dose_created,
type::thing(note_table,note_thing).updated as dose_updated,
type::thing(note_table,note_thing).active as medication_active,
type::thing(note_table,note_thing).user as user
from note where note_table = "medication" AND user = type::thing('user', $user) ORDER BY created);
RETURN $results;
};

//TODO: Review if statement in surrealdb to see if we can combine the all and dose, all and * based on id present or not
