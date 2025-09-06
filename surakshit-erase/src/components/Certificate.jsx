import { useState, useEffect } from 'react';
import { PDFDocument, rgb, StandardFonts } from 'pdf-lib';
import QRCode from 'qrcode';

const Certificate = ({ method, fileCount }) => {
  const [qrCodeUrl, setQrCodeUrl] = useState('');
  const [pdfUrl, setPdfUrl] = useState('');

  // Generate certificate data when component mounts
  useEffect(() => {
    const generateCertificate = async () => {
      // 1. Generate QR Code
      const verificationUrl = 'https://example.com/verify/mock-uuid-12345';
      const qrDataUrl = await QRCode.toDataURL(verificationUrl);
      setQrCodeUrl(qrDataUrl);

      // 2. Create PDF with pdf-lib
      const pdfDoc = await PDFDocument.create();
      const page = pdfDoc.addPage();
      const { width, height } = page.getSize();
      const font = await pdfDoc.embedFont(StandardFonts.Helvetica);

      page.drawText('Certificate of Erasure', { x: 50, y: height - 60, font, size: 24 });
      page.drawText(`Timestamp: ${new Date().toLocaleString()}`, { x: 50, y: height - 100 });
      page.drawText(`Method Used: ${method}`, { x: 50, y: height - 120 });
      page.drawText(`Files Erased: ${fileCount}`, { x: 50, y: height - 140 });
      page.drawText(`Verification: ${verificationUrl}`, { x: 50, y: height - 160 });

      // Embed QR code image
      const qrImageBytes = await fetch(qrDataUrl).then(res => res.arrayBuffer());
      const qrImage = await pdfDoc.embedPng(qrImageBytes);
      page.drawImage(qrImage, { x: 50, y: height - 300, width: 120, height: 120 });

      const pdfBytes = await pdfDoc.save();
      const blob = new Blob([pdfBytes], { type: 'application/pdf' });
      setPdfUrl(URL.createObjectURL(blob));
    };

    generateCertificate();
  }, [method, fileCount]);

  return (
    <div className="certificate-container">
      <h3>Erasure Complete!</h3>
      <p>Your certificate has been generated.</p>
      {qrCodeUrl && <img src={qrCodeUrl} alt="Verification QR Code" />}
      {pdfUrl ? (
        <a href={pdfUrl} download="erasure-certificate.pdf" className="download-button">
          Download Certificate PDF
        </a>
      ) : (
        <p>Generating PDF...</p>
      )}
    </div>
  );
};

export default Certificate;