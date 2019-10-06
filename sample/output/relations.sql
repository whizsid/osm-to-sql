DROP TABLE IF EXISTS relations;
CREATE TABLE relations (id INTEGER,version INTEGER,changeset INTEGER,user VARCHAR(150),uid INTEGER,visible TINYINT(2),date_time VARCHAR(150),CONSTRAINT relations_pk PRIMARY KEY(id));
INSERT INTO relations (id, version, changeset, user, uid, visible, date_time) VALUES ("56688","28","6947637","kmvar","56190","1","2011-01-12T14:23:49Z") 