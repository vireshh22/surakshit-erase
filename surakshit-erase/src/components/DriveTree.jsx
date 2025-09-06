import React from 'react';

// A simple component to render a collapsible tree view
const TreeNode = ({ title, children }) => (
  <details open>
    <summary>{title}</summary>
    <div className="tree-node-content">{children}</div>
  </details>
);

const DriveTree = ({ data }) => {
  return (
    <div className="drive-tree-container">
      <h2>Detected Storage Areas</h2>

      {/* Visible Data Area: /mnt/data */}
      <TreeNode title="✅ Visible User Data (/mnt/data)">
        <ul>
          <li>/Documents/sample.txt (100MB)</li>
          <li>/Media/photo1.jpg</li>
        </ul>
      </TreeNode>

      {/* Hidden HPA Region: /mnt/hpa */}
      <TreeNode title="🔒 Hidden HPA Region (/mnt/hpa)">
        <pre>{data.hpa || 'No HPA info detected.'}</pre>
        <ul>
          <li>/HiddenBackup/secret.db (50MB)</li>
        </ul>
      </TreeNode>
      
      {/* Hidden DCO Region: /mnt/dco */}
      <TreeNode title="🔒 Hidden DCO Region (/mnt/dco)">
        <pre>{data.dco || 'No DCO info detected.'}</pre>
        <ul>
          <li>/ManufacturerData/config.bin (20MB)</li>
        </ul>
      </TreeNode>
    </div>
  );
};

export default DriveTree;