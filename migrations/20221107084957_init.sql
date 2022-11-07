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
  `kaiin_id` BIGINT NOT NULL,
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
  `message_id` INT NOT NULL,
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
