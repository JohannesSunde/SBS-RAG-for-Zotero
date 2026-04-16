import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';

export interface OpenPdf {
  zoteroId: string;
  title: string;
  page: number;
  pdfPath: string;
}

interface PdfContextType {
  openPdfs: OpenPdf[];
  activePdfId: string | null;
  isOpen: boolean;
  openPdf: (pdf: OpenPdf) => void;
  closePdf: (zoteroId: string) => void;
  setActivePdf: (zoteroId: string) => void;
  setPage: (zoteroId: string, page: number) => void;
  setIsOpen: (isOpen: boolean) => void;
}

const PdfContext = createContext<PdfContextType | undefined>(undefined);

export const PdfProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [openPdfs, setOpenPdfs] = useState<OpenPdf[]>([]);
  const [activePdfId, setActivePdfId] = useState<string | null>(null);
  const [isOpen, setIsOpen] = useState(false);

  const openPdf = useCallback((pdf: OpenPdf) => {
    setOpenPdfs((prev) => {
      const existing = prev.find((p) => p.zoteroId === pdf.zoteroId);
      if (existing) {
        return prev.map((p) => p.zoteroId === pdf.zoteroId ? { ...p, page: pdf.page } : p);
      }
      return [...prev, pdf];
    });
    setActivePdfId(pdf.zoteroId);
    setIsOpen(true);
  }, []);

  const closePdf = useCallback((zoteroId: string) => {
    setOpenPdfs((prev) => {
      const filtered = prev.filter((p) => p.zoteroId !== zoteroId);
      if (activePdfId === zoteroId) {
        setActivePdfId(filtered.length > 0 ? filtered[filtered.length - 1].zoteroId : null);
        if (filtered.length === 0) setIsOpen(false);
      }
      return filtered;
    });
  }, [activePdfId]);

  const setActivePdf = useCallback((zoteroId: string) => {
    setActivePdfId(zoteroId);
    setIsOpen(true);
  }, []);

  const setPage = useCallback((zoteroId: string, page: number) => {
    setOpenPdfs((prev) => prev.map((p) => p.zoteroId === zoteroId ? { ...p, page } : p));
  }, []);

  return (
    <PdfContext.Provider value={{ 
      openPdfs, 
      activePdfId, 
      isOpen, 
      openPdf, 
      closePdf, 
      setActivePdf, 
      setPage, 
      setIsOpen 
    }}>
      {children}
    </PdfContext.Provider>
  );
};

export const usePdf = () => {
  const context = useContext(PdfContext);
  if (!context) {
    throw new Error('usePdf must be used within a PdfProvider');
  }
  return context;
};
