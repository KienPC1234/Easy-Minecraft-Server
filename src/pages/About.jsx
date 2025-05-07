import React from 'react';
import { Container } from 'react-bootstrap';
import { motion } from 'framer-motion';
import './PageStyles.css';

const About = () => {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Container className="minecraft-page">
        <h1>About Easy Minecraft Server</h1>
        <p>
          Easy Minecraft Server is a Tauri-based desktop application designed to
          simplify the setup and management of Minecraft servers. Built with love
          by KienPC.
        </p>
      </Container>
    </motion.div>
  );
};

export default About;