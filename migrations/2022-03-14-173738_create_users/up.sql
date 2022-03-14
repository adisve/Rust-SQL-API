-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `pillow_db`.`user` (
  `user_id` INT NOT NULL AUTO_INCREMENT,
  `email` VARCHAR(40) NOT NULL,
  `password` VARCHAR(45) NOT NULL,
  `name` VARCHAR(30) NOT NULL,
  PRIMARY KEY (`user_id`));