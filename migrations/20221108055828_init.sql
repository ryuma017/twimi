-- Add migration script here
SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Table `kaiin`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `kaiin` ;

CREATE TABLE IF NOT EXISTS `kaiin` (
  `kaiin_id` BIGINT NOT NULL AUTO_INCREMENT,
  `adana` VARCHAR(255) NOT NULL,
  `mail_address` VARCHAR(256) NOT NULL,
  `password` VARCHAR(255) NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL,
  PRIMARY KEY (`kaiin_id`),
  UNIQUE INDEX `mail_address_UNIQUE` (`mail_address` ASC) VISIBLE,
  UNIQUE INDEX `adana_UNIQUE` (`adana` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `message`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `message` ;

CREATE TABLE IF NOT EXISTS `message` (
  `message_id` INT NOT NULL AUTO_INCREMENT,
  `kaiin_id` BIGINT NOT NULL,
  `honbun` TEXT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL,
  PRIMARY KEY (`message_id`),
  INDEX `fk_message_kaiin_idx` (`kaiin_id` ASC) VISIBLE,
  CONSTRAINT `fk_message_kaiin`
    FOREIGN KEY (`kaiin_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `avatar`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `avatar` ;

CREATE TABLE IF NOT EXISTS `avatar` (
  `kaiin_id` BIGINT NOT NULL,
  `image_url` TEXT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL,
  INDEX `fk_avatar_kaiin1_idx` (`kaiin_id` ASC) VISIBLE,
  PRIMARY KEY (`kaiin_id`),
  CONSTRAINT `fk_avatar_kaiin1`
    FOREIGN KEY (`kaiin_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `badge`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `badge` ;

CREATE TABLE IF NOT EXISTS `badge` (
  `kaiin_id` BIGINT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  INDEX `fk_badge_kaiin1_idx` (`kaiin_id` ASC) VISIBLE,
  PRIMARY KEY (`kaiin_id`),
  CONSTRAINT `fk_badge_kaiin1`
    FOREIGN KEY (`kaiin_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `follower`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `follower` ;

CREATE TABLE IF NOT EXISTS `follower` (
  `kaiin_id` BIGINT NOT NULL,
  `follower_id` BIGINT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`kaiin_id`, `follower_id`),
  INDEX `fk_follower_kaiin2_idx` (`follower_id` ASC) VISIBLE,
  CONSTRAINT `fk_follower_kaiin1`
    FOREIGN KEY (`kaiin_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_follower_kaiin2`
    FOREIGN KEY (`follower_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `iine`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `iine` ;

CREATE TABLE IF NOT EXISTS `iine` (
  `message_id` INT NOT NULL,
  `kaiin_id` BIGINT NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  INDEX `fk_iine_message1_idx` (`message_id` ASC) VISIBLE,
  INDEX `fk_iine_kaiin1_idx` (`kaiin_id` ASC) VISIBLE,
  PRIMARY KEY (`kaiin_id`, `message_id`),
  CONSTRAINT `fk_iine_message1`
    FOREIGN KEY (`message_id`)
    REFERENCES `message` (`message_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_iine_kaiin1`
    FOREIGN KEY (`kaiin_id`)
    REFERENCES `kaiin` (`kaiin_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mongon`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mongon` ;

CREATE TABLE IF NOT EXISTS `mongon` (
  `mongon_id` INT NOT NULL AUTO_INCREMENT,
  `key` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`mongon_id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mongon_en`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mongon_en` ;

CREATE TABLE IF NOT EXISTS `mongon_en` (
  `mongon_id` INT NOT NULL,
  `word` VARCHAR(256) NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`mongon_id`),
  CONSTRAINT `fk_mongon_en_mongon1`
    FOREIGN KEY (`mongon_id`)
    REFERENCES `mongon` (`mongon_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mongon_ja`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mongon_ja` ;

CREATE TABLE IF NOT EXISTS `mongon_ja` (
  `mongon_id` INT NOT NULL,
  `word` VARCHAR(256) NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`mongon_id`),
  CONSTRAINT `fk_mongon_ja_mongon1`
    FOREIGN KEY (`mongon_id`)
    REFERENCES `mongon` (`mongon_id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
