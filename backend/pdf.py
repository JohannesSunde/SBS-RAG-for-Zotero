# pdf.py
import os
import tempfile
import json
from opendataloader_pdf import convert

class PDF:
    """
    Handles PDF file operations for Zotero library items using OpenDataLoader.
    Supports structured text extraction including tables as Markdown and page labels.
    """

    def __init__(self, zotero_item_or_path):
        if hasattr(zotero_item_or_path, "filepath"):
            self.filepath = zotero_item_or_path.filepath
            self.metadata = getattr(zotero_item_or_path, "metadata", {})
            self.zitem = zotero_item_or_path
        else:
            self.filepath = zotero_item_or_path
            self.metadata = {}
            self.zitem = None
        self._pages_data = None

    def _load_data(self):
        """Loads and parses the PDF using OpenDataLoader."""
        if self._pages_data is not None:
            return

        with tempfile.TemporaryDirectory() as temp_dir:
            # Convert PDF to markdown and json (json contains page-level structure)
            convert(
                input_path=self.filepath,
                output_dir=temp_dir,
                format=["markdown", "json"]
            )
            
            # Find the output files
            # OpenDataLoader usually names them after the input file
            base_name = os.path.splitext(os.path.basename(self.filepath))[0]
            md_path = os.path.join(temp_dir, f"{base_name}.md")
            json_path = os.path.join(temp_dir, f"{base_name}.json")
            
            if os.path.exists(json_path):
                with open(json_path, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    # OpenDataLoader JSON output structure usually has 'pages'
                    self._pages_data = data.get('pages', [])
            else:
                self._pages_data = []

    def extract_text(self, max_chars=2000):
        self._load_data()
        text = ""
        for page in self._pages_data:
            page_text = page.get('markdown', '') or page.get('text', '')
            text += page_text + "\n\n"
            if len(text) >= max_chars:
                break
        return text[:max_chars]
    
    def num_pages(self):
        self._load_data()
        return len(self._pages_data)

    def extract_text_by_page(self, page_number):
        """Extract text from the specified page (0-indexed)."""
        self._load_data()
        if 0 <= page_number < len(self._pages_data):
            page = self._pages_data[page_number]
            return page.get('markdown', '') or page.get('text', '')
        return ""
    
    def get_metadata(self):
        if self.zitem and self.zitem.metadata:
            return self.zitem.metadata
        return self.metadata
    
    def extract_all_text(self):
        """Retrieves all the text of the PDF by page as a list of strings."""
        self._load_data()
        return [p.get('markdown', '') or p.get('text', '') for p in self._pages_data]
    
    def extract_text_with_pages(self):
        """Extract text from PDF with page numbers and labels preserved.
        
        Returns:
            List of dicts with 'page_num', 'page_label', and 'text' keys
        """
        self._load_data()
        pages_data = []
        for i, page in enumerate(self._pages_data):
            text = page.get('markdown', '') or page.get('text', '')
            if text.strip():
                pages_data.append({
                    'page_num': i + 1,  # 1-indexed internal page number
                    'page_label': page.get('page_label', str(i + 1)), # Actual page label if available
                    'text': text
                })
        return pages_data
    
    def get_annotations(self):
        # OpenDataLoader might not provide annotations in the same way PyMuPDF does.
        # This is a placeholder as the primary focus is text/tables.
        return []

    @staticmethod
    def extract_text_for_items(items):
        enriched_items = []
        for zitem in items:
            if hasattr(zitem, "filepath"):
                pdf = PDF(zitem)
                text = pdf.extract_text()
                if text:
                    zitem.metadata['text'] = text
                    enriched_items.append(zitem)
        return enriched_items

    








