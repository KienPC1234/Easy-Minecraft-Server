import React from 'react';
import { Nav } from 'react-bootstrap';
import { NavLink } from 'react-router-dom';
import { motion } from 'framer-motion';

const links = [
  { path: '/', label: 'Home' },
  { path: '/about', label: 'About' },
  { path: '/contact', label: 'Contact' },
];

const Sidebar = () => {
  return (
    <motion.div
      initial={{ x: -200 }}
      animate={{ x: 0 }}
      transition={{ type: 'spring', stiffness: 70 }}
      className="h-100 d-flex flex-column justify-content-start p-3"
    >
      <h4 className="text-white mb-4">Easy Minecraft Server</h4>
      <Nav className="flex-column">
        {links.map(({ path, label }) => (
          <NavLink
            key={path}
            to={path}
            className={({ isActive }) =>
              `nav-link ${isActive ? 'text-warning' : 'text-light'}`
            }
          >
            {label}
          </NavLink>
        ))}
      </Nav>
    </motion.div>
  );
};

export default Sidebar;
