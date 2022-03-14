-- Your SQL goes here
SET FOREIGN_KEY_CHECKS=0;

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
    ON DELETE CASCADE
    ON UPDATE NO ACTION);