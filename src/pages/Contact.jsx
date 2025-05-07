import React from 'react';
import { Container } from 'react-bootstrap';
import { motion } from 'framer-motion';
import './PageStyles.css';

const Contact = () => {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Container className="minecraft-page">
        <h1>Contact Us</h1>
        <p>
          Reach out to us at{' '}
          <a href="mailto:kienpc872009@gmail.com">kienpc872009@gmail.com</a>.
        </p>
        <p>
          Check out the project on{' '}
          <a
            href="https://github.com/KienPC1234/Easy-Minecraft-Server"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub
          </a>
          .
        </p>
      </Container>
    </motion.div>
  );
};

export default Contact;