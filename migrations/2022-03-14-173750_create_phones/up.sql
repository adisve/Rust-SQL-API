-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `pillow_db`.`phone` (
  `imei` CHAR(15) NOT NULL,
  `uuid` CHAR(38) NOT NULL,
  `mac` CHAR(12) NOT NULL,
  `brand` VARCHAR(45)  NOT NULL,
  `model` VARCHAR(30) NOT NULL,
  `manufacturer` VARCHAR(30) NOT NULL,
  `user_id` INT NOT NULL,
  PRIMARY KEY (`imei`),
  INDEX `fk_Phone_User_idx` (`user_id` ASC) VISIBLE,
  CONSTRAINT `fk_Phone_User`
    FOREIGN KEY (`user_id`)
    REFERENCES `pillow_db`.`user` (`user_id`)
    ON DELETE CASCADE
    ON UPDATE NO ACTION);