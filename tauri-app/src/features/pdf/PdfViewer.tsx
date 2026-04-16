import React, { useEffect, useState } from 'react';
import { Viewer, Worker } from '@react-pdf-viewer/core';
import { defaultLayoutPlugin } from '@react-pdf-viewer/default-layout';
import '@react-pdf-viewer/core/lib/styles/index.css';
import '@react-pdf-viewer/default-layout/lib/styles/index.css';
import { usePdf } from '../../contexts/PdfContext';
import '../../styles/pdf-viewer.css';

const PdfViewer: React.FC = () => {
  const { openPdfs, activePdfId, closePdf, setActivePdf, isOpen } = usePdf();
  const defaultLayoutPluginInstance = defaultLayoutPlugin();
  
  const activePdf = openPdfs.find(p => p.zoteroId === activePdfId);

  if (!isOpen || openPdfs.length === 0) {
    return (
      <div className="pdf-viewer-empty">
        <div className="pdf-viewer-empty__content">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M14 2v6h6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M16 13H8M16 17H8M10 9H8" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
          <h3>No PDF Selected</h3>
          <p>Click on a citation in the chat to view the source PDF side-by-side.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="pdf-viewer-container">
      <div className="pdf-viewer-tabs">
        {openPdfs.map((pdf) => (
          <div 
            key={pdf.zoteroId}
            className={`pdf-viewer-tab ${activePdfId === pdf.zoteroId ? 'pdf-viewer-tab--active' : ''}`}
            onClick={() => setActivePdf(pdf.zoteroId)}
          >
            <span className="pdf-viewer-tab__title" title={pdf.title}>{pdf.title}</span>
            <button 
              className="pdf-viewer-tab__close"
              onClick={(e) => {
                e.stopPropagation();
                closePdf(pdf.zoteroId);
              }}
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
            </button>
          </div>
        ))}
      </div>
      
      <div className="pdf-viewer-content">
        {activePdf && (
          <Worker workerUrl={`https://unpkg.com/pdfjs-dist@3.11.174/build/pdf.worker.min.js`}>
            <Viewer
              fileUrl={`http://localhost:8000/api/pdf/${activePdf.zoteroId}`}
              initialPage={activePdf.page - 1} // 0-indexed
              plugins={[defaultLayoutPluginInstance]}
            />
          </Worker>
        )}
      </div>
    </div>
  );
};

export default PdfViewer;
