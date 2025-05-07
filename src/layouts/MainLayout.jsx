import React, { useState } from 'react';
import { Outlet, NavLink } from 'react-router-dom';
import { Navbar, Nav, Container, Offcanvas } from 'react-bootstrap';
import { motion, AnimatePresence } from 'framer-motion';
import 'bootstrap/dist/css/bootstrap.min.css';
import './MainLayout.css';

const MainLayout = () => {
  const [showSidebar, setShowSidebar] = useState(false);

  const toggleSidebar = () => setShowSidebar(!showSidebar);

  const sidebarVariants = {
    open: {
      x: 0,
      transition: { type: 'spring', stiffness: 300, damping: 30 },
    },
    closed: {
      x: '-100%',
      transition: { type: 'spring', stiffness: 300, damping: 30 },
    },
  };

  const pageVariants = {
    initial: { opacity: 0, y: 20 },
    animate: { opacity: 1, y: 0, transition: { duration: 0.5 } },
    exit: { opacity: 0, y: -20, transition: { duration: 0.3 } },
  };

  return (
    <div className="app-container">
      <Navbar bg="dark" variant="dark" expand="lg" className="minecraft-navbar">
        <Container fluid>
          <Navbar.Brand as={NavLink} to="/">
            Easy Minecraft Server
          </Navbar.Brand>
          <Navbar.Toggle onClick={toggleSidebar} />
        </Container>
      </Navbar>

      <div className="main-content">
        <Offcanvas
          show={showSidebar}
          onHide={() => setShowSidebar(false)}
          className="minecraft-sidebar"
        >
          <Offcanvas.Header closeButton>
            <Offcanvas.Title>Menu</Offcanvas.Title>
          </Offcanvas.Header>
          <Offcanvas.Body>
            <motion.div
              variants={sidebarVariants}
              initial="closed"
              animate={showSidebar ? 'open' : 'closed'}
            >
              <Nav className="flex-column">
                <Nav.Link
                  as={NavLink}
                  to="/"
                  className="minecraft-nav-link"
                  onClick={() => setShowSidebar(false)}
                >
                  Home
                </Nav.Link>
                <Nav.Link
                  as={NavLink}
                  to="/about"
                  className="minecraft-nav-link"
                  onClick={() => setShowSidebar(false)}
                >
                  About
                </Nav.Link>
                <Nav.Link
                  as={NavLink}
                  to="/contact"
                  className="minecraft-nav-link"
                  onClick={() => setShowSidebar(false)}
                >
                  Contact
                </Nav.Link>
              </Nav>
            </motion.div>
          </Offcanvas.Body>
        </Offcanvas>

        <motion.div
          className="content"
          variants={pageVariants}
          initial="initial"
          animate="animate"
          exit="exit"
        >
          <Container fluid>
            <AnimatePresence mode="wait">
              <Outlet />
            </AnimatePresence>
          </Container>
        </motion.div>
      </div>
    </div>
  );
};

export default MainLayout;