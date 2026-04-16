import React from "react";
import ReactMarkdown from "react-markdown";
import type { ChatMessage } from "../../types/domain";
import { Spinner } from "../../components/ui/Spinner";
import { useResponseSelection } from "../../contexts/ResponseSelectionContext";
import ReasoningSection from "../../components/ui/ReasoningSection";
import { usePdf } from "../../contexts/PdfContext";

interface Props {
  messages: ChatMessage[];
  loading: boolean;
}

const ChatMessages: React.FC<Props> = ({ messages, loading }) => {
  const { openPdf } = usePdf();
  const messagesEndRef = React.useRef<HTMLDivElement>(null);
  const [copiedId, setCopiedId] = React.useState<string | null>(null);
  const { selectedResponseId, setSelectedResponseId } = useResponseSelection();

  React.useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, loading]);

  const handleCopy = async (content: string, messageId: string) => {
    try {
      await navigator.clipboard.writeText(content);
      setCopiedId(messageId);
      setTimeout(() => setCopiedId(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  /**
   * Formats message content with clickable APA 7 citations.
   */
  function formatMessageWithCitations(content: string | undefined, msgCitations?: any[]): React.ReactNode {
    if (!content) return <span>No response available</span>;
    
    let processedContent = content;
    // Basic markdown cleanups
    processedContent = processedContent.replace(/\*\*([^\n*]+)\n+\*\*/g, '**$1**');
    processedContent = processedContent.replace(/\*\*\s*\n+\s*([^\n*]+?)\s*\n+\s*\*\*/g, '**$1**');
    
    const renderContentWithCitations = (text: string) => {
      const parts: React.ReactNode[] = [];
      let lastIndex = 0;
      const citationRegex = /\[\[ID:(\d+), Page:(.*?)\]\]/g;
      let match: RegExpExecArray | null;
      
      while ((match = citationRegex.exec(text)) !== null) {
        const id = match[1];
        const page = match[2];
        
        if (match.index > lastIndex) {
          parts.push(text.substring(lastIndex, match.index));
        }
        
        const citeMeta = msgCitations?.find(c => String(c.id) === id);
        const author = citeMeta?.authors?.split(',')[0] || citeMeta?.author?.split(',')[0] || "Unknown";
        const year = citeMeta?.year || "";
        const citationText = `${author}${year ? `, ${year}` : ""}, p. ${page}`;
        
        parts.push(
          <span 
            key={`cite-${match.index}`} 
            className="apa-citation"
            onClick={(e) => {
              e.stopPropagation();
              if (citeMeta && citeMeta.pdf_path) {
                openPdf({
                  zoteroId: citeMeta.zotero_id || citeMeta.id,
                  title: citeMeta.title || "Document",
                  page: parseInt(page) || 1,
                  pdfPath: citeMeta.pdf_path
                });
              }
            }}
            style={{
              cursor: citeMeta?.pdf_path ? 'pointer' : 'default',
              color: citeMeta?.pdf_path ? 'var(--accent-primary, #3b82f6)' : 'inherit',
              fontWeight: 500,
              textDecoration: citeMeta?.pdf_path ? 'underline' : 'none',
              textDecorationStyle: 'dotted'
            }}
          >
            ({citationText})
          </span>
        );
        lastIndex = match.index + match[0].length;
      }
      
      if (lastIndex < text.length) {
        parts.push(text.substring(lastIndex));
      }
      return parts;
    };

    return (
      <ReactMarkdown
        components={{
          p: ({children}) => {
            const processed = React.Children.map(children, (child) => 
              typeof child === 'string' ? renderContentWithCitations(child) : child
            );
            return <p>{processed}</p>;
          },
          li: ({children}) => {
            const processed = React.Children.map(children, (child) => 
              typeof child === 'string' ? renderContentWithCitations(child) : child
            );
            return <li>{processed}</li>;
          },
          strong: ({children}) => {
            const processed = React.Children.map(children, (child) => 
              typeof child === 'string' ? renderContentWithCitations(child) : child
            );
            return <strong>{processed}</strong>;
          },
          a: ({...props}) => <a target="_blank" rel="noopener noreferrer" {...props} />,
        }}
      >
        {processedContent}
      </ReactMarkdown>
    );
  }

  /**
   * Session-wide Bibliography Generator
   */
  const [showBibliography, setShowBibliography] = React.useState(false);

  const renderSessionBibliography = () => {
    // Collect all citations from all assistant messages
    const allCitations = messages
      .filter(m => m.role === 'assistant' && m.citations)
      .flatMap(m => m.citations!);

    if (allCitations.length === 0) return null;

    // Deduplicate citations by zotero_id/id
    const uniqueCitations = Array.from(new Map(allCitations.map(c => [c.zotero_id || c.id, c])).values());

    return (
      <div className="session-bibliography" style={{ 
        margin: '24px 16px', 
        padding: '20px', 
        background: 'var(--bg-secondary, #f9fafb)', 
        borderRadius: '12px',
        border: '1px solid var(--border-color, #e5e7eb)'
      }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
          <h3 style={{ fontSize: '16px', fontWeight: 600, margin: 0 }}>References (APA 7)</h3>
          <button 
            className="btn btn--sm" 
            onClick={() => handleCopy(
              uniqueCitations.map((cite: any, i) => `${cite.authors || "Unknown"}. (${cite.year || "n.d."}). ${cite.title}.`).join('\n'),
              "session-bib"
            )}
          >
            {copiedId === "session-bib" ? "Copied!" : "Copy All"}
          </button>
        </div>
        <ul style={{ listStyle: 'none', padding: 0, margin: 0, fontSize: '13px', lineHeight: '1.6' }}>
          {uniqueCitations.map((cite: any, i) => (
            <li key={i} style={{ marginBottom: '10px', paddingLeft: '24px', textIndent: '-24px' }}>
              <strong>[{i+1}]</strong> {cite.authors || "Unknown Authors"}. ({cite.year || "n.d."}). <em>{cite.title}</em>. 
              {cite.pdf_path && (
                <span 
                  onClick={() => openPdf({
                    zoteroId: cite.zotero_id || cite.id,
                    title: cite.title,
                    page: 1,
                    pdfPath: cite.pdf_path
                  })}
                  style={{ marginLeft: '12px', cursor: 'pointer', color: 'var(--accent-primary)', textDecoration: 'underline', fontSize: '11px' }}
                >
                  View PDF
                </span>
              )}
            </li>
          ))}
        </ul>
      </div>
    );
  };

  return (
    <div className="chat-view__messages">
      <div className="message-list">
        {messages.length === 0 && !loading && (
          <div className="message-list__empty">
            <div style={{ textAlign: "center", padding: "40px 20px", color: "var(--muted)" }}>
              <div style={{ fontSize: "18px", fontWeight: 600, marginBottom: "8px" }}>Welcome to RAG Assistant for Zotero</div>
              <div>Ask questions about your research library to get started.</div>
            </div>
          </div>
        )}
        {messages.map((m) => (
          <div
            key={m.id}
            className={`message message--${m.role === "user" ? "user" : "assistant"}`}
            data-response-id={m.role === "assistant" ? m.id : undefined}
            onClick={() => {
              if (m.role === "assistant") {
                setSelectedResponseId(m.id);
              }
            }}
            style={{
              cursor: m.role === "assistant" ? "pointer" : "default",
              background: m.role === "assistant" && selectedResponseId === m.id ? "var(--bg-selected, #f0f8ff)" : undefined,
              borderLeft: m.role === "assistant" && selectedResponseId === m.id ? "3px solid var(--accent)" : undefined,
              transition: "background 0.2s ease, border-left 0.2s ease",
              padding: '16px',
              marginBottom: '12px',
              borderRadius: '8px'
            }}
          >
            <div className="message__avatar" style={{ marginRight: '12px' }}>
              {m.role === "user" ? (
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z"/></svg>
              ) : (
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/></svg>
              )}
            </div>
            <div className="message__content" style={{ flex: 1 }}>
              <div className="message__role" style={{ fontWeight: 600, marginBottom: '4px', fontSize: '12px', textTransform: 'uppercase', opacity: 0.6 }}>
                {m.role === "user" ? "You" : "Assistant"}
              </div>
              <div className="message__body">{formatMessageWithCitations(m.content, m.citations)}</div>
              
              {m.role === "assistant" && m.reasoning && (
                <ReasoningSection reasoning={m.reasoning} messageId={m.id} />
              )}
              
              {m.role === "assistant" && (
                <div style={{ display: 'flex', gap: '8px', marginTop: '12px' }}>
                  <button
                    className="btn btn--sm"
                    onClick={(e) => { e.stopPropagation(); handleCopy(m.content || "", m.id); }}
                    style={{ padding: "4px 8px", fontSize: "11px" }}
                  >
                    {copiedId === m.id ? "Copied!" : "Copy Answer"}
                  </button>
                </div>
              )}
            </div>
          </div>
        ))}
        
        {loading && (
          <div className="message message--assistant" style={{ padding: '16px' }}>
            <div className="message__body" style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Spinner size="sm" />
              <span style={{ fontSize: '14px', color: 'var(--text-secondary)' }}>Analyzing library...</span>
            </div>
          </div>
        )}

        {/* Bibliography Section */}
        {messages.length > 0 && !loading && (
          <div style={{ textAlign: 'center', margin: '30px 0' }}>
            {!showBibliography ? (
              <button 
                className="btn btn--primary" 
                onClick={() => setShowBibliography(true)}
                style={{ padding: '10px 24px', fontSize: '13px' }}
              >
                Generate Bibliography
              </button>
            ) : (
              renderSessionBibliography()
            )}
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>
    </div>
  );
};

export default ChatMessages;
