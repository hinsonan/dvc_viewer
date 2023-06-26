'use client'
import React from 'react';
import { useEffect } from 'react';
import styles from './coolanimation.module.css';
import { startAnimation } from './animation.js';

const CoolAnimation: React.FC = () => {
  useEffect(() => {
    // Call the startAnimation function when the component mounts
    startAnimation();

    // Clean up the animation when the component unmounts
    return () => {
      // Perform any necessary cleanup tasks
    };
  }, []);
  return (

    <canvas className={styles.container} id='c'></canvas>

  );
};

export default CoolAnimation;
