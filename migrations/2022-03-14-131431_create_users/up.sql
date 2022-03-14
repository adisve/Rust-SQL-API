-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `pillow_db`.`user` (
  `user_id` INT NOT NULL AUTO_INCREMENT,
  `email` VARCHAR(40) NOT NULL,
  `password` VARCHAR(45) NOT NULL,
  `name` VARCHAR(30) NULL,
  PRIMARY KEY (`user_id`))

CREATE TABLE IF NOT EXISTS `pillow_db`.`phone` (
  `imei` CHAR(15) NOT NULL,
  `uuid` CHAR(38) NOT NULL,
  `mac` CHAR(12) NOT NULL,
  `brand` VARCHAR(45) NULL,
  `model` VARCHAR(30) NULL,
  `manufacturer` VARCHAR(30) NULL,
  `user_id` INT NOT NULL,
  PRIMARY KEY (`imei`),
  INDEX `fk_Phone_User_idx` (`user_id` ASC) VISIBLE,
  CONSTRAINT `fk_Phone_User`
    FOREIGN KEY (`user_id`)
    REFERENCES `pillow_db`.`user` (`user_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)

CREATE TABLE IF NOT EXISTS `pillow_db`.`schedule` (
  `schedule_id` INT NOT NULL AUTO_INCREMENT,
  `alarm_date` VARCHAR(40) NULL,
  `user_id` INT NOT NULL,
  `schedule_name` VARCHAR(45) NULL,
  PRIMARY KEY (`schedule_id`),
  INDEX `fk_Schedule_User1_idx` (`user_id` ASC) VISIBLE,
  CONSTRAINT `fk_Schedule_User1`
    FOREIGN KEY (`user_id`)
    REFERENCES `pillow_db`.`user` (`user_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)