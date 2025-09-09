import { useState, useEffect } from 'react';

const WipeProgress = ({ method, onComplete }) => {
  const [progress, setProgress] = useState(0);
  const [status, setStatus] = useState('Starting wipe...');

  useEffect(() => {
    // Simulate progress with setTimeout
    if (progress < 100) {
      const timeoutId = setTimeout(() => {
        const increment = Math.floor(Math.random() * 10) + 5;
        const newProgress = Math.min(progress + increment, 100);
        
        if (newProgress < 30) setStatus('Erasing visible files...');
        else if (newProgress < 70) setStatus('Purging hidden HPA area...');
        else setStatus('Clearing DCO partition...');
        
        setProgress(newProgress);
      }, 500); // Update every 500ms

      return () => clearTimeout(timeoutId);
    } else {
      setStatus('Wipe complete!');
      // Notify parent component after a short delay
      setTimeout(onComplete, 1000);
    }
  }, [progress, onComplete]);

  return (
    <div className="wipe-progress-container">
      <h3>Wiping Drive using '{method}' Method</h3>
      <div className="progress-bar-container">
        <div className="progress-bar" style={{ width: `${progress}%` }}>
          {progress}%
        </div>
      </div>
      <p>{status}</p>
    </div>
  );
};

export default WipeProgress;