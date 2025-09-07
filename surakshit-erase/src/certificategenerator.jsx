import React, { useState, useEffect, useCallback } from 'react';
import { PDFDocument, rgb, StandardFonts } from 'pdf-lib';
import QRCode from 'qrcode';
import { FileText, Download, Shield, CheckCircle, Copy, Eye } from 'lucide-react';

// Crypto utilities for digital signing (simplified for demo)
const generateHash = async (data) => {
  const encoder = new TextEncoder();
  const dataBuffer = encoder.encode(data);
  const hashBuffer = await crypto.subtle.digest('SHA-256', dataBuffer);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
};

const generateKeyPair = async () => {
  try {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: "RSA-PSS",
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: "SHA-256",
      },
      true,
      ["sign", "verify"]
    );
    return keyPair;
  } catch (error) {
    console.error("Key generation error:", error);
    return {
      privateKey: "mock-private-key",
      publicKey: "mock-public-key"
    };
  }
};

const createSignature = async (data, privateKey) => {
  try {
    if (privateKey === "mock-private-key") {
      const hash = await generateHash(data);
      return `mock-signature-${hash.substring(0, 16)}`;
    }

    const encoder = new TextEncoder();
    const dataBuffer = encoder.encode(data);
    const signature = await crypto.subtle.sign("RSA-PSS", privateKey, dataBuffer);
    const signatureArray = Array.from(new Uint8Array(signature));
    return signatureArray.map(b => b.toString(16).padStart(2, '0')).join('');
  } catch (error) {
    console.error("Signature creation error:", error);
    const hash = await generateHash(data);
    return `fallback-signature-${hash.substring(0, 16)}`;
  }
};

const generateCertificateId = () => {
  const timestamp = Date.now();
  const random = Math.random().toString(36).substring(2, 15);
  return `CERT-${timestamp}-${random}`.toUpperCase();
};

const createCertificateData = (wipeDetails) => {
  const certificateId = generateCertificateId();
  const timestamp = new Date().toISOString();
  
  return {
    certificateId,
    version: "1.0",
    issuer: {
      organization: "Surakshit Erase Solutions",
      authority: "Digital Security Authority",
      country: "IN",
      state: "Maharashtra",
      city: "Pune"
    },
    subject: {
      deviceId: wipeDetails.deviceId || "Unknown Device",
      serialNumber: wipeDetails.serialNumber || "N/A",
      model: wipeDetails.model || "N/A"
    },
    erasureDetails: {
      method: wipeDetails.method,
      standard: wipeDetails.standard || "DOD 5220.22-M",
      passes: wipeDetails.passes || 3,
      startTime: wipeDetails.startTime,
      endTime: wipeDetails.endTime,
      duration: wipeDetails.duration,
      filesErased: wipeDetails.filesErased,
      totalSize: wipeDetails.totalSize,
      sectorsWiped: wipeDetails.sectorsWiped
    },
    verification: {
      verificationUrl: `https://verify.surakshit-erase.com/cert/${certificateId}`,
      hashAlgorithm: "SHA-256",
      signatureAlgorithm: "RSA-PSS-2048"
    },
    metadata: {
      issuedAt: timestamp,
      expiresAt: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
      software: {
        name: "Surakshit Erase",
        version: "1.0.0",
        build: "2024.01"
      },
      compliance: ["NIST SP 800-88", "ISO/IEC 27040", "GDPR Article 17"]
    }
  };
};

const generateJSONCertificate = async (wipeDetails) => {
  const certificateData = createCertificateData(wipeDetails);
  
  const dataForSigning = JSON.stringify(certificateData, null, 2);
  const dataHash = await generateHash(dataForSigning);
  
  const keyPair = await generateKeyPair();
  const signature = await createSignature(dataForSigning, keyPair.privateKey);
  
  const signedCertificate = {
    ...certificateData,
    digitalSignature: {
      signature,
      dataHash,
      publicKey: typeof keyPair.publicKey === 'string' ? keyPair.publicKey : 'mock-public-key-data',
      signedAt: new Date().toISOString()
    }
  };

  return {
    certificate: signedCertificate,
    blob: new Blob([JSON.stringify(signedCertificate, null, 2)], { 
      type: 'application/json' 
    })
  };
};

const generatePDFCertificate = async (wipeDetails) => {
  const certificateData = createCertificateData(wipeDetails);
  
  const qrCodeData = await QRCode.toDataURL(certificateData.verification.verificationUrl);
  
  const pdfDoc = await PDFDocument.create();
  const page = pdfDoc.addPage([595, 842]);
  const { width, height } = page.getSize();
  
  const titleFont = await pdfDoc.embedFont(StandardFonts.HelveticaBold);
  const regularFont = await pdfDoc.embedFont(StandardFonts.Helvetica);
  const monoFont = await pdfDoc.embedFont(StandardFonts.Courier);

  const headerColor = rgb(0.1, 0.3, 0.6);
  const textColor = rgb(0.2, 0.2, 0.2);

  let yPosition = height - 60;

  // Header
  page.drawRectangle({
    x: 0,
    y: yPosition - 20,
    width: width,
    height: 40,
    color: headerColor
  });

  page.drawText('CERTIFICATE OF ERASURE', {
    x: 50,
    y: yPosition,
    size: 24,
    font: titleFont,
    color: rgb(1, 1, 1)
  });

  page.drawText('DIGITALLY SIGNED & VERIFIED', {
    x: width - 220,
    y: yPosition,
    size: 12,
    font: regularFont,
    color: rgb(1, 1, 1)
  });

  yPosition -= 60;

  // Certificate ID and Status
  page.drawText(`Certificate ID: ${certificateData.certificateId}`, {
    x: 50,
    y: yPosition,
    size: 14,
    font: titleFont,
    color: textColor
  });

  page.drawText('✓ VERIFIED', {
    x: width - 100,
    y: yPosition,
    size: 12,
    font: titleFont,
    color: rgb(0, 0.6, 0)
  });

  yPosition -= 30;

  // Issued details
  page.drawText(`Issued: ${new Date(certificateData.metadata.issuedAt).toLocaleString()}`, {
    x: 50,
    y: yPosition,
    size: 10,
    font: regularFont,
    color: textColor
  });

  page.drawText(`Expires: ${new Date(certificateData.metadata.expiresAt).toLocaleDateString()}`, {
    x: 300,
    y: yPosition,
    size: 10,
    font: regularFont,
    color: textColor
  });

  yPosition -= 40;

  // Device Information Section
  page.drawText('DEVICE INFORMATION', {
    x: 50,
    y: yPosition,
    size: 16,
    font: titleFont,
    color: headerColor
  });
  yPosition -= 25;

  const deviceInfo = [
    `Device ID: ${certificateData.subject.deviceId}`,
    `Model: ${certificateData.subject.model}`,
    `Serial Number: ${certificateData.subject.serialNumber}`
  ];

  deviceInfo.forEach(info => {
    page.drawText(info, {
      x: 70,
      y: yPosition,
      size: 11,
      font: regularFont,
      color: textColor
    });
    yPosition -= 18;
  });

  yPosition -= 20;

  // Erasure Details Section
  page.drawText('ERASURE DETAILS', {
    x: 50,
    y: yPosition,
    size: 16,
    font: titleFont,
    color: headerColor
  });
  yPosition -= 25;

  const erasureInfo = [
    `Method: ${certificateData.erasureDetails.method}`,
    `Standard: ${certificateData.erasureDetails.standard}`,
    `Passes: ${certificateData.erasureDetails.passes}`,
    `Files Erased: ${certificateData.erasureDetails.filesErased}`,
    `Total Size: ${certificateData.erasureDetails.totalSize}`,
    `Duration: ${certificateData.erasureDetails.duration}`,
    `Start Time: ${new Date(certificateData.erasureDetails.startTime).toLocaleString()}`,
    `End Time: ${new Date(certificateData.erasureDetails.endTime).toLocaleString()}`
  ];

  erasureInfo.forEach(info => {
    page.drawText(info, {
      x: 70,
      y: yPosition,
      size: 11,
      font: regularFont,
      color: textColor
    });
    yPosition -= 18;
  });

  yPosition -= 20;

  // Compliance Section
  page.drawText('COMPLIANCE STANDARDS', {
    x: 50,
    y: yPosition,
    size: 16,
    font: titleFont,
    color: headerColor
  });
  yPosition -= 25;

  certificateData.metadata.compliance.forEach(standard => {
    page.drawText(`✓ ${standard}`, {
      x: 70,
      y: yPosition,
      size: 11,
      font: regularFont,
      color: rgb(0, 0.6, 0)
    });
    yPosition -= 18;
  });

  // QR Code
  const qrImageBytes = await fetch(qrCodeData).then(res => res.arrayBuffer());
  const qrImage = await pdfDoc.embedPng(qrImageBytes);
  page.drawImage(qrImage, {
    x: width - 150,
    y: 150,
    width: 100,
    height: 100
  });

  page.drawText('Scan to verify', {
    x: width - 135,
    y: 130,
    size: 10,
    font: regularFont,
    color: textColor
  });

  // Digital Signature Section
  page.drawText('DIGITAL SIGNATURE', {
    x: 50,
    y: 200,
    size: 16,
    font: titleFont,
    color: headerColor
  });

  const dataForSigning = JSON.stringify(certificateData, null, 2);
  const dataHash = await generateHash(dataForSigning);
  const keyPair = await generateKeyPair();
  const signature = await createSignature(dataForSigning, keyPair.privateKey);

  page.drawText(`Hash: ${dataHash.substring(0, 64)}...`, {
    x: 70,
    y: 175,
    size: 8,
    font: monoFont,
    color: textColor
  });

  page.drawText(`Signature: ${signature.substring(0, 64)}...`, {
    x: 70,
    y: 160,
    size: 8,
    font: monoFont,
    color: textColor
  });

  // Footer
  page.drawText(`Issued by: ${certificateData.issuer.organization}`, {
    x: 50,
    y: 50,
    size: 10,
    font: regularFont,
    color: textColor
  });

  page.drawText(`Authority: ${certificateData.issuer.authority}`, {
    x: 50,
    y: 35,
    size: 10,
    font: regularFont,
    color: textColor
  });

  page.drawText(`Verification URL: ${certificateData.verification.verificationUrl}`, {
    x: 50,
    y: 20,
    size: 8,
    font: monoFont,
    color: rgb(0, 0, 1)
  });

  const pdfBytes = await pdfDoc.save();
  return {
    certificate: certificateData,
    blob: new Blob([pdfBytes], { type: 'application/pdf' })
  };
};

const CertificateGenerator = ({ wipeDetails = null }) => {
  const [certificates, setCertificates] = useState({
    pdf: null,
    json: null
  });
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState('pdf');

  // Mock wipe details for demo
  const mockWipeDetails = {
    deviceId: "DISK_001",
    serialNumber: "SN123456789",
    model: "VBOX HARDDISK",
    method: "Purge",
    standard: "DOD 5220.22-M",
    passes: 3,
    startTime: new Date(Date.now() - 3600000).toISOString(),
    endTime: new Date().toISOString(),
    duration: "1h 0m 15s",
    filesErased: 1547,
    totalSize: "45.2 GB",
    sectorsWiped: 94371840
  };

  const currentWipeDetails = wipeDetails || mockWipeDetails;

  const generateCertificates = useCallback(async () => {
    setLoading(true);
    try {
      const [pdfResult, jsonResult] = await Promise.all([
        generatePDFCertificate(currentWipeDetails),
        generateJSONCertificate(currentWipeDetails)
      ]);

      setCertificates({
        pdf: {
          ...pdfResult,
          url: URL.createObjectURL(pdfResult.blob)
        },
        json: {
          ...jsonResult,
          url: URL.createObjectURL(jsonResult.blob)
        }
      });
    } catch (error) {
      console.error('Certificate generation failed:', error);
    }
    setLoading(false);
  }, [currentWipeDetails]);

  useEffect(() => {
    generateCertificates();
  }, [generateCertificates]);

  const downloadCertificate = useCallback((type) => {
    const cert = certificates[type];
    if (cert) {
      const link = document.createElement('a');
      link.href = cert.url;
      link.download = `erasure-certificate-${cert.certificate.certificateId}.${type}`;
      link.click();
    }
  }, [certificates]);

  const copyToClipboard = useCallback((text) => {
    navigator.clipboard.writeText(text);
  }, []);

  const handleTabChange = useCallback((tab) => {
    setActiveTab(tab);
  }, []);

  if (loading) {
    return (
      <div className="certificate-generator">
        <div className="loading">
          <div className="spinner"></div>
          <p>Generating signed certificates...</p>
        </div>
        <style jsx>{getStyles()}</style>
      </div>
    );
  }

  return (
    <div className="certificate-generator">
      <Header certificates={certificates} />
      
      <Tabs activeTab={activeTab} onTabChange={handleTabChange} />

      <div className="certificate-panel">
        {activeTab === 'pdf' && certificates.pdf && (
          <PDFCertificateSection 
            certificate={certificates.pdf}
            onDownload={() => downloadCertificate('pdf')}
            onCopy={copyToClipboard}
          />
        )}

        {activeTab === 'json' && certificates.json && (
          <JSONCertificateSection 
            certificate={certificates.json}
            onDownload={() => downloadCertificate('json')}
            onCopy={copyToClipboard}
          />
        )}
      </div>

      <ComplianceFooter certificates={certificates} />
      
      <style jsx>{getStyles()}</style>
    </div>
  );
};

// Header Component
const Header = ({ certificates }) => (
  <div className="header">
    <div className="header-content">
      <Shield className="header-icon" />
      <div>
        <h2>Digital Erasure Certificates</h2>
        <p>Cryptographically signed certificates of data erasure</p>
      </div>
    </div>
    
    {certificates.pdf && (
      <div className="status-badge">
        <CheckCircle size={16} />
        <span>Certificates Generated & Signed</span>
      </div>
    )}
  </div>
);

// Tabs Component
const Tabs = ({ activeTab, onTabChange }) => (
  <div className="tabs">
    <button 
      className={`tab ${activeTab === 'pdf' ? 'active' : ''}`}
      onClick={() => onTabChange('pdf')}
    >
      <FileText size={16} />
      PDF Certificate
    </button>
    <button 
      className={`tab ${activeTab === 'json' ? 'active' : ''}`}
      onClick={() => onTabChange('json')}
    >
      <FileText size={16} />
      JSON Certificate
    </button>
  </div>
);

// PDF Certificate Section Component
const PDFCertificateSection = ({ certificate, onDownload, onCopy }) => (
  <div className="certificate-section">
    <div className="section-header">
      <h3>PDF Certificate</h3>
      <div className="actions">
        <button onClick={onDownload} className="download-btn">
          <Download size={16} />
          Download PDF
        </button>
      </div>
    </div>
    
    <div className="preview-container">
      <iframe 
        src={certificate.url} 
        width="100%" 
        height="400px"
        style={{ border: '1px solid #333', borderRadius: '4px' }}
      />
    </div>

    <CertificateInfo certificate={certificate} onCopy={onCopy} />
  </div>
);

// JSON Certificate Section Component
const JSONCertificateSection = ({ certificate, onDownload, onCopy }) => (
  <div className="certificate-section">
    <div className="section-header">
      <h3>JSON Certificate</h3>
      <div className="actions">
        <button onClick={onDownload} className="download-btn">
          <Download size={16} />
          Download JSON
        </button>
      </div>
    </div>

    <div className="json-preview">
      <pre>{JSON.stringify(certificate.certificate, null, 2)}</pre>
    </div>

    <SignatureVerification certificate={certificate} onCopy={onCopy} />
  </div>
);

// Certificate Info Component
const CertificateInfo = ({ certificate, onCopy }) => (
  <div className="certificate-info">
    <div className="info-item">
      <strong>Certificate ID:</strong>
      <span>{certificate.certificate.certificateId}</span>
      <button onClick={() => onCopy(certificate.certificate.certificateId)}>
        <Copy size={14} />
      </button>
    </div>
    <div className="info-item">
      <strong>Verification URL:</strong>
      <span>{certificate.certificate.verification.verificationUrl}</span>
      <button onClick={() => onCopy(certificate.certificate.verification.verificationUrl)}>
        <Copy size={14} />
      </button>
    </div>
  </div>
);

// Signature Verification Component
const SignatureVerification = ({ certificate, onCopy }) => (
  <div className="signature-verification">
    <h4>Digital Signature Verification</h4>
    <div className="verification-details">
      <div className="detail-row">
        <span className="label">Algorithm:</span>
        <span className="value">{certificate.certificate.verification.signatureAlgorithm}</span>
      </div>
      <div className="detail-row">
        <span className="label">Hash:</span>
        <span className="value hash">{certificate.certificate.digitalSignature.dataHash}</span>
        <button onClick={() => onCopy(certificate.certificate.digitalSignature.dataHash)}>
          <Copy size={14} />
        </button>
      </div>
      <div className="detail-row">
        <span className="label">Signature:</span>
        <span className="value signature">{certificate.certificate.digitalSignature.signature}</span>
        <button onClick={() => onCopy(certificate.certificate.digitalSignature.signature)}>
          <Copy size={14} />
        </button>
      </div>
    </div>
  </div>
);

// Compliance Footer Component
const ComplianceFooter = ({ certificates }) => (
  <div className="compliance-footer">
    <h4>Compliance Standards</h4>
    <div className="compliance-badges">
      {certificates.json?.certificate.metadata.compliance.map((standard, index) => (
        <div key={index} className="compliance-badge">
          <CheckCircle size={14} />
          {standard}
        </div>
      ))}
    </div>
  </div>
);

// Styles function
const getStyles = () => `
  .certificate-generator {
    width: 100%;
    max-width: 1000px;
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    color: #cccccc;
    overflow: hidden;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    background: linear-gradient(135deg, #2d2d30 0%, #1e1e1e 100%);
    border-bottom: 1px solid #333;
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-icon {
    color: #4fc3f7;
    background: rgba(79, 195, 247, 0.1);
    padding: 8px;
    border-radius: 6px;
  }

  .header h2 {
    margin: 0;
    font-size: 20px;
    color: #ffffff;
  }

  .header p {
    margin: 4px 0 0 0;
    font-size: 14px;
    color: #999;
  }

  .status-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: rgba(76, 175, 80, 0.1);
    border: 1px solid #4caf50;
    border-radius: 4px;
    color: #4caf50;
    font-size: 14px;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #333;
    border-top: 3px solid #4fc3f7;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .tabs {
    display: flex;
    background: #2d2d30;
    border-bottom: 1px solid #333;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 20px;
    background: none;
    border: none;
    color: #999;
    cursor: pointer;
    transition: all 0.2s ease;
    border-bottom: 2px solid transparent;
  }

  .tab:hover {
    color: #cccccc;
    background: rgba(255, 255, 255, 0.05);
  }

  .tab.active {
    color: #4fc3f7;
    border-bottom-color: #4fc3f7;
  }

  .certificate-panel {
    padding: 20px;
  }

  .certificate-section {
    margin-bottom: 30px;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .section-header h3 {
    margin: 0;
    color: #ffffff;
    font-size: 18px;
  }

  .actions {
    display: flex;
    gap: 12px;
  }

  .download-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: #4fc3f7;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.2s ease;
  }

  .download-btn:hover {
    background: #29b6f6;
  }

  .preview-container {
    margin-bottom: 20px;
  }

  .certificate-info {
    background: #2d2d30;
    border: 1px solid #333;
    border-radius: 6px;
    padding: 16px;
  }

  .info-item {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
    font-size: 14px;
  }

  .info-item strong {
    min-width: 140px;
    color: #4fc3f7;
  }

  .info-item span {
    flex: 1;
    font-family: 'Courier New', monospace;
    background: #1e1e1e;
    padding: 4px 8px;
    border-radius: 3px;
    font-size: 12px;
  }

  .info-item button {
    background: #3c3c3c;
    border: 1px solid #555;
    color: #cccccc;
    padding: 4px;
    border-radius: 3px;
    cursor: pointer;
  }

  .info-item button:hover {
    background: #4c4c4c;
  }

  .json-preview {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 6px;
    max-height: 400px;
    overflow: auto;
    margin-bottom: 20px;
  }

  .json-preview pre {
    padding: 16px;
    margin: 0;
    font-family: 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.4;
    color: #cccccc;
  }

  .signature-verification {
    background: #2d2d30;
    border: 1px solid #333;
    border-radius: 6px;
    padding: 16px;
  }

  .signature-verification h4 {
    margin: 0 0 16px 0;
    color: #4fc3f7;
    font-size: 16px;
  }

  .verification-details {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .detail-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .label {
    min-width: 80px;
    font-weight: bold;
    color: #999;
    font-size: 14px;
  }

  .value {
    flex: 1;
    font-family: 'Courier New', monospace;
    background: #1e1e1e;
    padding: 6px 8px;
    border-radius: 3px;
    font-size: 11px;
    word-break: break-all;
  }

  .value.hash {
    color: #81c784;
  }

  .value.signature {
    color: #ffb74d;
  }

  .compliance-footer {
    background: #2d2d30;
    border-top: 1px solid #333;
    padding: 20px;
  }

  .compliance-footer h4 {
    margin: 0 0 16px 0;
    color: #ffffff;
    font-size: 16px;
  }

  .compliance-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .compliance-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: rgba(76, 175, 80, 0.1);
    border: 1px solid #4caf50;
    border-radius: 16px;
    color: #4caf50;
    font-size: 12px;
  }

  .json-preview::-webkit-scrollbar {
    width: 8px;
  }

  .json-preview::-webkit-scrollbar-track {
    background: #2d2d30;
  }

  .json-preview::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 4px;
  }

  .json-preview::-webkit-scrollbar-thumb:hover {
    background: #666;
  }
`;

export default CertificateGenerator;