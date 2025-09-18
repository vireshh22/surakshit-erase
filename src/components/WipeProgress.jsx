// import { useState, useEffect } from 'react';

// const WipeProgress = ({ method, onComplete }) => {
//   const [progress, setProgress] = useState(0);
//   const [status, setStatus] = useState('Starting wipe...');

//   useEffect(() => {
//     // Simulate progress with setTimeout
//     if (progress < 100) {
//       const timeoutId = setTimeout(() => {
//         const increment = Math.floor(Math.random() * 10) + 5;
//         const newProgress = Math.min(progress + increment, 100);
        
//         if (newProgress < 30) setStatus('Erasing visible files...');
//         else if (newProgress < 70) setStatus('Purging hidden HPA area...');
//         else setStatus('Clearing DCO partition...');
        
//         setProgress(newProgress);
//       }, 500); // Update every 500ms

//       return () => clearTimeout(timeoutId);
//     } else {
//       setStatus('Wipe complete!');
//       // Notify parent component after a short delay
//       setTimeout(onComplete, 1000);
//     }
//   }, [progress, onComplete]);

//   return (
//     <div className="wipe-progress-container">
//       <h3>Wiping Drive using '{method}' Method</h3>
//       <div className="progress-bar-container">
//         <div className="progress-bar" style={{ width: `${progress}%` }}>
//           {progress}%
//         </div>
//       </div>
//       <p>{status}</p>
//     </div>
//   );
// };

// export default WipeProgress;



import { useState, useEffect } from 'react';

const WipeProgress = ({ method, drive, onComplete }) => {
  const [progress, setProgress] = useState(0);
  const [status, setStatus] = useState('Initializing wipe process...');
  const [currentPhase, setCurrentPhase] = useState(1);
  const [totalPhases] = useState(4);

  useEffect(() => {
    // Simulate progress with setTimeout
    if (progress < 100) {
      const timeoutId = setTimeout(() => {
        const increment = Math.floor(Math.random() * 8) + 3;
        const newProgress = Math.min(progress + increment, 100);
        
        // Update status based on progress
        if (newProgress < 15) {
          setStatus(`Preparing secure wipe of /dev/${drive}...`);
          setCurrentPhase(1);
        } else if (newProgress < 40) {
          setStatus('Erasing visible file system data...');
          setCurrentPhase(2);
        } else if (newProgress < 75) {
          setStatus('Purging hidden HPA regions...');
          setCurrentPhase(3);
        } else if (newProgress < 95) {
          setStatus('Clearing DCO manufacturer areas...');
          setCurrentPhase(4);
        } else {
          setStatus('Finalizing secure wipe and generating certificate...');
          setCurrentPhase(4);
        }
        
        setProgress(newProgress);
      }, Math.random() * 800 + 400); // Random delay between 400-1200ms

      return () => clearTimeout(timeoutId);
    } else {
      setStatus('Secure wipe completed successfully!');
      // Notify parent component after a short delay
      setTimeout(onComplete, 1500);
    }
  }, [progress, onComplete, drive]);

  const getPhaseStatus = (phase) => {
    if (phase < currentPhase) return 'completed';
    if (phase === currentPhase) return 'active';
    return 'pending';
  };

  return (
    <div className="wipe-progress-container">
      <h2>Secure Wipe in Progress</h2>
      <div className="wipe-info">
        <p><strong>Drive:</strong> /dev/{drive}</p>
        <p><strong>Method:</strong> {method}</p>
      </div>
      
      <div className="progress-bar-container">
        <div className="progress-bar" style={{ width: `${progress}%` }}>
          {progress}%
        </div>
      </div>
      
      <div className="status-text">
        <p>{status}</p>
      </div>

      <div className="phases-container">
        <h4>Wipe Phases:</h4>
        <div className="phases-list">
          <div className={`phase-item ${getPhaseStatus(1)}`}>
            <span className="phase-number">1</span>
            <span className="phase-text">Initialize & Scan</span>
          </div>
          <div className={`phase-item ${getPhaseStatus(2)}`}>
            <span className="phase-number">2</span>
            <span className="phase-text">Erase Visible Data</span>
          </div>
          <div className={`phase-item ${getPhaseStatus(3)}`}>
            <span className="phase-number">3</span>
            <span className="phase-text">Clear Hidden Areas</span>
          </div>
          <div className={`phase-item ${getPhaseStatus(4)}`}>
            <span className="phase-number">4</span>
            <span className="phase-text">Verify & Finalize</span>
          </div>
        </div>
      </div>

      <div className="warning-notice">
        <p><strong>⚠️ Do not power off or disconnect the drive during this process.</strong></p>
        <p>Interrupting the wipe may leave recoverable data fragments.</p>
      </div>
    </div>
  );
};

export default WipeProgress;