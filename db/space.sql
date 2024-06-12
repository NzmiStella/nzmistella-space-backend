-- MySQL dump 10.13  Distrib 5.7.42, for Linux (x86_64)
--
-- Host: localhost    Database: space
-- ------------------------------------------------------
-- Server version	5.7.42-0ubuntu0.18.04.1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `t_space_article`
--

DROP TABLE IF EXISTS `t_space_article`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `t_space_article` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '文章id',
  `title` varchar(255) NOT NULL DEFAULT '未命名文档' COMMENT '文章标题',
  `description` text NOT NULL COMMENT '文章描述',
  `content` mediumtext NOT NULL COMMENT '文章内容',
  `is_top` tinyint(1) NOT NULL DEFAULT '0' COMMENT '文章是否置顶',
  `status_type` int(11) NOT NULL DEFAULT '0' COMMENT '文章状态，0:草稿、1:已发布、2:隐藏、3:删除',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `create_user_id` int(11) NOT NULL COMMENT '创建用户id',
  `update_user_id` int(11) NOT NULL COMMENT '更新用户id',
  PRIMARY KEY (`id`),
  KEY `update_user_id` (`update_user_id`),
  KEY `i_status_type` (`status_type`),
  KEY `i_create_user_id` (`create_user_id`),
  CONSTRAINT `t_space_article_ibfk_1` FOREIGN KEY (`create_user_id`) REFERENCES `t_space_user` (`id`) ON DELETE CASCADE,
  CONSTRAINT `t_space_article_ibfk_2` FOREIGN KEY (`update_user_id`) REFERENCES `t_space_user` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `t_space_article_tag`
--

DROP TABLE IF EXISTS `t_space_article_tag`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `t_space_article_tag` (
  `article_id` int(11) NOT NULL COMMENT '文章id',
  `tag_id` int(11) NOT NULL COMMENT '标签id',
  PRIMARY KEY (`article_id`,`tag_id`),
  KEY `i_article_id` (`article_id`),
  KEY `i_tag_id` (`tag_id`),
  CONSTRAINT `t_space_article_tag_ibfk_1` FOREIGN KEY (`article_id`) REFERENCES `t_space_article` (`id`) ON DELETE CASCADE,
  CONSTRAINT `t_space_article_tag_ibfk_2` FOREIGN KEY (`tag_id`) REFERENCES `t_space_tag` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章-标签关系表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `t_space_tag`
--

DROP TABLE IF EXISTS `t_space_tag`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `t_space_tag` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '标签id',
  `name` varchar(255) NOT NULL COMMENT '标签名称',
  `parent_tag_id` int(11) DEFAULT NULL COMMENT '父标签id，NULL表示顶级标签',
  `thumbnail_url` varchar(512) DEFAULT NULL COMMENT '标签缩略图url',
  `status_type` int(11) NOT NULL DEFAULT '0' COMMENT '标签状态，0.正常、1.隐藏、2.删除',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `create_user_id` int(11) NOT NULL COMMENT '创建用户id',
  `update_user_id` int(11) NOT NULL COMMENT '更新用户id',
  PRIMARY KEY (`id`),
  KEY `parent_tag_id` (`parent_tag_id`),
  KEY `update_user_id` (`update_user_id`),
  KEY `i_status_type` (`status_type`),
  KEY `i_create_user_id` (`create_user_id`),
  CONSTRAINT `t_space_tag_ibfk_1` FOREIGN KEY (`parent_tag_id`) REFERENCES `t_space_tag` (`id`) ON DELETE CASCADE,
  CONSTRAINT `t_space_tag_ibfk_2` FOREIGN KEY (`create_user_id`) REFERENCES `t_space_user` (`id`) ON DELETE CASCADE,
  CONSTRAINT `t_space_tag_ibfk_3` FOREIGN KEY (`update_user_id`) REFERENCES `t_space_user` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='标签信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `t_space_user`
--

DROP TABLE IF EXISTS `t_space_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `t_space_user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL COMMENT '用户名',
  `nickname` varchar(255) NOT NULL COMMENT '用户昵称',
  `password` varchar(255) NOT NULL COMMENT '用户密码（加密）',
  `email` VARCHAR(255) NOT NULL COMMENT '用户邮箱',
  `avatar_url` varchar(512) DEFAULT NULL COMMENT '用户头像url',
  `signature` varchar(512) NOT NULL DEFAULT '' COMMENT '用户签名',
  `group_type` int(11) NOT NULL DEFAULT '0' COMMENT '用户组，0:普通用户、1:管理员',
  `status_type` int(11) NOT NULL DEFAULT '0' COMMENT '用户状态，0:等待、1:激活、2:禁用、3:删除',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  KEY `i_username` (`username`),
  KEY `i_status_type` (`status_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping routines for database 'space'
--
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-04-11 11:18:01
