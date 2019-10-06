DROP TABLE IF EXISTS ways;
CREATE TABLE ways (id INTEGER,version INTEGER,changeset INTEGER,user VARCHAR(150),uid INTEGER,visible TINYINT(2),date_time VARCHAR(150),CONSTRAINT ways_pk PRIMARY KEY(id));
INSERT INTO ways (id, version, changeset, user, uid, visible, date_time) VALUES ("26659127","5","4142606","Masch","55988","1","2010-03-16T11:47:08Z") 