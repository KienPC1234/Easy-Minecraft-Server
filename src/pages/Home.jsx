import React from 'react';
import { Container, Button } from 'react-bootstrap';
import { motion } from 'framer-motion';
import './PageStyles.css';

const Home = () => {
  const buttonVariants = {
    hover: { scale: 1.1, transition: { duration: 0.2 } },
    tap: { scale: 0.9 },
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Container className="minecraft-page">
        <h1>Welcome to Easy Minecraft Server</h1>
        <p>Manage your Minecraft server with ease!</p>
        <motion.div variants={buttonVariants} whileHover="hover" whileTap="tap">
          <Button variant="success" size="lg">
            Get Started
          </Button>
        </motion.div>
      </Container>
    </motion.div>
  );
};

export default Home;