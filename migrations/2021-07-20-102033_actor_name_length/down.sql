-- SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
--
-- SPDX-License-Identifier: AGPL-3.0-only



DROP VIEW person_alias_1;
DROP VIEW person_alias_2;

ALTER TABLE community ALTER COLUMN name TYPE varchar(20);
ALTER TABLE community ALTER COLUMN title TYPE varchar(100);
ALTER TABLE person ALTER COLUMN name TYPE varchar(20);
ALTER TABLE person ALTER COLUMN display_name TYPE varchar(20);

create view person_alias_1 as select * from person;
create view person_alias_2 as select * from person;
