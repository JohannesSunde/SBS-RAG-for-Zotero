from fastapi import APIRouter, HTTPException, Query, Body
from fastapi.responses import FileResponse
from typing import Optional
import os
from backend.deps import get_chatbot, DEFAULT_DB_PATH
from backend.zotero_dbase import ZoteroLibrary
from backend.pdf import PDF
from backend.zoteroitem import ZoteroItem
from backend.external_api_utils import fetch_semantic_scholar_data

router = APIRouter(prefix="/api")

@router.get("/pdf/{zotero_id}")
async def serve_pdf(zotero_id: str):
    """Serve a local PDF file from the Zotero library based on its ID."""
    try:
        # Note: In the refactored version, DB_PATH should be dynamic from active profile
        # For now we'll use a simplified approach or get it from chatbot
        chatbot = get_chatbot()
        zlib = ZoteroLibrary(chatbot.db_path)
        item_data = zlib.get_item_data(zotero_id)
        if not item_data or not item_data.get('pdf_path'):
            raise HTTPException(status_code=404, detail="PDF attachment not found for this item")
        
        pdf_path = item_data['pdf_path']
        if not os.path.exists(pdf_path):
            raise HTTPException(status_code=404, detail=f"PDF file not found at {pdf_path}")
            
        return FileResponse(pdf_path, media_type="application/pdf")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@router.get("/pdfsample")
def pdf_sample(
    filename: str = Query(..., description="Path to PDF file"),
    max_chars: Optional[int] = Query(2000, description="Maximum number of characters to extract"),
):
    """Extracts sample text from a PDF for testing purposes."""
    try:
        pdf = PDF(filepath=filename)
        text = pdf.extract_text(max_chars=max_chars)
        return {"sample": text}
    except Exception as e:
        return {"error": str(e)}

@router.get("/item_metadata")
def get_item_metadata(
    filename: str = Query(..., description="Path to PDF or metadata file"),
):
    """Retrieves metadata from the ZoteroItem class."""
    try:
        item = ZoteroItem(filepath=filename)
        return {
            "title": item.get_title(), 
            "author": item.get_author()
        }
    except Exception as e:
        return {"error": str(e)}

@router.get("/search_items")
def search_items(
    authors: Optional[str] = Query("", description="Comma separated authors"),
    titles: Optional[str] = Query("", description="Comma separated titles"),
    dates: Optional[str] = Query("", description="Comma separated dates")
):
    """Query the Zotero library using authors, titles, and dates."""
    try:
        authors_list = [a.strip() for a in authors.split(",") if a.strip()]
        titles_list = [t.strip() for t in titles.split(",") if t.strip()]
        dates_list = [d.strip() for d in dates.split(",") if d.strip()]
        
        chatbot = get_chatbot()
        zlib = ZoteroLibrary(chatbot.db_path)
        results = zlib.search_parent_items(authors=authors_list, titles=titles_list, dates=dates_list)
        return {"results": list(results)}
    except Exception as e:
        return {"error": str(e)}

@router.post("/external/semantic_scholar")
def lookup_semantic_scholar(payload: dict = Body(...)):
    """Look up paper metadata on Semantic Scholar."""
    try:
        result = fetch_semantic_scholar_data(
            doi=payload.get('doi'),
            title=payload.get('title'),
            authors=payload.get('authors')
        )
        if result:
            return {"success": True, "data": result}
        return {"success": False, "error": "Paper not found on Semantic Scholar"}
    except Exception as e:
        return {"success": False, "error": str(e)}
