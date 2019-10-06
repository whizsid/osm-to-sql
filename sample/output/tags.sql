DROP TABLE IF EXISTS tags;
CREATE TABLE tags (id INTEGER,name VARCHAR(150),CONSTRAINT tags_pk PRIMARY KEY(id));
INSERT INTO tags (id,name) VALUES ("0","name") , ("1","traffic_sign") , ("2","highway") , ("3","network") , ("4","operator") , ("5","ref") , ("6","route") , ("7","type") 